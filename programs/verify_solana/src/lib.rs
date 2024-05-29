use anchor_lang::prelude::*;

declare_id!("3BuQSyYwEV43R6HkeqSi7xwDRjRuAU2eCD2s87E3UAeA");

#[program]
pub mod verify_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn test1(_ctx: Context<Initialize>) -> Result<u16> {
        Ok(1234)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
