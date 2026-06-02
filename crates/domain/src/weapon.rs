use std::marker::PhantomData;

#[derive(Debug)]
pub struct MainHand;

#[derive(Debug)]
pub struct OffHand;

#[derive(Debug)]
pub struct Weapon<Kind> {
    pub damage: u8,
    pub range_type: WeaponRangeType,

    _kind: PhantomData<Kind>,
}

pub type MainHandWeapon = Weapon<MainHand>;
pub type OffHandWeapon = Weapon<OffHand>;

#[derive(Debug)]
pub enum WeaponRangeType {
    /// Melee is always 1 cell range
    Melee,
    /// Ranged attacks have a custom range in cells
    Ranged(u8),
}

impl<Kind> Weapon<Kind> {
    pub fn new(damage: u8, range_type: WeaponRangeType) -> Self {
        Self {
            damage,
            range_type,
            _kind: PhantomData,
        }
    }
}
