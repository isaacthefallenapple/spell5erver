use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Level(pub u32);

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Level(n) = self;

        match n {
            0 => write!(f, "Cantrip"),
            1 => write!(f, "1st level"),
            2 => write!(f, "2nd level"),
            3 => write!(f, "3rd level"),
            n => write!(f, "{}th level", n),
        }
    }
}
