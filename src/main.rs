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

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, stdout};
use std::io::{Read, Write};
use std::sync::Arc;

use async_std::prelude::*;
use curl::easy::List;
//use backrunner_rs::two_path_sequence::{is_arbitrage_pair,cyclic_order};
use ethers::prelude::Address;
use futures_util::{FutureExt, TryFutureExt};
use itertools::Itertools;
use rayon::prelude::*;


use crypto_pair::{CryptoPair, CryptoPairs};
use utils::uniswapv2_utils::{populate_sushiswap_pairs, populate_uniswapv2_pairs};
use color_eyre::{eyre::eyre, eyre::Report, Section};

use std::any::Any;

use backrunner_rs::three_path_sequence::{cyclic_order as other_cyclic_order, is_arbitrage_pair as other_is_arbitrage_pair};
pub mod uniswap_providers;
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
pub mod sequence_token;
pub mod swap_route;
pub mod two_path_sequence;
pub mod three_path_sequence;
pub mod uniswap_transaction;
pub mod uniswapv2_pairs;
pub mod uniswapv3_pools;
pub mod utils;
pub mod path_sequence;
pub mod path_sequence_factory;


/*
   Grafana API Key: eyJrIjoiVjY3bzNoWTFnTTNyTUpCVXRoUUJxcXZPTXJGbE1nVmUiLCJuIjoiYmFja3J1bm5lciIsImlkIjoxfQ==
    Ex:
    curl -H "Authorization: Bearer eyJrIjoiVjY3bzNoWTFnTTNyTUpCVXRoUUJxcXZPTXJGbE1nVmUiLCJuIjoiYmFja3J1bm5lciIsImlkIjoxfQ=="
                 https://soundscapes.grafana.net/api/dashboards/home

     Discord Public Key: 0c31e811154cd8b3396151aad0c4ecb650d158f2758e782bf2a0bf5724a4e0d4
     Discord App Id: 982459497320689684
     Discord Client Id: 982459497320689684
     Discord Client Secret: lREE9D8B7qrKsyfqJQqkegNrPhTCGsa2
     Discord Invite Link: https://discord.com/api/oauth2/authorize?client_id=982459497320689684&permissions=8&redirect_uri=http%3A%2F%2Fwww.google.com&response_type=code&scope=identify%20email%20rpc.notifications.read%20rpc%20gdm.join%20guilds.members.read%20guilds.join%20connections%20guilds%20rpc.activities.write%20rpc.voice.write%20rpc.voice.read%20bot%20webhook.incoming%20messages.read%20applications.builds.upload%20applications.builds.read%20dm_channels.read%20voice%20relationships.read%20activities.write%20activities.read%20applications.entitlements%20applications.store.update%20applications.commands
     Discord Webhook URL: https://discord.com/api/webhooks/982463881354035241/6XmX7RP90l1LW50WrLSsgnJPAQNM0Woqa2pG7Kk693ujqGQYdMr9jHoClgU6MmJXrpI0
    RUSTFLAGS="-C target-cpu=native" cargo build --release
    New token created
    Key:
    59877d4b-3f52-4df2-95e4-e03da24954cd
    Secret:
    JxTuoYxDo0QJ
    To configure relay CLI, run the following command:
    relay login -k 59877d4b-3f52-4df2-95e4-e03da24954cd -s JxTuoYxDo0QJ
    To use credentials as an environment variables:
                
    export RELAY_KEY=59877d4b-3f52-4df2-95e4-e03da24954cd
    export RELAY_SECRET=JxTuoYxDo0QJ

                
    To create Kubernetes Secret:
                
    kubectl create secret generic whr-credentials \
        --from-literal=key=59877d4b-3f52-4df2-95e4-e03da24954cd \
        --from-literal=secret=JxTuoYxDo0QJ
    Webhook:  https://mvdugj7gh2ansfe4jhz65z.hooks.webhookrelay.com
    BlockNative Key: 3b21bf22-0a3e-4908-9b2f-c9ac37c31b7b  - secret: 5e0323d1-80bc-4ec3-a8dc-25cab8ff0706
*/


