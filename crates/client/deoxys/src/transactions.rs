use std::sync::Arc;

use mp_starknet::{transaction::types::{Transaction, MaxArraySize, TxType}, execution::{types::{Felt252Wrapper, EntryPointTypeWrapper, ContractAddressWrapper, CallEntryPointWrapper, MaxCalldataSize, ClassHashWrapper}, felt252_wrapper}};
use sp_core::{bounded_vec::BoundedVec, U256};
use blockifier::execution::contract_class::{ContractClass, ContractClassV0, ContractClassV1, ContractClassV1Inner};
use starknet_client::reader::{objects::transaction::{IntermediateInvokeTransaction, IntermediateDeclareTransaction, DeployAccountTransaction, L1HandlerTransaction, DeployTransaction}, StarknetFeederGatewayClient, StarknetReader, GenericContractClass};
use starknet_ff::FieldElement;
use starknet_api::{api_core::{PatriciaKey, ClassHash}, stark_felt, hash::StarkFelt};
use starknet_api::state::ContractClass as StarknetContractClass;

pub async fn convert_to_contract_class(starknet_class: ClassHash, client: Arc<StarknetFeederGatewayClient>) -> Result<ContractClass, String> {
    let contract_class = client.class_by_hash(starknet_class)
        .await
        .map_err(|e| format!("Error: {:?}", e))?;

    match contract_class {
        Some(GenericContractClass::Cairo0ContractClass(class_v0)) => {
            let class_v0: ContractClassV0 = class_v0.try_into().map_err(|e| format!("Error: {:?}", e))?;
            Ok(ContractClass::V0(class_v0))
        },
        Some(GenericContractClass::Cairo1ContractClass(class_v1)) => {
            todo!();
        }
        None => todo!(),
    }
}


pub async fn declare_tx_to_starknet_tx(declare_transaction: IntermediateDeclareTransaction, client: Arc<StarknetFeederGatewayClient>) -> Transaction {

    let mut signature_vec: BoundedVec<Felt252Wrapper, MaxArraySize> = BoundedVec::new();
    for item in &declare_transaction.signature.0 {
        match signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap()) {
            Ok(_) => {},
            Err(_) => {
                panic!("Signature too long");
            }
        }
    }
    let calldata_vec: BoundedVec<Felt252Wrapper, MaxCalldataSize> = BoundedVec::new();

    let call_entry_point = CallEntryPointWrapper::new(
        Some(Felt252Wrapper::try_from(declare_transaction.class_hash.0.bytes()).unwrap()),
        EntryPointTypeWrapper::External,
        Some(Felt252Wrapper::default()),
        calldata_vec,
        ContractAddressWrapper::default(),
        ContractAddressWrapper::default(),
        Felt252Wrapper::ZERO,
        Some(ClassHashWrapper::ZERO)
    );

    let version_fw: Felt252Wrapper = Felt252Wrapper(FieldElement::from(declare_transaction.version.0));
    let version_u8: u8 = match u64::try_from(version_fw) {
        Ok(valeur) => {
            valeur as u8
        },
        Err(_) => {panic!("Version too long")}
    };

    let sender_address_fe: FieldElement =  FieldElement::from(*PatriciaKey::key(&declare_transaction.sender_address.0));
    let sender_address_fw: Felt252Wrapper = Felt252Wrapper(sender_address_fe);
    
    let contract_class = convert_to_contract_class(declare_transaction.class_hash, client).await;

    Transaction {
        tx_type: TxType::Declare,
        version: version_u8,
        hash: Felt252Wrapper(declare_transaction.transaction_hash.0.into()),
        signature: signature_vec,
        sender_address: sender_address_fw,
        nonce: Felt252Wrapper::try_from(declare_transaction.nonce.0.bytes()).unwrap(),
        call_entrypoint: call_entry_point,
        contract_class: Option::<ContractClass>::default(),
        contract_address_salt: Option::<U256>::default(),
        max_fee: Felt252Wrapper::from(declare_transaction.max_fee.0),
        is_query: false,
    }
}


