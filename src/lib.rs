use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
/// You have in here the price and the 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NFTAccount {
    /// number of greetings
    pub percent_royalty: u32,
    pub price: f64,
}


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the nft_list program was loaded into
    accounts: &[AccountInfo], // The account of the nft_owner
    instruction_data: &[u8], // The instruction data for the operations
) -> ProgramResult {
    // Accounts its an iterable
    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, program_id);
    }

    let nft_instructions_data = NFTAccount::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    // Data algorithm for storing data related to nft price into account 
    
    msg!("Attempting save data.");


    Ok(())
}