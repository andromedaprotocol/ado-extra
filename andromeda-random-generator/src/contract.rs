#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, Storage};
use andromeda_std::{
    ado_base::{InstantiateMsg as BaseInstantiateMsg, MigrateMsg},
    ado_contract::ADOContract,
    common::{actions::call_action, context::ExecuteContext, encode_binary},
    error::ContractError,
};
use md5;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetNonceResponse, GetRandomResponse},
    state::{NONCE, DEFAULT_NONCE},
};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:andromeda-random-generator";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    

    let contract = ADOContract::default();

    let resp = contract.instantiate(
        deps.storage,
        env,
        deps.api,
        &deps.querier,
        info.clone(),
        BaseInstantiateMsg {
            ado_type: CONTRACT_NAME.to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    NONCE.save(deps.storage, &DEFAULT_NONCE)?;

    Ok(resp
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    if let ExecuteMsg::AMPReceive(pkt) = msg {
        ADOContract::default().execute_amp_receive(
            ctx,
            pkt,
            handle_execute,
        )
    } else {
        handle_execute(ctx, msg)
    }
}

pub fn handle_execute(
    mut ctx: ExecuteContext,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let action_response = call_action(
        &mut ctx.deps,
        &ctx.info,
        &ctx.env,
        &ctx.amp_ctx,
        msg.as_ref(),
    )?;

    let res = match msg {
        ExecuteMsg::IncreaseNonce {} => execute_increase_nonce(ctx),
        _ => ADOContract::default().execute(ctx, msg)
    }?;

    Ok(res
        .add_submessages(action_response.messages)
        .add_attributes(action_response.attributes)
        .add_events(action_response.events))
}

pub fn execute_increase_nonce(ctx: ExecuteContext) -> Result<Response, ContractError> {
    let nonce = NONCE.load(ctx.deps.storage)? + 1;
    NONCE.save(ctx.deps.storage, &nonce)?;

    Ok(
        Response::new()
        .add_attribute("method", "execute_increase_nonce")
        .add_attribute("nonce", nonce.to_string())
    )
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::GetNonce {} => encode_binary(&query_nonce(deps.storage)?),
        QueryMsg::GetRandom {} => encode_binary(&query_generate_random(deps.storage, env)?),
        _ => ADOContract::default().query(deps, env, msg),
    }
}

pub fn query_nonce(storage: &dyn Storage) -> Result<GetNonceResponse, ContractError> {
    let nonce = NONCE.load(storage)?;
    Ok(GetNonceResponse { nonce })
}

pub fn query_generate_random(storage: &dyn Storage, env: Env) -> Result<GetRandomResponse, ContractError> {
    let nonce = NONCE.load(storage)?;
    let block_info = env.block;

    let chain_id = block_info.chain_id;
    let height = block_info.height.to_string();
    let time = block_info.time.to_string();
    let nonce_str = nonce.to_string();

    let data = format!("{}{}{}{}", chain_id, height, time, nonce_str);

    let digest: md5::Digest = md5::compute(data.to_string());
    let hash = format!("{:x}", digest);
    
    Ok(GetRandomResponse { random: hash })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    ADOContract::default().migrate(deps, CONTRACT_NAME, CONTRACT_VERSION)
}
