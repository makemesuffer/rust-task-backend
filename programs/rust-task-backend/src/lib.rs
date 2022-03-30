use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod rust_task_backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let donators = &mut ctx.accounts.donators;
        donators.total_donators = 0;
        Ok(())
    }

    pub fn send_transaction(ctx: Context<SendTransaction>, amount: u64) -> ProgramResult {
        let donators = &mut ctx.accounts.donators;
        let user = &mut ctx.accounts.user;

        system_instruction::transfer(
            user.to_account_info().key,
            donators.to_account_info().key,
            amount,
        );

        let donator = DonatorStruct {
            amount: amount,
            address: *user.to_account_info().key,
        };

        donators.total_donators += 1;
        donators.donators_list.push(donator);

        Ok(())
    }

    pub fn receive_transaction(ctx: Context<ReceiveTransaction>) -> ProgramResult {
        let donators = &mut ctx.accounts.donators;
        let owner = donators.to_account_info().owner;
        let user = &mut ctx.accounts.user;

        if user.to_account_info().key != &owner.key() {
            Err("You are not the owner!")
        } else {
            system_instruction::transfer(
                donators.to_account_info().key,
                user.to_account_info().key,
                donators.donators_list[0].amount,
                // donators.to_account_info().lamports(),
            );
            Ok(())
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user,space=9000)]
    pub donators: Account<'info, Donators>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendTransaction<'info> {
    #[account(mut)]
    pub donators: Account<'info, Donators>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReceiveTransaction<'info> {
    #[account(mut)]
    pub donators: Account<'info, Donators>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DonatorStruct {
    pub amount: u64,
    pub address: Pubkey,
}

#[account]
pub struct Donators {
    pub total_donators: u64,
    pub donators_list: Vec<DonatorStruct>,
}
