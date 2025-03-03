use anchor_lang::prelude::*;

use crate::{
  constants::CAMPAIGN_SEED, 
  errors::CustomError, 
  state::Campaign, 
  utils::deposit_sol,
  events::DonatedFundEvent
};

#[derive(Accounts)]
#[instruction(creator: Pubkey, campaign_index: u64)]
pub struct DonateFund<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    mut,
    seeds = [CAMPAIGN_SEED, creator.as_ref(), campaign_index.to_le_bytes().as_ref()],
    bump = campaign_account.bump,
  )]
  pub campaign_account: Account<'info, Campaign>,

  pub system_program: Program<'info, System>,

  pub rent: Sysvar<'info, Rent>,
}

pub fn donate_fund(
  ctx: Context<DonateFund>,
  creator: Pubkey,
  campaign_index: u64,
  amount: u64,
) -> Result<()> {
  let campaign_account = &mut ctx.accounts.campaign_account;
  let rent = &ctx.accounts.rent;

  require!(
    campaign_account.creator == creator,
    CustomError::CampaignCreatorNotMatch
  );
  require!(
    campaign_account.index == campaign_index,
    CustomError::CampaignIndexNotMatch
  );

  let campaign_balance = campaign_account.get_lamports();
  let minimum_rent_exemption = rent.minimum_balance(Campaign::LEN);
  let deposited_balance = campaign_balance
    .checked_sub(minimum_rent_exemption)
    .unwrap();
  require!(
    deposited_balance < campaign_account.donation_goal,
    CustomError::CampaignReachedGoal
  );
  let now = Clock::get()?.unix_timestamp;
  if campaign_account.deposit_deadline > 0 {
    require!(
      now < campaign_account.deposit_deadline,
      CustomError::CampaignDepositDeadlinePassed
    );
  }

  deposit_sol(
    &ctx.accounts.signer,
    &campaign_account.to_account_info(),
    amount,
    &ctx.accounts.system_program,
  )?;

  emit!(DonatedFundEvent {
    campaign_index,
    donated_amount: amount,
    timestamp: now,
  });

  Ok(())
}
