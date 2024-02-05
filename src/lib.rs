mod abi;
mod pb;

// use hex_literal::hex;
// use pb::eth::erc721::v1 as erc721;
use pb::eth::seaport::v1 as seaport;

// use hex_literal::hex;
// use substreams::scalar::BigDecimal;
// use substreams::{key, prelude::*};
use substreams::{log}; // Hex
//use substreams::{log, store::StoreAddInt64, Hex};
// use substreams_database_change::pb::database::DatabaseChanges;
// use substreams_database_change::tables::Tables;
use substreams_ethereum::{pb::sf::ethereum::r#type::v2 as eth, Event};

// Bored Ape Club Contract
// const TRACKED_METHOD: [u8; 4] = hex!("7034d120");

substreams_ethereum::init!();

// todo: abstract this
// ref: https://docs.opensea.io/reference/seaport-enums#itemtype
// enum ItemType {
//     Native,
//     Erc20,
//     Erc721,
//     Erc1155,
//     Erc721WithCriteria,
//     Erc1155WithCriteria,
// }

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Option<seaport::SeaportPurchases>, substreams::errors::Error> {
    let mut purchases: Vec<seaport::Purchase> = Vec::new();
    for log in blk.logs() {
        if let Some(event) = abi::seaport::events::OrderFulfilled::match_and_decode(log.log) {
            // https://docs.opensea.io/reference/seaport-structs#spentitem
            let item_type = event.offer[0].0.to_u64();
            
            if item_type == 0 {
                // log::info!("NATIVE ASSET (ETH) PURCHASE!");
                log::info!("Amount: {:?}", substreams::scalar::BigDecimal::from(event.offer[0].3.clone()) /  substreams::scalar::BigDecimal::new(substreams::scalar::BigInt::from(1), 18));
            }
            // erc20 purchase event
            else if item_type == 1 {
                // log::info!("ERC20 PURCHASE!");
                let amount = substreams::scalar::BigDecimal::from(event.offer[0].3.clone());
                // 18 decimals is the standard for erc20 but not necessary. this needs to be adjusted for each ERC20.
                log::info!("erc20 quantity: {:?}", amount);
                // log::info!("total USD: {:?}", price.unwrap() * amount);
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
    }
    Ok(Some(seaport::SeaportPurchases { purchases } ))
}

// #[substreams::handlers::map]
// fn map_prices(blk: eth::Block) ->> Result<Some(), substreams::errors::Error> {
//     unimplemented!()
// }

// const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";

// /// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
// #[substreams::handlers::store]
// fn store_transfers(transfers: erc721::Transfers, s: StoreAddInt64) {
//     log::info!("NFT holders state builder");
//     for transfer in transfers.transfers {
//         if transfer.from != NULL_ADDRESS {
//             log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
//             s.add(transfer.ordinal, generate_key(&transfer.from), -1);
//         }

//         if transfer.to != NULL_ADDRESS {
//             log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
//             s.add(transfer.ordinal, generate_key(&transfer.to), 1);
//         }
//     }
// }

// #[substreams::handlers::map]
// fn db_out(
//     clock: substreams::pb::substreams::Clock,
//     transfers: erc721::Transfers,
//     owner_deltas: Deltas<DeltaInt64>,
// ) -> Result<DatabaseChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();
//     for transfer in transfers.transfers {
//         tables
//             .create_row(
//                 "transfer",
//                 format!("{}-{}", &transfer.trx_hash, transfer.ordinal),
//             )
//             .set("trx_hash", transfer.trx_hash)
//             .set("from", transfer.from)
//             .set("to", transfer.to)
//             .set("token_id", transfer.token_id)
//             .set("ordinal", transfer.ordinal);
//     }

//     for delta in owner_deltas.into_iter() {
//         let holder = key::segment_at(&delta.key, 1);
//         let contract = key::segment_at(&delta.key, 2);

//         tables
//             .create_row("owner_count", format!("{}-{}", contract, holder))
//             .set("contract", contract)
//             .set("holder", holder)
//             .set("balance", delta.new_value)
//             .set("block_number", clock.number);
//     }

//     Ok(tables.to_database_changes())
// }

// fn generate_key(holder: &String) -> String {
//     return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
// }


// if transfers.len() == 0 {
//     return Ok(None);
// }

// Ok(Some(erc721::Transfers { transfers }))

// let transfers: Vec<_> = blk
//     .events::<abi::erc721::events::Transfer>(&TRACKED_CONTRACTS)
//     .map(|(transfer, log)| {
//         substreams::log::info!("log: {}", log);

//         erc721::Transfer {
//             trx_hash: Hex::encode(&log.receipt.transaction.hash),
//             from: Hex::encode(&transfer.from),
//             to: Hex::encode(&transfer.to),
//             token_id: transfer.token_id.to_u64(),
//             ordinal: log.block_index() as u64,
//         }
//     })
//     .collect();