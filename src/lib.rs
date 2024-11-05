pub use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 4 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the 4-byte instruction identifier
    let instruction = u32::from_le_bytes(
        instruction_data[0..4]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    match instruction {
        0 => process_register_node(program_id, accounts, &instruction_data[4..]),
        1 => process_remove_node(program_id, accounts, &instruction_data[4..]),
        2 => process_dispatch_task(program_id, accounts, &instruction_data[4..]),
        3 => process_return_answer(program_id, accounts, &instruction_data[4..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn process_register_node(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let node_id = parse_node_id(instruction_data)?;
    msg!("Registering node with nodeId: {}", node_id);
    Ok(())
}

pub fn process_remove_node(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let node_id = parse_node_id(instruction_data)?;
    msg!("Removing node with nodeId: {}", node_id);
    Ok(())
}

pub fn process_dispatch_task(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 5 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse task id and node id
    let task_id_len = instruction_data[0] as usize;
    let node_id_len = instruction_data[1 + task_id_len] as usize;
    if instruction_data.len() < 2 + task_id_len + node_id_len {
        return Err(ProgramError::InvalidInstructionData);
    }

    let task_id = &instruction_data[1..1 + task_id_len];
    let node_id = &instruction_data[2 + task_id_len..2 + task_id_len + node_id_len];
    msg!(
        "Dispatching task with task_id: {:?} to node_id: {:?}",
        task_id,
        node_id
    );

    Ok(())
}

pub fn process_return_answer(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 5 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let task_id_len = instruction_data[0] as usize;
    let answer_len = instruction_data[1 + task_id_len] as usize;
    if instruction_data.len() < 2 + task_id_len + answer_len {
        return Err(ProgramError::InvalidInstructionData);
    }

    let task_id = &instruction_data[1..1 + task_id_len];
    let answer_data = &instruction_data[2 + task_id_len..2 + task_id_len + answer_len];
    msg!(
        "Received answer for task_id {:?}: {:?}",
        task_id,
        answer_data
    );

    Ok(())
}

fn parse_node_id(instruction_data: &[u8]) -> Result<String, ProgramError> {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let node_id_len = instruction_data[0] as usize;
    if instruction_data.len() < 1 + node_id_len {
        return Err(ProgramError::InvalidInstructionData);
    }

    let node_id_data = &instruction_data[1..1 + node_id_len];
    let node_id =
        std::str::from_utf8(node_id_data).map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(node_id.to_string())
}
