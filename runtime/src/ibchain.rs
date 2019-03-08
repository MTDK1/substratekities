use parity_codec::Encode;
use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure, decl_event};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec_derive::{Encode, Decode};

use rstd::prelude::*;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Asset<Hash> {
    id: Hash,
    name: Vec<u8>,
    issueqty: u64,
    open: bool
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance
    {
        Issued(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
        Transferred(AccountId, AccountId, Hash),
        Bought(AccountId, AccountId, Hash, Balance),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as IbchainStorage {

        Assets get(asset): map T::Hash => Asset<T::Hash>;
        AssetOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllAssetsArray get(asset_by_index): map u64 => T::Hash;
        AllAssetsCount get(all_asset_count): u64;
        AllAssetsIndex: map T::Hash => u64;

        OwnedAssetsArray get(asset_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedAssetsCount get(owned_asset_count): map T::AccountId => u64;
        OwnedAssetsIndex: map T::Hash => u64;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;

        fn issue(origin, name: Vec<u8>, issue_qty: u64, open: bool) -> Result {

            let sender = ensure_signed(origin)?;

            let owned_asset_count = Self::owned_asset_count(&sender);

            let new_owned_asset_count = owned_asset_count.checked_add(1)
                .ok_or("Overflow adding a new Asset to account balance")?;

            let all_asset_count = Self::all_asset_count();

            let new_all_asset_count = all_asset_count.checked_add(1)
                .ok_or("Overflow adding a new Asset to total supply")?;

            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<AssetOwner<T>>::exists(random_hash), "Asset already exists");

            let new_asset = Asset {
                id: random_hash,
                name: name,
                issueqty: issue_qty,
                open: open
            };

            <Assets<T>>::insert(random_hash, new_asset);
            <AssetOwner<T>>::insert(random_hash, &sender);

            <AllAssetsArray<T>>::insert(all_asset_count, random_hash);
            <AllAssetsCount<T>>::put(new_all_asset_count);
            <AllAssetsIndex<T>>::insert(random_hash, all_asset_count);

            <OwnedAssetsArray<T>>::insert((sender.clone(), owned_asset_count), random_hash);
            <OwnedAssetsCount<T>>::insert(&sender, new_owned_asset_count);
            <OwnedAssetsIndex<T>>::insert(random_hash, owned_asset_count);

            <Nonce<T>>::mutate(|n| *n += 1);
            
            Self::deposit_event(RawEvent::Issued(sender, random_hash));

            Ok(())
        }

        fn issuemore(origin, asset_id: T::Hash, _issue_qty: u64) -> Result {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;
            ensure!(owner == sender, "You do not own this asset");

            let mut _asset = Self::asset(asset_id);


            Ok(())
        }

        fn sendasset(origin) -> Result {
            let _sender = ensure_signed(origin)?;
            Ok(())
        }
    }
}
