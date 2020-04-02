use lazy_static::lazy_static;
use parts::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub mod parts;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Spell {
    pub(crate) name: String,
    pub(crate) school: School,
    pub(crate) level: Level,
    pub(crate) casting_time: CastingTime,
    pub(crate) range: Range,
    pub(crate) components: Components,
    pub(crate) duration: Duration,
    pub(crate) text: String,
    pub(crate) at_higher_levels: Option<String>,
    pub(crate) reference: String,
    pub(crate) classes: ClassSet,
}

impl Spell {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn sanitized_name(&self) -> String {
        self.name()
            .chars()
            .filter_map(|b: char| {
                if b == '’' || b == '\'' {
                    None
                } else if !b.is_ascii_alphanumeric() {
                    Some(' ')
                } else {
                    Some(b.to_ascii_lowercase())
                }
            })
            .collect::<String>()
    }

    pub fn file_name(&self) -> String {
        self.sanitized_name()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }

    pub fn school(&self) -> School {
        self.school
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn school_and_level(&self) -> String {
        if let Level(0) = self.level() {
            format!("{} {}", self.school(), self.level())
        } else {
            format!("{} {}", self.level(), self.school())
        }
    }

    pub fn casting_time(&self) -> &CastingTime {
        &self.casting_time
    }

    pub fn range(&self) -> &Range {
        &self.range
    }

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &str> {
        self.text().split('\n')
    }

    pub fn at_higher_levels(&self) -> Option<impl Iterator<Item = &str>> {
        self.at_higher_levels.as_ref().map(|s| s.split('\n'))
    }

    pub fn reference(&self) -> &str {
        self.reference.as_ref()
    }

    pub fn classes(&self) -> ClassSet {
        self.classes
    }
}

impl Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
{}
{}

Casting Time: {}
Range: {}
Components: {}
Duration: {}
Classes: {}

{}

Reference: {}
",
            self.name,
            self.school_and_level(),
            self.casting_time,
            self.range,
            self.components,
            self.duration,
            self.classes,
            self.text,
            self.reference,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Spell2 {
    pub(crate) name: String,
    pub(crate) school: School,
    pub(crate) level: Level,
    pub(crate) casting_time: CastingTime,
    pub(crate) range: Range,
    pub(crate) components: Components,
    pub(crate) duration: Duration,
    pub(crate) text: String,
    pub(crate) at_higher_levels: Option<String>,
    pub(crate) reference: String,
    pub(crate) classes: ClassSet,
}

impl From<Spell> for Spell2 {
    fn from(s: Spell) -> Self {
        Self {
            name: s.name,
            school: s.school,
            level: s.level,
            casting_time: s.casting_time,
            range: s.range,
            components: s.components,
            duration: s.duration,
            text: s.text,
            at_higher_levels: None,
            reference: s.reference,
            classes: s.classes,
        }
    }
}

pub fn transform_spell(spell: Spell) -> Spell2 {
    lazy_static! {
        static ref AT_HIGHER_LEVELS: Regex =
            Regex::new(r"(?ism)\s*^at higher level\s*$\s*(.*)\z").unwrap();
        static ref DAMAGE_INCRESES: Regex =
            Regex::new(r"(?i)this spell’s damage increases").unwrap();
    }
    let mut spell2 = Spell2::from(spell);
    let mut locs = AT_HIGHER_LEVELS.capture_locations();
    if let Some(_) = AT_HIGHER_LEVELS.captures_read(&mut locs, &spell2.text) {
        let (start, _) = locs.get(0).unwrap();
        let (match_start, match_end) = locs.get(1).unwrap();
        let new_text =
            unsafe { String::from_utf8_unchecked(spell2.text.as_bytes()[..start].into()) };
        let at_higher_levels = unsafe {
            String::from_utf8_unchecked(spell2.text.as_bytes()[match_start..match_end].into())
        };
        if DAMAGE_INCRESES.is_match(&at_higher_levels) {
            spell2.text = new_text + "\n" + &at_higher_levels;
            return spell2;
        }
        spell2.at_higher_levels.replace(at_higher_levels);
        spell2.text = new_text;
    }
    spell2
}

pub fn transform_spells(path: impl AsRef<std::path::Path>) {
    use std::fs::File;
    use std::io::prelude::*;
    let f = File::open(path.as_ref()).unwrap();
    let spells: Vec<Spell> = ron::de::from_reader(&f).unwrap();
    let spells2: Vec<_> = spells.into_iter().map(transform_spell).collect();
    let mut out = File::create("./static/spells2.ron").unwrap();
    write!(
        &mut out,
        "{}",
        ron::ser::to_string(&spells2).expect("serialize")
    )
    .expect("write to file");
}

lazy_static! {
    pub static ref DUMMY: Spell = Spell {
        name: String::from("Zone of Truth"),
        school: School::Enchantment,
        level: Level(2),
        casting_time: CastingTime(TimeUnit::Action, false),
        range: Range::Feet(60),
        components: Components::new(true, true, None),
        duration: Duration(TimeUnit::Minutes(10), false),
        classes: vec![Class::Bard, Class::Cleric, Class::Paladin].into_iter().collect(),
        text: String::from("You create a magical zone that guards against deception in a 15-foot-radius sphere centered on a point of your choice within range.\n\
        Until the spell ends, a creature that enters the spell’s area for the first time on a turn or starts its turn there must make a Charisma saving throw. On a failed save, a creature can’t speak a deliberate lie while in \
        the radius. You know whether each creature succeeds or fails on its saving throw.\n\
        An affected creature is aware of the spell and can thus avoid answering questions to which it would normally \
        respond with a lie. Such creatures can be evasive in its answers as long as it remains within the boundaries \
        of the truth."),
        at_higher_levels: None,
        reference: String::from("289 Players Handbook"),
    };
}
