pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9VsxokCtBWQJvbXz9YL2F8Tc441secNScqHid9pj83Qj");

#[program]
pub mod nft_staking_anchor {
    use super::*;
}
