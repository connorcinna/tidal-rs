#![allow(dead_code)]
use reqwest;
extern crate dotenv;
extern crate serde_json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;

macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

//TODO: move all these structs to their own file
//TODO: move all auth stuff into its own file
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

#[allow(non_snake_case)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
struct DeviceCodeResponse
{
    device_code: String,
    user_code: String, 
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u32,
    interval: u32
}

impl Default for DeviceCodeResponse
{
    fn default() -> Self 
    {
        DeviceCodeResponse 
        { 
            device_code: String::new(),
            user_code: String::new(),
            verification_uri: String::new(),
            verification_uri_complete: String::new(),
            expires_in: 0,
            interval: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[allow(non_snake_case)]
struct User
{
    user_id: Option<u64>,
    email: Option<String>,
    country_code: Option<String>,
    full_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    nickname: Option<String>,
    username: Option<String>,
    address: Option<String>,
    city: Option<String>,
    postalcode: Option<String>,
    us_state: Option<String>,
    phone_number: Option<String>,
    birthday: Option<String>,
    channel_id: Option<u64>,
    parent_id: Option<u64>,
    accepted_EULA: bool,
    created: Option<u64>,
    updated: Option<u64>,
    facebook_uid: Option<u64>,
    apple_uid: Option<u64>,
    google_uid: Option<u64>,
    account_link_created: bool,
    email_verified: bool,
    new_user: bool
}

impl Default for User
{
    fn default() -> Self
    {
        User
        {
            user_id: Some(0),
            email: Some(String::new()),
            country_code: Some(String::new()),
            full_name: Some(String::new()),
            first_name: Some(String::new()),
            last_name: Some(String::new()),
            nickname: Some(String::new()),
            username: Some(String::new()),
            address: Some(String::new()),
            city: Some(String::new()),
            postalcode: Some(String::new()),
            us_state: Some(String::new()),
            phone_number: Some(String::new()),
            birthday: Some(String::new()),
            channel_id: Some(0),
            parent_id: Some(0),
            accepted_EULA: false,
            created: Some(0),
            updated: Some(0),
            facebook_uid: Some(0),
            apple_uid: Some(0),
            google_uid: Some(0),
            account_link_created: false,
            email_verified: false,
            new_user: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DlBasicAuthResponse
{
    scope: String,
    user: User,
    clientName: String,
    token_type: String,
    access_token: String,
    expires_in: u32,
    user_id: u64,
}

impl Default for DlBasicAuthResponse
{
    fn default() -> Self
    {
        DlBasicAuthResponse
        {
            scope: String::new(),
            user: User::default(),
            clientName: String::new(),
            token_type: String::new(),
            access_token: String::new(),
            expires_in: 0,
            user_id: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media
{
    id: String,
    #[serde(rename = "type")] 
    _type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links
{
    #[serde(rename = "self")] 
    _self: String,
    next: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse
{
    data: Vec<Media>,
    links: Links
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
    match client
        .get(endpoint)
        .header(reqwest::header::ACCEPT, "application/vnd.api+json")
        .header(reqwest::header::AUTHORIZATION, bearer_token.clone())
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

pub async fn search_get_track(client: &reqwest::Client, query: String) -> Vec<String>
{
    let search = Search 
    {
        search_type: SearchType::Track,
        query,
        country_code: s!("US"),
        array: None,
        page: None,
    };
    let get = search_get(&client, search).await;
    let arr: SearchResponse = serde_json::from_str(get.as_str()).unwrap();
    return arr
        .data
        .iter()
        .map(|m| m.id.clone())
        .collect()
}

//oauth2 login 
async fn dl_login_web(client: &reqwest::Client) -> DlBasicAuthResponse
{
    let response = device_auth(&client).await;
    println!("Go to the following link in your browser to authenticate, then press any button to continue -- {0}", response.verification_uri_complete);
    let _ = std::io::stdin().read_line(&mut String::new());
    let auth_response = dl_basic_auth(&client, response).await;
    auth_response
}

//check if we are authenticated already, or if it expired
async fn dl_check_auth(client: &reqwest::Client, auth: &DlBasicAuthResponse) -> bool
{
    let url = "https://api.tidal.com/v1/sessions";
    match client
        .get(url)
        .bearer_auth(auth.access_token.clone())
        .send()
        .await
        {
            Ok(response) => 
            {
                let ret = response.status() == reqwest::StatusCode::OK;
                return ret;
            }
            Err(e) => 
            {
                eprintln!("{e}");
                false
            }
        }
}

//general GET function for the unofficial API
//TODO return result instead of String
async fn dl_get(client: &reqwest::Client, endpoint: String, params: &mut HashMap<String, String>, auth: &mut DlBasicAuthResponse) -> String
{
    if !dl_check_auth(&client, &auth).await
    {
        *auth = dl_login_web(&client).await;
    }
    if let Some(country_code) = &auth.user.country_code 
    {
        params.insert(s!("countryCode"), country_code.to_owned());
    }
    else
    {
        return s!("No countryCode found in dl_get");
    }
    let url = reqwest::Url::parse_with_params(format!("https://api.tidalhifi.com/v1/{0}", endpoint).as_str(), params.clone()).expect("Unable to parse URL");
    match client
        .get(url)
        .bearer_auth(auth.access_token.clone())
        .send()
        .await
        {
            Ok(response) => 
            {
                response
                    .text()
                    .await
                    .unwrap()
            }
            Err(e) =>
            {
                eprintln!("{e}");
                String::new()
            }
        }
}

async fn dl_get_track(client: &reqwest::Client, query: String, auth: &mut DlBasicAuthResponse) -> String
{
//    let endpoint = format!("tracks/{0}/playbackinfopostpaywall", query);
    let endpoint = format!("tracks/{0}", query);
    let mut params = HashMap::new();
    params.insert(s!("audioquality"), s!("LOSSLESS"));
    params.insert(s!("playbackmode"), s!("STREAM"));
    params.insert(s!("assetpresentation"), s!("FULL"));
    let res = dl_get(&client, endpoint, &mut params, auth).await;
    res
}

async fn device_auth(client: &reqwest::Client) -> DeviceCodeResponse
{
    dotenv().ok();
    let dl_client_id = env::var("DL_CLIENT_ID").expect("Did not find DL_CLIENT_ID in environment. Make sure to have a .env file defining CLIENT_ID");
    let endpoint = s!("https://auth.tidal.com/v1/oauth2/device_authorization");

    let mut form = HashMap::new();
    form.insert("client_id", dl_client_id.as_str());
    form.insert("scope",  "r_usr+w_usr+w_sub");
    match client
        .post(endpoint)
        .header(reqwest::header::ACCEPT, "application/json")
        .form(&form)
        .send()
        .await
        {
            Ok(response) =>
            {
                let resp_text: &str = &response
                    .text()
                    .await
                    .unwrap();
                serde_json::from_str(&resp_text).expect("Unable to deserialize response from device_authorization endpoint")
            }
            Err(e) => 
            {
                println!("ERROR : {:?}", e);
                DeviceCodeResponse::default()
            }
        }

}

async fn dl_basic_auth(client: &reqwest::Client, device_code_response: DeviceCodeResponse) -> DlBasicAuthResponse
{
    dotenv().ok();
    let dl_client_id = env::var("DL_CLIENT_ID").expect("Did not find DL_CLIENT_ID in environment. Make sure to have a .env file defining CLIENT_ID");
    let dl_client_secret = env::var("DL_CLIENT_SECRET").expect("Did not find DL_CLIENT_SECRET in environment. Make sure to have a .env file defining DL_CLIENT_SECRET");
    let endpoint = s!("https://auth.tidal.com/v1/oauth2/token");
    let mut form = HashMap::new();
    form.insert("client_id", dl_client_id.as_str());
    form.insert("device_code", device_code_response.device_code.as_str());
    form.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
    form.insert("scope",  "r_usr+w_usr+w_sub");

    match client
        .post(endpoint)
        .basic_auth(&dl_client_id, Some(dl_client_secret))
        .form(&form)
        .send()
        .await
        {
            Ok(response) =>
            {
                let resp_text: &str = &response
                    .text()
                    .await
                    .unwrap();
                serde_json::from_str(&resp_text).expect("Unable to deserialize response from device_authorization endpoint") 
            }
            Err(e) =>
            {
                eprintln!("{0}", e);
                DlBasicAuthResponse::default()
            }
        }
}

async fn basic_auth(client: &reqwest::Client) -> String
{
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("Did not find CLIENT_ID in environment. Make sure to have a .env file defining CLIENT_ID");
    let client_secret = env::var("CLIENT_SECRET").expect("Did not find CLIENT_SECRET in environment. Make sure to have a .env file defining CLIENT_SECRET");
    let endpoint = s!("https://auth.tidal.com/v1/oauth2/token");
    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");
    match client
        .post(endpoint)
        .basic_auth(client_id, Some(client_secret))
        .form(&form)
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
    async fn it_works() 
    {
        let client : reqwest::Client = reqwest::Client::builder()
            .http1_title_case_headers()
            .build()
            .expect("Unable to build reqwest client");
        let mut auth = DlBasicAuthResponse::default();
        let track_search = search_get_track(&client, s!("pablo honey")).await;
        let pablo_honey = dl_get_track(&client, track_search[0].clone(), &mut auth).await;
        println!("pablo_honey: {0}", pablo_honey);
    }
}
