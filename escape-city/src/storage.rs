use crate::escape::Escape;
use crate::runaways::RunAway;
use crate::auction::Auction;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {

    #[storage_mapper("escapeLastIndex")]
    fn escape_last_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("escapes")]
    fn escapes(&self, id: usize) -> SingleValueMapper<Escape<Self::Api>>;

    #[storage_mapper("dnaDigits")]
    fn trail_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("runawayLastIndex")]
    fn runaway_last_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("runaways")]
    fn runaways(&self, id: usize) -> SingleValueMapper<RunAway<Self::Api>>;

    #[storage_mapper("ownedRunaways")]
    fn owned_runaways(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;

    #[storage_mapper("runawayOwner")]
    fn runaway_owner(&self, id: usize) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("totalRunAways")]
    fn total_runaways(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("nrOwnedRunAways")]
    fn nr_owned_runaways(&self, address: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("approvedAddress")]
    fn approved_address(&self, runaway_id: usize) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("auction")]
    fn auction(&self, runaway_id: usize) -> SingleValueMapper<Auction<Self::Api>>;

}