use anchor_lang::prelude::*;

use crate::{constants::CONFIG_SEED, state::Config, errors::CustomError};

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,

  #[account(
    mut,
    seeds = [CONFIG_SEED],
    bump = config.bump,
    has_one = admin,
  )]
  pub config: Account<'info, Config>,
}

pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
  let config = &mut ctx.accounts.config;

  require!(new_admin != config.admin, CustomError::AdminIsSame);

  config.admin = new_admin;

  Ok(())
}