use diem_types::{
    account_address::AccountAddress as LegacyAddress,
    transaction::authenticator::AuthenticationKey as LegacyAuthKey,
};
use zapatos_types::{
    account_address::AccountAddress, transaction::authenticator::AuthenticationKey,
};

/// Accounts have changed from v6 to v7, so we need to convert them. This is a helper function to do that.
pub fn convert_account(acc: LegacyAddress) -> anyhow::Result<AccountAddress> {
    let acc_str = acc.to_string();
    let a = AccountAddress::from_hex_literal(&format!("0x{}", acc_str))?;
    Ok(a)
}

/// helper to convert auth key structs. Note the struct appears to not have changed, but we need to convert the type and use the new codebase
pub fn convert_auth_key(key: LegacyAuthKey) -> anyhow::Result<AuthenticationKey> {
    let key_vec = key.to_vec();
    dbg!(&hex::encode(&key_vec));
    let a = AuthenticationKey::try_from(key_vec.as_slice())?;

    dbg!(&hex::encode(&a.to_vec()));

    Ok(a)
}
