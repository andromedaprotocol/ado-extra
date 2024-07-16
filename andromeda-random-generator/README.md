# ADO Purpose
With the Random Generator ADO, user can generate the random value based on blockhash, because all of the block hash are different.

### Messages

***Instantiation (What is specified and stored at instantiation)***
```
pub struct InstantiateMsg {
    pub kernel_address: String,
    pub owner: Option<String>,
}
```


***Execute Messages (What are the messages that can be executed)***
```
#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    IncreaseNonce {},
}
```

**IncreaseNonce**: Nonce value is increased every time user call this.

```
IncreaseNonce {}
```


***Query Messages (What are the messages that can be queried, what does each return)***

1. **GetNonce**: Returns the nonce value for random generator.
```
GetNonce {},
```

**Returns**:
```
#[cw_serde]
pub struct GetNonceResponse {
    pub nonce: u128,
}
```


2. **Generate Random**: Returns MD5 hash value from block informations.
```
GetRandom {},
```

**Returns**:
```
#[cw_serde]
pub struct GetRandomResponse {
    pub random: String,
}
```


### State
The contract maintains the following state:
```
pub const DEFAULT_NONCE: u128 = 0;

pub const NONCE: Item<u128> = Item::new("nonce");
```
