use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
  #[msg("Admin is the same")]
  AdminIsSame,

  #[msg("Invalid protocol fee percentage")]
  InvalidProtocolFeePercentage,

  #[msg("Invalid tip percentage")]
  InvalidTipPercentage,

  #[msg("Operator is the same")]
  OperatorIsSame,

  #[msg("Deposit deadline must be in the future")]
  DepositDeadlineMustBeInTheFuture,

  #[msg("Trade deadline must be after deposit deadline")]
  TradeDeadlineMustBeAfterDepositDeadline,

  #[msg("Campaign Creator not match")]
  CampaignCreatorNotMatch,

  #[msg("Campaign Index not match")]
  CampaignIndexNotMatch,

  #[msg("Campaign reached goal")]
  CampaignReachedGoal,

  #[msg("Campaign deposit deadline has passed")]
  CampaignDepositDeadlinePassed,

  #[msg("Campaign deposit deadline not passed")]
  CampaignDepositDeadlineNotPassed,

  #[msg("Campaign already claimed fund")]
  CampaignAlreadyClaimed,

  #[msg("Campaign donation goal not reached")]
  CampaignDonationGoalNotReached,

  #[msg("Campaign token already created")]
  TokenAlreadyCreated,

  #[msg("Campaign token not created")]
  TokenNotCreated,

  #[msg("Campaign already bought token")]
  CampaignAlreadyBoughtToken,

  #[msg("Campaign already sold all token")]
  CampaignAlreadySoldAllToken,

  #[msg("Campaign trade deadline not passed")]
  CampaignTradeDeadlineNotPassed,

  #[msg("Claim amount exceeds total token bought")]
  ExceedsTotalTokenBought,
}
