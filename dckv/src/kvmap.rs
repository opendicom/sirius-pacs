use async_trait::async_trait;
use indexmap::IndexMap;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{Key, Parse, Value};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KVMap {
    map: IndexMap<Vec<u8>, Vec<u8>>,
}

impl KVMap {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get(&self, key: Key) -> Option<&Vec<u8>> {
        self.map.get(key.bytes())
    }

    #[inline]
    pub fn get_key_value(&self, key: Key) -> Option<(&Vec<u8>, &Vec<u8>)> {
        self.map.get_key_value(key.bytes())
    }

    #[inline]
    pub fn remove(&mut self, key: Key) -> Option<Vec<u8>> {
        self.map.shift_remove(key.bytes())
    }

    #[inline]
    pub fn insert(&mut self, key: Key, value: Value) {
        self.map.insert(key.into_bytes(), value.into_bytes());
    }
}

#[async_trait(?Send)]
impl Parse for KVMap {
    #[inline]
    async fn append<R: AsyncReadExt + AsyncSeekExt + Unpin>(
        &mut self,
        reader: &mut R,
        key: Key,
        length: usize,
        _vr: Option<u16>,
    ) {
        let value = Value::read(reader, length).await.unwrap();
        self.insert(key, value);
    }
}
