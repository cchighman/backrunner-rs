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
pub mod uniswap_transaction;
pub mod uniswapv2_pairs;
pub mod uniswapv3_pools;
pub mod utils;
pub mod uniswap_providers;


pub mod two_path_sequence;
pub mod path_sequence;
pub mod path_sequence_factory;

use crate::three_path_sequence::ThreePathSequence;
use crate::two_path_sequence::TwoPathSequence;
use crate::path_sequence::PathSequence;

use std::any::*;

fn print_if_two_sequence(s: &dyn Any) {
  if let Some(string) = s.downcast_ref::<String>() {
      println!("It's a string({}): '{}'", string.len(), string);
  } else {
      println!("Not a string...");
  }
}

#[test]
fn test_main() {
  // Create your struct that implements your trait
  let value_any = value as &dyn Any;

  // Use the trait to abstract away everything that is not needed
  let x: &dyn Print = &t;
  println!("{}", type_name_of_val(&x));


    dbg!(  std::any::type_name::<Option<String>>());
  // Now there's an edge case that uses the original type..
  // How do you change it back?
  let printer: &TwoSequence = 
        x.as_any()
         .downcast_ref::<TwoSequence>()
         .expect("Failed to turn into");
         
  printer.start_printing();
}