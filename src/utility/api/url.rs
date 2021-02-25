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

pub fn doujin(id: u32) -> String {
    format!("https://{}/api/gallery/{}", NET, id.to_string())
}

#[derive(Debug)]
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

    pub fn cover(&self, media_id: &String, file_type: &String) -> String {
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

    pub fn cover_thumbnail(&self, media_id: &String, file_type: &String) -> String {
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

    pub fn page(&self, media_id: &String, number: u32, file_type: &String) -> String {
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

    pub fn page_thumbnail(&self, media_id: &String, number: u32, file_type: &String) -> String {
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
