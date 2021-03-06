use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Range {
    Self_(Option<(Box<Range>, String)>),
    Touch,
    Feet(u32),
    Sight,
    Miles(u32),
    Other(String),
}

impl Range {
    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

impl FromStr for Range {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Range::*;

        match self {
            Feet(n) => write!(f, "{} feet", n),
            Miles(n) => write!(f, "{}", pluralize(*n, "mile")),
            Self_(None) => write!(f, "Self"),
            Self_(Some((range, ref shape))) => write!(f, "{} {}", range, shape),
            Touch => write!(f, "Touch"),
            Sight => write!(f, "Sight"),
            Other(s) => write!(f, "{}", s),
        }
    }
}
