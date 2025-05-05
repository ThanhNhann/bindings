use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Reply};
use cw_storage_plus::{Item, Map};

const CONFIG_KEY: &str = "config";
const RESULT_PREFIX: &str = "result";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub fn config() -> Item<'static, State> {
    Item::new(CONFIG_KEY)
}

pub fn replies() -> Map<'static, u64, Reply> {
    Map::new(RESULT_PREFIX)
}
