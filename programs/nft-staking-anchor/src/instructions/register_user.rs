use anchor_lang::prelude::*;

use crate::{UserAccount, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR as usize + UserAccount::INIT_SPACE,
        seeds=[
            b"user_account",
            user.key().as_ref()
        ],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterUser<'info> {
    pub fn register(&mut self, bumps: &RegisterUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            amount_staked: 0,
            points: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }
}
