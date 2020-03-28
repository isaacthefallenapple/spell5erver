use crate::spell::Spell;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<im::HashMap<String, Spell>>>;

pub fn build_db() -> Db {
    let data = File::open("./static/spells.ron").expect("spells.ron not found");
    let spells: Vec<Spell> = ron::de::from_reader(data).expect("serialization failed");
    let mut map = im::HashMap::new();
    for spell in spells {
        let fname = spell.file_name().to_string();
        map.insert(fname, spell);
    }
    Arc::new(Mutex::new(map))
}
