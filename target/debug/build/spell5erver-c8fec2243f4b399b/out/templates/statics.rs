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

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static\\spells.ron"
#[allow(non_upper_case_globals)]
pub static spells_ron: StaticFile = StaticFile {
  content: include_bytes!("C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static\\spells.ron"),
  name: "spells-pCoLBVmt.ron",
mime: &mime::APPLICATION_OCTET_STREAM,
};

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static\\spells_bkp.ron"
#[allow(non_upper_case_globals)]
pub static spells_bkp_ron: StaticFile = StaticFile {
  content: include_bytes!("C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static\\spells_bkp.ron"),
  name: "spells_bkp-8gb2Phyo.ron",
mime: &mime::APPLICATION_OCTET_STREAM,
};

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static/js\\filter_form.js"
#[allow(non_upper_case_globals)]
pub static filter_form_js: StaticFile = StaticFile {
  content: include_bytes!("C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static/js\\filter_form.js"),
  name: "filter_form-EtvElu2p.js",
mime: &mime::TEXT_JAVASCRIPT,
};

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static/css/spell_card.css"
#[allow(non_upper_case_globals)]
pub static spell_card_css: StaticFile = StaticFile {
  content: b"*{font-family:\"Fira Sans\",sans-serif}.spell-card{width:450px;margin:auto}.spell-card .title{display:flex;align-items:baseline;justify-content:flex-start;flex-wrap:wrap;margin-bottom:10px}.spell-card .title .name{font-size:28pt;font-family:\"Roboto\",sans-serif;font-weight:900;white-space:nowrap;margin-right:10px}.spell-card .title .school-level{font-style:italic;color:#606060;white-space:nowrap;position:relative;bottom:0}.spell-card .inline-heading{font-size:11pt;font-weight:600}.spell-card .stats-container{display:grid;grid-template-columns:max-content auto;grid-template-rows:repeat(5, auto);column-gap:1em}.spell-card .stats-container .inline-heading{justify-self:end}.spell-card ul{list-style-type:none;margin:0;padding:0}.spell-card .footnote{font-size:9pt;color:#606060}\n",
  name: "spell_card-77Y5pM6u.css",
mime: &mime::TEXT_CSS,
};

/// From "C:\\Users\\timob\\Documents\\github.com\\isaacthefallenapple\\spell5erver\\static/css/index.css"
#[allow(non_upper_case_globals)]
pub static index_css: StaticFile = StaticFile {
  content: b"*{font-family:\"Fira sans\",sans-serif}details{-moz-user-select:none;-khtml-user-select:none;-webkit-user-select:none;-ms-user-select:none;user-select:none}fieldset{border:0}.fieldset-container{display:flex;flex-wrap:wrap}.any-of input{position:absolute;opacity:0;cursor:pointer;height:0;width:0}.any-of input+label{margin:0 5px;cursor:pointer;padding:3px 12px;border-radius:5px;background-color:lightgray;text-decoration:line-through}.any-of input:checked+label{background-color:lightskyblue;text-decoration:none}\n",
  name: "index-XiRkmWxT.css",
mime: &mime::TEXT_CSS,
};

pub static STATICS: &[&StaticFile] = &[&filter_form_js, &index_css, &spell_card_css, &spells_ron, &spells_bkp_ron];
