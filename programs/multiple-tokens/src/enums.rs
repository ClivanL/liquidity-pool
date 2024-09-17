use anchor_lang::prelude::*;
use crate::errors::*;
use std::str::FromStr;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TokenPair {
    TokenATokenB,
    TokenATokenC,
    TokenATokenD,
    TokenATokenE,
    TokenBTokenC,
    TokenBTokenD,
    TokenBTokenE,
    TokenCTokenD,
    TokenCTokenE,
    TokenDTokenE
}

impl FromStr for TokenPair {
    type Err = CustomError;
    
    fn from_str(s:&str) -> std::result::Result<TokenPair, CustomError> {
        match s.to_lowercase().as_str(){
            "ab" => Ok(TokenPair::TokenATokenB),
            "ac" => Ok(TokenPair::TokenATokenC),
            "ad" => Ok(TokenPair::TokenATokenD),
            "ae" => Ok(TokenPair::TokenATokenE),
            "bc" => Ok(TokenPair::TokenBTokenC),
            "bd" => Ok(TokenPair::TokenBTokenD),
            "be" => Ok(TokenPair::TokenBTokenE),
            "cd" => Ok(TokenPair::TokenCTokenD),
            "ce" => Ok(TokenPair::TokenCTokenE),
            "de" => Ok(TokenPair::TokenDTokenE),
            _ => Err(CustomError::InvalidTokenPair.into())
        }
    }
}

// Implement fmt::Display to convert TokenPair back to a string
impl fmt::Display for TokenPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TokenPair::TokenATokenB => "ab",
            TokenPair::TokenATokenC => "ac",
            TokenPair::TokenATokenD => "ad",
            TokenPair::TokenATokenE => "ae",
            TokenPair::TokenBTokenC => "bc",
            TokenPair::TokenBTokenD => "bd",
            TokenPair::TokenBTokenE => "be",
            TokenPair::TokenCTokenD => "cd",
            TokenPair::TokenCTokenE => "ce",
            TokenPair::TokenDTokenE => "de",
        };
        write!(f, "{}", s)
    }
}

impl Space for TokenPair {
    const INIT_SPACE: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Buy,
    Sell
}

impl FromStr for Direction {
    type Err = CustomError;
    
    fn from_str(s:&str) -> std::result::Result<Direction, CustomError> {
        match s.to_lowercase().as_str(){
            "buy" => Ok(Direction::Buy),
            "sell" => Ok(Direction::Sell),
            _ => Err(CustomError::WrongDirectionInput.into())
        }
    }
}

impl Space for Direction {
    const INIT_SPACE: usize = 1;
}