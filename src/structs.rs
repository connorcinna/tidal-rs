use std::fmt;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct TidalError(pub String);

impl fmt::Display for TidalError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}", self.0)
    }
}

impl Error for TidalError {}

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
    pub search_type: SearchType,
    pub query: String,
    pub country_code: String,
    pub array: Option<Vec<String>>,
    pub page: Option<String>,
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
pub struct DeviceCodeResponse
{
    pub device_code: String,
    pub user_code: String, 
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u32,
    pub interval: u32
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
pub struct User
{
    pub user_id: Option<u64>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postalcode: Option<String>,
    pub us_state: Option<String>,
    pub phone_number: Option<String>,
    pub birthday: Option<String>,
    pub channel_id: Option<u64>,
    pub parent_id: Option<u64>,
    pub accepted_EULA: bool,
    pub created: Option<u64>,
    pub updated: Option<u64>,
    pub facebook_uid: Option<u64>,
    pub apple_uid: Option<u64>,
    pub google_uid: Option<u64>,
    pub account_link_created: bool,
    pub email_verified: bool,
    pub new_user: bool
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
pub struct DlBasicAuthResponse
{
    pub scope: String,
    pub user: User,
    pub clientName: String,
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u32,
    pub user_id: u64,
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
    pub id: String,
    #[serde(rename = "type")] 
    pub _type: String
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
    pub data: Vec<Media>,
    pub links: Links
}
