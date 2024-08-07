use cairo_lang_starknet_classes::casm_contract_class::{CasmContractClass, StarknetSierraCompilationError};
use cairo_lang_starknet_classes::contract_class::{
    ContractClass as SierraContractClass, ContractEntryPoint, ContractEntryPoints,
};
use cairo_lang_utils::bigint::BigUintAsHex;
use num_bigint::BigUint;

fn starknet_api_entry_point_to_contract_entry_point(value: &starknet_api::state::EntryPoint) -> ContractEntryPoint {
    ContractEntryPoint {
        function_idx: value.function_idx.0,
        selector: BigUint::from_bytes_be(&value.selector.0.to_bytes_be()),
    }
}

pub fn sierra_to_casm_contract_class(
    contract_class: starknet_api::state::ContractClass,
) -> Result<CasmContractClass, StarknetSierraCompilationError> {
    let sierra_contract_entry_points = ContractEntryPoints {
        external: contract_class
            .entry_points_by_type
            .get(&starknet_api::state::EntryPointType::External)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(starknet_api_entry_point_to_contract_entry_point)
            .collect(),
        constructor: contract_class
            .entry_points_by_type
            .get(&starknet_api::state::EntryPointType::Constructor)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(starknet_api_entry_point_to_contract_entry_point)
            .collect(),
        l1_handler: contract_class
            .entry_points_by_type
            .get(&starknet_api::state::EntryPointType::L1Handler)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(starknet_api_entry_point_to_contract_entry_point)
            .collect(),
    };

    let sierra_contract_class = SierraContractClass {
        sierra_program: contract_class
            .sierra_program
            .iter()
            .map(|v| BigUintAsHex { value: BigUint::from_bytes_be(&v.to_bytes_be()) })
            .collect(),
        sierra_program_debug_info: None,
        contract_class_version: "0.1.0".to_string(),
        entry_points_by_type: sierra_contract_entry_points,
        abi: None, // we can convert the ABI but for now, to convert to Casm, the ABI isn't needed
    };
    let casm_contract_class = CasmContractClass::from_contract_class(sierra_contract_class, false, usize::MAX)?;

    Ok(casm_contract_class)
}
