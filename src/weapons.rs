use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum WeaponKind {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Weapon {
    pub kind: WeaponKind,
    pub draws: WeaponKind,
    pub beats: WeaponKind,
    pub losses: WeaponKind,
}

pub fn weapons_map() -> HashMap<String, Weapon> {
    HashMap::from([
        (
            String::from("rock"),
            Weapon {
                kind: WeaponKind::Rock,
                draws: WeaponKind::Rock,
                beats: WeaponKind::Scissor,
                losses: WeaponKind::Paper,
            },
        ),
        (
            String::from("paper"),
            Weapon {
                kind: WeaponKind::Paper,
                draws: WeaponKind::Paper,
                beats: WeaponKind::Rock,
                losses: WeaponKind::Scissor,
            },
        ),
        (
            String::from("scissor"),
            Weapon {
                kind: WeaponKind::Scissor,
                draws: WeaponKind::Scissor,
                beats: WeaponKind::Paper,
                losses: WeaponKind::Rock,
            },
        ),
    ])
}
