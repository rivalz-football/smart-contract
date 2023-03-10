mod access_control;
mod errors;
pub mod randomness_tools;
pub mod recent_blockhashes;

use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

declare_id!("rivi27uFE2UGJCR2WmzddviqS6RWiTzciC8KnVp2rhi");
const ADMIN_PUBKEY: &[u8] = b"aut69244nPQ5A23MKwScxMiZxvsYeepBkNuaxK2TqSd";

#[program]
pub mod rivalz_goalflip {

    use crate::access_control::is_admin;
    use anchor_lang::solana_program;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

    use super::*;

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn init_game(ctx: Context<GameContext>) -> Result<()> {
        ctx.accounts.game.commission = 0;
        ctx.accounts.game.init_at = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game.multiplier = 2;
        ctx.accounts.game.name = b"myGame".to_vec();

        Ok(())
    }

    pub fn play(ctx: Context<PlayContext>, position: String, corner: String) -> Result<()> {
        let game_match = &mut ctx.accounts.game_match;
        let game = &mut ctx.accounts.game;

        // Check if the game is already completed
        if game_match.status == Status::Completed {
            return Err(ErrorCode::GameAlreadyCompleted.into());
        }

        game_match.position = match parse_position(&position) {
            Some(position) => position,
            None => return Err(ErrorCode::InvalidPosition.into()),
        };

        game_match.corner = match parse_corner(&corner) {
            Some(corner) => corner,
            None => return Err(ErrorCode::InvalidCorner.into()),
        };
        game_match.bump = *ctx.bumps.get("game").unwrap();

        let randomness =
            recent_blockhashes::last_blockhash_accessor(&ctx.accounts.recent_blockhashes)?;

        // Select a random corner
        let random_corner: Corner = match randomness_tools::expand(randomness, 2) {
            0 => Corner::TopLeft,
            _ => Corner::TopRight,
        };

        // Get the recipient account
        let to = game_match.player;
        let amount = LAMPORTS_PER_SOL / 10;

        // Transfer Solana lamports from the game account to the winner account or the program player account
        let (transfer_to_winner_or_bot_or_player, is_bot_winner) =
            if game_match.position == Position::Goalkeeper {
                match game_match.corner {
                    Corner::TopLeft => (None, random_corner == Corner::TopRight),
                    _ => (Some(to), random_corner == Corner::TopLeft),
                }
            } else {
                match game_match.corner {
                    Corner::TopRight => (None, random_corner == Corner::TopLeft),
                    _ => (Some(to), random_corner == Corner::TopRight),
                }
            };

        if is_bot_winner {
            let transfer_to_bot = solana_program::system_instruction::transfer(
                &game.to_account_info().key,
                &to,
                amount * game.multiplier as u64,
            );
            solana_program::program::invoke_signed(
                &transfer_to_bot,
                &[
                    ctx.accounts.authority.to_account_info(),
                    game.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[],
            )?;
        } else if let Some(winner_or_bot) = transfer_to_winner_or_bot_or_player {
            let transfer_to_winner = solana_program::system_instruction::transfer(
                &game.to_account_info().key,
                &winner_or_bot,
                amount,
            );
            solana_program::program::invoke_signed(
                &transfer_to_winner,
                &[
                    ctx.accounts.authority.to_account_info(),
                    game.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[],
            )?;
        }
        // Update the game status and random corner
        // game_match.status = Status::Completed;
        // game_match.corner = random_corner;

        Ok(())
    }
}

#[account]
pub struct GameMatch {
    pub game: Pubkey,
    pub player: Pubkey,
    pub position: Position,
    pub corner: Corner,
    pub payment: u64,
    pub status: Status,
    pub bump: u8,
}

#[account]
pub struct Game {
    pub multiplier: u8,
    // 1
    // lamport cinsinden
    pub commission: u64,
    // 8
    pub init_at: u64,
    // 8
    pub last_play_date: u64,
    //8
    pub lose_count: u64,
    //8
    pub win_count: u64,
    //8
    pub total_volume: u64,
    //8
    pub name: Vec<u8>, //  30
}

#[derive(Accounts)]
pub struct PlayContext<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 100,
        // seeds = [b"game", authority.key().as_ref()], bump
        )]
    pub game_match: Account<'info, GameMatch>,

    #[account(mut)]
    pub game: Account<'info, Game>,

    /// CHECK: ?
    #[account(mut)]
    authority: Signer<'info>,
    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GameContext<'info> {
    #[account(
    init,
    payer = admin,
    space = 8 + 79,
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    admin: Signer<'info>,
    system_program: Program<'info, System>,
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

fn parse_position(s: &str) -> Option<Position> {
    match s {
        "Goalkeeper" => Some(Position::Goalkeeper),
        "Forward" => Some(Position::Forward),
        _ => None,
    }
}

fn parse_corner(s: &str) -> Option<Corner> {
    match s {
        "TopLeft" => Some(Corner::TopLeft),
        "TopRight" => Some(Corner::TopRight),
        _ => None,
    }
}

fn parse_status(s: &str) -> Option<Status> {
    match s {
        "InProgress" => Some(Status::InProgress),
        "Completed" => Some(Status::Completed),
        _ => None,
    }
}
