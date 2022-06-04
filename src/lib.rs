#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types, dead_code)]
#![allow(non_snake_case, unused_imports, unused_results)]
#![allow(unused_variables, unused_assignments, unused_must_use)]
#![recursion_limit = "512"]

pub mod arb_signal;
pub mod arb_thread_pool;
pub mod arbitrage_path;
pub mod call_julia;
pub mod contracts;
pub mod crypto_math;
pub mod crypto_pair;
pub mod dex_pool;
pub mod flashbot_strategy;
pub mod graphql_uniswapv2;
pub mod graphql_uniswapv3;
pub mod sequence_token;
pub mod sources;
pub mod swap_route;
pub mod three_path_sequence;
pub mod uniswap_providers;
pub mod uniswap_transaction;
pub mod uniswapv2_pairs;
pub mod uniswapv3_pools;
pub mod utils;
