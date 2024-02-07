mod abi;
mod pb;

use std::str::FromStr;

// use hex_literal::hex;
// use pb::eth::erc721::v1 as erc721;
use pb::eth::seaport::v1 as seaport;

use pb::eth::seaport_metrics::v1 as seaport_metrics;

// use hex_literal::hex;
// use substreams::scalar::BigDecimal;
use substreams::{key, prelude::StoreAppend};
use substreams::{log}; // Hex
//use substreams::{log, store::StoreAddInt64, Hex};
// use substreams_database_change::pb::database::DatabaseChanges;
// use substreams_database_change::tables::Tables;
use substreams_ethereum::{pb::sf::ethereum::r#type::v2 as eth, Event};
use substreams::pb::substreams::Clock;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreDelete, StoreGetBigDecimal, StoreGetBigInt, StoreNew};
use substreams::scalar::{BigInt, BigDecimal};
use substreams::prelude::StoreAddBigDecimal;
use substreams::store::StoreGet;
use num_traits::cast::ToPrimitive;


pub struct ERC20 {
    name: &'static str,
    symbol: &'static str,
    decimals: u64,
    contract: &'static str
}

// start with the basics.
static WHITELIST: [ERC20; 7] = [
    ERC20 {
        name: "Wrapped Ethereum",
        symbol: "WETH",
        decimals: 18,
        contract: "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
    },
    ERC20 {
        name: "Circle USD",
        symbol: "USDC",
        decimals: 6,
        contract: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
    },
    ERC20 {
        name: "Dai Stablecoin",
        symbol: "DAI",
        decimals: 18,
        contract: "6b175474e89094c44da98b954eedeac495271d0f"
    },
    ERC20 {
        name: "Tether USD",
        symbol: "USDT",
        decimals: 6,
        contract: "dac17f958d2ee523a2206206994597c13d831ec7"
    },
    ERC20 {
        name: "TrueUSD",
        symbol: "TUSD",
        decimals: 18,
        contract: "0000000000085d4780b73119b644ae5ecd22b376"
    },
    ERC20 {
        name: "4dd28568d05f09b02220b09c2cb307bfd837cb95",
        symbol: "PRINTS",
        decimals: 18,
        contract: "0000000000085d4780b73119b644ae5ecd22b376"
    },
    ERC20 {
        name: "Fei USD",
        symbol: "FEI",
        decimals: 18,
        contract: "956f47f50a910163d8bf957cf5846d573e7f87ca"
    },
];

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_seaport_purchases(blk: eth::Block) -> Result<Option<seaport::SeaportPurchases>, substreams::errors::Error> {
    let mut purchases: Vec<seaport::Purchase> = Vec::new();
    for log in blk.logs() {
        if let Some(event) = abi::seaport::events::OrderFulfilled::match_and_decode(log.log) {

            // https://docs.opensea.io/reference/seaport-structs#spentitem
            if !event.offer.is_empty() {
                let item_type = event.offer[0].0.to_u64();
                
                // there are 6 different event types, all different types of purchase.
                // ie, purchase with native eth
                // or purchase with erc20
                // the last two I am unclear about. "with criteria"
                if item_type == 0 {
                    // unimplemented!("native eth");
                }
                else if item_type == 1 { // erc20 purchase event
                    if !event.offer.is_empty() && !event.consideration.is_empty() {
                        let (amount_in, token_in): (seaport::BigDecimal, String) = match find_token_by_contract(&hex::encode(event.offer[0].1.clone())){
                            Some(token) => {
                                let a_i = seaport::BigDecimal { 
                                    unscaled_value: event.offer[0].3.to_decimal(token.decimals).to_string(), 
                                    scale: token.decimals as i32,
                                };
                                (a_i, token.symbol.to_owned())
                            }
                            None => {
                                // if the amount was one, it was 99.99% likely to be an nft. dont scale by decimals.
                                if event.offer[0].3 == BigInt::one() {
                                    (seaport::BigDecimal {
                                        unscaled_value: event.offer[0].3.to_decimal(0).to_string(),
                                        scale: 0,
                                    },
                                    hex::encode(event.offer[0].1.to_owned()))                                }
                                else {
                                    (seaport::BigDecimal {
                                        unscaled_value: event.offer[0].3.to_decimal(18).to_string(),
                                        scale: 18
                                    }, hex::encode(event.offer[0].1.to_owned()))
                                }
                            }
                        };
                        let (amount_out, token_out): (seaport::BigDecimal, String)  = match find_token_by_contract(&hex::encode(event.consideration[0].1.clone())){
                            Some(token) => {
                                (seaport::BigDecimal {
                                    unscaled_value: event.consideration[0].3.to_decimal(token.decimals).to_string(),
                                    scale: token.decimals as i32,
                                }, token.symbol.to_owned()) 
                            }
                            None => {
                                // if the amount was one, it was 99.99% likely to be an nft. dont scale by decimals.
                                if event.consideration[0].3 == BigInt::one() {
                                    (seaport::BigDecimal {
                                        unscaled_value: event.consideration[0].3.to_decimal(0).to_string(),
                                        scale: 0
                                    }, hex::encode(event.consideration[0].1.to_owned()))
                                }
                                else {
                                    (seaport::BigDecimal {
                                        unscaled_value: event.consideration[0].3.to_decimal(18).to_string(),
                                        scale: 18,
                                    }, hex::encode(event.consideration[0].1.to_owned()))
                                }
                            }
                        };
                        purchases.push(seaport::Purchase {
                            order_type: item_type,
                            from: hex::encode(event.offerer),
                            to: hex::encode(event.recipient),
                            token_in: token_in,
                            token_in_amount: Some(amount_in),
                            token_out: token_out,
                            token_out_amount: Some(amount_out),
                        })
                    }
                }
                else if item_type == 2 {
                    // log::info!("order purchased with ERC721!");
    
                }
                else if item_type == 3 {
                    // log::info!("order purchased with ERC1155!");
    
                }
                else if item_type == 4 {
                    // log::info!("order purchased with ERC721_WITH_CRITERIA!");
    
                }
                else if item_type == 5 {
                    // log::info!("order purchased with ERC1155_WITH_CRITERIA!");
    
                }
            }
        }
    }
    Ok(Some(seaport::SeaportPurchases { purchases } ))
}

