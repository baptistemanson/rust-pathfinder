use flagset::flags;
use flagset::FlagSet;
pub mod implementation;

pub type TraitSet = FlagSet<Trait>;

flags! {
    pub enum Trait: u64 {
        DeadlyD6,
        DeadlyD8,
        DeadlyD10,
        Propulsive,
        Unarmed,
        Finesse,
        Silver,
        Adamantine,
        ColdIron,
    }
}

pub fn none() -> TraitSet {
    TraitSet::default()
}
// some doc
// let set = !FlagSet::from(Flag::Foo);
// assert!(!set.is_empty());
// assert!(!set.is_full());
// assert!(!set.contains(Flag::Foo));
// assert!(set.contains(Flag::Bar));
// assert!(set.contains(Flag::Baz));
