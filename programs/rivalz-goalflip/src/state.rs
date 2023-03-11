use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

#[account]
pub struct GameMatch {
    pub game: Pubkey,
    //32
    pub player: Pubkey,
    //32
    pub won: bool,
    //1
    pub position: Position,
    //1
    pub player_corner: Corner,
    //1
    pub bet_amount: u64,
    //8
    pub won_amount: u64,
    //8
    pub match_date: u64, //8
    pub status: GameMatchStatus,
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
    //32
    pub latest_match: Pubkey,
    //32
    //8
    pub name: Vec<u8>, //  30
}

#[derive(Accounts)]
pub struct PlayContext<'info> {
    #[account(
    init,
    payer = player,
    space = 8 + 64 + 27 + 1,
    )]
    pub game_match: Account<'info, GameMatch>,

    #[account(mut)]
    pub game: Account<'info, Game>,

    /// CHECK: ?
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResultGameMatchContext<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub game_match: Account<'info, GameMatch>,

    /// CHECK: we will check this in the instruction
    #[account(mut)]
    pub player: AccountInfo<'info>,

    /// CHECK: we will check this in the instruction
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GameContext<'info> {
    #[account(
    init,
    payer = admin,
    space = 8 + 79 + 32,
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Position {
    Goalkeeper,
    Forward,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Corner {
    Left,
    Right,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum GameMatchStatus {
    Pending,
    Won,
    Lost,
}

pub fn parse_position(s: &str) -> Option<Position> {
    match s {
        "Goalkeeper" => Some(Position::Goalkeeper),
        "Forward" => Some(Position::Forward),
        _ => None,
    }
}

pub fn parse_corner(s: &str) -> Option<Corner> {
    match s {
        "Left" => Some(Corner::Left),
        "Right" => Some(Corner::Right),
        _ => None,
    }
}
