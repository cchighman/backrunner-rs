#![recursion_limit = "256"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types, dead_code)]
#![allow(non_snake_case, unused_imports, unused_results)]
#![allow(
    unused_doc_comments,
    unused_variables,
    unused_assignments,
    unused_must_use,
    unused_mut
)]

use async_std::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use std::sync::Arc;

use ethers::prelude::Address;
use futures_util::{FutureExt, TryFutureExt};

use itertools::Itertools;
use rayon::prelude::*;

use arbitrage_path::ArbitragePath;
use crypto_pair::{CryptoPair, CryptoPairs};
use utils::uniswapv2_utils::{populate_sushiswap_pairs, populate_uniswapv2_pairs};

use crate::utils::common::cyclic_order;

pub mod arb_signal;
pub mod arb_thread_pool;
pub mod arbitrage_path;
pub mod arbitrage_paths;

pub mod contracts;
pub mod crypto_math;
pub mod crypto_pair;
pub mod dex_pool;
pub mod flashbot_strategy;
pub mod graphql_uniswapv2;
pub mod graphql_uniswapv3;
pub mod swap_route;
mod uniswap_transaction;
pub mod uniswapv2_pairs;
pub mod uniswapv3_pools;
pub mod utils;

/*
   Grafana API Key: eyJrIjoiVjY3bzNoWTFnTTNyTUpCVXRoUUJxcXZPTXJGbE1nVmUiLCJuIjoiYmFja3J1bm5lciIsImlkIjoxfQ==
    Ex:
    curl -H "Authorization: Bearer eyJrIjoiVjY3bzNoWTFnTTNyTUpCVXRoUUJxcXZPTXJGbE1nVmUiLCJuIjoiYmFja3J1bm5lciIsImlkIjoxfQ=="
                 https://soundscapes.grafana.net/api/dashboards/home
*/

use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    /// Implicitly using `std::str::FromStr`
    #[clap(short = 'O')]
    optimization: Option<usize>,

    /// Allow invalid UTF-8 paths
    #[clap(short = 'I', parse(from_os_str), value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    include: Option<std::path::PathBuf>,

    /// Handle IP addresses
    #[clap(long)]
    bind: Option<std::net::IpAddr>,

    /// Hand-written parser for tuples
    #[clap(short = 'D', parse(try_from_str = parse_key_val), multiple_occurrences(true))]
    defines: Vec<(String, i32)>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[allow(dead_code)]
fn main() {
    let args = Args::parse();
    /*
    for _ in 0..args.count {
        println!("Hello {}!", args)
    }
    */

    tracing_subscriber::fmt::init();
    /*
    TODO
    1.) Populate paths from GraphQL
    2.) Init dedicated listener for Pending/New Transactions
    3.) Flow:
         Event -> Invoke CryptoPair Update -> Evaluate Paths -> Generate Arbitrage
     */

    let mut crypto_pairs: HashMap<Address, Arc<CryptoPair>> = HashMap::new();
    let mut crypto_pairs_unsafe: HashMap<Address, CryptoPair> = HashMap::new();
    let mut arb_paths: Vec<Arc<ArbitragePath>> = Default::default();
    println!("Test - ");
    /* 1.) Populate a map of all possible crypto pairs */
    populate_uniswapv2_pairs(&mut crypto_pairs_unsafe);
    // populate_uniswapv3_pools(&mut crypto_pairs);
    populate_sushiswap_pairs(&mut crypto_pairs_unsafe);

    /* 2) Load some source to populate and init arb paths */
    /* 3.) Begin listening to pending / completed tx */
    /*
    let paths = ArbitragePaths::new();
        paths
            .generate_arbitrage_paths(&crypto_pairs, &mut arb_paths)
            .await;
    */
    /*
        let x = crypto_pairs_unsafe
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .combinations(3);

        let t = x.collect::<Vec<_>>();

        dbg!(t.clone().len());
        let mut new_vec: Vec<Vec<CryptoPair>> = Vec::default();
        let mut ser_pairs = CryptoPairs { pairs: new_vec };

        for crypto_paths in t.clone() {
            let mut y = Vec::default();

            y.push(crypto_paths[0].clone());
            y.push(crypto_paths[1].clone());
            y.push(crypto_paths[2].clone());
            if is_arbitrage_pair(&y) {
                let mut y_2 = Vec::default();
                y_2.push(crypto_paths[0].clone());
                y_2.push(crypto_paths[1].clone());
                y_2.push(crypto_paths[2].clone());
                ser_pairs.pairs.push(y_2);
            }
        }
        println!("ser_pairs: {}", ser_pairs.pairs.len());
        let file = File::create("pairs.json").unwrap();
        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &ser_pairs).unwrap();
        writer.flush().unwrap();
    */
    /*
        /* Read Pairs from file */
        let file = File::open("pairs_500.json").unwrap();
        let mut reader = BufReader::new(file);
        let cached_pairs: CryptoPairs = serde_json::from_reader(reader).unwrap();

        /* Re-creating path cache from file */
        // First populate CryptoPair objects

        /*
          This will iterate over a vector containing a vector of arbitrage path with cryptopairs.  We're now individually
          adding them to the main map as this copy as many duplicates.
        */

        for pair_path in cached_pairs.pairs.iter() {
            for crypto_pair in pair_path {
                if !crypto_pairs.contains_key::<Address>(crypto_pair.pair_id()) {
                    let pair = Arc::new(crypto_pair.clone());
                    crypto_pairs.insert(crypto_pair.pair_id().clone(), pair);
                }
            }
        }

        /*
        Next we want to create arbitrage paths based on the contents of the serialized vector, except we will instead look to the map map above for references.
         */
        for unordered_pair in cached_pairs.pairs.iter() {
            let sequence = cyclic_order(unordered_pair.clone(), &crypto_pairs).unwrap();
            let arb_path = ArbitragePath::new(sequence);
            arb_path.init(arb_path.clone());
            arb_paths.push(arb_path);
        }

        println!("pairs: {}, paths: {}", crypto_pairs.len(), arb_paths.len());

    */
    use std::{thread, time};
    loop {
        let ten = time::Duration::from_millis(10000);
        let now = time::Instant::now();

        thread::sleep(ten);
        /*
                crypto_pairs.par_iter().for_each(|pair| {
                    futures::executor::block_on(pair.1[0].update(U256::from(7)))
                    //spawn(pair.1[1].update(U256::from(4)));
                });
        */
        println!("Simulated Pair Updated.");
    }
}
