use async_trait::async_trait;
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio::time::Instant;
use tokio::{fs::File, io::AsyncSeekExt};

use dckv::{Key, Parse, Value};

#[derive(Clone)]
struct Shared;

#[async_trait(?Send)]
impl Parse for Shared {
    async fn append<R: AsyncReadExt + AsyncSeekExt + Unpin>(
        &mut self,
        reader: &mut R,
        key: Key,
        length: usize,
        vr: Option<u16>,
    ) {
        let value = Value::read(reader, length).await.unwrap();

        println!(
            "({:04x},{:04x}) {} {} {}",
            key.group(),
            key.element(),
            key.vr(),
            key.level(),
            value.to_string(vr),
        );
    }
}

#[tokio::main]
async fn main() {
    let now = Instant::now();

    let mut buffer = vec![];

    let mut file = File::open("img.dcm").await.unwrap();
    file.read_to_end(&mut buffer).await.unwrap();

    let cursor = Cursor::new(buffer);

    let mut data = Shared;

    data.parse(cursor).await.unwrap();

    println!("{}", now.elapsed().as_micros());
}
