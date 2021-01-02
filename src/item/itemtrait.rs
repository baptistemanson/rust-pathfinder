use flagset::{flags, FlagSet};
flags! {
    pub enum  ItemTrait: u64 {
        Agile,
        Attached,
        Backstabber,
    }
}
