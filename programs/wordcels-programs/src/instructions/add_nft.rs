use anchor_lang::prelude::*;

use anchor_spl::token::TokenAccount;

use crate::state::{GlobalState, NftVaultInfo};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddNftParams {
    /// The keccak256 hash for the NFT's word.
    pub hash: [u8; 32],
    /// The amount of lamports required to deposit for unlock.
    pub amount: u64,
    pub signer: Option<Pubkey>,
}

/// Adds an NFT into the vault.
#[derive(Accounts)]
#[instruction(params: AddNftParams)]
pub struct AddNft<'info> {
    #[account(init, payer = payer)]
    nft_vault: Account<'info, NftVaultInfo>,
    #[account(seeds = [b"global"], bump)]
    global_state: Account<'info, GlobalState>,
    #[account(
        constraint = nft_token_account.owner == global_state.key(),
        constraint = nft_token_account.delegate.is_none(),
        constraint = nft_token_account.close_authority.is_none() || nft_token_account.close_authority.contains(&global_state.key())
    )]
    nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddNft>, params: AddNftParams) -> ProgramResult {
    let nft_vault = &mut ctx.accounts.nft_vault;

    nft_vault.account = ctx.accounts.nft_token_account.key();
    nft_vault.hash = params.hash;
    if params.signer.is_some() {
        nft_vault.signer = params.signer.unwrap();
    }
    nft_vault.amount = params.amount;

    Ok(())
}
