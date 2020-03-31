use crate::spell::Spell;
use std::sync::Arc;
use std::{fs, path::Path};
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<im::HashMap<String, Spell>>>;

pub fn build(path: impl AsRef<Path>) -> Db {
    let data = fs::File::open(path.as_ref()).expect("spells.ron not found");
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
    use std::{fs, path::Path};
    use tantivy::{collector::TopDocs, query::QueryParser, schema::*, Index, IndexWriter};

    pub type Idx = Arc<Mutex<Indexer>>;

    pub struct SpellIndex {
        name: Field,
        body: Field,
        title: Field,
        tantivy_index: Index,
    }

    impl SpellIndex {
        pub fn new(index_dir: impl AsRef<Path>) -> tantivy::Result<Self> {
            let index_dir = index_dir.as_ref().join("tantivy3");
            if !index_dir.exists() {
                return Self::empty(&index_dir);
            }
            let tantivy_index = Index::open_in_dir(&index_dir)?;
            let schema = tantivy_index.schema();

            Ok(Self {
                tantivy_index,
                name: schema.get_field("name").expect("schema"),
                body: schema.get_field("body").expect("schema"),
                title: schema.get_field("title").expect("schema"),
            })
        }

        pub fn empty(index_dir: &Path) -> tantivy::Result<Self> {
            let _ = fs::create_dir_all(index_dir);

            let mut schema_builder = Schema::builder();
            let name = schema_builder.add_text_field("name", TEXT);
            let body = schema_builder.add_text_field("body", TEXT);
            let title = schema_builder.add_text_field("title", TEXT | STORED);
            let schema = schema_builder.build();
            let tantivy_index = Index::create_in_dir(index_dir, schema.clone())?;

            Ok(Self {
                tantivy_index,
                name,
                body,
                title,
            })
        }

        pub fn search(&self, query_text: &str) -> tantivy::Result<Vec<String>> {
            let query_text = query_text.trim();
            let mut query_parser =
                QueryParser::for_index(&self.tantivy_index, vec![self.name, self.body]);
            query_parser.set_conjunction_by_default();

            let query = query_parser.parse_query(query_text)?;
            let reader = self.tantivy_index.reader()?;
            let searcher = reader.searcher();
            let top_docs = searcher.search(&*query, &TopDocs::with_limit(50))?;

            top_docs
                .into_iter()
                .map(|(_relevance_score, doc_address)| {
                    let doc = searcher.doc(doc_address)?;
                    // let mut doc = self.tantivy_index.schema().to_named_doc(&retrieved_doc).0;
                    let title = doc.get_first(self.title).unwrap().text().unwrap();
                    Ok(title.to_string())
                })
                .collect::<tantivy::Result<Vec<_>>>()
        }
    }

    pub struct Indexer {
        pub index: SpellIndex,
        writer: IndexWriter,
    }

    impl Indexer {
        pub fn new(index: SpellIndex) -> tantivy::Result<Idx> {
            Ok(Arc::new(Mutex::new(Self {
                writer: index.tantivy_index.writer(50_000_000)?,
                index,
            })))
        }

        pub fn add(&mut self, spell: &Spell) {
            let mut doc = Document::default();
            doc.add_text(self.index.name, spell.name());
            doc.add_text(self.index.body, spell.text());
            doc.add_text(self.index.title, &spell.file_name());
            self.writer.add_document(doc);
        }

        pub fn commit(&mut self) -> tantivy::Result<()> {
            self.writer.commit()?;
            Ok(())
        }

        pub async fn search(&self, query_text: &str) -> tantivy::Result<Vec<String>> {
            self.index.search(query_text)
        }
    }

    /* lazy_static! {
        static ref STOP_WORDS: HashSet<&'static str> =
            ["a", "an", "the", "and", "or", "it", "it’s", "its", "can", "can’t",]
                .iter()
                .copied()
                .collect();
    } */
}
