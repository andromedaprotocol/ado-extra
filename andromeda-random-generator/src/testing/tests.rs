use super::mock::{
    proper_initialization, increase_nonce, query_nonce, query_generate_random,
};

#[test]
fn test_instantiation() {
    let (deps, _) = proper_initialization();
    let res = query_nonce(deps.as_ref()).unwrap();
    assert_eq!(res.nonce, 0);
}

#[test]
fn test_increase_nonce() {
    let (mut deps, info) = proper_initialization();
    increase_nonce(deps.as_mut(), info.sender.as_ref()).unwrap();

    let res = query_nonce(deps.as_ref()).unwrap();
    assert_eq!(res.nonce, 1);
}

#[test]
fn test_random_generator() {
    let (mut deps, info) = proper_initialization();

    increase_nonce(deps.as_mut(), info.sender.as_ref()).unwrap();
    let res_1 = query_generate_random(deps.as_ref()).unwrap();

    increase_nonce(deps.as_mut(), info.sender.as_ref()).unwrap();
    let res_2 = query_generate_random(deps.as_ref()).unwrap();

    assert_ne!(res_1, res_2);
}
