use candid::Nat;
use ic_ledger_types::MAINNET_CYCLES_MINTING_CANISTER_ID;

use crate::{
    api_error::ApiError, cycles_minting::CyclesMintingService, misc::generic::TRILLION_CYCLES,
    CanisterResult,
};

use super::misc::{e12s_to_f64, e8s_to_f64, f64_to_e8s};

pub async fn cycles_per_icp() -> CanisterResult<Nat> {
    let cycles_minting_service = CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID);
    let result = cycles_minting_service
        .get_icp_xdr_conversion_rate()
        .await
        .map(|(rate,)| rate)
        .map_err(|_| ApiError::bad_request().add_message("Error getting XDR conversion rate"))?;

    Ok(Nat::from(
        (result.data.xdr_permyriad_per_icp * TRILLION_CYCLES) / 10_000,
    ))
}

pub async fn cycles_per_icp_e8s(e8s: Nat) -> CanisterResult<Nat> {
    let cycles_per_icp = e8s_to_f64(&cycles_per_icp().await?);
    let tokens_e8s = e8s_to_f64(&e8s);

    let cycles = (tokens_e8s) * cycles_per_icp;
    Ok(f64_to_e8s(cycles))
}

pub async fn icp_per_cycles_e12s(e12s: Nat) -> CanisterResult<Nat> {
    let cycles_per_icp = e8s_to_f64(&cycles_per_icp().await?);
    let tokens_e12s = e12s_to_f64(&e12s);

    let icp = tokens_e12s / (cycles_per_icp / 10_000f64);
    Ok(f64_to_e8s(icp))
}