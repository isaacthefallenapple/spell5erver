use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use super::statics::spell_card_css;

pub fn _spell_card_html<W>(mut out: &mut W) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all("<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n\n    <title>Abi-Dalzim’s Horrid Wilting</title>\n\n    <link\n      rel=\"stylesheet\"\n      type=\"text/css\"\n      href=\"/static/".as_bytes())?;
spell_card_css.name.to_html(&mut out)?;
out.write_all(b"\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css?family=Fira+Sans:400,400i,600,600i,800,800i&display=swap\"\n      rel=\"stylesheet\"\n    />\n    <link\n      href=\"https://fonts.googleapis.com/css2?family=Roboto:wght")?;
900.to_html(&mut out)?;
out.write_all("&display=swap\"\n      rel=\"stylesheet\"\n    />\n  </head>\n\n  <body>\n    <div class=\"spell-card\">\n      <span class=\"title\">\n        <h3>Abi-Dalzim’s Horrid Wilting</h3>\n        <p>8th level Necromancy</p>\n      </span>\n      <ul>\n        <li class=\"casting-time\">Casting Time:&emsp;<span>1 Action</span></li>\n        <li class=\"range\">Range:&emsp;<span>150 feet</span></li>\n        <li class=\"components\">\n          Components:&emsp;<span>V, S, M (a bit of sponge)</span>\n        </li>\n        <li class=\"duration\">Duration:&emsp;<span>Instantaneous</span></li>\n        <li class=\"classes\">Classes:&emsp;<span>Sorcerer, Wizard</span></li>\n      </ul>\n      <p>\n        You draw the moisture from every creature in a 30-foot cube centered on\n        a point you choose within range. Each creature in that area must make a\n        Constitution saving throw. Constructs and undead aren’t affected, and\n        plants and water elementals make this saving throw with disadvantage. A\n        creature takes 12d8 necrotic damage on a failed save, or half as much\n        damage on a successful one.\n      </p>\n      <p>\n        Nonmagical plants in the area that aren’t creatures, such as trees and\n        shrubs, wither and die instantly.\n      </p>\n      <p class=\"footnote\">Reference: Page 15 from EE Players Companion</p>\n    </div>\n  </body>\n</html>\n".as_bytes())?;
Ok(())
}
