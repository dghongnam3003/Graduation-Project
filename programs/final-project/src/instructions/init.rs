use anchor_lang::prelude::*;

use crate::{
  constants::{CONFIG_SEED, TREASURY_SEED},
  state::{Config, Treasury},
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Init<'info> {
  #[account(mut)]
  pub admin: Singer<'info>,

  #[account(
    init,
    seeds = [CONFIG_SEED],
    bump = bump,
    payer = admin,
    space = config::LEN,
  )]
  pub config: Account<'info, Config>,

  #[account(
    init,
    seeds = [TREASURY_SEED],
    bump = bump,
    payer = admin,
    space = treasury::LEN,
  )]
  pub treasury: Account<'info, Treasury>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn init(
  ctx: Context<Init>,
  bump: u8,
  treasury_bump: u8,
  operator: Pubkey,
  protocal_fee_percentage: u16,
  tip_percentage: u16,
) -> Result<()> {
  let config = &mut ctx.accounts.config;
  let treasury = &mut ctx.accounts.treasury;
  config.init(
    bump,
    *ctx.accounts.admin.key,
    operator,
    protocal_fee_percentage,
    tip_percentage,
  )?;
  treasury.init(treasury_bump)
}