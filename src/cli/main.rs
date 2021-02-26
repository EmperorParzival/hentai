use hentai::{Hentai, Result, Website};

#[tokio::main]
async fn main() -> Result<()> {
    let response = Hentai::new(165961, Website::NET).await?; 
    println!("{:?}", response); // makes use of the Debug trait on Hentai
    
    Ok(())
}