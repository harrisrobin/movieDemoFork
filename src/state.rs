use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone,
)]
pub struct MovieInfo {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String,
    pub funding: u32,
    pub recipient: String,
    // pub backers: Vec<String>,
    pub entry: u32,
}

impl Sealed for MovieInfo {}

impl IsInitialized for MovieInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
