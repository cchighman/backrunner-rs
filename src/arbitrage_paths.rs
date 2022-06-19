use std::borrow::Borrow;
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;
use crate::crypto_pair::CryptoPair;
/* 
impl ArbitragePaths {
    pub fn new() -> ArbitragePaths {
        ArbitragePaths {}
    }
    //bd1b9sZowNIdkWfrbBO6wmMopiV2aSO0MXBAVRTrlTk0r7R
    pub async fn generate_arbitrage_paths(
        &self,
        pair_map: &HashMap<String, Vec<Arc<CryptoPair>>>,
        arb_paths: &mut Vec<Arc<ArbitragePath>>,
    ) {
        /*
        let mut g: Graph<String, String> = Graph::new();

        let mut symbol_map: HashMap<_, _> = HashMap::<_, _>::new();
        let mut visited_map: HashMap<String, NodeIndex> = HashMap::new();
        let mut paths_to_pair_map: HashMap<usize, Arc<CryptoPair>> = HashMap::new();
        let mut cnt = 1;
        /* pair_map is map("ETHUSDT", Vec<Pairs of ETHUSDT>) */
        for key in pair_map {
            // println!("{}", cnt.to_string());
            cnt = cnt + 1;
            /* Concerned with multiple nodes from like pairs */
            let mut node_map: HashMap<_, _> = HashMap::<String, Vec<NodeIndex>>::new();
            let key_clone = key.clone();
            for pair in key_clone.1 {
                let left_symbol = pair.left_symbol().clone();
                let right_symbol = pair.right_symbol().clone();
                let left_key = left_symbol.clone() + &pair.dex();
                let right_key = right_symbol.clone() + &pair.dex();
                let _left_node_index: NodeIndex = Default::default();
                let _right_node_index: NodeIndex = Default::default();

                if !visited_map.contains_key::<String>(&left_key) {
                    let crypto_symbol = left_symbol.clone() + &pair.dex();
                    let crypto_symbol_clone = crypto_symbol.clone();

                    let left_node = g.add_node(crypto_symbol_clone);
                    symbol_map.insert(left_node.index(), crypto_symbol.clone());
                    visited_map.insert(left_key.clone(), left_node);
                    paths_to_pair_map.insert(left_node.index(), pair.clone());
                }
                if !visited_map.contains_key::<String>(&right_key) {
                    let crypto_symbol = right_symbol.clone() + &pair.dex();
                    let crypto_symbol_clone2 = crypto_symbol.clone();
                    let right_node = g.add_node(crypto_symbol);
                    symbol_map.insert(right_node.index(), crypto_symbol_clone2.clone());
                    paths_to_pair_map.insert(right_node.index(), pair.clone());
                    visited_map.insert(right_key.clone(), right_node);
                }

                if !node_map.contains_key::<String>(&left_symbol) {
                    node_map.insert(left_symbol.clone(), Vec::new());
                }
                node_map.insert(right_symbol.clone(), Vec::new());

                let left_node_index = visited_map[&left_key];
                let right_node_index = visited_map[&right_key];

                let mut left_pool_vec: &mut Vec<NodeIndex> =
                    node_map.mut(&left_symbol).unwrap();
                left_pool_vec.push(left_node_index.clone());

                let mut right_pool_vec: &mut Vec<NodeIndex> =
                    node_map.mut(&right_symbol).unwrap();
                right_pool_vec.push(right_node_index.clone());

                let str1 = left_symbol.clone() + "-" + &right_symbol;
                let str2 = right_symbol.clone() + "-" + &left_symbol;

                Graph::add_edge(
                    &mut g,
                    left_node_index.clone(),
                    right_node_index.clone(),
                    str1.clone(),
                );
                Graph::add_edge(
                    &mut g,
                    right_node_index.clone(),
                    left_node_index.clone(),
                    str2.clone(),
                );

                /* Add edges when more than one pool exists */
                if key.1.len() > 1 {
                    /* For each CryptoPair in Vec (ethusdt univ2, ethusdt sushi)  -- (0,1),(1,0)..(0,4), (4,0), (1,3),(3,1) , (3,4), (4,3)*/
                    for pair in key.1 {
                        let left_symbol = pair.left_symbol().clone();
                        let right_symbol = pair.right_symbol().clone();

                        let left_pool_vec = node_map.get(&left_symbol).unwrap(); // Has 0,3
                        let right_pool_vec = node_map.get(&right_symbol).unwrap(); // Has 1,4
                        let str1 = left_symbol.clone() + "!!!" + &right_symbol;
                        let str2 = right_symbol.clone() + "!!!" + &left_symbol;
                        for node1 in left_pool_vec {
                            for node2 in right_pool_vec {
                                if !Graph::contains_edge(&g, node1.clone(), node2.clone()) {
                                    Graph::add_edge(
                                        &mut g,
                                        node1.clone(),
                                        node2.clone(),
                                        str1.clone(),
                                    );
                                }

                                if !Graph::contains_edge(&g, node2.clone(), node1.clone()) {
                                    Graph::add_edge(
                                        &mut g,
                                        node2.clone(),
                                        node1.clone(),
                                        str2.clone(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("g - {:#?}", &g);
        println!("connected - {}", algo::connected_components(&g));
        //println!("{}", Dot::new(&g));
        println!("Cyclic? {}", petgraph::algo::is_cyclic_directed(&g));

        let mut way_list: Vec<Vec<Vec<_>>> = Default::default();

        let mut ii: i32 = 0;
        for i in 0..g.node_count() - 1 {
            for j in 0..g.node_count() - 1 {
                let path = algo::all_simple_paths::<Vec<_>, _>(
                    &g,
                    NodeIndex::new(i),
                    NodeIndex::new(j),
                    1,
                    Option::new(1),
                )
                .collect::<Vec<_>>();

                if !path.is_empty() {
                    let mut node_list: Vec<Vec<_>> = Default::default();

                    for paths in path {
                        if !paths.is_empty() {
                            let mut nodes: Vec<_> = Default::default();
                            let mut crypto_paths: Vec<_> = Default::default();
                            for way in paths {
                                nodes.push(symbol_map[&way.index()].clone());
                                crypto_paths.push(paths_to_pair_map[&way.index()].clone());
                            }
                            /*
                            if (crypto_paths[0].left_symbol() == "WETH" || crypto_paths[0].right_symbol() == "WETH")
                                &&
                                (crypto_paths[2].left_symbol() == "WETH" || crypto_paths[2].right_symbol() == "WETH")
                            {
                                println!("{}-{}-{}-{}-{}-{}", crypto_paths[0].left_symbol(), crypto_paths[0].right_symbol(),
                                         crypto_paths[1].left_symbol(), crypto_paths[1].right_symbol(),
                                         crypto_paths[2].left_symbol(), crypto_paths[2].right_symbol());
                                */

                            // First and Last Token must have a similar token
                            let firstAndThird = (((crypto_paths[1].left_symbol()
                                == crypto_paths[0].right_symbol()
                                || crypto_paths[1].left_symbol()
                                    == crypto_paths[0].left_symbol())
                                && (crypto_paths[1].right_symbol()
                                    == crypto_paths[2].right_symbol()
                                    || crypto_paths[1].right_symbol()
                                        == crypto_paths[2].left_symbol()))
                                || ((crypto_paths[1].right_symbol()
                                    == crypto_paths[0].right_symbol()
                                    || crypto_paths[1].right_symbol()
                                        == crypto_paths[0].left_symbol())
                                    && (crypto_paths[1].left_symbol()
                                        == crypto_paths[2].right_symbol()
                                        || crypto_paths[1].left_symbol()
                                            == crypto_paths[2].left_symbol())));

                            let secondAndThird = ((crypto_paths[1].right_symbol()
                                == crypto_paths[2].right_symbol()
                                || crypto_paths[1].right_symbol()
                                    == crypto_paths[2].left_symbol())
                                && (crypto_paths[1].left_symbol()
                                    == crypto_paths[0].right_symbol()
                                    || crypto_paths[1].left_symbol()
                                        == crypto_paths[0].left_symbol()))
                                || ((crypto_paths[1].left_symbol()
                                    == crypto_paths[2].right_symbol()
                                    || crypto_paths[1].left_symbol()
                                        == crypto_paths[2].left_symbol())
                                    && (crypto_paths[1].right_symbol()
                                        == crypto_paths[0].right_symbol()
                                        || crypto_paths[1].right_symbol()
                                            == crypto_paths[0].left_symbol()));

                            let pair_match = crypto_paths[0].right_symbol()
                                == crypto_paths[2].right_symbol()
                                || crypto_paths[0].right_symbol() == crypto_paths[2].left_symbol()
                                || crypto_paths[0].left_symbol() == crypto_paths[2].right_symbol()
                                || crypto_paths[0].left_symbol() == crypto_paths[2].left_symbol();

                            let firstAndSecondLeft = (crypto_paths[0].left_symbol()
                                == crypto_paths[1].left_symbol()
                                && (crypto_paths[1].right_symbol()
                                    == crypto_paths[2].right_symbol())
                                || (crypto_paths[1].right_symbol()
                                    == crypto_paths[2].right_symbol()));

                            let secondAndThird2 = (crypto_paths[1].left_symbol()
                                == crypto_paths[2].left_symbol()
                                || crypto_paths[1].left_symbol() == crypto_paths[2].right_symbol())
                                || (crypto_paths[1].right_symbol()
                                    == crypto_paths[2].left_symbol()
                                    || crypto_paths[1].right_symbol()
                                        == crypto_paths[2].right_symbol());

                            let secondFirstMatchesFirstFirst =
                                crypto_paths[1].left_symbol() == crypto_paths[0].left_symbol();
                            let secondFirstMatchesFirstSecond =
                                crypto_paths[1].left_symbol() == crypto_paths[0].right_symbol();
                            let secondSecondMatchesFirstFirst =
                                crypto_paths[1].right_symbol() == crypto_paths[0].left_symbol();
                            let secondSecondMatchesFirstSecond =
                                crypto_paths[1].right_symbol() == crypto_paths[0].right_symbol();

                            let secondFirstMatchesThirdFirst =
                                crypto_paths[1].left_symbol() == crypto_paths[2].left_symbol();
                            let secondFirstMatchesThirdSecond =
                                crypto_paths[1].left_symbol() == crypto_paths[2].right_symbol();
                            let secondSecondMatchesThirdFirst =
                                crypto_paths[1].right_symbol() == crypto_paths[2].left_symbol();
                            let secondSecondMatchesThirdSecond =
                                crypto_paths[1].right_symbol() == crypto_paths[2].right_symbol();

                            let noDuplicates0 =
                                crypto_paths[0].pair_id() != crypto_paths[1].pair_id();
                            let noDuplicates1 =
                                crypto_paths[0].pair_id() != crypto_paths[2].pair_id();
                            let noDuplicates2 =
                                crypto_paths[1].pair_id() != crypto_paths[2].pair_id();

                            let first_cyclic = (secondFirstMatchesFirstFirst
                                || secondFirstMatchesFirstSecond)
                                && (secondSecondMatchesThirdFirst
                                    || secondSecondMatchesThirdSecond);
                            let second_cyclic = (secondSecondMatchesFirstFirst
                                || secondSecondMatchesFirstSecond)
                                && (secondFirstMatchesThirdFirst || secondFirstMatchesThirdSecond);
                            /*
                                                       let cyclic =  ((crypto_paths[0].left_symbol == "WETH" ||  crypto_paths[0].right_symbol == "WETH") &&
                                                           (crypto_paths[0].left_symbol == "DAI" ||  crypto_paths[0].right_symbol == "DAI"))
                                                            &&
                                                           ((crypto_paths[2].left_symbol == "WETH" ||  crypto_paths[2].right_symbol == "WETH") &&
                                                               (crypto_paths[2].left_symbol == "DAI" ||  crypto_paths[2].right_symbol == "DAI"))
                            */

                            let a1_b3 =
                                crypto_paths[0].left_symbol() == crypto_paths[2].right_symbol();
                            let b1_a2 =
                                crypto_paths[0].right_symbol() == crypto_paths[1].left_symbol();
                            let b2_a3 =
                                crypto_paths[1].right_symbol() == crypto_paths[2].left_symbol();
                            let a1_a2 =
                                crypto_paths[0].left_symbol() == crypto_paths[1].left_symbol();
                            let b1_b3 =
                                crypto_paths[0].right_symbol() == crypto_paths[2].right_symbol();
                            let b1_b2 =
                                crypto_paths[0].right_symbol() == crypto_paths[1].right_symbol();
                            let a2_a3 =
                                crypto_paths[1].left_symbol() == crypto_paths[2].left_symbol();
                            let a1_b2 =
                                crypto_paths[0].left_symbol() == crypto_paths[1].right_symbol();
                            let b1_a3 =
                                crypto_paths[0].right_symbol() == crypto_paths[2].left_symbol();
                            let b2_b3 =
                                crypto_paths[1].right_symbol() == crypto_paths[2].right_symbol();
                            let a1_a3 =
                                crypto_paths[0].left_symbol() == crypto_paths[2].left_symbol();
                            let a2_b3 =
                                crypto_paths[1].left_symbol() == crypto_paths[2].right_symbol();

                            let scenario_1 = a1_b3 && b1_a2 && b2_a3;
                            let scenario_2 = a1_a2 && b1_b3 && b2_a3;
                            let scenario_3 = a1_b3 && b1_b2 && a2_a3;
                            let scenario_4 = a1_b2 && b1_b3 && a2_a3;
                            let scenario_5 = a1_a2 && b1_a3 && b2_b3;
                            let scenario_6 = a1_a3 && b1_a2 && b2_b3;
                            let scenario_7 = a1_a3 && b1_b2 && a2_b3;
                            let scenario_8 = a1_b2 && b1_a3 && a2_b3;

                            if (first_cyclic || second_cyclic)
                                && noDuplicates0
                                && noDuplicates1
                                && noDuplicates2
                                && (firstAndThird && secondAndThird)
                            {
                                let arb_path = ArbitragePath::new(crypto_paths);
                                arb_paths.push(arb_path.clone());
                                arb_path.init(arb_path.clone()).await;

                                node_list.push(nodes);
                            }
                        }
                    }
                    way_list.push(node_list);
                }
                ii += 1;
            }
        }

         */
    }
}
pub struct ArbitragePaths {}

pub struct Intolterator {}
*/