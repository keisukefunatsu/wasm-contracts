#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod nft {
    use ink::prelude::string::String;

    // imports from openbrush
    use openbrush::contracts::psp34::extensions::metadata::*;
    use openbrush::contracts::psp34::extensions::mintable::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Nft {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    // Section contains default implementation without any modifications
    impl PSP34 for Nft {}
    impl PSP34Metadata for Nft {}
    impl PSP34Mintable for Nft {}

    impl Nft {
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut _instance = Self::default();
            // let collection_id = _instance.collection_id();
            _instance._set_attribute(
                id.clone(),
                String::from("name").into_bytes(),
                name.into_bytes(),
            );
            _instance._set_attribute(id, String::from("symbol").into_bytes(), symbol.into_bytes());
            _instance
        }
        #[ink(message)]
        pub fn test(&self) -> bool {
            return true;
        }
    }
}
