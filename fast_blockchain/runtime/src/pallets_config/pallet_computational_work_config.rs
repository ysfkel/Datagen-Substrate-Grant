use crate::*;

/// Configure the pallet-computational-work in pallets/computational_work.
impl pallet_computational_work::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
}