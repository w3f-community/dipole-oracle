// Creating mock runtime here
use super::*;
use crate::{Module, Trait};
use frame_support::{impl_outer_origin, parameter_types, traits::Time, weights::Weight};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use std::cell::RefCell;
use utilities::{Did, Operator, OperatorCategory, OperatorRole, VolumeType};

pub const VOLUME_TYPE_PEAK: VolumeType = VolumeType::Peak;
pub const VOLUME_TYPE_FLAT: VolumeType = VolumeType::Flat;
pub const VOLUME_TYPE_VALLEY: VolumeType = VolumeType::Valley;
impl_outer_origin! {
	pub enum Origin for Test {}
}
pub type AccountId = u64;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type BaseCallFilter = ();
}

thread_local! {
	static TIME: RefCell<u32> = RefCell::new(0);
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;
	fn now() -> Self::Moment {
		TIME.with(|v| *v.borrow())
	}
}

pub struct MockOperator {
	pub owner: AccountId,
	pub role: OperatorRole,
	pub category: OperatorCategory,
	pub is_legal: bool,
}

impl OperatorManager<Did, AccountId, OperatorRole, OperatorCategory> for MockOperator {
	fn register_operator(
		_who: AccountId,
		_role: OperatorRole,
		_category: OperatorCategory,
	) -> DispatchResult {
		Ok(())
	}

	fn get_operator(_id: Did) -> Option<Operator<AccountId, OperatorRole, OperatorCategory>> {
		Some(Operator {
			owner: 2,
			role: OperatorRole::PrivateProducer,
			category: OperatorCategory::ElectricMeter,
			is_legal: true,
		})
	}

	fn get_owned_operators(_id: AccountId) -> Vec<Did> {
		Vec::new()
	}
}

impl Trait for Test {
	type Event = ();
	type Time = Timestamp;
	type Operator = MockOperator;
}

pub type GoodsOracleModule = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
