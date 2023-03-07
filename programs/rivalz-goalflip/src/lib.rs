pub mod randomness_tools;
pub mod recent_blockhashes;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

declare_id!("AfkTvJCbHWTYwrsxVqVW7sxTbgp561ExzrzXS6BuLqQT");

#[program]
pub mod rivalz_goalflip {
    use anchor_lang::solana_program;

    use super::*;

    pub fn play(ctx: Context<Play>, position: Position, corner: Corner) -> Result<()> {
        let game = &mut ctx.accounts.game;

        // Check if the game is already completed
        if game.status == Status::Completed {
            return Err(ErrorCode::GameAlreadyCompleted.into());
        }

        // Set the player's position and corner
        game.position = position;
        game.corner = corner;

        let randomness =
            recent_blockhashes::last_blockhash_accessor(&ctx.accounts.recent_blockhashes)?;

        // Select a random corner
        let random_corner: Corner = match randomness_tools::expand(randomness, 2) {
            0 => Corner::TopLeft,
            _ => Corner::TopRight,
        };

        // Get the recipient account
        let to = game.player;
        let amount = 100000000;

        // Transfer Solana lamports from the user account to the game account
        let transfer_to_game = solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &game.to_account_info().key,
            amount,
        );
        solana_program::program::invoke_signed(
            &transfer_to_game,
            &[
                ctx.accounts.authority.to_account_info(),
                game.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"transfer".as_ref(), &[0u8; 32]][..]],
        )?;

        // Transfer Solana lamports from the game account to the player account or the program owner account
        let transfer_to_winner_or_owner = if game.position == Position::Goalkeeper {
            match game.corner {
                Corner::TopLeft => None,
                _ => Some(to),
            }
        } else {
            match game.corner {
                Corner::TopRight => None,
                _ => Some(to),
            }
        };

        msg!(
            "transfer_to_winner_or_owner: {:?}",
            transfer_to_winner_or_owner
        );
        msg!(
            "game.to_account_info().owner: {:?}",
            game.to_account_info().owner
        );
        msg!(
            "game.to_account_info().key: {:?}",
            game.to_account_info().key
        );
        msg!("amount: {:?}", amount);

        // TODO:
        // 1. Fix Issue:     Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: An account required by the instruction is missing
        // 2. Fix Issue:     Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Program failed to completeError: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Cross-program invocation with unauthorized signer or writable account

        if let Some(winner) = transfer_to_winner_or_owner {
            let transfer_to_winner = solana_program::system_instruction::transfer(
                &game.to_account_info().key,
                &winner,
                amount,
            );
            solana_program::program::invoke_signed(
                &transfer_to_winner,
                &[
                    game.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[&[b"transfer".as_ref(), &[0u8; 32]][..]],
            )?;
        } else {
            let transfer_to_owner = solana_program::system_instruction::transfer(
                &game.to_account_info().key,
                &game.to_account_info().owner,
                amount,
            );
            solana_program::program::invoke_signed(
                &transfer_to_owner,
                &[
                    game.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[&[b"transfer".as_ref(), &[0u8; 32]][..]],
            )?;
        }

        // Update the game status and random corner
        game.status = Status::Completed;
        game.corner = random_corner;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 100,
    )]
    pub game: Account<'info, Game>,
    /// CHECK: ?
    #[account(mut)]
    authority: Signer<'info>,
    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
    /// CHECK: ?
    system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub position: Position,
    pub corner: Corner,
    pub payment: u64,
    pub status: Status,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Position {
    Goalkeeper,
    Forward,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Corner {
    TopLeft,
    TopRight,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Status {
    InProgress,
    Completed,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Game is already completed")]
    GameAlreadyCompleted,
}
