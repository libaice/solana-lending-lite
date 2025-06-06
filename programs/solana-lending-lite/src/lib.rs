use anchor_lang::prelude::*;

declare_id!("BXzmx4NywU7prvi6Xz9MTELhvTkLT4g8VuNsyLWMjURs");

#[program]
pub mod solana_lending_lite {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
