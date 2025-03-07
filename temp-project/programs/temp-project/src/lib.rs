use anchor_lang::prelude::*;

declare_id!("4Qno9PmRC5RV5yjaM8zPmTps3TH5T1PhLDqrDdqdYscH");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
