use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use super::statics::spell_card_css;
use crate::spell::Spell;

pub fn spell_card_html<W>(mut out: &mut W, spell: &Spell) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n\n    <title>")?;
spell.name().to_html(&mut out)?;
out.write_all(b"</title>\n\n    <link\n      rel=\"stylesheet\"\n      type=\"text/css\"\n      href=\"/static/")?;
spell_card_css.name.to_html(&mut out)?;
out.write_all(b"\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css?family=Fira+Sans:400,400i,600,600i,800,800i&display=swap\"\n      rel=\"stylesheet\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css2?family=Roboto:wght")?;
900.to_html(&mut out)?;
out.write_all(b"&display=swap\"\n      rel=\"stylesheet\"\n    />\n  </head>\n\n  <body>\n    <div class=\"spell-card\">\n      <div class=\"title\">\n        <span class=\"name\">")?;
spell.name().to_html(&mut out)?;
out.write_all(b"</span>\n        <span class=\"school-level\">")?;
spell.school_and_level().to_html(&mut out)?;
out.write_all(b"</span>\n      </div>\n      <div class=\"stats-container\">\n        <span class=\"inline-heading\">Casting Time</span><span>")?;
spell.casting_time().to_html(&mut out)?;
out.write_all(b"</span>\n        <span class=\"inline-heading\">Range</span><span>")?;
spell.range().to_html(&mut out)?;
out.write_all(b"</span>\n        <span class=\"inline-heading\">Components</span><span>")?;
spell.components().to_html(&mut out)?;
out.write_all(b"</span>\n        <span class=\"inline-heading\">Duration</span><span>")?;
spell.duration().to_html(&mut out)?;
out.write_all(b"</span>\n        <span class=\"inline-heading\">Classes</span><span>")?;
spell.classes().to_html(&mut out)?;
out.write_all(b"</span>\n      </div>\n      \n      ")?;
for par in spell.paragraphs() {
out.write_all(b"\n      <p>\n        ")?;
par.to_html(&mut out)?;
out.write_all(b"\n      </p>\n      ")?;
}
if let Some(mut at_higher_levels) = spell.at_higher_levels() {
out.write_all(b"\n        <p>\n          <span class=\"inline-heading\">At higher levels</span>. ")?;
at_higher_levels.next().unwrap().to_html(&mut out)?;
out.write_all(b"\n        </p>\n        ")?;
for par in at_higher_levels {
out.write_all(b"\n        <p>\n          ")?;
par.to_html(&mut out)?;
out.write_all(b"\n        </p>\n        ")?;
}
}
out.write_all(b"\n      <p class=\"footnote\">Reference: ")?;
spell.reference().to_html(&mut out)?;
out.write_all(b"</p>\n    </div>\n  </body>\n</html>\n")?;
Ok(())
}
