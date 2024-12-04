use reqwest;
extern crate dotenv;
extern crate serde_json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use std::fmt;
use std::collections::HashMap;


//TODO: move all these structs to their own file
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
struct User
{
    user_id: String,
    email: String,
    country_code: String,
    full_name: String,
    first_name: String,
    last_name: String,
    nickname: String,
    username: String,
    address: String,
    city: String,
    postalcode: String,
    us_state: String,
    phone_number: String,
    birthday: String,
    channel_id: u32,
    parent_id: u32,
    accepted_eula: bool,
    created: u32,
    updated: u32,
    facebook_uid: u32,
    apple_uid: u32,
    google_uid: u32,
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
            user_id: String::new(),
            email: String::new(),
            country_code: String::new(),
            full_name: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            nickname: String::new(),
            username: String::new(),
            address: String::new(),
            city: String::new(),
            postalcode: String::new(),
            us_state: String::new(),
            phone_number: String::new(),
            birthday: String::new(),
            channel_id: 0,
            parent_id: 0,
            accepted_eula: false,
            created: 0,
            updated: 0,
            facebook_uid: 0,
            apple_uid: 0,
            google_uid: 0,
            account_link_created: false,
            email_verified: false,
            new_user: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
struct DlBasicAuthResponse
{
    scope: String,
    user: User,
    client_name: String,
    token_type: String,
    access_token: String,
    expires_in: String,
    user_id: u32
}

impl Default for DlBasicAuthResponse
{
    fn default() -> Self
    {
        DlBasicAuthResponse
        {
            scope: String::new(),
            user: User::default(),
            client_name: String::new(),
            token_type: String::new(),
            access_token: String::new(),
            expires_in: String::new(),
            user_id: 0,
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

//oauth2 login 
async fn dl_login_web(client: &reqwest::Client) -> DlBasicAuthResponse
{
    let response = device_auth(&client).await;
    println!("Go to the following link in your browser to authenticate, then press any button to continue -- {0}", response.verification_uri_complete);
    let _ = std::io::stdin().read_line(&mut String::new());
    dl_basic_auth(&client, response).await
}

//check if we are authenticated already, or if it expired
async fn dl_check_auth(client: &reqwest::Client) -> bool
{
    //TODO
    todo!()
}

//general GET function for the unofficial API
//TODO: make this part of a class so we can keep track of the auth status?
async fn dl_get(client: &reqwest::Client, endpoint: String) -> String
{
    let url = format!("https://api.tidalhifi.com/v1/{0}", endpoint);
    let mut auth: DlBasicAuthResponse = DlBasicAuthResponse::default();
    if !dl_check_auth(&client).await
    {
        auth = dl_login_web(&client).await;
    }
    match client
        .get(url)
        .header(reqwest::header::AUTHORIZATION, auth.access_token)
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

async fn device_auth(client: &reqwest::Client) -> DeviceCodeResponse
{
    dotenv().ok();
    let dl_client_id = env::var("DL_CLIENT_ID").expect("Did not find DL_CLIENT_ID in environment. Make sure to have a .env file defining CLIENT_ID");
    let endpoint = String::from("https://auth.tidal.com/v1/oauth2/device_authorization");

    let mut body = HashMap::new();
    body.insert("client_id", dl_client_id.as_str());
    body.insert("scope",  "r_usr+w_usr+w_sub");
    match client
        .post(endpoint)
        .header(reqwest::header::ACCEPT, "application/json")
        .form(&body)
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
    let endpoint = String::from("https://auth.tidal.com/v1/oauth2/token");
    let mut body = HashMap::new();
    body.insert("client_id", dl_client_id.as_str());
    body.insert("device_code", device_code_response.device_code.as_str());
    body.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
    body.insert("scope",  "r_usr+w_usr+w_sub");

    match client
        .post(endpoint)
        .basic_auth(&dl_client_id, Some(dl_client_secret))
        .form(&body)
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
//        println!("{result}");
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
//        println!("{space_result}");
        assert!(!space_result.contains("ERROR"));
        let response = device_auth(&client).await;
        println!("{:?}", response);
    }
}
