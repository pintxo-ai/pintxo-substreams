use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // Abigen::new("ERC721", "abi/erc721.json")?
    //     .generate()?
    //     .write_to_file("src/abi/erc721.rs")?;
    // Abigen::new("BLUR_v2", "abi/blur_v2.json")?
    // .generate()?
    // .write_to_file("src/abi/blur_v2.rs")?;
    Abigen::new("BLUR_POOL", "abi/blur_pool.json")?
    .generate()?
    .write_to_file("src/abi/blur_pool.rs")?;
    Ok(())
}
