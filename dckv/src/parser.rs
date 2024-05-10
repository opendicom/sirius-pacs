use async_recursion::async_recursion;
use async_trait::async_trait;
use std::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{vr, DCKVError, Filter, Key, KeyBlocks, Result};

#[inline]
#[async_recursion(?Send)]
async fn parser<'r, S, R>(
    shared: &mut S,
    reader: &mut R,
    key_depth: &mut usize,
    key_blocks: &mut KeyBlocks,
    length: u64,
    filter: &Filter,
) -> Result<()>
where
    S: Clone + Deserializer,
    R: AsyncReadExt + AsyncSeekExt + Unpin,
{
    loop {
        // End if length is bigger than current position.
        if reader.stream_position().await? >= length {
            break;
        }

        // Read group and element. Use the first read to
        // end parsing if the EOF is reached.
        let group = match reader.read_u16_le().await {
            Ok(t) => t as u32,
            Err(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => break,
                _ => return Err(err.into()),
            },
        };
        let element = reader.read_u16_le().await? as u32;
        let tag = element + (group << 16);

        // End parsing if found SQ item delimitation tag.
        if tag == 0xFFFEE00D {
            // skip item length (4 bytes).
            reader.seek(SeekFrom::Current(4)).await?;
            break;
        }

        // Read VR and VL.
        let vr = reader.read_u16().await?;
        let vl = reader.read_u16_le().await?;

        match vr {
            // Short length value VR.
            vr::AE
            | vr::AS
            | vr::AT
            | vr::CS
            | vr::DA
            | vr::DS
            | vr::DT
            | vr::FL
            | vr::FD
            | vr::IS
            | vr::LO
            | vr::LT
            | vr::PN
            | vr::SH
            | vr::SL
            | vr::SS
            | vr::ST
            | vr::TM
            | vr::UI
            | vr::UL
            | vr::US => {
                key_blocks[*key_depth] = ((tag as u64) << 32) + ((vr as u64) << 16);
                let key = Key::from_key_blocks(key_blocks);

                shared.append(reader, key, vl as usize, Some(vr)).await;
            }
            // Long length value VR.
            vr::UC
            | vr::UT
            | vr::UR
            | vr::SV
            | vr::UV
            | vr::OB
            | vr::OD
            | vr::OF
            | vr::OL
            | vr::OV
            | vr::OW
            | vr::UN => {
                key_blocks[*key_depth] = ((tag as u64) << 32) + ((vr as u64) << 16);
                let key = Key::from_key_blocks(key_blocks);
                let vll = reader.read_u32_le().await?;

                shared.append(reader, key, vll as usize, Some(vr)).await;
            }
            // Sequence
            vr::SQ => {
                key_blocks[*key_depth] = (tag as u64) << 32;
                let key = Key::from_key_blocks(key_blocks);

                shared.append(reader, key, 0, Some(vr)).await;

                let seq_length = reader.read_u32_le().await?;

                let seq_offset = if seq_length == u32::MAX {
                    u64::MAX
                } else {
                    reader.stream_position().await? + seq_length as u64
                };

                let mut item_number: u64 = 1;

                loop {
                    if reader.stream_position().await? <= seq_offset {
                        // read item tag
                        let group = reader.read_u16_le().await? as u32;
                        let element = reader.read_u16_le().await? as u32;
                        let item_tag = element + (group << 16);

                        match item_tag {
                            0xFFFEE000 => {
                                key_blocks[*key_depth] = ((tag as u64) << 32) + item_number;

                                *key_depth += 1;
                                key_blocks[*key_depth] = 0x2b2b0000;

                                let key = Key::from_key_blocks(key_blocks);
                                shared.append(reader, key, 0, None).await;

                                let item_length = reader.read_u32_le().await?;

                                if item_length > seq_length {
                                    return Err(DCKVError::InvalidSQItemLength);
                                }

                                let item_offset = if seq_length == u32::MAX {
                                    u64::MAX
                                } else {
                                    reader.stream_position().await? + seq_length as u64
                                };

                                parser(shared, reader, key_depth, key_blocks, item_offset, filter)
                                    .await?;

                                key_blocks[*key_depth] = 0xFFFFFFFF5F5F0000;

                                let key = Key::from_key_blocks(key_blocks);
                                shared.append(reader, key, 0, None).await;

                                key_blocks[*key_depth] = 0x0;
                                *key_depth -= 1;

                                item_number += 1;
                            }
                            0xFFFEE0DD => {
                                key_blocks[*key_depth] &= 0xFFFFFFFF00000000;
                                key_blocks[*key_depth] |= 0x00000000FFFF0000;

                                let key = Key::from_key_blocks(key_blocks);
                                shared.append(reader, key, 0, None).await;

                                // skip item length (4 bytes).
                                reader.seek(SeekFrom::Current(4)).await?;

                                break;
                            }
                            _ => return Err(DCKVError::InvalidSQTag),
                        }
                    } else {
                        key_blocks[*key_depth] &= 0xFFFFFFFF00000000;
                        key_blocks[*key_depth] |= 0x00000000FFFF0000;

                        let key = Key::from_key_blocks(key_blocks);
                        shared.append(reader, key, 0, None).await;

                        break;
                    }
                }
            }
            _ => {
                let bytes = vr.to_le_bytes().to_vec();
                let vr_text = String::from_utf8(bytes).map_err(|_| DCKVError::InvalidVR)?;
                return Err(DCKVError::UnsupportedVR(vr_text));
            }
        }
    }

    Ok(())
}

#[async_trait(?Send)]
pub trait Deserializer
where
    Self: Clone,
{
    #[inline]
    async fn deserialize<R>(&mut self, mut reader: R, filter: &Filter) -> Result<()>
    where
        R: AsyncReadExt + AsyncSeekExt + Unpin,
    {
        let mut key_depth = 0;
        let mut key_blocks = KeyBlocks::default();

        // skip preamble
        reader.seek(SeekFrom::Start(144)).await?;

        parser(
            self,
            &mut reader,
            &mut key_depth,
            &mut key_blocks,
            u64::MAX,
            filter,
        )
        .await?;

        Ok(())
    }

    async fn append<R>(&mut self, reader: &mut R, key: Key, length: usize, vr: Option<u16>)
    where
        R: AsyncReadExt + AsyncSeekExt + Unpin;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    bytes: Vec<u8>,
}

impl Value {
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[inline]
    pub async fn read<R>(reader: &mut R, offset: usize) -> Result<Self>
    where
        R: AsyncReadExt + Unpin,
    {
        let mut bytes = vec![0; offset];
        reader.read_exact(&mut bytes).await?;

        Ok(Self { bytes })
    }

    #[inline]
    pub async fn seek<R>(reader: &mut R, offset: u64) -> Result<()>
    where
        R: AsyncSeekExt + Unpin,
    {
        reader.seek(SeekFrom::Start(offset)).await?;

        Ok(())
    }

    #[inline]
    pub fn to_string(&self, vr: Option<u16>) -> String {
        match vr {
            Some(vr::OB) | Some(vr::OD) | Some(vr::OF) | Some(vr::OL) | Some(vr::OV)
            | Some(vr::OW) | Some(vr::SV) | Some(vr::UC) | Some(vr::UR) | Some(vr::UT)
            | Some(vr::UV) | Some(vr::UN) | Some(vr::FD) => String::new(),
            Some(_) => String::from_utf8_lossy(&self.bytes).to_string(),
            None => String::new(),
        }
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
