use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{NFT_METADATA_SPEC, NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata};
use near_sdk::{AccountId, BorshStorageKey, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};

use crate::web4::*;
use crate::helper::*;
use crate::qr::*;

mod web4;
mod helper;
mod qr;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NfTicket {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl NfTicket {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "NFTicket".to_string(),
                symbol: "NFTkt".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    //n/ any user has access to mint
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        mut token_metadata: TokenMetadata,
    ) -> Token {
        token_metadata.media = Some(get_nft_image_url(&token_id));
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }

    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path.expect("Path expected");

        if path == "/robots.txt" {
            return Web4Response::plain_response("User-agent: *\nDisallow:".to_string());
        }

        if path == "/mint" {
            return Web4Response::webpage_template(
                make_replacements(
                    include_str!("./design/mint.html").to_string(),
                    &[env::current_account_id().to_string(), "testnet".to_string()]
                ));
        }

        let token_id = get_token_id(&path).unwrap_or_default();

        if !token_id.is_empty() {
            if path.starts_with(NFT_IMAGE_SOURCE) {
                return Web4Response::svg_response(
                    generate_qr(&get_nft_target_url(&token_id))
                );
            }

            if path.starts_with(NFT_TOKEN_SOURCE) {
                if let Some(token) = self.tokens.nft_token(token_id.clone()) {
                    if let Some(metadata) = token.metadata {
                        let title =  metadata.title.unwrap_or_else(|| "Unknown NFT".to_string());
                        let description =  metadata.description.unwrap_or_else(|| "Unknown NFT".to_string());
                        return Web4Response::webpage_template(
                            make_replacements(
                                include_str!("./design/token.html").to_string(),
                                &[title, token_id, description, token.owner_id.to_string(), NFT_IMAGE_SOURCE.to_string()])
                        );
                    }
                }
            }
        }

        Web4Response::webpage_template(include_str!("./design/index.html").to_string())
    }
}

near_contract_standards::impl_non_fungible_token_core!(NfTicket, tokens);
near_contract_standards::impl_non_fungible_token_approval!(NfTicket, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(NfTicket, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for NfTicket {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
