mod abi;
mod pb;

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
use substreams::store::{StoreAdd, StoreAddBigInt, StoreGetBigInt, StoreNew};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::StoreGet;


substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_seaport_purchases(blk: eth::Block) -> Result<Option<seaport::SeaportPurchases>, substreams::errors::Error> {
    let mut purchases: Vec<seaport::Purchase> = Vec::new();
    for log in blk.logs() {
        if let Some(event) = abi::seaport::events::OrderFulfilled::match_and_decode(log.log) {
            // https://docs.opensea.io/reference/seaport-structs#spentitem
            let item_type = event.offer[0].0.to_u64();
            
            if item_type == 0 {
                // log::info!("NATIVE ASSET (ETH) PURCHASE!");
                // log::info!("Amount: {:?}", substreams::scalar::BigDecimal::from(event.offer[0].3.clone()) /  substreams::scalar::BigDecimal::new(substreams::scalar::BigInt::from(1), 18));
            }
            // erc20 purchase event
            else if item_type == 1 {
                // log::info!("ERC20 PURCHASE!");
                let amount = substreams::scalar::BigDecimal::from(event.offer[0].3.clone());
                // 18 decimals is the standard for erc20 but not necessary. this needs to be adjusted for each ERC20.
                // log::info!("erc20 quantity: {:?}", amount);
                // log::info!("total USD: {:?}", price.unwrap() * amount);
                purchases.push(seaport::Purchase {
                    order_type: item_type,
                    from: hex::encode(event.offerer),
                    to: hex::encode(event.recipient),
                    token_in: hex::encode(event.offer[0].1.clone()),
                    token_in_amount: event.offer[0].3.to_u64(),
                    token_out: hex::encode(event.consideration[0].1.clone()),
                    token_out_amount: event.consideration[0].3.to_u64(),
                })
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
    Ok(Some(seaport::SeaportPurchases { purchases } ))
}

#[substreams::handlers::store]
pub fn store_seaport_activity(clock: Clock, purchases: seaport::SeaportPurchases, output: StoreAddBigInt) {
    // unimplemented!("filter and store purchases. this is where I could blacklist bad actors.");
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let hour_id = timestamp_seconds / 3600;
    let prev_hour_id = hour_id - 1;

    for purchase in purchases.purchases {
        // ordinal (what is this?) , key, value to add to store.
        output.add(1, format!("seaport_volume:{}", purchase.token_in), &BigInt::from(purchase.token_in_amount));
        output.add(1, format!("seaport_volume:{}", purchase.token_out), &BigInt::from(purchase.token_out_amount));
        output.add(2, format!("seaport_activity"), &BigInt::one());
    }
}

#[substreams::handlers::map]
pub fn metrics_out(
    purchases: seaport::SeaportPurchases,
    store: StoreGetBigInt,
) -> Result<Option<seaport_metrics::Metrics>, substreams::errors::Error> {
    let mut metrics = Vec::new();
    for purchase in purchases.purchases {
        metrics.push(seaport_metrics::Metric {key: format!("seaport_volume:{}", purchase.token_in), value: store.get_at(1, format!("seaport_volume:{}", purchase.token_in)).unwrap().to_u64()});
    }

    Ok(Some(seaport_metrics::Metrics { metrics }))
}