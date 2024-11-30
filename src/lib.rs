use reqwest;
extern crate dotenv;
extern crate serde_json;
use serde_json::Value;
use dotenv::dotenv;
use std::env;
use std::fmt;
use std::collections::HashMap;

//for efficiency reasons, this API will expect a reqwest client passed in so that it can be reused

#[derive(Debug)]
pub enum SearchType 
{
    Album,
    Artist,
    Playlist,
    TopHits,
    Track,
    Video,
}

pub struct Search 
{
    search_type: SearchType,
    query: String,
    country_code: String,
    array: Option<Vec<String>>,
    page: Option<String>,
}

impl fmt::Display for SearchType
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result 
    {
        match &self
        {
            SearchType::Album => write!(fmt, "albums"),
            SearchType::Artist => write!(fmt, "artists"),
            SearchType::Playlist => write!(fmt, "playlists"),
            SearchType::TopHits => write!(fmt, "topHits"),
            SearchType::Track => write!(fmt, "tracks"),
            SearchType::Video => write!(fmt, "videos"),
        }
    }
}

//handles all GET requests under the search results endpoint
pub async fn search_get(client: &reqwest::Client, search: Search) -> String
{
    let bearer_token = basic_auth(&client).await;
    let mut endpoint =  format!("https://openapi.tidal.com/v2/searchresults/{0}/relationships/{1}?countryCode={2}", search.query, search.search_type.to_string(), search.country_code);
    match search.array
    {
        Some(arr) =>
        {
            for s in arr
            {
                {
                    endpoint.push_str("&include=");
                    endpoint.push_str(s.as_str());
                }
            }
        }
        None => {}
    }
    match search.page
    {
        Some(p) =>
        {
            endpoint.push_str(format!("page%5Bcursor%5D={0}", p).as_str()); 
        }
        None => {}
    }
    endpoint = sanitize_url(endpoint);
//    println!("{0}", endpoint);
    match client
        .get(endpoint)
        .header(reqwest::header::ACCEPT, "application/vnd.api+json")
        .header(reqwest::header::AUTHORIZATION, bearer_token)
        .send()
        .await
        {
            Ok(resp) => 
            {
                resp.text().await.unwrap()
            }
            Err(e) => 
            {
                e.to_string()
            }
        }
}

async fn basic_auth(client: &reqwest::Client) -> String
{
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("Did not find CLIENT_ID in environment. Make sure to have a .env file defining your bearer token CLIENT_ID");
    let client_secret = env::var("CLIENT_SECRET").expect("Did not find CLIENT_SECRET in environment. Make sure to have a .env file defining your bearer token CLIENT_SECRET");
    let endpoint = String::from("https://auth.tidal.com/v1/oauth2/token");
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    match client
        .post(endpoint)
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await
        {
            Ok(resp) => 
            {
                let out = resp.text().await.unwrap();
                let json: Value = serde_json::from_str(&out).unwrap();
                let token = json.get("access_token").unwrap().to_string().replace("\"", "");
                format!("Bearer {0}", token)
            }
            Err(e) => 
            {
                e.to_string()
            }
        }
}

fn sanitize_url(url: String) -> String
{
    url.replace(' ', "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client : reqwest::Client = reqwest::Client::new();
        let search = Search 
        {
            search_type: SearchType::Track,
            query: String::from("radiohead"),
            country_code: String::from("US"),
            array: None,
            page: None,
        };
        let result = search_get(&client, search).await;
        println!("{result}");
        assert!(!result.contains("ERROR"));
        let space_search = Search
        {
            search_type: SearchType::Track,
            query: String::from("this is a query with a string"),
            country_code: String::from("US"),
            array: None,
            page: None,
        };
        let space_result = search_get(&client, space_search).await;
        println!("{space_result}");
        assert!(!space_result.contains("ERROR"));
    }
}
