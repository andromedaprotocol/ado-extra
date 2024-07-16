use cw_storage_plus::Item;

pub const DEFAULT_NONCE: u128 = 0;

pub const NONCE: Item<u128> = Item::new("nonce");
