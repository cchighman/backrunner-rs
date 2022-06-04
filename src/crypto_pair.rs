use std::fmt;

use ethers::prelude::{Address, U256};
use futures_signals::signal::{Mutable, MutableSignal};
use serde::{Deserialize, Serialize};

use crate::dex_pool::DexPool;
use crate::utils::common::DIRECTION;

pub struct PairUpdateParams {}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
impl fmt::Display for CryptoPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CryptoPair {
    /* TODO - ID uses pool id, mempool event uses id to trigger an update method.
        Update method updates reserves and then parallel iterates path references to
        invoke recalculate.
    */
    pub fn new(pair: DexPool) -> Self {
        Self {
            pair: pair.clone(),
            left_reserves: Mutable::new(pair.token0.reserve.clone()),
            right_reserves: Mutable::new(pair.token1.reserve.clone()),
        }
    }

    pub fn left_symbol(&self) -> &String {
        return &self.pair.token0.symbol;
    }

    pub fn right_symbol(&self) -> &String {
        return &self.pair.token1.symbol;
    }

    pub fn left_decimal(&self) -> i32 {
        return self.pair.token0.decimals;
    }
    pub fn get_symbol(&self, direction: DIRECTION) -> &String {
        if direction == DIRECTION::Left {
            return &self.pair.token0.symbol;
        } else {
            return &self.pair.token1.symbol;
        }
    }

    pub fn right_decimal(&self) -> i32 {
        return self.pair.token1.decimals;
    }

    pub fn get_decimal(&self, direction: DIRECTION) -> i32 {
        if direction == DIRECTION::Left {
            return self.pair.token0.decimals;
        } else {
            return self.pair.token1.decimals;
        }
    }

    pub fn left_reserves(&self) -> U256 {
        return self.left_reserves.get();
    }

    pub fn right_reserves(&self) -> U256 {
        return self.right_reserves.get();
    }

    pub fn pair_id(&self) -> &Address {
        return &self.pair.id;
    }

    pub fn dex(&self) -> &String {
        return &self.pair.dex;
    }

    pub fn router(&self) -> Address {
        return self.pair.router;
    }

    pub fn left_id(&self) -> &Address {
        return &self.pair.token0.id;
    }

    pub fn right_id(&self) -> &Address {
        return &self.pair.token1.id;
    }

    pub fn get_id(&self, direction: DIRECTION) -> &Address {
        if direction == DIRECTION::Left {
            return self.left_id();
        } else {
            return self.right_id();
        }
    }

    pub fn pair_symbol(&self) -> String {
        return self.left_symbol().to_owned() + self.right_symbol();
    }

    pub fn update(&self, new: &U256) {
        self.left_reserves.set(&self.left_reserves.get() + new);
        self.right_reserves
            .set(&self.right_reserves.get() + new.clone());
    }

    pub fn left_reserves_signal(&self) -> MutableSignal<U256> {
        self.left_reserves.signal()
    }

    pub fn get_reserves_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.left_reserves_signal();
        } else {
            return self.right_reserves_signal();
        }
    }

    pub fn get_reserve(&self, direction: DIRECTION) -> U256 {
        if direction == DIRECTION::Left {
            return self.left_reserves();
        } else {
            return self.right_reserves();
        }
    }

    pub fn get_signal(&self, direction: DIRECTION) -> MutableSignal<U256> {
        if direction == DIRECTION::Left {
            return self.left_reserves_signal();
        } else {
            return self.right_reserves_signal();
        }
    }

    pub fn right_reserves_signal(&self) -> MutableSignal<U256> {
        self.right_reserves.signal()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPair {
    pub(crate) pair: DexPool,
    pub(crate) left_reserves: Mutable<U256>,
    pub(crate) right_reserves: Mutable<U256>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPairs {
    pub pairs: Vec<Vec<CryptoPair>>,
}
