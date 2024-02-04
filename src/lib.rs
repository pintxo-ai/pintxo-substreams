mod abi;
mod pb;
// use hex_literal::hex;
use pb::eth::erc721::v1 as erc721;
use hex_literal::hex;
use substreams::scalar::BigDecimal;
// use substreams::{key, prelude::*};
use substreams::{log, Hex};
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
fn map_transfers(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {

    for log in blk.logs() {
        if let Some(event) = abi::seaport::events::OrderFulfilled::match_and_decode(log) {
            
            // https://docs.opensea.io/reference/seaport-structs#spentitem
            let item_type = event.offer[0].0.to_u64();
            if item_type == 0 {
                log::info!("NATIVE ASSET (ETH) PURCHASE!");
                // log::info!("{:?}", Hex(event.recipient.clone()));
                // 18 decimals is the standard for erc20 but not necessary. this may not always scale correctly.
                log::info!("Amount: {:?}", substreams::scalar::BigDecimal::from(event.offer[0].3.clone()) /  substreams::scalar::BigDecimal::new(substreams::scalar::BigInt::from(1), 18));
            }
            // erc20 purchase event
            else if item_type == 1 {
                log::info!("ERC20 PURCHASE!");
                
                // below is where price should be retrieved, or maybe price should be input into the mapping (that would make more sense, 
                // because then price could get consumed by all downstream tasks.)
                // TODO: non-chainlink price aggregator.
                // let price = get_price(blk.number, event.offer[0].1.clone());
                // log::info!("the price of the underlying asset used to purchase the nft: {:?}", price);

                let amount = substreams::scalar::BigDecimal::from(event.offer[0].3.clone()) /  substreams::scalar::BigDecimal::new(substreams::scalar::BigInt::from(1), 18);
                // 18 decimals is the standard for erc20 but not necessary. this needs to be adjusted for each ERC20.
                log::info!("erc20 quantity: {:?}", amount);
                // log::info!("total USD: {:?}", price.unwrap() * amount);
            }
        }
    }

    Ok(None)
}

const CONFIG: Config = Config {
    ethereum: NetworkConfig {
        chainlink_feed_registry: hex!("47fb2585d2c56fe188d0e6ec628a38b74fceeedf"),
        chainlink_feed_registry_start_block: 12864088,
        usdc_decimals: 6,
        usd_denominations: hex!("0000000000000000000000000000000000000348"),
    },
};
struct Config {
    ethereum: NetworkConfig,
}
/// Price lib config for each supported network
struct NetworkConfig {
    chainlink_feed_registry: [u8; 20],
    chainlink_feed_registry_start_block: u64,
    usdc_decimals: u8,
    usd_denominations: [u8; 20],
}

pub fn get_price(
    block_number: u64,
    token_address: Vec<u8>,
) -> Result<BigDecimal, String> {
    let network_config = CONFIG.ethereum;
        via_chainlink_feed_registry(&network_config, block_number, token_address.clone())
        .ok_or("price error".to_string())
}

/// Reference: https://docs.chain.link/docs/feed-registry
fn via_chainlink_feed_registry(
    network_config: &NetworkConfig,
    block_number: u64,
    token_address: Vec<u8>,
) -> Option<BigDecimal> {
    if block_number < network_config.chainlink_feed_registry_start_block {
        None
    } else {
        let price_mantissa_res = abi::chainlink_feed_registry::functions::LatestRoundData {
            base: token_address.clone(),
            quote: network_config.usd_denominations.to_vec(),
        }
        .call(network_config.chainlink_feed_registry.to_vec());

        log::info!("PMR: {:?}", price_mantissa_res);
        let decimals_res = abi::chainlink_feed_registry::functions::Decimals {
            base: token_address,
            quote: network_config.usd_denominations.to_vec(),
        }
        .call(network_config.chainlink_feed_registry.to_vec());

        if let (Some(price_mantissa), Some(decimals)) = (price_mantissa_res, decimals_res) {
            Some(price_mantissa.1.to_decimal(decimals.to_u64()))
        } else {
            None
        }
    }
}

#[substreams::handlers::map]
fn map_blur_pool(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {

    for log in blk.logs() {
        // if abi::erc721::events::Transfer::match_log(log.log) {
        //     let temp = abi::erc721::events::Transfer::decode(log.log).expect("Failed to get MyStruct");

            
        //     substreams::log::info!("from: {}, to: {}", Hex(temp.from), Hex(temp.to));
          
        //     // substreams::log::info!("Tranfer event!");
        // }
        


        // if abi::blur_v2::events::Execution721TakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721TakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
        // if abi::blur_v2::events::Execution721MakerFeePacked ::match_log(log.log) {
        //     substreams::log::info!("{:?}!", Hex(abi::blur_v2::events::Execution721MakerFeePacked::decode(log.log).unwrap().order_hash));
        // }
    }

    Ok(None)
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