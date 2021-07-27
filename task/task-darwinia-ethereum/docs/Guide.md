darwinia-ethereum
===

## Prepare

1. `cp .maintain/config/task-darwinia-ethereum.toml ~/.bridger`
   The dir can be any path you like, and fill the content.
2. `cp ~/.bridger/linked-darwinia.toml ~/.bridger`
3. `cargo build --release`

## Run

4. `./target/release/bridger server`
   it may take a while to run all services if the network is not good. this will run all services except the darwinia and ethereum subscribe services. These two services should be started manually.

5. Open another shell

6. Start the darwinia subscribe service
    ```bash
    ./target/release/bridger task exec --name task-darwinia-ethereum --api start-darwinia
    ```
   or start it with a block_number
    ```bash
    ./target/release/bridger task exec --name task-darwinia-ethereum --api start-darwinia --param block_number=4230622
    ```

7. Start the ethereum subscribe service
    ```bash
    ./target/release/bridger task exec --name task-darwinia-ethereum --api start-ethereum
    ```
   or start it with a block_number
    ```bash
    ./target/release/bridger task exec --name task-darwinia-ethereum --api start-ethereum --param block_number=12856303
    ```

Note: Darwinia web socket node connected requires enabling [offchain-indexing](https://github.com/darwinia-network/bridger/issues/196#issuecomment-884056708).

## Security config

If you want to encrypt your private key in config file. the first you can use `crypto encrypt` command to get your encrypted value.

```bash
./target/release/bridger crypto encrypt --value abcdefg
```

When you got it. then update your config. change follow this.

```toml
[darwinia]
# private key of relayer, or, private key of proxy
relayer_private_key = "<YOUR CRYPTED DATA>"
# ...
[task]
is_enable_crypto = true
interval_ethereum = 120
interval_relay = 60
# ...
```

Not we support these filed to encrypt and decrypt

- darwinia.relayer_private_key

> NOTE: please use same password to encrypt your keys.

When the configuration is encrypted, the task will not start until you provide the password.

your can start with password

```bash
./target/release/bridger start -n task-darwinia-ethereum --password
```

or without password to start. then run `set-password` command.

```bash
./target/release/bridger task start -n task-darwinia-ethereum
./target/release/bridger task set-password -n task-darwinia-ethereum
```

## Custom command

### start-darwinia

**params**

| name         | type |
| ------------ | ---- |
| block_number | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-darwinia --api start-darwinia

# If you hit the error `No darwinia scan start`, the block number is the start point Darwinia to Ethereum bridge launched at:
./target/release/bridger task exec --name task-darwinia-darwinia --api start-darwinia --param block_number=4230622
```

### start-ethereum

**params**

| name         | type |
| ------------ | ---- |
| block_number | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api start-darwinia
./target/release/bridger task exec --name task-darwinia-ethereum --api start-darwinia --param block_number=4230622
```

### affirm


**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm --param block=125468
```


### affirm-force

**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm-force --param block=125468
```


### affirm-raw

**params**

| name | type   |
| ---- | ------ |
| json | string |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api affirm-raw --param 'json={"block": 234557}'
```

### confirm

**params**

| name  | type |
| ----- | ---- |
| block | u64  |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api confirm --param block=125468
```

### ecdsa

**params**

| name    | type   |
| ------- | ------ |
| message | string |

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api ecdsa --param "message=Hello"
```

### info-d2e

**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| txblock   | u64    |
| mmrblock  | u64    |
| signblock | u64    |

**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api info-d2e \
  --param network=<NETWORK> \
  --param txblock=12345 \
  --param mmrblock=2345346 \
  --param signblock=824864
```

### keys


**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api keys
```

### sign-mmr-root


**params**

| name      | type   |
| --------- | ------ |
| network   | string |
| mmrblock  | u64    |


**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api sign-mmr-root \
  --param network=<NETWORK> \
  --param mmrblock=2345346
```

### show-parcel


**params**

| name   | type   | allow      | default |
| ------ | ------ | ---------- | ------- |
| output | string | raw / json | raw     |
| block  | u64    |            |         |


**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api show-parcel \
  --param output=json \
  --param block=2345346
```

### relay


**params**

| name         | type |
| ------------ | ---- |
| block_number | u64  |

**examples**

```bash
./target/release/bridger task exec \
  --name task-darwinia-ethereum \
  --api relay \
  --param block_number=2354684
```

**For users who want to relay messages from Darwinia to Ethereum, you must request to become a member of the authority set first**:

1. Open [Extrinsics in apps.darwinia.network](https://apps.darwinia.network/#/extrinsics)
2. If you're not using a proxy account or you have the permission to sign extrinsics using the "real" account, switch to the "real" account and submit `ethereumRelayAuthorities.requestAuthority(stake_amount, signer)`:
   - `stake_amount` is the amount of RINGs to stake.
   - `signer` is the public key of your **Ethereum** account.
     Otherwise, you must switch to your proxy account and submit `proxy.proxy(real_account, ethereumRelayAuthorities.requestAuthority(stake_amount, signer))`. The `stake_amount` will be deducted from your "real" account.
3. Notify council members to submit `ethereumRelayAuthorities.addAuthority(your_account)`.

> How it works: authorities are the validators/nodes in the source chain consensus system to resolve Byzantine Generals' Problem and finalize the blocks. Grandpa authorities are BFT alike authorities, our authority concept comes from the similar meaning, is to be used as a replacement for grandpa authorites.
>
> Updating the authority set involves 2 times of cross-chain: 1) relay the new authority set from Darwinia to Ethereum; 2) relay from Ethereum to Darwinia to send rewards to `ethereum.relayer_beneficiary_darwinia_account` (see below).

Comment out if you don't want to relay authorities change messages to Ethereum (which sends transactions on Ethereum and consumes gas fee), and of course, you will not get rewards.


### guard

**examples**

```bash
./target/release/bridger task exec --name task-darwinia-ethereum --api guard
```