pub fn invoke_tx_to_starknet_tx(invoke_transaction : IntermediateInvokeTransaction, client: Arc<StarknetFeederGatewayClient>) -> Transaction {
    let mut signature_vec: BoundedVec<Felt252Wrapper, MaxArraySize> = BoundedVec::new();
    for item in &invoke_transaction.signature.0 {
        match signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap()) {
            Ok(_) => {},
            Err(_) => {
                panic!("Signature too long");
            }
        }
        //signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap());
    }
    let calldata_vec: BoundedVec<Felt252Wrapper, MaxCalldataSize> = BoundedVec::new();

    let call_entry_point = CallEntryPointWrapper::new(
        Some(Felt252Wrapper::default()),
        EntryPointTypeWrapper::External,
        Some(Felt252Wrapper::default()),
        calldata_vec,
        ContractAddressWrapper::default(),
        ContractAddressWrapper::default(),
        Felt252Wrapper::ZERO,
        Some(ClassHashWrapper::ZERO)
    );

    //let version_invoke = StarkFelt::from(invoke_transaction.version.0);
    let version_fw: Felt252Wrapper = Felt252Wrapper(FieldElement::from(invoke_transaction.version.0));
    let version_u8: u8 = match u64::try_from(version_fw) {
        Ok(valeur) => {
            valeur as u8
        },
        Err(_) => {panic!("Version too long")}
    };

    let sender_address_fe: FieldElement =  FieldElement::from(*PatriciaKey::key(&invoke_transaction.sender_address.0));
    let sender_address_fw: Felt252Wrapper = Felt252Wrapper(sender_address_fe);
    
    Transaction {
        tx_type: TxType::Invoke,
        version: version_u8,
        hash: Felt252Wrapper(invoke_transaction.transaction_hash.0.into()),
        signature: signature_vec,
        sender_address: sender_address_fw,
        nonce: Felt252Wrapper::default(),
        call_entrypoint: call_entry_point,
        contract_class: Option::<ContractClass>::default(),
        contract_address_salt: Option::<U256>::default(),
        max_fee: Felt252Wrapper::from(invoke_transaction.max_fee.0),
        is_query: false,
    }
}

pub async fn deploy_tx_to_starknet_tx(deploy_transaction : DeployTransaction, client: Arc<StarknetFeederGatewayClient>) -> Transaction {
    // let mut signature_vec: BoundedVec<Felt252Wrapper, MaxArraySize> = BoundedVec::new();
    // for item in &deploy_transaction.signature.0 {
    //     match signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap()) {
    //         Ok(_) => {},
    //         Err(_) => {
    //             panic!("Signature too long");
    //         }
    //     }
    //     signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap());
    // }
    let calldata_vec: BoundedVec<Felt252Wrapper, MaxCalldataSize> = BoundedVec::new();

    let call_entry_point = CallEntryPointWrapper::new(
        Some(Felt252Wrapper::try_from(deploy_transaction.class_hash.0.bytes()).unwrap()),   //class_hash: Option<ClassHashWrapper>,
        EntryPointTypeWrapper::External, //entrypoint_type: EntryPointTypeWrapper,
        Some(Felt252Wrapper::default()),
        calldata_vec,
        ContractAddressWrapper::default(), //storage_address: ContractAddressWrapper,
        ContractAddressWrapper::default(), //caller_address: ContractAddressWrapper,
        Felt252Wrapper::ZERO,
        Some(ClassHashWrapper::ZERO)
    );

       let version_fw: Felt252Wrapper = Felt252Wrapper(FieldElement::from(deploy_transaction.version.0));
       let version_u8: u8 = match u64::try_from(version_fw) {
           Ok(valeur) => {
               valeur as u8
           },
           Err(_) => {panic!("Version too long")}
       };

    //  let sender_address_fe: FieldElement =  FieldElement::from(*PatriciaKey::key(&deploy_transaction.sender_address.0));
    //  let sender_address_fw: Felt252Wrapper = Felt252Wrapper(sender_address_fe);

    let contract_class = convert_to_contract_class(deploy_transaction.class_hash, client).await;

    Transaction {
            tx_type: TxType::DeployAccount,
            version: version_u8,
            hash: Felt252Wrapper(deploy_transaction.transaction_hash.0.into()),
            signature: BoundedVec::new(),
            sender_address: Felt252Wrapper::default(),
            nonce: Felt252Wrapper::default(),
            call_entrypoint: call_entry_point,
            contract_class: Option::<ContractClass>::default(),
            contract_address_salt: Option::<U256>::default(),
            max_fee: Felt252Wrapper::default(),
            is_query: false, // Assuming default value
    }
}

