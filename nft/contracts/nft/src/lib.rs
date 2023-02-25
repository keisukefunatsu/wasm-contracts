#![cfg_attr(not(feature = "std"), no_std)]

// pub use ink;

#[openbrush::contract]
mod nft {
    use openbrush::{
        contracts::psp34::{extensions, *},
        modifiers,
        traits::Storage,
    };
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Nft {
        psp34: psp34::Data,
        metadata: extensions::metadata::Data,
    }
    // We have to implement the "main trait" for our contract to have the PSP22 methods available.
    impl psp34::PSP34 for Nft {}
    impl Nft {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            instance
        }
        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            true
        }
    }

    // impl metadata::PSP34Metadata for Nft {}
}
