use super::*;
use std::ops::{BitAnd, BitOr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl School {
    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

impl FromStr for School {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct SchoolSet(pub(crate) u32);

impl SchoolSet {
    pub fn new() -> Self {
        SchoolSet(0)
    }

    pub fn add(&mut self, school: School) {
        self.0 |= 1 << school as u32;
    }

    pub fn remove(&mut self, school: School) {
        self.0 &= !(1 << school as u32);
    }

    pub fn contains(self, school: School) -> bool {
        self.0 >> school as u32 & 1 == 1
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn has_intersection(self, other: Self) -> bool {
        !(self & other).is_empty()
    }
}

/// `BitAnd (&)` returns the intersection of two `SchoolSet`s.
impl BitAnd for SchoolSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        SchoolSet(self.0 & rhs.0)
    }
}

/// `BitOr (|)` returns the union of two `SchoolSet`s.
impl BitOr for SchoolSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        SchoolSet(self.0 | rhs.0)
    }
}

impl IntoIterator for SchoolSet {
    type Item = School;
    type IntoIter = SchoolSetIter;

    fn into_iter(self) -> Self::IntoIter {
        SchoolSetIter(self, 0)
    }
}

impl FromIterator<School> for SchoolSet {
    fn from_iter<I: IntoIterator<Item = School>>(iter: I) -> Self {
        let mut set = SchoolSet::new();

        for school in iter {
            set.add(school);
        }

        set
    }
}

impl Display for SchoolSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .into_iter()
            .map(|class| class.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

pub struct SchoolSetIter(SchoolSet, u32);

const SCHOOLS: [School; 8] = {
    use School::*;
    [
        Abjuration,
        Conjuration,
        Divination,
        Enchantment,
        Evocation,
        Illusion,
        Necromancy,
        Transmutation,
    ]
};

impl Iterator for SchoolSetIter {
    type Item = School;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.1 as usize;
        if idx >= SCHOOLS.len() {
            return None;
        }
        self.1 += 1;
        let school = SCHOOLS[idx];
        if self.0.contains(school) {
            Some(school)
        } else {
            self.next()
        }
    }
}
