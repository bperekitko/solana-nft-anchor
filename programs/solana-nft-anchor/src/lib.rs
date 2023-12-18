use anchor_lang::prelude::*;

declare_id!("tYKuFRz4agpQv4ZRDLUN3Bzfdfbkbzv7Tdg3ev2Ta7c");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
