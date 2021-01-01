use super::dice;

pub trait Item {
    fn get_info(&self) -> &ItemInfo;
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum EquippableItem {
    FaceItem(FaceItem),
    TwoHandItem(TwoHandItem),
    OneHandItem(OneHandItem),
}


#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub name: String,
    pub bulk: i64,
}

// will be able to generalize
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DamageFormula {
    ClassicDamageFormula { dice_faces: i64, nb_dice: i64, bonus: i64,}
}

pub struct DamageRollResults {
    pub value: i64,
    pub details: String
}

impl DamageFormula {
    pub fn roll(&self) -> DamageRollResults {
        match self {
            DamageFormula::ClassicDamageFormula {dice_faces, nb_dice, bonus} => {
                let roll = dice::dx(*dice_faces);
                let value = roll*nb_dice+ bonus;
                DamageRollResults {value: roll*nb_dice+ bonus, details: format!("{} x d{} + {} = {}", nb_dice, dice_faces, bonus, value )}
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct OneHandItem {
    pub info: ItemInfo,
    pub damage: DamageFormula
}

impl Item for OneHandItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

impl Item for TwoHandItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub struct TwoHandItem {
    pub info: ItemInfo,
    pub damage: DamageFormula
}

#[derive(Clone, Debug)]
pub struct FaceItem {
    pub info: ItemInfo,
}

