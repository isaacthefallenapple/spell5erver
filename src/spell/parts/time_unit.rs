use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum TimeUnit {
    Instantaneous,
    Reaction(String),
    BonusAction,
    Action,
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Other(String),
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TimeUnit::*;

        match self {
            Instantaneous => write!(f, "Instantaneous"),
            Action => write!(f, "1 Action"),
            BonusAction => write!(f, "1 Bonus Action"),
            Minutes(n) => write!(f, "{}", pluralize(*n, "minute")),
            Hours(n) => write!(f, "{}", pluralize(*n, "hour")),
            Days(n) => write!(f, "{}", pluralize(*n, "day")),
            Reaction(trigger) => write!(f, "Reaction: {}", trigger),
            Other(s) => write!(f, "{}", s),
        }
    }
}

/// `CastingTime` wraps a `TimeUnit` and a `bool` that indicates
/// whether or not the spell can be performed as a ritual.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CastingTime(pub TimeUnit, pub bool);

impl PartialOrd for CastingTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CastingTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Display for CastingTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastingTime(time, false) => Display::fmt(time, f),
            CastingTime(time, true) => write!(f, "{} (ritual)", time),
        }
    }
}

/// `Duration` wraps a `TimeUnit` and a `bool` that indicates
/// whether or not the spell requires concentration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Duration(pub TimeUnit, pub bool);

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration(time, false) => Display::fmt(time, f),
            Duration(time, true) => write!(f, "Concentration, up to {}", time),
        }
    }
}