pub async fn deploy_account_tx_to_starknet_tx(deploy_account_transaction : DeployAccountTransaction, client: Arc<StarknetFeederGatewayClient>) -> Transaction {
    let mut signature_vec: BoundedVec<Felt252Wrapper, MaxArraySize> = BoundedVec::new();
    for item in &deploy_account_transaction.signature.0 {
        match signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap()) {
            Ok(_) => {},
            Err(_) => {
                panic!("Signature too long");
            }
        }
        signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap());
    }
    let calldata_vec: BoundedVec<Felt252Wrapper, MaxCalldataSize> = BoundedVec::new();

    let call_entry_point = CallEntryPointWrapper::new(
        Some(Felt252Wrapper::try_from(deploy_account_transaction.class_hash.0.bytes()).unwrap()),   //class_hash: Option<ClassHashWrapper>,
        EntryPointTypeWrapper::External, //entrypoint_type: EntryPointTypeWrapper,
        Some(Felt252Wrapper::default()),
        calldata_vec,
        ContractAddressWrapper::default(), //storage_address: ContractAddressWrapper,
        ContractAddressWrapper::default(), //caller_address: ContractAddressWrapper,
        Felt252Wrapper::ZERO,
        Some(ClassHashWrapper::ZERO)
    );

    let version_fw: Felt252Wrapper = Felt252Wrapper(FieldElement::from(deploy_account_transaction.version.0));
    let version_u8: u8 = match u64::try_from(version_fw) {
        Ok(valeur) => {
            valeur as u8
        },
        Err(_) => {panic!("Version too long")}
    };

    //let sender_address_fe: FieldElement =  FieldElement::from(*PatriciaKey::key(&deploy_account_transaction.sender_address.0));
    //let sender_address_fw: Felt252Wrapper = Felt252Wrapper(sender_address_fe);

    let contract_class = convert_to_contract_class(deploy_account_transaction.class_hash, client).await;

    Transaction {
        tx_type: TxType::DeployAccount,
        version: version_u8,
        hash: Felt252Wrapper(deploy_account_transaction.transaction_hash.0.into()),
        signature: signature_vec,
        sender_address: Felt252Wrapper::default(),
        nonce: Felt252Wrapper::try_from(deploy_account_transaction.nonce.0.bytes()).unwrap(),
        call_entrypoint: call_entry_point,
        contract_class: contract_class.ok(),
        contract_address_salt: Option::<U256>::default(),
        max_fee: Felt252Wrapper::from(deploy_account_transaction.max_fee.0),
        is_query: false, // Assuming default value
    }
}

pub fn l1handler_tx_to_starknet_tx(l1handler_transaction : L1HandlerTransaction, client: Arc<StarknetFeederGatewayClient>) -> Transaction {
    let mut signature_vec: BoundedVec<Felt252Wrapper, MaxArraySize> = BoundedVec::new();
    // for item in &l1handler_transaction.signature.0 {
    //     match signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap()) {
    //         Ok(_) => {},
    //         Err(_) => {
    //             panic!("Signature too long");
    //         }
    //     }
    //     signature_vec.try_push(Felt252Wrapper::try_from(item.bytes()).unwrap());
    // }
    let calldata_vec: BoundedVec<Felt252Wrapper, MaxCalldataSize> = BoundedVec::new();

    let call_entry_point = CallEntryPointWrapper::new(
        Some(Felt252Wrapper::default()),   //class_hash: Option<ClassHashWrapper>,
        EntryPointTypeWrapper::External, //entrypoint_type: EntryPointTypeWrapper,
        Some(Felt252Wrapper::default()),
        calldata_vec,
        ContractAddressWrapper::default(), //storage_address: ContractAddressWrapper,
        ContractAddressWrapper::default(), //caller_address: ContractAddressWrapper,
        Felt252Wrapper::ZERO,
        Some(ClassHashWrapper::ZERO)
    );

    let version_fw: Felt252Wrapper = Felt252Wrapper(FieldElement::from(l1handler_transaction.version.0));
    let version_u8: u8 = match u64::try_from(version_fw) {
        Ok(valeur) => {
            valeur as u8
        },
        Err(_) => {panic!("Version too long")}
    };

    //let sender_address_fe: FieldElement =  FieldElement::from(*PatriciaKey::key(&l1handler_transaction.sender_address.0));
    //let sender_address_fw: Felt252Wrapper = Felt252Wrapper(sender_address_fe);    

    Transaction {
        tx_type: TxType::L1Handler,
        version: version_u8,
        hash: Felt252Wrapper(l1handler_transaction.transaction_hash.0.into()),
        signature: signature_vec,
        sender_address: Felt252Wrapper::default(),
        nonce: Felt252Wrapper::try_from(l1handler_transaction.nonce.0.bytes()).unwrap(),
        call_entrypoint: call_entry_point,
        contract_class: Option::<ContractClass>::default(),
        contract_address_salt:  Option::<U256>::default(),
        max_fee: Some(Felt252Wrapper::default()).unwrap(),
        is_query: false, // Assuming default value
    }
}
