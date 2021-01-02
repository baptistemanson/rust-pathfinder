use flagset::flags;
use flagset::FlagSet;

pub type TraitSet = FlagSet<Trait>;

flags! {
    pub enum Trait: u64 {
        Agile,
        Attached,
        Backstabber,
        Backswing,
        DeadlyD10
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
