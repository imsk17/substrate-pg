#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a count is incremented. [who, new_count]
		CountIncremented(T::AccountId, u32),
		GetCount(T::AccountId, u32),
		CountDeleted(T::AccountId),
		CountStarted(T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAssociatedCount,
		CountAlreadyExists
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Counts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn increment_counter(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			ensure!(Counts::<T>::contains_key(&sender), Error::<T>::NoAssociatedCount);

			let current_count = Counts::<T>::get(&sender);

			Counts::<T>::insert(&sender, 0);

			Self::deposit_event(Event::CountIncremented(sender, current_count+1));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn start_count(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!(!Counts::<T>::contains_key(&sender), Error::<T>::CountAlreadyExists);

			Counts::<T>::insert(&sender, 0);

			Self::deposit_event(Event::CountStarted(sender, 0));

			Ok(().into())
		}


		#[pallet::weight(10_000)]
		pub fn reset_count(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Counts::<T>::contains_key(&sender), Error::<T>::NoAssociatedCount);

			// Remove count from storage.
			Counts::<T>::remove(&sender);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::CountDeleted(sender));

			Ok(().into())
		}
	}
}
