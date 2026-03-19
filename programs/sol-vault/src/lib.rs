use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("F4nmdd2VdE1fnc2wSphDm4rAepUxULovXkWtN3m1J863");

#[program]
pub mod sol_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_state =&mut ctx.accounts.vault_state;
        vault_state.owner = ctx.accounts.owner.key();
        vault_state.bump = ctx.bumps.vault;
        msg!("Vault initialized! Owner: {}", vault_state.owner);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount:u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
             system_program::Transfer{
                from:ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
             },
        );
        system_program::transfer(cpi_ctx, amount)?;
        msg!("Deposited {} lamports into vault", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount:u64) -> Result<()> {
        let vault_state = &ctx.accounts.vault_state;
        let owner_key =  vault_state.owner;
        let bump = vault_state.bump;

        let seeds: &[&[u8]] = &[
            b"vault",
            owner_key.as_ref(),
            &[bump],
        ];

        let signer_seeds = &[seeds];

        let cpi_ctx =CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
             system_program::Transfer{
                from : ctx.accounts.vault.to_account_info(),
                to : ctx.accounts.owner.to_account_info(),
             },
             signer_seeds,
        );
        system_program::transfer(cpi_ctx, amount)?;
         msg!("Withdrew {} lamports from vault", amount);

        Ok(())

    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 1,
        seeds = [b"vault_state", owner.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>, 


}

#[derive(Accounts)]

pub struct Deposit<'info> {
     #[account(mut)]
     pub signer: Signer<'info>,

     #[account(
        mut,
        seeds = [b"vault", vault_state.owner.as_ref()],
        bump
     )]

     pub vault: SystemAccount<'info>,

     #[account(
        seeds = [b"vault_state", vault_state.owner.as_ref()],
         bump = vault_state.bump
     )]

    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Withdraw<'info> {
     #[account(mut)]
     pub owner: Signer<'info>,

     #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

     #[account(
        seeds = [b"vault_state", owner.key().as_ref()],
        bump = vault_state.bump,
        has_one = owner
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey, 
    pub bump: u8,   
}

