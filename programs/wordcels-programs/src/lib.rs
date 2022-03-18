use anchor_lang::prelude::*;

declare_id!("ELP7c23X8MgeGZV3i2HxYz1bzFjcKpqApPVsCvUuaTN2");

pub const VAULT_FUNDS: &str = "2kMAyG2WyibnFVCPxnqKmFaCvfZekMitezR7rTBGtK81";

pub mod error;
pub mod instructions;
pub mod state;

use crate::instructions::*;

#[program]
pub mod wordcels_programs {
    use super::*;
    pub fn init_global_state(ctx: Context<InitGlobalState>) -> ProgramResult {
        init_global_state::handler(ctx)
    }

    pub fn add_nft(ctx: Context<AddNft>, params: AddNftParams) -> ProgramResult {
        add_nft::handler(ctx, params)
    }

    pub fn unlock_nft_with_hash(
        ctx: Context<UnlockNftWithHash>,
        params: UnlockNftWithHashParams,
    ) -> ProgramResult {
        unlock_nft_with_hash::handler(ctx, params)
    }

    pub fn unlock_nft_with_signer(ctx: Context<UnlockNftWithSigner>) -> ProgramResult {
        unlock_nft_with_signer::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
