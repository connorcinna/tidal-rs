use reqwest::Client;
use std::error::Error;
use crate::structs::TidalError;

macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

fn sanitize_url(mut url: String) -> String
{
    url = url.replace(' ', "%20");
    url = url.replace('\'', "");
    url = url.replace('\"', "");
    url
}

fn sanitize_filename(mut name: String) -> String
{
    name = name.replace("\"", "");
    name = name.replace(" ", "_");
    name
}

async fn trim_last_char(value: &str) -> &str
{
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

fn generate_filename(query: String, filetype: String) -> String
{
    let date_string = chrono::offset::Local::now();
    let filename = format!("{0}_{1}.{2}", query, date_string, filetype);
    sanitize_filename(filename)
}

fn which_filetype(url: String) -> String
{
    for s in vec!["flac", "mp4"]
    {
        if url.contains(s)
        {
            return s!(s);
        }
    }
    //default to mp4
    s!(".mp4")
}

//TODO when the database gets introduced, maybe check it before downloading to see if we've already
//downloaded this file before
async fn download_file(client: &Client, url: String, dest: String) -> Result<(), Box<dyn Error>>
{
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    match response.error_for_status()
    {
        Ok(res) => 
        {
            let mut file = std::fs::File::create(dest)?;
            let mut content =  std::io::Cursor::new(res.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
            Ok(())
        }
        Err(e) => 
        {
            Err(Box::new(TidalError(e.to_string())))
        }
    }
}
