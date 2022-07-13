use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

static STORE_KEY: &[u8] = b"store";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Certificate {
    pub owner: Addr,
    pub id: String
}

pub const CERTIFICATE: Item<Certificate> = Item::new("state");
