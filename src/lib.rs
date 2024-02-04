mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::erc721::v1 as erc721;
use substreams::{key, prelude::*};
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

// Bored Ape Club Contract
// const TRACKED_METHOD: [u8; 4] = hex!("7034d120");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {

    for log in blk.logs() {
        // if abi::erc721::events::Transfer::match_log(log.log) {
        //     let temp = abi::erc721::events::Transfer::decode(log.log).expect("Failed to get MyStruct");

            
        //     substreams::log::info!("from: {}, to: {}", Hex(temp.from), Hex(temp.to));
          
        //     // substreams::log::info!("Tranfer event!");
        // }
        
        if abi::blur_v2::events::Execution721Packed::match_log(log.log) {
            substreams::log::info!("{:?}!", Hex(log.log.address.clone()));
            substreams::log::info!("{:?}!", abi::blur_v2::events::Execution721Packed::decode(log.log).unwrap());
        }

        // if abi::blur_v2::events::Execution721TakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721TakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
        // if abi::blur_v2::events::Execution721MakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721MakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
    }

    Ok(None)
}

#[substreams::handlers::map]
fn map_blur_pool(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {

    for log in blk.logs() {
        // if abi::erc721::events::Transfer::match_log(log.log) {
        //     let temp = abi::erc721::events::Transfer::decode(log.log).expect("Failed to get MyStruct");

            
        //     substreams::log::info!("from: {}, to: {}", Hex(temp.from), Hex(temp.to));
          
        //     // substreams::log::info!("Tranfer event!");
        // }
        
        if abi::blur_v2::events::Execution721Packed::match_log(log.log) {
            substreams::log::info!("{:?}!", Hex(log.log.address.clone()));
            substreams::log::info!("{:?}!", abi::blur_v2::events::Execution721Packed::decode(log.log).unwrap());
        }

        // if abi::blur_v2::events::Execution721TakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721TakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
        // if abi::blur_v2::events::Execution721MakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721MakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
    }

    Ok(None)
}

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