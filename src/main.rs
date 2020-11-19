use db::{
    index::{Idx, Indexer, SpellIndex},
    Db,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
pub use templates::{statics::StaticFile, RenderRucte};
use warp::{
    http::{Response, StatusCode},
    Filter, Rejection, Reply,
};

mod db;
mod spell;

#[tokio::main]
async fn main() {
    // spell::transform_spells("./static/spells.ron");
    let db = db::build("./static/spells.ron");
    let idx = Indexer::new(SpellIndex::new("./tantivy").expect("spellindex")).expect("indexer");
    let routes = filters::routes(db, idx);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::*;
    use warp::Filter;

    pub fn routes(
        db: Db,
        idx: Idx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        index()
            .or(spell(db.clone()))
            .or(search(idx.clone(), db.clone()))
            .or(post_json())
            .or(static_file())
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn with_idx(
        idx: Idx,
    ) -> impl Filter<Extract = (Idx,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || idx.clone())
    }

    pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end().and_then(handlers::index)
    }

    pub fn spell(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("spell")
            .and(warp::path::param())
            .and(with_db(db))
            .and_then(handlers::spell_page)
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct JsonReq {
        classes: Vec<String>,
        schools: Vec<String>,
        components: Vec<String>,
    }

    #[derive(Deserialize, Serialize)]
    struct JsonRes {
        msg: String,
    }

    pub fn search(
        idx: Idx,
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("search")
            .and(warp::get())
            .and(warp::query())
            .and(with_idx(idx))
            .and(with_db(db))
            .and_then(handlers::search)
    }

    pub fn post_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("postjson")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handlers::spell_filters)
    }

    pub fn static_file() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("static")
            .and(warp::path::param())
            .and_then(handlers::static_file)
    }
}

mod handlers {
    use super::*;

    pub async fn spell_page(name: String, db: Db) -> Result<impl Reply, Rejection> {
        if let Some(spell) = db.get(&name) {
            Response::builder().html(|o| templates::spell_card(o, spell))
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn static_file(name: String) -> Result<impl Reply, Rejection> {
        if let Some(data) = StaticFile::get(&name) {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", data.mime.as_ref())
                .body(data.content))
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn index() -> Result<impl Reply, Rejection> {
        Response::builder().html(|o| templates::index(o))
    }

    #[derive(Deserialize, Debug)]
    pub struct Query {
        q: String,
        filterclass: Vec<String>,
    }

    pub async fn search(query: Query, idx: Idx, db: Db) -> Result<impl Reply, Rejection> {
        let query = dbg!(query);
        if let Ok(results) = idx.index.search(&query.q) {
            let spells: Vec<_> = results.iter().map(|s| &db[s]).collect();
            Response::builder().html(|o| templates::search_results(o, &query.q, &spells))
        } else {
            Err(warp::reject())
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct SpellFiltersJson {
        schools: Vec<String>,
        classes: Vec<String>,
        components: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct QueryJson {
        query: String,
        filters: SpellFiltersJson,
    }

    pub struct SpellFilters {
        schools: spell::parts::ClassSet,
        classes: spell::parts::SchoolSet,
        components: Vec<String>,
    }

    pub async fn spell_filters(json: QueryJson) -> Result<impl Reply, Rejection> {
        use spell::{filters::*, parts::*};
        let classes: ClassFilter = match json
            .filters
            .classes
            .into_iter()
            .map(|s| Class::from_str(&s))
            .collect::<Result<ClassSet, _>>()
        {
            Ok(set) => ClassFilter(set),
            Err(_) => return Ok(StatusCode::BAD_REQUEST),
        };
        let schools: SchoolFilter = match json
            .filters
            .schools
            .into_iter()
            .map(|s| School::from_str(&s))
            .collect::<Result<SchoolSet, _>>()
        {
            Ok(set) => SchoolFilter(set),
            Err(_) => return Ok(StatusCode::BAD_REQUEST),
        };
        Ok(StatusCode::OK)
    }
}

#[derive(Deserialize, Debug)]
struct SearchFilter {
    school: spell::parts::School,
    class: spell::parts::Class,
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
