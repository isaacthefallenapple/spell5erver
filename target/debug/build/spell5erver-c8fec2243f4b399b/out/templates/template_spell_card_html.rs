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
out.write_all(b"&display=swap\"\n      rel=\"stylesheet\"\n    />\n  </head>\n\n  <body>\n    <div class=\"spell-card\">\n      <span class=\"title\">\n        <h3>")?;
spell.name().to_html(&mut out)?;
out.write_all(b"</h3>\n        <p>")?;
spell.school_and_level().to_html(&mut out)?;
out.write_all(b"</p>\n      </span>\n      <ul>\n        <li class=\"casting-time\">\n          Casting Time:&emsp;<span>")?;
spell.casting_time().to_html(&mut out)?;
out.write_all(b"</span>\n        </li>\n        <li class=\"range\">Range:&emsp;<span>")?;
spell.range().to_html(&mut out)?;
out.write_all(b"</span></li>\n        <li class=\"components\">\n          Components:&emsp;<span>")?;
spell.components().to_html(&mut out)?;
out.write_all(b"</span>\n        </li>\n        <li class=\"duration\">Duration:&emsp;<span>")?;
spell.duration().to_html(&mut out)?;
out.write_all(b"</span></li>\n        <li class=\"classes\">Classes:&emsp;<span>")?;
spell.classes().to_html(&mut out)?;
out.write_all(b"</span></li>\n      </ul>\n      ")?;
for par in spell.paragraphs() {
out.write_all(b"\n      <p>\n        ")?;
par.to_html(&mut out)?;
out.write_all(b"\n      </p>\n      ")?;
}
out.write_all(b"<p class=\"footnote\">Reference: ")?;
spell.reference().to_html(&mut out)?;
out.write_all(b"</p>\n    </div>\n  </body>\n</html>\n")?;
Ok(())
}