#[substreams::handlers::store]
pub fn store_seaport_activity(clock: Clock, purchases: seaport::SeaportPurchases, output: StoreAddBigDecimal) {
    // unimplemented!("filter and store purchases. this is where I could blacklist bad actors.");
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let hour_id = timestamp_seconds / 3600;
    let prev_hour_id = hour_id - 1;

    for purchase in purchases.purchases {
        // delete prior metrics from KV store.
        output.delete_prefix(0, &format!("seaport_volume:{prev_hour_id}:{}", purchase.token_in));
        output.delete_prefix(0, &format!("seaport_volume:{prev_hour_id}:{}", purchase.token_out));
        output.delete_prefix(0, &format!("seaport_activity:{prev_hour_id}"));

        
        // for graceful error handling, this should be a match, no .unwrap()
        let in_amount = BigDecimal::from_str(&purchase.token_in_amount.unwrap().unscaled_value).unwrap();
        let out_amount = BigDecimal::from_str(&purchase.token_out_amount.unwrap().unscaled_value).unwrap();
        log::info!("token in amount: {}", in_amount);
        log::info!("token out: {}", out_amount);

        // ordinal, key, value to add to store.
        // btw - right now I convert a f64 to a str to a BigDecimal - that can't be the most efficient.
        output.add(0, format!("seaport_volume:{hour_id}:{}", purchase.token_in), in_amount);
        output.add(0, format!("seaport_volume:{hour_id}:{}", purchase.token_out), out_amount);
        output.add(0, format!("seaport_activity:{}", hour_id), &BigDecimal::one());
    }
}

#[substreams::handlers::map]
pub fn metrics_out(
    clock: Clock,
    purchases: seaport::SeaportPurchases,
    store: StoreGetBigDecimal,
) -> Result<Option<seaport_metrics::Metrics>, substreams::errors::Error> {
    let mut metrics = Vec::new();
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let hour_id = timestamp_seconds / 3600;

    for purchase in purchases.purchases {
        // again, should be a match statement, not .unwrap()
        let val = store.get_at(1, format!("seaport_volume:{hour_id}:{}", purchase.token_in)).unwrap(); 
        log::info!("value retrieved: {}", val);
        metrics.push(seaport_metrics::Metric {key: format!("seaport_volume:hour:{hour_id}:{}", purchase.token_in), value: val.to_string()});
    }

    Ok(Some(seaport_metrics::Metrics { metrics }))
}

fn find_token_by_contract(contract_address: &str) -> Option<&'static ERC20> {
    WHITELIST.iter().find(|&token| token.contract == contract_address)
}