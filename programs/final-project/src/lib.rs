use anchor_lang::prelude::*;

declare_id!("CLCBzkzZr1CCRfwmNXmTSYiZxSR1M9HV9CrBEKWFZvgw");

#[program]
pub mod final_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
