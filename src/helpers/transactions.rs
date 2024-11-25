use candid::{Nat, Principal};
use ic_cdk::{api::time, id};
use ic_ledger_types::{
    account_balance, query_archived_blocks, query_blocks, transfer, AccountBalanceArgs,
    AccountIdentifier, Block, BlockIndex, GetBlocksArgs, Operation, Subaccount, Timestamp, Tokens,
    TransferArgs, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID,
};
use icrc_ledger_types::{
    icrc1::account::Account,
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        approve::ApproveArgs,
        transfer_from::{TransferFromArgs, TransferFromError},
    },
    icrc3::transactions::Approve,
};

use crate::{
    api_error::ApiError,
    cycles_minting::{
        CyclesMintingService, NotifyCreateCanisterArg, NotifyCreateCanisterResult, NotifyError,
        NotifyTopUpArg, NotifyTopUpResult,
    },
    misc::generic::{ICP_TRANSACTION_FEE, MEMO_TOP_UP_CANISTER},
    CanisterResult,
};

use super::misc::{nat_to_u64, principal_to_account_identifier};

pub async fn transfer_icp(to: Principal, amount_e8s: u64) -> CanisterResult<BlockIndex> {
    let args = TransferArgs {
        to: principal_to_account_identifier(to),
        amount: Tokens::from_e8s(amount_e8s),
        fee: Tokens::from_e8s(10_000),
        memo: ic_ledger_types::Memo(0),
        from_subaccount: None,
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn transfer_icp_from_subaccount(
    to: Principal,
    principal: Principal,
    amount_e8s: u64,
) -> CanisterResult<BlockIndex> {
    let args = TransferArgs {
        to: principal_to_account_identifier(to),
        amount: Tokens::from_e8s(amount_e8s),
        fee: Tokens::from_e8s(10_000),
        memo: ic_ledger_types::Memo(0),
        from_subaccount: Some(Subaccount::from(principal)),
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn transfer_icp_by_account_identifier(
    to: AccountIdentifier,
    amount_e8s: u64,
) -> CanisterResult<BlockIndex> {
    let args = TransferArgs {
        to,
        amount: Tokens::from_e8s(amount_e8s),
        fee: Tokens::from_e8s(10_000),
        memo: ic_ledger_types::Memo(0),
        from_subaccount: None,
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn top_up_cycles_and_notify(
    icp_amount: u64,
    principal: Principal,
) -> CanisterResult<Nat> {
    let block_index = top_up_cycles(icp_amount, principal).await?;
    notify_top_up_cycles(block_index).await
}

pub async fn top_up_cycles(icp_amount: u64, canister: Principal) -> CanisterResult<BlockIndex> {
    let amount = Tokens::from_e8s(icp_amount - ICP_TRANSACTION_FEE);

    let args = TransferArgs {
        memo: ic_ledger_types::Memo(MEMO_TOP_UP_CANISTER),
        amount,
        fee: Tokens::from_e8s(ICP_TRANSACTION_FEE),
        from_subaccount: None,
        to: AccountIdentifier::new(
            &MAINNET_CYCLES_MINTING_CANISTER_ID,
            &Subaccount::from(canister),
        ),
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn topup_self(
    icp_amount: u64,
    canister: Principal,
    principal: Principal,
) -> CanisterResult<Nat> {
    let amount = Tokens::from_e8s(icp_amount - ICP_TRANSACTION_FEE);

    let args = TransferArgs {
        memo: ic_ledger_types::Memo(MEMO_TOP_UP_CANISTER),
        amount,
        fee: Tokens::from_e8s(ICP_TRANSACTION_FEE),
        from_subaccount: Some(Subaccount::from(principal)),
        to: AccountIdentifier::new(
            &MAINNET_CYCLES_MINTING_CANISTER_ID,
            &Subaccount::from(canister),
        ),
        created_at_time: None,
    };

    let transfer = transfer(MAINNET_LEDGER_CANISTER_ID, args).await;

    match transfer {
        Ok(result) => match result {
            Ok(block_index) => self::notify_top_up_cycles(block_index).await,
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn send_to_canister_after_approve(
    icp_amount: u64,
    canister: Principal,
    user_principal: Principal,
) -> CanisterResult<u64> {
    let args = TransferFromArgs {
        spender_subaccount: Some(Subaccount::from(user_principal).0),
        from: Account {
            owner: user_principal,
            subaccount: None,
        },
        memo: None,
        amount: Nat::from(icp_amount - ICP_TRANSACTION_FEE),
        fee: None,
        to: Account {
            owner: canister,
            subaccount: Some(Subaccount::from(user_principal).0),
        },
        created_at_time: None,
    };

    let (result,): (Result<Nat, TransferFromError>,) =
        ic_cdk::call(MAINNET_LEDGER_CANISTER_ID, "icrc2_transfer_from", (args,))
            .await
            .map_err(|err| ApiError::unexpected().add_message(format!("{:?}", err)))?;

    match result {
        Ok(block_index) => Ok(nat_to_u64(&block_index)),
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn notify_top_up_cycles(block_index: u64) -> CanisterResult<Nat> {
    match CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID)
        .notify_top_up(NotifyTopUpArg {
            block_index,
            canister_id: id(),
        })
        .await
    {
        Ok((result,)) => {
            match result {
                NotifyTopUpResult::Ok(cycles) => Ok(cycles),
                NotifyTopUpResult::Err(err) => match err {
                    NotifyError::Refunded {
                        block_index,
                        reason,
                    } => Err(ApiError::bad_request().add_message(format!(
                        "Refunded: block_index: {:?}, reason: {:?}",
                        block_index, reason
                    ))),
                    NotifyError::InvalidTransaction(value) => Err(ApiError::bad_request()
                        .add_message(format!("InvalidTransaction: {:?}", value))),
                    NotifyError::Other {
                        error_message,
                        error_code,
                    } => Err(ApiError::bad_request().add_message(format!(
                        "Other: error_message: {}, error_code: {:?}",
                        error_message, error_code
                    ))),
                    NotifyError::Processing => {
                        Err(ApiError::bad_request().add_message("Processing"))
                    }
                    NotifyError::TransactionTooOld(value) => Err(ApiError::bad_request()
                        .add_message(format!("TransactionTooOld: {:?}", value))),
                },
            }
        }
        Err((_, err)) => Err(ApiError::bad_request().add_message(&err)),
    }
}

pub async fn notify_create(block_index: u64) -> CanisterResult<Principal> {
    match CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID)
        .notify_create_canister(NotifyCreateCanisterArg {
            block_index,
            controller: id(),
            subnet_selection: None,
            settings: None,
            subnet_type: None,
        })
        .await
    {
        Ok((result,)) => {
            match result {
                NotifyCreateCanisterResult::Ok(principal) => Ok(principal),
                NotifyCreateCanisterResult::Err(err) => match err {
                    NotifyError::Refunded {
                        block_index,
                        reason,
                    } => Err(ApiError::bad_request().add_message(format!(
                        "Refunded: block_index: {:?}, reason: {:?}",
                        block_index, reason
                    ))),
                    NotifyError::InvalidTransaction(value) => Err(ApiError::bad_request()
                        .add_message(format!("InvalidTransaction: {:?}", value))),
                    NotifyError::Other {
                        error_message,
                        error_code,
                    } => Err(ApiError::bad_request().add_message(format!(
                        "Other: error_message: {}, error_code: {:?}",
                        error_message, error_code
                    ))),
                    NotifyError::Processing => {
                        Err(ApiError::bad_request().add_message("Processing"))
                    }
                    NotifyError::TransactionTooOld(value) => Err(ApiError::bad_request()
                        .add_message(format!("TransactionTooOld: {:?}", value))),
                },
            }
        }
        Err((_, err)) => Err(ApiError::bad_request().add_message(&err)),
    }
}

pub async fn get_icp_balance(principal: Principal) -> CanisterResult<Tokens> {
    let args = AccountBalanceArgs {
        account: principal_to_account_identifier(principal),
    };

    match account_balance(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn get_icp_allowance_by_canister_subaccount(
    principal: Principal,
) -> CanisterResult<Allowance> {
    let args = AllowanceArgs {
        account: Account {
            owner: principal,
            subaccount: None,
        },
        spender: Account {
            owner: id(),
            subaccount: Some(Subaccount::from(principal).0),
        },
    };

    let (allowance,): (Allowance,) =
        ic_cdk::call(MAINNET_LEDGER_CANISTER_ID, "icrc2_allowance", (args,))
            .await
            .map_err(|err| ApiError::unexpected().add_message(format!("{:?}", err)))?;
    Ok(allowance)
}

pub async fn set_icp_approve_by_canister_subaccount(
    principal: Principal,
    amount: Nat,
    expires_at: Option<u64>,
) -> CanisterResult<Approve> {
    let args = ApproveArgs {
        from_subaccount: None,
        spender: Account {
            owner: id(),
            subaccount: Some(Subaccount::from(principal).0),
        },
        amount,
        expected_allowance: None,
        expires_at,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let (approve,): (Approve,) = ic_cdk::call(MAINNET_LEDGER_CANISTER_ID, "icrc2_approve", (args,))
        .await
        .map_err(|err| ApiError::unexpected().add_message(format!("{:?}", err)))?;
    Ok(approve)
}

pub async fn get_icp_balance_by_canister_subaccount(
    subaccount: Principal,
) -> CanisterResult<Tokens> {
    let args = AccountBalanceArgs {
        account: AccountIdentifier::new(&id(), &Subaccount::from(subaccount)),
    };

    match account_balance(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn get_icp_balance_by_account_identifier(
    account_identifier: AccountIdentifier,
) -> CanisterResult<Tokens> {
    let args = AccountBalanceArgs {
        account: account_identifier,
    };

    match account_balance(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn validate_transaction(
    from_principal: Principal,
    to_principal: Principal,
    block_index: BlockIndex,
) -> Option<Tokens> {
    let block = get_block(block_index).await?;

    match block.transaction.operation? {
        Operation::Transfer {
            from,
            to,
            amount,
            fee: _, // Ignore fee
        } => {
            if from != principal_to_account_identifier(from_principal) {
                return None;
            }
            if to != principal_to_account_identifier(to_principal) {
                return None;
            }
            Some(amount)
        }
        _ => None,
    }
}

async fn get_block(block_index: BlockIndex) -> Option<Block> {
    let args = GetBlocksArgs {
        start: block_index,
        length: 1,
    };

    if let Ok(blocks_result) = query_blocks(MAINNET_LEDGER_CANISTER_ID, args.clone()).await {
        if !blocks_result.blocks.is_empty() {
            return blocks_result.blocks.into_iter().next();
        }

        if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
            (b.start <= block_index && (block_index - b.start) < b.length).then_some(b.callback)
        }) {
            if let Ok(range) = query_archived_blocks(&func, args).await {
                match range {
                    Ok(_range) => return _range.blocks.into_iter().next(),
                    Err(_) => return None,
                }
            }
        }
    }

    None
}
