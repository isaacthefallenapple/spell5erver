use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Components {
    pub(crate) v: bool,
    pub(crate) s: bool,
    pub(crate) m: Option<String>,
}

impl Components {
    pub fn new(v: bool, s: bool, m: Option<&str>) -> Self {
        Components {
            v,
            s,
            m: m.map(|s| s.to_string()),
        }
    }

    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

impl FromStr for Components {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl FromIterator<Component> for Components {
    fn from_iter<I: IntoIterator<Item = Component>>(iter: I) -> Self {
        let mut v = false;
        let mut s = false;
        let mut m = None;
        for comp in iter {
            match comp {
                Component::V => v = true,
                Component::S => s = true,
                Component::M(s) => m = Some(s),
            }
        }
        Components { v, s, m }
    }
}

impl Display for Components {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        let Components { v, s, m } = self;
        if *v {
            parts.push(String::from("V"));
        }
        if *s {
            parts.push(String::from("S"));
        }
        if let Some(s) = m {
            parts.push(format!("M ({})", s));
        }
        write!(f, "{}", parts.join(", "))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Component {
    V,
    S,
    M(String),
}

impl Component {
    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

impl FromStr for Component {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Component::*;

        match self {
            V | S => Debug::fmt(self, f),
            M(s) => write!(f, "M ({})", s),
        }
    }
}
