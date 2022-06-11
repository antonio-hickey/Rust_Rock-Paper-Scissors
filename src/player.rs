use std::fmt;

pub use crate::weapons::Weapon;
pub use crate::weapons::WeaponKind;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Player {
    pub name: String,
    pub weapon: Weapon,
    pub wins: i32,
    pub losses: i32,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from("Robot"),
            weapon: Weapon {
                kind: WeaponKind::Rock,
                draws: WeaponKind::Rock,
                beats: WeaponKind::Scissor,
                losses: WeaponKind::Paper,
            },
            wins: 0,
            losses: 0,
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Name: {}, Wins: {}, Losses: {})",
            self.name, self.wins, self.losses
        )
    }
}

pub fn render_players(p1_name: String) -> Vec<Player> {
    [
        Player {
            name: p1_name,
            ..Default::default()
        },
        Player {
            ..Default::default()
        },
    ]
    .to_vec()
}
