use index::Db;
pub use templates::{statics::StaticFile, RenderRucte};
use warp::http::{Response, StatusCode};
use warp::{Filter, Rejection, Reply};

mod index;
mod spell;

#[tokio::main]
async fn main() {
    let db = index::build_db();
    let routes = warp::get()
        .and(
            warp::path("spell")
                .and(warp::path::param())
                .and(with_db(db))
                .and_then(spell_page),
        )
        .or(warp::path("static")
            .and(warp::path::param())
            .and_then(static_file));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
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

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
