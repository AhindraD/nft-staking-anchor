use anchor_lang::prelude::*;

// use crate::ANCHOR_DISCRIMINATOR;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub freeze_period: u32,
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub rewards_mint_bump: u8,
    //instead of storing the mint pubkey - just storing bump as PDA derived for the reawrds Mint is deterministic
    pub bump: u8,
}

// impl Space for StakeConfig {
//     const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR as usize + 4 + 1 + 1 + 1 + 1;
// }
