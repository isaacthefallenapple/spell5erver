use super::*;
use std::ops::{Range, RangeBounds};

pub trait Filter {
    fn matches(&self, s: &Spell) -> bool;

    fn and<F>(self, other: F) -> And<Self, F>
    where
        Self: Sized,
        F: Filter,
    {
        And {
            lhs: self,
            rhs: other,
        }
    }

    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Sized,
        F: Filter,
    {
        Or {
            lhs: self,
            rhs: other,
        }
    }
}

pub struct Filters<LR, CTR, DR>
where
    LR: RangeBounds<Level>,
    CTR: RangeBounds<CastingTime>,
    DR: RangeBounds<Duration>,
{
    classes: Option<ClassFilter>,
    schools: Option<SchoolFilter>,
    components: Option<ComponentsFilter>,
    levels: Option<LevelFilter<LR>>,
    casting_time: Option<CastingTimeFilter<CTR>>,
    duration: Option<DurationFilter<DR>>,
}

impl<LR, CTR, DR> Filters<LR, CTR, DR>
where
    LR: RangeBounds<Level>,
    CTR: RangeBounds<CastingTime>,
    DR: RangeBounds<Duration>,
{
    pub fn new() -> Self {
        Self {
            classes: None,
            schools: None,
            components: None,
            levels: None,
            casting_time: None,
            duration: None,
        }
    }

    pub fn with_classes(&mut self, filter: ClassFilter) -> &mut Self {
        self.classes.replace(filter);
        self
    }

    pub fn with_schools(&mut self, filter: SchoolFilter) -> &mut Self {
        self.schools.replace(filter);
        self
    }

    pub fn with_components(&mut self, filter: ComponentsFilter) -> &mut Self {
        self.components.replace(filter);
        self
    }

    pub fn with_levels(&mut self, filter: LevelFilter<LR>) -> &mut Self {
        self.levels.replace(filter);
        self
    }

    pub fn with_casting_time(&mut self, filter: CastingTimeFilter<CTR>) -> &mut Self {
        self.casting_time.replace(filter);
        self
    }

    pub fn with_duration(&mut self, filter: DurationFilter<DR>) -> &mut Self {
        self.duration.replace(filter);
        self
    }
}

impl<LR, CTR, DR> Filter for Filters<LR, CTR, DR>
where
    LR: RangeBounds<Level>,
    CTR: RangeBounds<CastingTime>,
    DR: RangeBounds<Duration>,
{
    fn matches(&self, s: &Spell) -> bool {
        (match &self.classes {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        }) && (match &self.schools {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        }) && (match &self.components {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        }) && (match &self.levels {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        }) && (match &self.casting_time {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        }) && (match &self.duration {
            None => true,
            Some(f) if f.matches(s) => true,
            _ => false,
        })
    }
}

pub struct ClassFilter(pub(crate) ClassSet);

impl Filter for ClassFilter {
    fn matches(&self, s: &Spell) -> bool {
        s.classes().has_intersection(self.0)
    }
}

pub struct SchoolFilter(pub(crate) SchoolSet);

impl Filter for SchoolFilter {
    fn matches(&self, s: &Spell) -> bool {
        self.0.contains(s.school())
    }
}

pub struct LevelFilter<R: RangeBounds<Level>>(pub(crate) R);

impl<R: RangeBounds<Level>> Filter for LevelFilter<R> {
    fn matches(&self, s: &Spell) -> bool {
        self.0.contains(&s.level())
    }
}

pub struct CastingTimeFilter<R: RangeBounds<CastingTime>>(pub(crate) R);

impl<R: RangeBounds<CastingTime>> Filter for CastingTimeFilter<R> {
    fn matches(&self, s: &Spell) -> bool {
        self.0.contains(&s.casting_time())
    }
}

pub struct DurationFilter<R: RangeBounds<Duration>>(pub(crate) R);

impl<R: RangeBounds<Duration>> Filter for DurationFilter<R> {
    fn matches(&self, s: &Spell) -> bool {
        self.0.contains(&s.duration())
    }
}

pub struct AtHigherLevelsFilter(pub(crate) bool);

impl Filter for AtHigherLevelsFilter {
    fn matches(&self, s: &Spell) -> bool {
        self.0 == matches!(s.at_higher_levels(), Some(_))
    }
}

pub struct ComponentsFilter {
    v: bool,
    s: bool,
    m: bool,
}

impl ComponentsFilter {
    pub fn new(v: bool, s: bool, m: bool) -> Self {
        ComponentsFilter { v, s, m }
    }
}

impl Filter for ComponentsFilter {
    fn matches(&self, s: &Spell) -> bool {
        let &Components { s, v, ref m } = s.components();
        self.v == v && self.s == s && self.m == matches!(m, Some(_))
    }
}

pub struct And<Lhs: Filter, Rhs: Filter> {
    lhs: Lhs,
    rhs: Rhs,
}

impl<Lhs: Filter, Rhs: Filter> Filter for And<Lhs, Rhs> {
    fn matches(&self, s: &Spell) -> bool {
        self.lhs.matches(s) && self.rhs.matches(s)
    }
}

pub struct Or<Lhs: Filter, Rhs: Filter> {
    lhs: Lhs,
    rhs: Rhs,
}

impl<Lhs: Filter, Rhs: Filter> Filter for Or<Lhs, Rhs> {
    fn matches(&self, s: &Spell) -> bool {
        self.lhs.matches(s) || self.rhs.matches(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filters() {
        let spell = &super::DUMMY;
        let mut sset = SchoolSet::new();
        sset.add(School::Enchantment);
        let succ_filter = And {
            lhs: LevelFilter(Level(1)..=Level(2)),
            rhs: SchoolFilter(sset),
        };
        assert!(succ_filter.matches(&spell));
        let cset: ClassSet = vec![Class::Bard, Class::Wizard].into_iter().collect();
        assert!(ClassFilter(cset).matches(&spell));
        let succ_filter = Or {
            lhs: ClassFilter(cset),
            rhs: LevelFilter(Level(0)..Level(2)),
        };
        assert!(succ_filter.matches(&spell));
        let fail_filter = And {
            lhs: SchoolFilter(vec![School::Illusion].into_iter().collect()),
            rhs: ClassFilter(cset),
        };
        let f = succ_filter.or(fail_filter);
        assert!(f.matches(&spell));
    }

    #[test]
    fn test_filters_builder() {
        let cf = ClassFilter(ClassSet(0b1110));
        let sf = SchoolFilter(SchoolSet(0b1000));
        let lf = LevelFilter(Level(1)..Level(3));
        let compf = ComponentsFilter::new(true, true, false);
        let filters = Filters::new()
            .with_classes(cf)
            .with_schools(sf)
            .with_levels(lf)
            .with_components(compf);
    }
}
