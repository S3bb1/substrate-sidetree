#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use crate::weights::WeightInfo;
#[frame_support::pallet]
pub mod pallet {
	// Import various types used to declare pallet in scope.
	use super::*;
	use frame_support::{
		dispatch::DispatchResult, inherent::Vec, pallet_prelude::*, scale_info::TypeInfo,
		sp_runtime::SaturatedConversion,
	};

	use frame_system::pallet_prelude::*;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Anchor {
		pub hash: Vec<u8>,
		pub operations: u32,
	}
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Information on runtime weights.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Storage definitions

	// Map for anchored hashes with mapping from transaction_count -> anchor
	#[pallet::storage]
	pub type AnchoredHashes<T> = StorageMap<_, Blake2_128Concat, u32, Anchor>;

	// Public count of already submitted transactions
	#[pallet::storage]
	#[pallet::getter(fn transaction_number)]
	pub(super) type TransactionNumber<T> = StorageValue<_, u32, ValueQuery>;

	// Public value for getting the BaseFee for sidetree transactions
	#[pallet::storage]
	pub(super) type BaseFee<T> = StorageValue<_, u32, ValueQuery>;

	// Events definitions

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Every time a hash is anchored, emit a "Anchor" event to notifiy listeners
		Anchor(u32, Anchor),
	}

	#[pallet::error]
	pub enum Error<T> {
		TransactionCountOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Main functionality to anchor a hash. The weight is determined by the number of operations
		// multiplied by the BaseFee See https://identity.foundation/sidetree/spec/#base-fee-variable
		#[pallet::weight(T::WeightInfo::anchor_hash((anchor.operations).saturated_into()))]
		pub fn anchor_hash(origin: OriginFor<T>, anchor: Anchor) -> DispatchResult {
			// Someone has to sign the transaction
			ensure_signed(origin)?;

			// increment the transaction number from the storage
			let new_transaction_number = Self::transaction_number()
				.checked_add(1)
				.ok_or(<Error<T>>::TransactionCountOverflow)?;

			// clone the anchor for the event
			let anchor_clone = anchor.clone();

			// Update storage and set the new transaction number
			<AnchoredHashes<T>>::insert(new_transaction_number, anchor);
			<TransactionNumber<T>>::put(new_transaction_number);

			// Emit the Anchor event for notification
			Self::deposit_event(Event::Anchor(new_transaction_number, anchor_clone));

			Ok(())
		}
	}
}
