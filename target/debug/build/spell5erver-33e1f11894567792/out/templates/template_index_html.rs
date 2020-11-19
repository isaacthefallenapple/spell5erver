use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use super::statics::{filter_form_js, index_css};
use crate::spell::parts::*;

pub fn index_html<W>(mut out: &mut W) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n\n    <title>Spell5</title>\n    <link\n      href=\"https://fonts.googleapis.com/css?family=Fira+Sans:400,400i,600,600i,800,800i&display=swap\"\n      rel=\"stylesheet\"\n    />\n    <link\n      rel=\"stylesheet\"\n      type=\"text/css\"\n      href=\"/static/")?;
index_css.name.to_html(&mut out)?;
out.write_all(b"\"\n    />\n    <script src=\"/static/")?;
filter_form_js.name.to_html(&mut out)?;
out.write_all(b"\" defer></script>\n  </head>\n    <form role=\"search\" id=\"search\" action=\"/search\" method=\"get\">\n      <input autofocus autocapitalize=\"off\" autocorrect=\"off\" autocomplete=\"off\" tabindex=\"1\" type=\"search\" name=\"q\">\n      <button type=\"submit\">Search</button>\n\n      <details>\n        <summary>Filters</summary>\n        <ul>\n          <li>\n            <details>\n              <summary>Classes</summary>\n              <fieldset id=\"filter-set-classes\" class=\"any-of\">\n                <div class=\"fieldset-container\">\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-bard\" value=\"")?;
Class::Bard.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-bard\">Bard</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-cleric\" value=\"")?;
Class::Cleric.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-cleric\">Cleric</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-druid\" value=\"")?;
Class::Druid.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-druid\">Druid</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-paladin\" value=\"")?;
Class::Paladin.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-paladin\">Paladin</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-ranger\" value=\"")?;
Class::Ranger.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-ranger\">Ranger</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-sorcerer\" value=\"")?;
Class::Sorcerer.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-sorcerer\">Sorcerer</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-warlock\" value=\"")?;
Class::Warlock.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-warlock\">Warlock</label>\n                  <input type=\"checkbox\" checked name=\"filter-classes\" id=\"filter-classes-wizard\" value=\"")?;
Class::Wizard.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-classes-wizard\">Wizard</label>\n                </div>\n              </fieldset>\n            </details>\n          </li>\n          <li>\n            <details>\n              <summary>Schools</summary>\n              <fieldset id=\"filter-set-schools\" class=\"any-of\">\n                <div class=\"fieldset-container\">\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-abjuration\" value=\"")?;
School::Abjuration.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-abjuration\">Abjuration</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-conjuration\" value=\"")?;
School::Conjuration.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-conjuration\">Conjuration</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-divination\" value=\"")?;
School::Divination.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-divination\">Divination</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-enchantment\" value=\"")?;
School::Enchantment.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-enchantment\">Enchantment</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-evocation\" value=\"")?;
School::Evocation.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-evocation\">Evocation</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-illusion\" value=\"")?;
School::Illusion.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-illusion\">Illusion</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-necromancy\" value=\"")?;
School::Necromancy.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-necromancy\">Necromancy</label>\n                  <input type=\"checkbox\" checked name=\"filter-schools\" id=\"filter-schools-transmutation\" value=\"")?;
School::Transmutation.to_ron().unwrap().to_html(&mut out)?;
out.write_all(b"\"><label for=\"filter-schools-transmutation\">Transmutation</label>\n                </div>\n              </fieldset>\n            </details>\n          </li>\n          <li>\n            <details>\n              <summary>Components</summary>\n              <fieldset class=\"any-of\">\n                <div class=\"fieldset-container\">\n                  <input type=\"checkbox\" checked name=\"filter-components\" id=\"filter-components-v\" value=\"v\"><label for=\"filter-components-v\">V</label>\n                  <input type=\"checkbox\" checked name=\"filter-components\" id=\"filter-components-s\" value=\"s\"><label for=\"filter-components-s\">S</label>\n                  <input type=\"checkbox\" checked name=\"filter-components\" id=\"filter-components-m\" value=\"m\"><label for=\"filter-components-m\">M</label>\n                </div>\n              </fieldset>\n            </details>\n          </li>\n        </ul>\n      </details>\n    </form>\n\n    <button id=\"post-button\">Click Me, You Asshole, I dare you, I double dare you</button>\n  </body>\n</html>\n")?;
Ok(())
}
