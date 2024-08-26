use super::*;
use ic_cdk::export::Principal;

fn setup() -> TokenWallet {
    let mut wallet = TokenWallet::new();
    wallet.init(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
    wallet
}

#[test]
fn test_send_tokens() {
    let mut wallet = setup();
    let sender = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let receiver = Principal::from_text("aaaaa-aa").unwrap();
    
    wallet.balances.insert(sender, 100);

    let result = wallet.send_tokens(sender, receiver, 50);
    assert!(result.is_ok());

    assert_eq!(wallet.get_balance(sender), 50);
    assert_eq!(wallet.get_balance(receiver), 50);
}

#[test]
fn test_receive_tokens() {
    let mut wallet = setup();
    let receiver = Principal::from_text("aaaaa-aa").unwrap();

    let result = wallet.receive_tokens(receiver, 100);
    assert!(result.is_ok());

    assert_eq!(wallet.get_balance(receiver), 100);
}

#[test]
fn test_insufficient_balance() {
    let mut wallet = setup();
    let sender = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let receiver = Principal::from_text("aaaaa-aa").unwrap();

    wallet.balances.insert(sender, 50);

    let result = wallet.send_tokens(sender, receiver, 100);
    assert!(result.is_err());
}

#[test]
fn test_get_owner() {
    let wallet = setup();
    let owner = wallet.get_owner();
    assert_eq!(owner, Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
}