use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
    CreateMetadataAccountsV3, Metadata,
};
use anchor_spl::token::{
    self, initialize_account, mint_to, Mint, MintTo, SyncNative, Token, TokenAccount, Transfer,
};
use mpl_token_metadata::types::DataV2;
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;
use std::collections::HashMap;
use std::str::FromStr;

declare_id!("CyTSYotcgX7oMNEbX7G13ynfjMiMhFDXQ1baqA5rY5M6");

#[program]
mod dedns {
    use super::*;
    /*
        Mint So11111111111111111111111111111111111111112
        Seed "QeNB"
        Signer ATA EiAuui6kX5NBb6HyZG1PsnL8T8fmbu1GVQSnoNvUSvq5
        PDA ELv4hGedfB5D3WLPaLnTCtep9duAjbZe4qcya59BKUZT
        ATA 7r9iafubynkTGWbcpGjWFcdRz3sW5qbTobuF6qLszB5r 
    */
    pub fn fund_lease(
        ctx: Context<FundLease>,
        lamports_per_minute: u64,
        duration_minutes: u64,
        hostname: String,
    ) -> Result<()> {
        let payer = &ctx.accounts.payer;
        // Transfer SOL to the wSOL mint account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &payer.key(),
            &ctx.accounts.wsol_account.key(),
            lamports_per_minute * duration_minutes,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                payer.to_account_info(),
                ctx.accounts.wsol_account.to_account_info(),
            ],
        )?;
        msg!("Transferred to ATA");
        {
            token::sync_native(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SyncNative {
                    account: ctx.accounts.wsol_account.to_account_info(),
                },
            ))?;
            msg!("Synchronized Native token balance.");
        }
        {
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.wsol_account.to_account_info(),
                        to: ctx.accounts.lease_ata_account.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info(),
                    },
                ),
                lamports_per_minute * duration_minutes,
            )?;
            msg!("Transferred to ATA");
        }
        {
            token::sync_native(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SyncNative {
                    account: ctx.accounts.wsol_account.to_account_info(),
                },
            ))?;
            msg!("Synchronized Native token balance.");
        }
        msg!(
            "Wrapped {} lamports into wSOL at {}",
            lamports_per_minute * duration_minutes,
            ctx.accounts.wsol_mint.key()
        );
        Ok(())
    }

    pub fn pay_from_lease_balance(ctx: Context<PayFromLeaseBalance>, amount: u64) -> Result<()> {
        let pda_bump = ctx.bumps.program_authority;
        // TODO need to check proof confirmations account
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sender_wsol_account.to_account_info(),
                    to: ctx.accounts.recipient_wsol_account.to_account_info(),
                    authority: ctx.accounts.program_authority.to_account_info(),
                },
                &[&[b"DEDNS", &[pda_bump]]],
            ),
            amount,
        )?;

        msg!(
            "Transferred {} lamports worth of wSOL from {} to {}",
            amount,
            ctx.accounts.sender_wsol_account.key(),
            ctx.accounts.recipient_wsol_account.key(),
        );
        Ok(())
    }

    pub fn withdraw_from_lease_balance(
        ctx: Context<WithdrawFromLeaseBalance>,
        amount: u64,
    ) -> Result<()> {
        let pda_bump = ctx.bumps.program_authority;
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sender_wsol_account.to_account_info(),
                    to: ctx.accounts.recipient_wsol_account.to_account_info(),
                    authority: ctx.accounts.program_authority.to_account_info(),
                },
                &[&[b"DEDNS", &[pda_bump]]],
            ),
            amount,
        )?;

        msg!(
            "Withdrew {} lamports worth of wSOL from {} to {}",
            amount,
            ctx.accounts.sender_wsol_account.key(),
            ctx.accounts.recipient_wsol_account.key(),
        );
        Ok(())
    }

    pub fn mint_lease_NFT(ctx: Context<MintLeaseNFT>, symbol: String) -> Result<()> {
        let pda_bump: u8 = ctx.bumps.program_authority;
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.associated_token_account.to_account_info(),
                    authority: ctx.accounts.program_authority.to_account_info(),
                },
                &[&[b"DEDNS", &[pda_bump]]],
            ),
            1,
        ).unwrap();
        Ok(())
    }

    // TODO: pub fn register_domain

}

#[account]
pub struct Domain {
    name: String,
    provider_pubkey: Pubkey,
}

#[derive(Accounts)]
pub struct FundLease<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = 256,
        payer = payer,
        seeds = [
            b"DEDNS", 
        ],
        bump
    )]
    pub pda: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = wsol_mint,
        associated_token::authority = payer
    )]
    pub wsol_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wsol_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = wsol_mint,
        associated_token::authority = pda
    )]
    pub lease_ata_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct WithdrawFromLeaseBalance<'info> {
    #[account(mut)]
    pub lease_owner: Signer<'info>,
    #[account(mut)]
    pub wsol_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = wsol_mint,
        associated_token::authority = program_authority
    )]
    pub sender_wsol_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lease_owner,
        associated_token::mint = wsol_mint,
        associated_token::authority = lease_owner
    )]
    pub recipient_wsol_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        seeds = [
            b"DEDNS",
        ],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayFromLeaseBalance<'info> {
    #[account(mut)]
    pub sender_authority: Signer<'info>,
    #[account(mut)]
    pub lease_owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub recv_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub wsol_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = wsol_mint,
        associated_token::authority = program_authority
    )]
    pub sender_wsol_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender_authority,
        associated_token::mint = wsol_mint,
        associated_token::authority = recv_authority
    )]
    pub recipient_wsol_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        mut,
        seeds = [
            b"DEDNS",
        ],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintLeaseNFT<'info> {
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = program_authority,
        mint::freeze_authority = program_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    // TODO: Add domain account check
    #[account(
        mut,
        seeds = [
            b"DEDNS",
        ],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RegisterDomain<'info> {
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = program_authority,
        mint::freeze_authority = program_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 128,
    )]
    pub domain_account: Account<'info, Domain>,
    #[account(
        mut,
        seeds = [
            b"DEDNS",
        ],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