#[allow(dead_code)]
#[async_std::main]
async fn main()->Result<(), Report> {
    let args: Vec<String> = env::args().collect();
    color_eyre::install()?;
  
    /*
    TODO
    1.) Populate paths from GraphQL
    2.) Init dedicated listener for Pending/New Transactions
    3.) Flow:
         Event -> Invoke CryptoPair Update -> Evaluate Paths -> Generate Arbitrage
     */

    let mut crypto_pairs: HashMap<Address, Arc<CryptoPair>> = HashMap::new();
    let mut crypto_pairs_unsafe: HashMap<Address, CryptoPair> = HashMap::new();
    let mut arb_paths: Vec<Arc<(dyn Any + 'static + Sync + Send)>> = Default::default();

    println!("Test - ");
    /* 1.) Populate a map of all possible crypto pairs */
    populate_uniswapv2_pairs(&mut crypto_pairs_unsafe).await;
    // populate_uniswapv3_pools(&mut crypto_pairs);
    populate_sushiswap_pairs(&mut crypto_pairs_unsafe).await;

    if args.contains(&"generate".to_string()) {
        println!("Generating...");
        let x = crypto_pairs_unsafe
            .values()
            .collect::<Vec<_>>()
            .into_iter()
            .combinations(3);

        let t = x.collect::<Vec<_>>();

        dbg!(t.clone().len());
        let mut new_vec: Vec<Vec<CryptoPair>> = Vec::default();
        let mut ser_pairs = CryptoPairs { pairs: new_vec };

        use serde_json::{json, Value};
        let addy = "test";
        use std::collections::HashSet;
        let mut r: HashSet<Address> = Default::default();

        t.iter().for_each(|crypto_paths| {
            let mut y = Vec::default();

            y.push(crypto_paths[0].clone());
            y.push(crypto_paths[1].clone());
            y.push(crypto_paths[2].clone());

            if three_path_sequence::is_arbitrage_pair(&y) {
                println!("{}-{:.?} {}-{:.?} {}-{:.?}", crypto_paths[0].pair_symbol(), crypto_paths[0].pair_id(), crypto_paths[1].pair_symbol(), crypto_paths[1].pair_id(), crypto_paths[2].pair_symbol(), crypto_paths[2].pair_id());
                let mut y_2 = Vec::default();
                y_2.push(crypto_paths[0].clone());
                y_2.push(crypto_paths[1].clone());
                y_2.push(crypto_paths[2].clone());

       
                    r.insert(crypto_paths[0].pair_id().clone());                    
                    r.insert(crypto_paths[1].pair_id().clone());
                    r.insert(crypto_paths[2].pair_id().clone());
                    ser_pairs.pairs.push(y_2);
            }        
        });
    
    for a in r {
    
    let watch_addy = json!({"apiKey":"3b21bf22-0a3e-4908-9b2f-c9ac37c31b7b","address": a,"blockchain":"ethereum","networks":["main"]});

    use curl::easy::Easy;
    let mut easy = Easy::new();
    
    easy.url("https://api.blocknative.com/address").unwrap();
    easy.post(true).unwrap();
    let mut list = List::new();
    list.append("Content-type: application/json").unwrap();
    easy.http_headers(list).unwrap();
    easy.post_field_size(watch_addy.to_string().as_bytes().len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;

    transfer.read_function(|buf| {
    Ok(std::io::Read::read(&mut watch_addy.to_string().as_bytes(), buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
    
    }

        println!("ser_pairs: {}", ser_pairs.pairs.len());
        let file = File::create("pairs_2_500.json").unwrap();
        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &ser_pairs).unwrap();
        writer.flush().unwrap();
    }

    if args.contains(&"run".to_string()) || args.len() == 1 {
        println!("Running..");
        /* Read Pairs from file */
        let path = if args.len() > 2 && !args.contains(&"RUST_BACKTRACE=full".to_string()) {
            args[1].clone().to_string()
        } else {
            "pairs_2_500.json".clone().to_string()
        };
        println!("path: {}", path.clone());
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut cached_pairs: CryptoPairs = serde_json::from_reader(reader).unwrap();

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
        for unordered_pair in cached_pairs.pairs {
            let sequence = path_sequence_factory::create(unordered_pair.clone(), &crypto_pairs)
                .await
                .unwrap();
            arb_paths.push(sequence);
        }

        println!("pairs: {}, paths: {}", &crypto_pairs.len(), arb_paths.len());
    }
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

