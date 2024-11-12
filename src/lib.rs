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
        4 => process_start_session(program_id, accounts, &instruction_data[4..]),
        5 => process_end_session(program_id, accounts, &instruction_data[4..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Register a node with ipAddress and hardwareId
pub fn process_register_node(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (node_id, ip_address, hardware_id) = parse_register_node(instruction_data)?;
    msg!(
        "Registering node with nodeId: {}, ipAddress: {}, hardwareId: {}",
        node_id,
        ip_address,
        hardware_id
    );
    Ok(())
}

// Remove node based on node_id
pub fn process_remove_node(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let node_id = parse_node_id(instruction_data)?;
    msg!("Removing node with nodeId: {}", node_id);
    Ok(())
}

// Dispatch task to node with task_id and node_id
pub fn process_dispatch_task(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (task_id, node_id) = parse_dispatch_task(instruction_data)?;
    msg!(
        "Dispatching task with task_id: {} to node_id: {}",
        task_id,
        node_id
    );
    Ok(())
}

// Return answer for a task from node
pub fn process_return_answer(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (task_id, answer) = parse_return_answer(instruction_data)?;
    msg!("Received answer for task_id {}: {}", task_id, answer);
    Ok(())
}

// Start session for a node
pub fn process_start_session(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let node_id = parse_node_id(instruction_data)?;
    msg!("Starting session for node with nodeId: {}", node_id);
    Ok(())
}

// End session for a node
pub fn process_end_session(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let node_id = parse_node_id(instruction_data)?;
    msg!("Ending session for node with nodeId: {}", node_id);
    Ok(())
}

fn parse_register_node(instruction_data: &[u8]) -> Result<(String, String, String), ProgramError> {
    if instruction_data.len() < 3 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let node_id = parse_string(&instruction_data[0..])?;
    let ip_address = parse_string(&instruction_data[node_id.len() + 1..])?;
    let hardware_id = parse_string(&instruction_data[node_id.len() + ip_address.len() + 2..])?;
    Ok((node_id, ip_address, hardware_id))
}

fn parse_dispatch_task(instruction_data: &[u8]) -> Result<(String, String), ProgramError> {
    let task_id = parse_string(&instruction_data)?;
    let node_id = parse_string(&instruction_data[task_id.len() + 1..])?;
    Ok((task_id, node_id))
}

fn parse_return_answer(instruction_data: &[u8]) -> Result<(String, String), ProgramError> {
    let task_id = parse_string(&instruction_data)?;
    let answer = parse_string(&instruction_data[task_id.len() + 1..])?;
    Ok((task_id, answer))
}

fn parse_node_id(instruction_data: &[u8]) -> Result<String, ProgramError> {
    parse_string(instruction_data)
}

fn parse_string(data: &[u8]) -> Result<String, ProgramError> {
    let len = data[0] as usize;
    if data.len() < 1 + len {
        return Err(ProgramError::InvalidInstructionData);
    }
    let s =
        std::str::from_utf8(&data[1..1 + len]).map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(s.to_string())
}
