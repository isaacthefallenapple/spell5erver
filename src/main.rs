use db::{
    index::{Indexer, SpellIndex},
    Db,
};
use serde::Deserialize;
use std::sync::Arc;
pub use templates::{statics::StaticFile, RenderRucte};
use tokio::sync::Mutex;
use warp::{
    http::{Response, StatusCode},
    Filter, Rejection, Reply,
};

mod db;
mod spell;

pub type Idx = Arc<Mutex<Indexer>>;

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
            .or(static_file())
    }

    pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end().and(warp::fs::file("./static/html/index.html"))
    }

    pub fn spell(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("spell")
            .and(warp::path::param())
            .and(with_db(db))
            .and_then(spell_page)
    }

    pub fn search(
        idx: Idx,
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("search")
            .and(warp::query())
            .and(with_idx(idx))
            .and(with_db(db))
            .and_then(super::search)
    }

    pub fn static_file() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("static")
            .and(warp::path::param())
            .and_then(super::static_file)
    }
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_idx(idx: Idx) -> impl Filter<Extract = (Idx,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || idx.clone())
}

async fn spell_page(name: String, db: Db) -> Result<impl Reply, Rejection> {
    let map = db.lock().await;
    if let Some(spell) = map.get(&name) {
        Response::builder().html(|o| templates::spell_card(o, spell))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn static_file(name: String) -> Result<impl Reply, Rejection> {
    if let Some(data) = StaticFile::get(&name) {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", data.mime.as_ref())
            .body(data.content))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn search(query: Query, idx: Idx, db: Db) -> Result<impl Reply, Rejection> {
    let idx = idx.lock().await;
    if let Ok(results) = idx.index.search(&query.q) {
        let db = db.lock().await;
        let spells: Vec<_> = results.iter().map(|s| &db[s]).collect();
        Response::builder().html(|o| templates::search_results(o, &query.q, &spells))
    } else {
        Err(warp::reject())
    }
}

#[derive(Deserialize)]
struct Query {
    q: String,
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
