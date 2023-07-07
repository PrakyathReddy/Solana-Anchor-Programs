use solana_program::{
    account_info::(AccountInfo),
    entrypoint,
    entrypoint::ProgramResult,
    msg
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub counter: u32,
}

// declare and exit the program's entrypoint
entrypoint!(process_instruction);

// program's entrypoint implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("program starts here");

    Ok(())

    // Iterating a accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account
    let account = next_account_info(accounts.iter)?;

    // the account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("account does not have the correct program id");
        return 
    Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the value of the counter value of the account\
    let mut counter_account = CounterAccount:try_from_slice(&account.data.borrow())?;
    counter_account.counter += 1;
    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;



    
}