use crate::*;
use std::collections::HashMap;
use near_sdk::AccountId;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
   #[serde(rename = "accountId")]
   pub(crate) account_id: Option<AccountId>,
   pub(crate) path: Option<String>,
   pub(crate) params: Option<HashMap<String, String>>,
   pub(crate) query: Option<HashMap<String, Vec<String>>>,
   pub(crate) preloads: Option<HashMap<String, Web4Response>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Response {
   #[serde(rename = "contentType")]
   pub(crate) content_type: Option<String>,
   pub(crate) status: Option<u32>,
   pub(crate) body: Option<Vec<u8>>,
   #[serde(rename = "bodyUrl")]
   pub(crate) body_url: Option<String>,
   #[serde(rename = "preloadUrls")]
   pub(crate) preload_urls: Option<Vec<String>>,
}

impl Web4Response {
   // returns vertical & horizontal centered block
   pub fn webpage_template(html: String) -> Self {
      let page = format!("\
      <div style='height: 100%; position: relative;'>\
         <div style='margin: 0; position: absolute; top: 50%; left: 50%; margin-right: -50%; transform: translate(-50%, -50%); text-align: center;'>\
            {}\
         </div>\
      </div>", html);
      Web4Response::html_response(page)
   }

   pub fn html_response(text: String) -> Self {
      Self {
         content_type: Some(String::from("text/html; charset=UTF-8")),
         body: Some(text.into_bytes()),
         ..Default::default()
      }
   }

   pub fn plain_response(text: String) -> Self {
      Self {
         content_type: Some(String::from("text/plain; charset=UTF-8")),
         body: Some(text.into_bytes()),
         ..Default::default()
      }
   }

   pub fn svg_response(text: String) -> Self {
      Self {
         content_type: Some(String::from("image/svg+xml")),
         body: Some(text.into_bytes()),
         ..Default::default()
      }
   }


   pub fn preload_urls(urls: Vec<String>) -> Self {
      Self {
         preload_urls: Some(urls),
         ..Default::default()
      }
   }

   pub fn body_url(url: String) -> Self {
      Self {
         body_url: Some(url),
         ..Default::default()
      }
   }

   pub fn status(status: u32) -> Self {
      Self {
         status: Some(status),
         ..Default::default()
      }
   }
}
