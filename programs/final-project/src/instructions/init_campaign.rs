use anchor_lang::prelude::*;

use crate::{
  constants::{CAMPAIGN_SEED, CONFIG_SEED, CREATOR_SEED},
  errors::CustomError,
  state::{Campaign, Config, Creator},
  utils::deposit_sol,
  events::CreatedCampaignEvent,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitCampaign<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    mut,
    seeds = [CONFIG_SEED],
    bump = config.bump,
  )]
  pub config: Account<'info, Config>,

  #[account(
    mut,
    seeds = [CREATOR_SEED, creator.key().as_ref()],
    bump = creator_account.bump,
    has_one = creator,
  )]
  pub creator_account: Account<'info, Creator>,

  #[account(
    init,
    seeds = [CAMPAIGN_SEED, creator.key().as_ref(), creator_account.last_campaign_index.to_le_bytes().as_ref()],
    bump,
    payer = creator,
    space = Campaign::LEN
  )]
  pub campaign_account: Account<'info, Campaign>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

#[allow(clippy::too_many_arguments)]
pub fn init_campaign(
  ctx: Context<InitCampaign>,
  bump: u8,
  name: String,
  symbol: String,
  uri: String,
  deposit_deadline: i64,
  trade_deadline: i64,
  donation_goal: u64,
) -> Result<()> {
  let config = &ctx.accounts.config;
  let creator_account = &mut ctx.accounts.creator_account;
  let campaign_account = &mut ctx.accounts.campaign_account;

  let now = Clock::get()?.unix_timestamp;
  if deposit_deadline > 0 {
    require!(
      now < deposit_deadline,
      CustomError::DepositDeadlineMustBeInTheFuture
    );
  }
  if trade_deadline > 0 {
    require!(
      trade_deadline > deposit_deadline,
      CustomError::TradeDeadlineMustBeAfterDepositDeadline
    );
  }

  campaign_account.init(
    bump,
    *ctx.accounts.creator.key,
    creator_account.last_campaign_index,
    name.clone(),
    symbol.clone(),
    uri.clone(),
    deposit_deadline,
    trade_deadline,
    donation_goal,
  )?;
  creator_account.increment_last_campaign_index()?;

  // Initial deposit
  deposit_sol(
    &ctx.accounts.creator,
    &campaign_account.to_account_info(),
    config.initial_deposit_amount,
    &ctx.accounts.system_program,
  )?;

  // Emit event
  emit!(CreatedCampaignEvent {
    creator: *ctx.accounts.creator.key,
    campaign_index: creator_account.last_campaign_index,
    name,
    symbol,
    uri,
    deposit_deadline,
    trade_deadline,
    donation_goal,
    timestamp: now,
  });

  Ok(())
}
