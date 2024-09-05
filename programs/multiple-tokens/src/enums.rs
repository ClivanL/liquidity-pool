use anchor_lang::prelude::*;
use crate::errors::*;
use core::str::FromStr;

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