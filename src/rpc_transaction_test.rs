use std::sync::Arc;

use rstest::rstest;
use starknet_types_core::felt::Felt;

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce, PatriciaKey};
use crate::rpc_transaction::{
    ContractClass, DataAvailabilityMode, RPCDeclareTransaction, RPCDeclareTransactionV3,
    RPCDeployAccountTransaction, RPCDeployAccountTransactionV3, RPCInvokeTransaction,
    RPCInvokeTransactionV3, RPCTransaction, ResourceBoundsMapping,
};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, ResourceBounds, Tip,
    TransactionSignature,
};
use crate::{contract_address, felt, patricia_key};

fn create_resource_bounds_for_testing() -> ResourceBoundsMapping {
    ResourceBoundsMapping {
        l1_gas: ResourceBounds { max_amount: 100, max_price_per_unit: 12 },
        l2_gas: ResourceBounds { max_amount: 58, max_price_per_unit: 31 },
    }
}

fn create_declare_v3() -> RPCDeclareTransaction {
    RPCDeclareTransaction::V3(RPCDeclareTransactionV3 {
        contract_class: ContractClass::default(),
        resource_bounds: create_resource_bounds_for_testing(),
        tip: Tip(1),
        signature: TransactionSignature(vec![Felt::ONE, Felt::TWO]),
        nonce: Nonce(Felt::ONE),
        compiled_class_hash: CompiledClassHash(Felt::TWO),
        sender_address: contract_address!("0x3"),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L2,
        paymaster_data: PaymasterData(vec![Felt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![Felt::THREE]),
    })
}

fn create_deploy_account_v3() -> RPCDeployAccountTransaction {
    RPCDeployAccountTransaction::V3(RPCDeployAccountTransactionV3 {
        resource_bounds: create_resource_bounds_for_testing(),
        tip: Tip::default(),
        contract_address_salt: ContractAddressSalt(felt!("0x23")),
        class_hash: ClassHash(Felt::TWO),
        constructor_calldata: Calldata(Arc::new(vec![Felt::ZERO])),
        nonce: Nonce(felt!("0x60")),
        signature: TransactionSignature(vec![Felt::TWO]),
        nonce_data_availability_mode: DataAvailabilityMode::L2,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![Felt::TWO, Felt::ZERO]),
    })
}

fn create_invoke_v3() -> RPCInvokeTransaction {
    RPCInvokeTransaction::V3(RPCInvokeTransactionV3 {
        resource_bounds: create_resource_bounds_for_testing(),
        tip: Tip(50),
        calldata: Calldata(Arc::new(vec![felt!("0x2000"), felt!("0x1000")])),
        sender_address: contract_address!("0x53"),
        nonce: Nonce(felt!("0x32")),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![Felt::TWO, Felt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![felt!("0x87")]),
    })
}

// We are testing the `RPCTransaction` serialization. Passing non-default values.
#[rstest]
#[case(RPCTransaction::Declare(create_declare_v3()))]
#[case(RPCTransaction::DeployAccount(create_deploy_account_v3()))]
#[case(RPCTransaction::Invoke(create_invoke_v3()))]
fn test_rpc_transactions(#[case] tx: RPCTransaction) {
    let serialized = serde_json::to_string(&tx).unwrap();
    let deserialized: RPCTransaction = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tx, deserialized);
}
