use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{Hash, hash};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use anchor_spl::token::{transfer as token_transfer, Token, TokenAccount, Transfer};

use std::str::FromStr;

use crate::error::ErrorCode;
use crate::state::{GlobalState, NftVaultInfo};
use crate::VAULT_FUNDS;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UnlockNftWithHashParams {
    /// The word.
    pub word: String,
}

/// For NFTs without a signer, one can unlock the NFT by passing the correct
/// word. The word is checked against the hash.
#[derive(Accounts)]
#[instruction(params: UnlockNftWithHashParams)]
pub struct UnlockNftWithHash<'info> {
    #[account(mut)]
    nft_vault: Account<'info, NftVaultInfo>,
    #[account(seeds = [b"global"], bump)]
    global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        constraint = nft_token_account.owner == global_state.key(),
        constraint = nft_token_account.delegate.is_none(),
        constraint = nft_token_account.close_authority.is_none() || nft_token_account.close_authority.contains(&global_state.key())
    )]
    nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    user_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = Pubkey::from_str(VAULT_FUNDS).unwrap())]
    vault_funds: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

impl<'info> UnlockNftWithHash<'info> {
    pub fn transfer_ctx<'a, 'b, 'c>(
        &self,
        signer: &'a [&'b [&'c [u8]]],
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.nft_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.global_state.to_account_info(),
        };

        CpiContext::new_with_signer(cpi_program, cpi_accounts, signer)
    }
}

pub fn handler(ctx: Context<UnlockNftWithHash>, params: UnlockNftWithHashParams) -> ProgramResult {
    let nft_vault = &mut ctx.accounts.nft_vault;

    if nft_vault.signer != Pubkey::default() {
        return Err(ErrorCode::MustClaimWithSigner.into());
    }

    let word_hash = hash(params.word.as_bytes());
    if Hash::new_from_array(nft_vault.hash) != word_hash {
        return Err(ErrorCode::IncorrectHash.into());
    }

    let transfer_ix = transfer(
        ctx.accounts.user.key,
        ctx.accounts.vault_funds.key,
        nft_vault.amount,
    );

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault_funds.to_account_info(),
        ],
    )?;

    let seeds: [&[u8]; 2] = [b"global", &[ctx.accounts.global_state.nonce]];
    let signer = &[&seeds[..]];
    let transfer_nft_ctx = ctx.accounts.transfer_ctx(signer);
    token_transfer(transfer_nft_ctx, 1)?;

    Ok(())
}
