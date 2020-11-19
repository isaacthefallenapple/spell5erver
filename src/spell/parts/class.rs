use super::*;
use std::ops::{BitAnd, BitOr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl From<Class> for u32 {
    fn from(c: Class) -> Self {
        1 << c as u32
    }
}

pub const CASTERS: ClassSet = ClassSet(0b111011001110);

impl Class {
    pub fn to_ron(&self) -> ron::ser::Result<String> {
        ron::ser::to_string(self)
    }
}

impl BitOr for Class {
    type Output = ClassSet;
    fn bitor(self, rhs: Self) -> Self::Output {
        ClassSet(u32::from(self) | u32::from(rhs))
    }
}

impl FromStr for Class {
    type Err = ron::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::de::from_str(s)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct ClassSet(pub(crate) u32);

impl ClassSet {
    pub fn new() -> Self {
        ClassSet(0)
    }

    pub fn add(&mut self, class: Class) {
        self.0 |= u32::from(class);
    }

    pub fn remove(&mut self, class: Class) {
        self.0 &= !(u32::from(class));
    }

    pub fn contains(self, class: Class) -> bool {
        self.0 & u32::from(class) == 1
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn has_intersection(self, other: Self) -> bool {
        !(self & other).is_empty()
    }
}

impl BitOr<Class> for ClassSet {
    type Output = Self;
    fn bitor(mut self, rhs: Class) -> Self::Output {
        self.add(rhs);
        self
    }
}

/// `BitAnd (&)` returns the intersection of two `ClassSet`s.
impl BitAnd for ClassSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        ClassSet(self.0 & rhs.0)
    }
}

/// `BitOr (|)` returns the union of two `ClassSet`s.
impl BitOr for ClassSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ClassSet(self.0 | rhs.0)
    }
}

impl IntoIterator for ClassSet {
    type Item = Class;
    type IntoIter = ClassSetIter;

    fn into_iter(self) -> Self::IntoIter {
        ClassSetIter(self, 0)
    }
}

impl FromIterator<Class> for ClassSet {
    fn from_iter<I: IntoIterator<Item = Class>>(iter: I) -> Self {
        let mut set = ClassSet::new();

        for class in iter {
            set.add(class);
        }

        set
    }
}

impl Display for ClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .into_iter()
            .map(|class| class.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

pub struct ClassSetIter(ClassSet, u32);

const CLASSES: [Class; 12] = {
    use Class::*;
    [
        Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock,
        Wizard,
    ]
};

impl Iterator for ClassSetIter {
    type Item = Class;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.1 as usize;
        if idx >= CLASSES.len() {
            return None;
        }
        self.1 += 1;
        let class = CLASSES[idx];
        if self.0.contains(class) {
            Some(class)
        } else {
            self.next()
        }
    }
}
