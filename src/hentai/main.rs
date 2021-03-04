// use clap::{clap_app, crate_authors, crate_description, crate_version};
use hentai::{Hentai, Result, Website};

#[tokio::main]
async fn main() -> Result<()> {
    // let matches = clap_app!(hentai =>
    //     (author: crate_authors!())
    //     (version: crate_version!())
    //     (about: crate_description!())

    //     (@arg domain: -d --domain "Use nhentai.xxx for image URLs instead of nhentai.net")
    //     (@subcommand download =>
    //         (about: "Download the image URLs of a doujin to the specified directory")

    //         (@arg code: -c --code [CODE] +takes_value "nhentai's code for the doujin")
    //         (@arg target: -t --target <DIRECTORY> +takes_value "Directory images will be downloaded to")
    //     )
    // )
    // .get_matches();

    let response = Hentai::random(Website::XXX).await?;
    println!("{:?}", response);

    // let mut path = std::env::current_exe()?;
    // path.pop();
    // path.push("sample.json");

    // let response = Hentai::from_json(path, Website::XXX)?;
    // println!("{:?}", response);

    Ok(())
}
