use anchor_lang::prelude::*;

use crate::{
  constants::CREATOR_SEED, state::Creator,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitCreator<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    init,
    seeds = [CREATOR_SEED, creator.key().as_ref()],
    bump,
    payer = creator,
    space = Creator::LEN,
  )]
  pub creator_account: Account<'info, Creator>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn init_creator( ctx: Context<InitCreator>, bump: u8) -> Result<()> {
  let creator_account = &mut ctx.accounts.creator_account;
  creator_account.init(bump, *ctx.accounts.creator.key)
}