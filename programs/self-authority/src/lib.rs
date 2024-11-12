use anchor_lang::prelude::*;

declare_id!("9Wa4scbexRG8A6DSMAakxckyy3EsCnWLtFAFzNAbKkkK");

#[program]
pub mod self_authority {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
