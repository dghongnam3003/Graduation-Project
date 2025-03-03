use anchor_lang::prelude::*;

use crate::{
  constants::{CAMPAIGN_SEED, CONFIG_SEED, PERCENTAGE_DENOMINATOR, TREASURY_SEED},
  errors::CustomError,
  state::{Campaign, Config, Treasury},
  utils::withdraw_sol,
  events::ClaimedFundEvent,
};

#[derive(Accounts)]
pub struct ClaimFundRaised<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    seeds = [CONFIG_SEED],
    bump = config.bump,
  )]
  pub config: Account<'info, Config>,

  #[account(
    mut,
    seeds = [TREASURY_SEED],
    bump = treasury.bump,
  )]
  pub treasury: Account<'info, Treasury>,

  #[account(
    mut,
    seeds = [CAMPAIGN_SEED, creator.key().as_ref(), campaign_account.index.to_le_bytes().as_ref()],
    bump = campaign_account.bump,
    has_one = creator,
  )]
  pub campaign_account: Account<'info, Campaign>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn claim_fund_raised(ctx: Context<ClaimFundRaised>) -> Result<()> {
  let config: &Account<'_, Config> = &ctx.accounts.config;
  let campaign_account = &mut ctx.accounts.campaign_account;
  let rent = &ctx.accounts.rent;

  let now = Clock::get()?.unix_timestamp;
  if campaign_account.deposit_deadline > 0 {
    require!(
      now > campaign_account.deposit_deadline,
      CustomError::CampaignDepositDeadlineNotPassed
    );
  }
  require!(
    campaign_account.mint == Pubkey::default(),
    CustomError::TokenAlreadyCreated
  );
  require!(
    !campaign_account.is_claimed_fund,
    CustomError::CampaignAlreadyClaimed
  );

  campaign_account.claim_fund()?;

  let campaign_balance = campaign_account.get_lamports();
  let minimum_rent_exemption = rent.minimum_balance(Campaign::LEN);
  let deposited_balance = campaign_balance
    .checked_sub(minimum_rent_exemption)
    .unwrap();
  let fee = deposited_balance
    .checked_mul(config.protocol_fee_percentage as u64)
    .unwrap()
    .checked_div(PERCENTAGE_DENOMINATOR as u64)
    .unwrap();

  withdraw_sol(
    &campaign_account.to_account_info(),
    &ctx.accounts.creator,
    deposited_balance.checked_sub(fee).unwrap(),
  )?;
  withdraw_sol(
    &campaign_account.to_account_info(),
    &ctx.accounts.treasury.to_account_info(),
    fee,
  )?;

  emit!(ClaimedFundEvent {
    creator: *ctx.accounts.creator.key,
    campaign_index: campaign_account.index,
    claimed_amount: deposited_balance.checked_sub(fee).unwrap(),
    timestamp: now,
  });

  Ok(())
}
