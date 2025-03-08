use anchor_lang::prelude::*;

#[event]
pub struct CreatedCampaignEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub name: String,
  pub symbol: String,
  pub uri: String,
  pub deposit_deadline: i64,
  pub trade_deadline: i64,
  pub donation_goal: u64,
  pub timestamp: i64,
}

#[event]
pub struct ClaimedFundEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub claimed_amount: u64,
  pub timestamp: i64,
}

#[event]
pub struct DonatedFundEvent {
  pub campaign_index: u64,
  pub donated_amount: u64,
  pub timestamp: i64,
}

#[event]
pub struct CreatedCampaignTokenEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub mint: Pubkey,
  pub bought_amount: u64,
  pub timestamp: i64,
}

#[event]
pub struct ClaimedTokenEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub claimed_amount: u64,
  pub mint: Pubkey,
  pub timestamp: i64,
}

#[event]
pub struct SoldCampaignTokenEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub mint: Pubkey,
  pub timestamp: i64,
}

#[event]
pub struct ClaimableTokenAmountUpdatedEvent {
  pub creator: Pubkey,
  pub campaign_index: u64,
  pub mint: Pubkey,
  pub new_amount: u64,
  pub timestamp: i64,
}