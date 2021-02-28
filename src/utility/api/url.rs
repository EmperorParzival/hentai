const NET: &str = "nhentai.net";
const XXX: &str = "nhentai.xxx";

fn get_type(file_type: &str) -> &str {
    if file_type == "j" {
        "jpg"
    } else if file_type == "p" {
        "png"
    } else {
        "gif"
    }
}

/// In cases where the standard output domain [nhentai.net](https://nhentai.net) may be restricted
/// on the end user's network, an option can be provided to use [nhentai.xxx](https://nhentai.xxx)
/// URLs instead. Note that the `Hentai::new()` constructor will not work if the standard output
/// domain is blocked. This is because [nhentai.xxx](https://nhentai.xxx) does not reimplement
/// nhentai's API and requests must still be made to the original URL.
///
/// However, if the JSON is provided, one may generate either type of URL regardless of whether or
/// not the original domain is blocked. This feature's primary purpose is to retrieve information
/// for the doujin under an unrestricted network, and create alternative URLs for the user with the
/// restricted network.
///
/// One of these options must be provided in the constructor.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Website {
    NET,
    XXX,
}

pub struct Make {
    pub mode: Website,
}

impl Make {
    pub fn new(mode: Website) -> Make {
        Make { mode }
    }

    pub fn doujin_url(&self, id: u32) -> String {
        match self.mode {
            Website::NET => format!("https://{}/g/{}", NET, id.to_string()),
            Website::XXX => format!("https://{}/g/{}", XXX, id.to_string()),
        }
    }

    pub fn cover(&self, media_id: &str, file_type: &str) -> String {
        match self.mode {
            Website::NET => format!(
                "https://t.{}/galleries/{}/cover.{}",
                NET,
                media_id,
                get_type(file_type)
            ),
            Website::XXX => format!(
                "https://cdn.{}/g/{}/cover.{}",
                XXX,
                media_id,
                get_type(file_type)
            ),
        }
    }

    pub fn cover_thumbnail(&self, media_id: &str, file_type: &str) -> String {
        match self.mode {
            Website::NET => format!(
                "https://t.{}/galleries/{}/thumb.{}",
                NET,
                media_id,
                get_type(file_type)
            ),
            Website::XXX => format!(
                "https://cdn.{}/g/{}/thumb.{}",
                XXX,
                media_id,
                get_type(file_type)
            ),
        }
    }

    pub fn page(&self, media_id: &str, number: u32, file_type: &str) -> String {
        match self.mode {
            Website::NET => format!(
                "https://i.{}/galleries/{}/{}.{}",
                NET,
                media_id,
                number.to_string(),
                get_type(file_type)
            ),
            Website::XXX => format!(
                "https://cdn.{}/g/{}/{}.{}",
                XXX,
                media_id,
                number.to_string(),
                get_type(file_type)
            ),
        }
    }

    #[allow(dead_code)]
    pub fn page_thumbnail(&self, media_id: &str, number: u32, file_type: &str) -> String {
        match self.mode {
            Website::NET => format!(
                "https://t.{}/galleries/{}/{}t.{}",
                NET,
                media_id,
                number.to_string(),
                get_type(file_type)
            ),
            Website::XXX => format!(
                "https://cdn.{}/g/{}/{}t.{}",
                XXX,
                media_id,
                number.to_string(),
                get_type(file_type)
            ),
        }
    }
}

pub fn doujin(id: u32) -> String {
    format!("https://{}/api/gallery/{}", NET, id.to_string())
}

pub fn random(mode: &Website) -> String {
    format!(
        "https://{}/random",
        match mode {
            Website::NET => NET,
            Website::XXX => XXX,
        }
    )
}
