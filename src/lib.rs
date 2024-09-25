use scrypto::prelude::*;

#[blueprint]
mod token_migration {
    struct TokenMigration {
        old_token: Vault,
        new_token: Vault,
    }

    impl TokenMigration {
        pub fn instantiate(
            old_address: ResourceAddress,
            new_token: Bucket,
            dapp_definition: ComponentAddress
        ) -> Global<TokenMigration> {
            if let Some(old_total_supply) = ResourceManager::from_address(old_address).total_supply() {
                assert_eq!(
                    old_total_supply,
                    new_token.amount(),
                    "New token amount needs to be equal to the total supply of the old token."
                );
            }
            Self::instantiate_without_supply_validation(old_address, new_token, dapp_definition)
        }

        pub fn instantiate_without_supply_validation(
            old_address: ResourceAddress,
            new_token: Bucket,
            dapp_definition: ComponentAddress
        ) -> Global<TokenMigration> {
            assert_ne!(
                old_address,
                new_token.resource_address(),
                "Old and new token addresses are not allowed to be equal."
            );
            (Self {
                old_token: Vault::new(old_address),
                new_token: Vault::with_bucket(new_token),
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .metadata(
                    metadata! {
                        init {
                            "dapp_definition" => dapp_definition, locked;
                        }
                    }
                )
                .globalize()
        }

        pub fn swap(&mut self, old_token: Bucket) -> Bucket {
            let old_amount = old_token.amount();
            self.old_token.put(old_token);
            self.new_token.take(old_amount)
        }

        pub fn add_new_tokens(&mut self, new_token: Bucket) {
            assert_eq!(
                self.new_token.resource_address(),
                new_token.resource_address(),
                "You can only deposit the new tokens"
            );
            self.new_token.put(new_token);
        }
    }
}
