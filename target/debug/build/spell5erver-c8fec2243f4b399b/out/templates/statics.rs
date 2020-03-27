extern crate mime;
use self::mime::Mime;

/// A static file has a name (so its url can be recognized) and the
/// actual file contents.
///
/// The name includes a short (48 bits as 8 base64 characters) hash of
/// the content, to enable long-time caching of static resourses in
/// the clients.
#[allow(dead_code)]
pub struct StaticFile {
    pub content: &'static [u8],
    pub name: &'static str,
    pub mime: &'static Mime,
}
#[allow(dead_code)]
impl StaticFile {
    /// Get a single `StaticFile` by name, if it exists.
    pub fn get(name: &str) -> Option<&'static Self> {
        if let Ok(pos) = STATICS.binary_search_by_key(&name, |s| s.name) {
            Some(STATICS[pos])
        } else {None}
    }
}

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static/css/spell_card.css"
#[allow(non_upper_case_globals)]
pub static spell_card_css: StaticFile = StaticFile {
  content: b"*{font-family:\'Fira Sans\',sans-serif}.spell-card{width:450px;margin:auto}.spell-card .title{display:block;margin-bottom:10px}.spell-card .title h3{font-size:28pt;font-family:\'Roboto\',sans-serif;padding:0;display:inline}.spell-card .title p{padding-top:0;font-style:italic;color:#606060;margin-left:5px;display:inline}.spell-card ul{list-style-type:none;margin:0;padding:0}.spell-card ul li{font-size:11pt;font-weight:600}.spell-card ul li span{font-size:medium;font-weight:400}.spell-card .footnote{font-size:9pt;color:#606060}\n",
  name: "spell_card-8uncm6Pe.css",
mime: &mime::TEXT_CSS,
};

pub static STATICS: &[&StaticFile] = &[&spell_card_css];
