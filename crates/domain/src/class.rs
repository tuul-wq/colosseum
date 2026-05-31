use crate::{mage::MageClass, warrior::WarriorClass};

#[derive(Debug)]
pub enum HeroClass {
    Warrior(WarriorClass),
    Mage(MageClass),
}
