use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod donation_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let donation_account = &mut ctx.accounts.donation_account;
        donation_account.owner = *ctx.accounts.user.to_account_info().key;
        Ok(())
    }

    pub fn make_donation(
        ctx: Context<Donation>,
        lamports: u64,
    ) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
        let tx = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.donation_account.key(),
            lamports,
        );

        anchor_lang::solana_program::program::invoke(
            &tx,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.donation_account.to_account_info(),
            ],
        )
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
    ) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
        let tx = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donation_account.key(),
            &ctx.accounts.donation_account.owner,
            ctx.accounts.donation_account.to_account_info().lamports(),
        );
        anchor_lang::solana_program::program::invoke(
            &tx,
            &[
                ctx.accounts.donation_account.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub donation_account: Account<'info, DonationAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DonationAccount {
    pub donators_list: Vec<Donator>,
    pub owner: Pubkey,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Donator {
    pub user: Pubkey,
}

#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}
