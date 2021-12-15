use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::env;

pub(crate) const NFT_IMAGE_SOURCE: &str = "/images";
pub(crate) const NFT_TOKEN_SOURCE: &str = "/nft";

pub(crate) fn get_nft_target_url(token_id: &TokenId) -> String {
   format!("{}{}/{}.html", get_website_url(), NFT_TOKEN_SOURCE, token_id)
}

pub(crate) fn get_nft_image_url(token_id: &TokenId) -> String {
   format!("{}{}/{}.svg", get_website_url(), NFT_IMAGE_SOURCE, token_id)
}

pub(crate) fn get_website_url() -> String {
   format!("https://{}.page", env::current_account_id())
}

pub(crate) fn make_replacements(mut page: String, replacements: &[String]) -> String {
   for (i, item) in replacements.iter().enumerate()  {
      page = page.replace(&format!("{{{}}}", i.to_string()), item);
   }
   page
}

pub(crate) fn get_token_id(token_id: &TokenId) -> Option<String> {
   let start = token_id.rfind('/').unwrap_or(0);
   let end = token_id.rfind('.').unwrap_or(0);

   if start * end != 0 && start + 1 < end {
      Some(token_id[start+1..end].to_string())
   }
   else {
      None
   }
}
