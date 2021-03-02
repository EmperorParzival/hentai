use clap::{clap_app, crate_authors, crate_description, crate_version};
use hentai::{Hentai, Result, Website};
use std::{env, path};

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

    // println!("{:?}", matches.subcommand_matches("download"));
    // let doujin = if let Some(ref matches) = matches.subcommand_matches("download") {
    //     let domain = if matches.is_present("domain") {
    //         Website::XXX
    //     } else {
    //         Website::NET
    //     };

    //     println!("got here");
    //     let _directory = if let Some(target) = matches.value_of("target") {
    //         path::PathBuf::from(target)
    //     } else {
    //         env::current_dir()?
    //     };

    //     println!("here");
    //     if let Some(code) = matches.value_of("code") {
    //         Hentai::new(code.parse::<u32>().unwrap(), domain).await?
    //     } else {
    //         Hentai::random(domain).await?
    //     };
    // };

    // println!("hi {:?}", doujin);

    // let response = Hentai::new(165961, Website::NET).await?;
    // println!("{:?}", response); // makes use of the Debug trait on Hentai

    let response = Hentai::random(Website::NET).await?;
    println!("{:?}", response);

    Ok(())
}
