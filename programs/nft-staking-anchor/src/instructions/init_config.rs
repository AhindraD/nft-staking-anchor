use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{StakeConfig, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space=ANCHOR_DISCRIMINATOR as usize + StakeConfig::INIT_SPACE,
        seeds=[
            b"stake_config",
        ],
        bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer=admin,
        seeds=[
            b"rewards",
            config.key().as_ref()
        ],
        bump,
        mint::authority=config,
        mint::decimals=6
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init(
        &mut self,
        freeze_period: u32,
        points_per_stake: u8,
        max_stake: u8,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            freeze_period,
            points_per_stake,
            max_stake,
            rewards_mint_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
