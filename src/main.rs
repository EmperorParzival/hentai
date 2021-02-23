use hentai::Hentai;
use std::{env, error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("sample.json");

    let result = Hentai::from_json(path);

    println!("{}", result.raw.title.pretty);
    Ok(())
}
