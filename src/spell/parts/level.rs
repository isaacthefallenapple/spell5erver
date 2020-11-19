use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Copy)]
pub struct Level(pub(crate) u32);

impl Level {
    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

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

impl FromStr for Level {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            if let &Level(0) = self {
                write!(f, "Cantrip")
            } else {
                write!(f, "{}", self.0)
            }
        } else {
            f.debug_tuple("Level").field(&self.0).finish()
        }
    }
}
