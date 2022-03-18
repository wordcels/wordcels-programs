use anchor_lang::prelude::*;

use crate::state::GlobalState;

/// Initializes the global state.
#[derive(Accounts)]
#[instruction(params: ())]
pub struct InitGlobalState<'info> {
    #[account(init, seeds = [b"global"], bump, payer = payer)]
    global_state: Account<'info, GlobalState>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGlobalState>) -> ProgramResult {
    let global_state = &mut ctx.accounts.global_state;

    global_state.nonce = *ctx.bumps.get("global_state").unwrap();

    Ok(())
}
