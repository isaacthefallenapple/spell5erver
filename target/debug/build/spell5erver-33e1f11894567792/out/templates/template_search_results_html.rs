use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use super::statics::spell_card_css;
use crate::spell::Spell;

pub fn search_results_html<W>(mut out: &mut W, query: &str, spells: &[&Spell]) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n\n    <title>Results for \"")?;
query.to_html(&mut out)?;
out.write_all(b"\"</title>\n\n    <link\n      rel=\"stylesheet\"\n      type=\"text/css\"\n      href=\"/static/")?;
spell_card_css.name.to_html(&mut out)?;
out.write_all(b"\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css?family=Fira+Sans:400,400i,600,600i,800,800i&display=swap\"\n      rel=\"stylesheet\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css2?family=Roboto:wght")?;
900.to_html(&mut out)?;
out.write_all(b"&display=swap\"\n      rel=\"stylesheet\"\n    />\n  </head>\n\n  <body>\n    <table>\n      <tr>\n        <th>Name</th>\n        <th>Level</th>\n        <th>School</th>\n        <th>Components</th>\n        <th>Casting Time</th>\n        <th>Range</th>\n        <th>Duration</th>\n        <th>Classes</th>\n      </tr>\n      ")?;
for spell in spells.iter() {
out.write_all(b"\n      <tr>\n        <td>\n          <a href=\"spell/")?;
spell.file_name().to_html(&mut out)?;
out.write_all(b"\">")?;
spell.name().to_html(&mut out)?;
out.write_all(b"</a>\n        </td>\n        <td>\n          ")?;
format!("{:#?}", spell.level()).to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.school().to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.components().to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.casting_time().to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.range().to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.duration().to_html(&mut out)?;
out.write_all(b"\n        </td>\n        <td>\n          ")?;
spell.classes().to_html(&mut out)?;
out.write_all(b"\n        </td>\n      </tr>\n      ")?;
}
out.write_all(b"</table>\n  </body>\n</html>\n")?;
Ok(())
}
