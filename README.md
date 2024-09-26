# Token Migration
## Incremental deposits fork

This [TokenMigration](src/lib.rs) blueprint aims to transition tokens from Olympia to Babylon, leveraging all the advanced resource capabilities of the Radix Babylon Engine (for further details refer to [Metadata Standard](https://docs-babylon.radixdlt.com/main/standards/metadata-standard-introduction.html)).  

The incremental deposit fork allows the new token to be deposited in multiple tranches, rather than once at component creation.

## Create Your New Token
To use the migration blueprint you need to create your own new token first (preferrable with the same total supply than your old token).
Any fungible token can be used with the migration contract. To demonstrate how this can be achieved through the transaction manifest, we've provided two example manifests:

- [Create Owner Badge](manifests/create_owner_badge.rtm)
- [Create Token](manifests/create_token.rtm)

You only need to customize the values indicated by `<dummy value>`:
```
"<name>" -> "Megatoken"
```

Start by creating an owner badge, and then create your token with the owner badge assigned to it.
The owner of the owner badge has the authority to modify metadata. Currently, the `name` and `symbol` fields are locked and cannot be altered by the owner.
However, other fields, such as `description` or `info_url` can be updated.
You have the option to change this behavior by setting the `locked` boolean flag to `true` but it's generally considered a fair trade-off.

By default, `track_total_supply` is set to `false` (second parameter), optimizing scalability for sharded Xian. If you don't anticipate frequent querying or modification of the `total_supply` in Scrypto, you can set it to `true`. Note that when set to `false`, reading the total supply in Scrypto is not possible, although it can still be achieved via the off-ledger gateway.

Manifests can be submitted through the following platforms:
- [Stokenet Developer Console](https://stokenet-console.radixdlt.com)
- [Mainnet Developer Console](https://console.radixdlt.com)

We recommend testing on Stokenet first to ensure that the wallet displays the information correctly before deploying to Mainnet. Once satisfied, you can proceed to instantiate the migration component.

## Instantiation
Now call the `instantiate` function on the blueprint passing the `old_address` (Babylon address of your old token - every Olympia token will have a new address on Bablyon) and `new_token` (full new total supply):

```rust
pub fn instantiate(old_address: ResourceAddress, new_token: Bucket, dapp_definition: ComponentAddress) -> Global<TokenMigration>
```
At instantiation the blueprint checks that the amount of `new_token` bucket provided is equal to the total supply of the old token for additional safety.

If you have a mutable old token you should not mint or burn any of the old tokens after instantiating the `TokenMigration` blueprint. If you have minted more old tokens after instantiation you would need to create another new instance of the `TokenMigration` blueprint for the same addresses.

### Transaction Manifest
Package addresses of blueprint:
- Stokenet: `package_tdx_2_1phm8qwpumehacz53qygcg7ta2xshuuskfw25l0dxw0esp6ng2q5j5y`
- Mainnet: `package_rdx1phw2qpdwkz5wdhj7ynl5e0vhckgvuc5yym7tfc3hm56837klhtmc95`

```
CALL_METHOD
    Address("<account_your_account_with_new_tokens>")
    "withdraw"
    Address("resource_new_token")
    Decimal("<total supply of new token>")
;
TAKE_FROM_WORKTOP
    Address("resource_new_token")
    Decimal("<total supply of new token>")
    Bucket("new_token")
;
CALL_FUNCTION
    Address("<package_address>")
    "TokenMigration"
    "instantiate"
    Address("<resource_old_address>")
    Bucket("new_token")
    Address("<dapp_definition_address>")
;
```

## Token Migration
To swap your old tokens to the new ones you can simply send a bucket of old tokens to the `swap` method on the instantiated `TokenMigration` component.

```rust
pub fn swap(old_token: Bucket) -> Bucket
```

You get the new tokens returned then which you need to deposit in your user's wallet in the transaction manifest.

### Transaction Manifest
```
CALL_METHOD
    Address("account_your_account_with_old_tokens")
    "withdraw"
    Address("resource_old_token")
    Decimal("5")
;
TAKE_FROM_WORKTOP
    Address("resource_old_token")
    Decimal("5")
    Bucket("old_token")
;
CALL_METHOD
    Address("component_address_of_the_migration_component")
    "swap"
    Bucket("old_token")
;
CALL_METHOD
    Address("account_your_account_address")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

## Incremental deposit of new tokens
In case you want to deposit new tokens incrementally in the component you can do that via the `add_new_tokens` method.

### Transaction Manifest
```
CALL_METHOD
    Address("account_your_account_with_new_tokens")
    "withdraw"
    Address("resource_new_token")
    Decimal("5")
;
TAKE_FROM_WORKTOP
    Address("resource_new_token")
    Decimal("5")
    Bucket("new_token")
;
CALL_METHOD
    Address("component_address_of_the_migration_component")
    "add_new_tokens"
    Bucket("new_token")
;
```

## Ociswap Integration

For every Radix project we are offering to add their `TokenMigration` component to Ociswap website which would then automatically ask users after connecting their wallet whether they want to migrate their old tokens to the new ones. This will be implemented as a simple one-click solution to make the user journey as smooth as possible.

Please reach out to us either via `info@ociswap.com` or on [Telegram](https://t.me/ociswap) if you want us to add your own `TokenMigration` component to `ociswap.com`.

## Disable Total Supply Validation
In some cases, there may be a need to migrate to a new token with a reduced total supply. For instance, when you have 'burned' tokens from the old supply by sending them to a permanent lock address
To accommodate such scenarios, you can utilize the following 'instantiate' method:
```rust
pub fn instantiate_without_supply_validation(
    old_address: ResourceAddress,
    new_token: Bucket,
    dapp_definition: ComponentAddress
) -> Global<TokenMigration>
```
However, if you're uncertain about whether this method is suitable for your project, it's advisable to opt for the standard `instantiate` method. You'll realize the need for the former when it's necessary.

Exercise caution when using this method, as it could potentially result in a situation where not everyone can migrate to the new token due to a higher number of old tokens still in circulation compared to the new tokens you've introduced. In such cases, deploying another migration contract may be required."

## Verifiable Scrypto Build
The deployed package has been constructed using the `scrypto-builder` to ensure the integrity and verifiability of the build process.

You can recreate the build locally by running the following command:
```sh
DOCKER_DEFAULT_PLATFORM=linux/amd64 sudo docker run -v .:/src radixdlt/scrypto-builder:v1.0.0
```

To verify the integrity of the build, compare the `SHA256` hashes of the generated files with the following values:
```sh
sha256sum target/wasm32-unknown-unknown/release/token_migration.wasm
sha256sum target/wasm32-unknown-unknown/release/token_migration.rpd
```

SHA256 Hashes of the Scrypto Build:
- token_migration.wasm `1a93957d5fa07cd0eafacd3accb80f19cf8aeebbaac1228215c8afce630fc902`
- token_migration.rpd `e129ac154371e03619db30d41c2cb8cbe9ae3fd55cc591b68d39238e3f81d512`

These hashes serve as a cryptographic fingerprint to confirm the authenticity and integrity of the build artifacts.
