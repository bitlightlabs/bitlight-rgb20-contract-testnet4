# bitlight-rgb20-contract-testnet4
Demonstrating the RGB20 Contract using RGB v0.11.0-beta and Testing RGB Features on Testnet4

## Pre-request

To complete the demo, you need to set up the following toolchains:

**⚠️ Warning:** Because the `RGB-WG` official repository has not yet merged the Testnet4-related PR, we are temporarily relying on the bitlightlabs repository.

1. [Git][git]
2. [Rust][rust]
3. [RGB-CLI][rgb-cli]

[git]: https://git-scm.com/

[rust]: https://www.rust-lang.org/tools/install

[rgb-cli]: https://github.com/will-bitlight/rgb

### RGB-CLI

First, clone it from GitHub:

```bash
git clone https://github.com/will-bitlight/rgb.git
cd rgb
git checkout testnet4
```

Then, build it, copy the binary to `~/.cargo/bin/`:

```
cargo build --package rgb-wallet --bin rgb --release 
cp target/release/rgb ~/.cargo/bin/
```

### Key preparation

Users should prepare two funded keys in advance on bitcoin-testnet4.

If not, you can use the public test key provided by bitlight below.

```
# Alice
[5183a8d8/86'/1'/0']tpubDDtdVYn7LWnWNUXADgoLGu48aLH4dZ17hYfRfV9rjB7QQK3BrphnrSV6pGAeyfyiAM7DmXPJgRzGoBdwWvRoFdJoMVpWfmM9FCk8ojVhbMS/<0;1;9;10>/*"

# Bob
[3abb3cbb/86'/1'/0']tpubDDeBXqUqyVcbe75SbHAPNXMojntFu5C8KTUEhnyQc74Bty6h8XxqmCavPBMd1fqQQAAYDdp6MAAcN4e2eJQFH3v4txc89s8wvHg98QSUrdL/<0;1;9;10>/*
```

## Create and Import Contract

To create a RGB20 contract, just clone this repo to you local machine. Then
compile and run it.

```bash
git clone https://github.com/bitlightlabs/bitlight-rgb20-contract-testnet4.git
cd bitlight-rgb20-contract-testnet4
```

edit main.rs, change the beneficiary to alice's address

Note: The following `1240a...f6d51e` is the transaction-id in which Alice receives the payment. `0` is the index of the output of the transaction.

```rust
let beneficiary_txid = Txid::from_hex("1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e").unwrap();
let beneficiary = Outpoint::new(beneficiary_txid, 0);
```

```
make run
```

```text
The issued contract data:
{"ticker":"TEST","name":"Test asset","details":null,"precision":"centiMicro"}
amount=adMhBHaQ, owner=bc:tapret1st:1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e:0, witness=~
totalSupply=adMhBHaQ

Contracts are available in the examples directory
---------------------------------
./examples:
-rw-r--r--  1 bitlight  staff  5639 Jul 12 21:52 rgb20-simplest.rgb
-rw-r--r--  1 bitlight  staff  7548 Jul 12 21:52 rgb20-simplest.rgba
---------------------------------
```

Now, we are creating a RGB20 #TEST contract, which stores in `examples` fold.


Before importing contracts, let's import our wallets to rgb.

Export `LNPBP_NETWORK` env set the network environment used by the wallet to bitcoin-testnet4.

Export `MEMPOOL_SERVER` env set the data source used by the wallet to mempool-testnet4

```bash
export LNPBP_NETWORK=testnet4
export MEMPOOL_SERVER="https://mempool.space/testnet4/api"
```

Create rgb wallet container for Alice:

```bash
# rgb -d .alice create default --tapret-key-only <alice-fixed-xpub-descriptor>
$ rgb -d .alice create default --tapret-key-only "[5183a8d8/86'/1'/0']tpubDDtdVYn7LWnWNUXADgoLGu48aLH4dZ17hYfRfV9rjB7QQK3BrphnrSV6pGAeyfyiAM7DmXPJgRzGoBdwWvRoFdJoMVpWfmM9FCk8ojVhbMS/<0;1;9;10>/*"

Loading descriptor from command-line argument ... success
Syncing keychain 0 ........... keychain 1 .......... keychain 9 ........... keychain 10 .......... success
Saving the wallet as 'default' ... success

$ rgb -d .alice utxos

Height     Amount, ṩ    Outpoint                                                            
tb1pn0s2pajhsw38fnpgcj79w3kr3c0r89y3xyekjt8qaudje70g4shs8keguu  &0/0
34489         245069    f5fcd7d27fe8e0915942f65c97886a2bffc674fcd1d77f5e952f4bebac8cf601:0
34506         100000    f656506f6909ab44b78320859dd5bd8b3f2634a471c706649574cba485392dc7:1

tb1pr3rupmav8a7av7dqfyvynu2wk02lduggnh9ln4ndze9aqvuv9y3smxy992  &9/0
34527         108000    1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e:0

Loading descriptor from wallet default ... success

Wallet total balance: 453069 ṩ

```

