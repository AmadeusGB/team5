#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
        Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransfer(AccountId, AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
	}
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

        /// Allow a user to claim ownership of an unclaimed proof
        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not been claimed yet or error with the message
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // Store the proof with the sender and the current block number
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        /// Allow the owner to revoke their claim
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim
            let (owner, _) = Proofs::<T>::get(&proof);

            // Verify that sender of the current call is the claim owner
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }

        /// Allow the owner to revoke their claim
        #[weight = 10_000]
        fn transfer_claim(origin, proof: Vec<u8>, to: T::AccountId) {

            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, block_number) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::insert(&proof, (&to, block_number));

            Self::deposit_event(RawEvent::ClaimTransfer(sender, to, proof));
        }
    }
}