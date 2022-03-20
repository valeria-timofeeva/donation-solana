use anchor_lang::prelude::*;

declare_id!("3vKFQUmdGGiHR6hK3tQ1JJnD3YCF4b4ktDUutgCsgScG");

#[program]
pub mod donation_solana {
    use super::*;

    //initialize initial program state
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let donation_account = &mut ctx.accounts.donation_account;
        donation_account.owner = *ctx.accounts.user.to_account_info().key;
        Ok(())
    }

    //sent donate
    pub fn make_donation(
        ctx: Context<Donation>,
        lamports: u64,
    ) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
        let donation_account = &mut ctx.accounts.donation_account;
        donation_account
            .donators_list
            .push(ctx.accounts.user.key());

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.donation_account.key(),
            lamports,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.donation_account.to_account_info(),
            ],
        )
    }

    //withdraw all donates to owner
    pub fn withdraw(ctx: Context<Withdraw>) -> std::result::Result<(), ProgramError> {
        if ctx.accounts.user.key() != ctx.accounts.donation_account.owner {
            return Err(ProgramError::IllegalOwner);
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donation_account.key(),
            &ctx.accounts.donation_account.owner,
            ctx.accounts.donation_account.to_account_info().lamports(),
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.donation_account.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )
    }

    //return list of donators
    pub fn get_all_donators(ctx: Context<AllDonators>) -> Result<Vec<Pubkey>> {
        let mut donators_list: Vec<Pubkey> = Vec::new();
        for x in ctx.accounts.donation_account.donators_list.iter() {
            donators_list.push(*x);
        }
        Ok(donators_list)
    }
}

//data for initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub donation_account: Account<'info, DonationAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//main program account that hold business logic
#[account]
pub struct DonationAccount {
   // pub donators_list: HashSet<Pubkey>,
    pub donators_list: Vec<Pubkey>,
    pub owner: Pubkey,
}

//donate function data context
#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}

//withdraw function data context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AllDonators<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}