Create rgb wallet container for Bob:

```bash
# rgb -d .bob create default --tapret-key-only <bob-fixed-xpub-descriptor>

$ rgb -d .bob create default --tapret-key-only "[3abb3cbb/86'/1'/0']tpubDDeBXqUqyVcbe75SbHAPNXMojntFu5C8KTUEhnyQc74Bty6h8XxqmCavPBMd1fqQQAAYDdp6MAAcN4e2eJQFH3v4txc89s8wvHg98QSUrdL/<0;1;9;10>/*"

Loading descriptor from command-line argument ... success
Syncing keychain 0 ........... keychain 1 .......... keychain 9 ........... keychain 10 .......... success
Saving the wallet as 'default' ... success

$ rgb -d .bob utxos

Height     Amount, ṩ    Outpoint                                                            
tb1plphl407vyfpml2thhypzuqk232256njnaw4zhtmyrrku66pqn9ustvvjsd  &0/0
34491          66123    6b5c3df74418e04a9f580625288f53c321121a2028c572c984940b60f8de7725:0

tb1p9yjaffzhuh9p7d9gnwfunxssngesk25tz7rudu4v69dl6e7w7qhqellhrw  &9/0
34528         121000    ce8dc940f5ff7c5867b9102f2e9e6588fa8f668c0d41d01d803b50f41c0ba56c:1

Loading descriptor from wallet default ... success

Wallet total balance: 187123 ṩ

```

Import contract for Alice

```
$ rgb -d .alice import examples/rgb20-simplest.rgb

Importing consignment rgb:csg:OzVmRfgj-9IJSfE1-mA6T3a0-j3eTOk8-nOgjsaQ-etElCHc#athena-voyage-everest:
- validating the contract rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4 ... success
Consignment is imported
```

After that, we can inspect contracts state with `rgb state` command.

get the state of the contract for Alice

```bash
$ rgb -d .alice contracts

rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4    bitcoin                 2024-07-12      rgb:sch:KzMZV9bO7gFhox97!klj0FonG2ZKnjuOIg2tFChu$YA#lucas-episode-silicon       
  Developer: ssi:anonymous

# rgb -d <DATA_DIR> state <CONTRACT_ID> <IFACE>
$ rgb -d .alice state 'rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4' RGB20Fixed

Global:
  spec := (ticker=("TEST"), name=("Test asset"), details=~, precision=8)
  terms := (text=(""), media=~)
  issuedSupply := (100000000000)

Owned:
  assetOwner:
    value=100000000000, utxo=bc:tapret1st:1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e:0, witness=~ 
```

Import contract For Bob:

```bash
$ rgb -d .bob import examples/rgb20-simplest.rgb
$ rgb -d .bob contracts
$ rgb -d .bob state 'rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4' RGB20Fixed

Global:
  spec := (ticker=("TEST"), name=("Test asset"), details=~, precision=8)
  terms := (text=(""), media=~)
  issuedSupply := (100000000000)

Owned:
  assetOwner:
```

Now we have successfully created an rgb20 token, and the owner
is `bc:tapret1st:1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e:0`, which belongs to Alice.

## Transfer

There are about five steps in a complete transfer:

1. Create an invoice
2. Construct a PSBT
3. Make a transfer
4. Accept the transfer

### Create an invoice

To receive 1,000 #Test, Bob needs to create an invoice and send it to Alice.

```bash
$ rgb -d .bob invoice 'rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4' RGB20Fixed 2000

rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4/RGB20Fixed/TadF+tb:utxob:!TvWPzkY-Sx6U9fY-S5Cjx3V-!D3v75N-DefZ5W7-ymNJceu-vAgLw
```

### Make a transfer

```bash
$ rgb -d .alice transfer 'rgb:WvHfDZDt-8UNnXDY-JuOjr2A-1n8T3hV-avzUeh2-sfNyku4/RGB20Fixed/TadF+tb:utxob:!TvWPzkY-Sx6U9fY-S5Cjx3V-!D3v75N-DefZ5W7-ymNJceu-vAgLw' transfer.consignment alice.psbt
```

The consignment is saved in the `transfer.consignment` file, and needs to be sent to Bob, who is waiting to accept it.

### Accept transfer

After receiving the `transfer.consignment` file, Bob could validate it before accepting.

```bash
$ rgb -d .bob validate transfer.consignment
The provided consignment is valid
```

Bob accepts the consignment:

```bash
$ rgb -d .bob accept -f transfer.consignment
Transfer accepted into the stash
```