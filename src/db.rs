use crate::spell::Spell;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<im::HashMap<String, Spell>>>;

pub fn build() -> Db {
    let data = File::open("./static/spells.ron").expect("spells.ron not found");
    let spells: Vec<Spell> = ron::de::from_reader(data).expect("serialization failed");
    let mut map = im::HashMap::new();
    for spell in spells {
        let fname = spell.file_name().to_string();
        map.insert(fname, spell);
    }
    Arc::new(Mutex::new(map))
}

pub mod index {
    use super::*;
    use lazy_static::lazy_static;
    use std::collections::{HashMap, HashSet};
    use unicode_segmentation::UnicodeSegmentation;

    pub type Index = HashMap<String, Vec<(String, f64)>>;

    lazy_static! {
        static ref STOP_WORDS: HashSet<&'static str> =
            ["a", "an", "the", "and", "or", "it", "it’s", "its", "can", "can’t",]
                .iter()
                .copied()
                .collect();
    }

    pub async fn build(db: Db) -> Index {
        let mut dict = HashMap::new();
        let data = db.lock().await;
        for spell in data.values() {
            let mut spell_dict: HashMap<String, Vec<u32>> = HashMap::new();
            let i: u32 = 1;
            let text = spell.text().to_lowercase();

            for (i, word) in (1..)
                .zip(text.unicode_words())
                .filter(|(_, w)| !STOP_WORDS.contains(w))
            {
                spell_dict
                    .entry(word.to_string())
                    .or_insert_with(Vec::new)
                    .push(i);
            }

            for (word, positions) in spell_dict {
                let relevance = positions.iter().fold(0.0f64, |b: f64, &pos: &u32| {
                    b + f64::from(i) / f64::from(pos)
                });
                let entry = dict.entry(word).or_insert_with(Vec::new);
                entry.push((spell.file_name(), relevance));
                entry.sort_by(|(_, a), (_, b)| b.partial_cmp(&a).unwrap());
            }
        }
        dict
    }
}
