use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // Abigen::new("ERC721", "abi/erc721.json")?
    //     .generate()?
    //     .write_to_file("src/abi/erc721.rs")?;
    // Abigen::new("BLUR_v2", "abi/blur_v2.json")?
    // .generate()?
    // .write_to_file("src/abi/blur_v2.rs")?;
    // Abigen::new("BLUR_POOL", "abi/blur_pool.json")?
    // .generate()?
    // .write_to_file("src/abi/blur_pool.rs")?;
   
    // Abigen::new("seaport", "abi/seaport.json")?
    // .generate()?
    // .write_to_file("src/abi/seaport.rs")?;
    Abigen::new("chainlink_feed_registry", "abi/chainlink_feed_registry.json")?
    .generate()?
    .write_to_file("src/abi/chainlink_feed_registry.rs")?;

    Ok(())
}
