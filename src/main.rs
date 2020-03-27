use templates::{statics::StaticFile, RenderRucte};
use warp::http::{Response, StatusCode};
use warp::{Filter, Rejection, Reply};

mod spell;

#[tokio::main]
async fn main() {
    let routes = warp::get().and(
        warp::path::end()
            .and_then(spell_page)
            .or(warp::path("static")
                .and(warp::path::param())
                .and_then(static_file)),
    );
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn spell_page() -> Result<impl Reply, Rejection> {
    let dummy: &spell::Spell = &*spell::DUMMY;
    Response::builder().html(|o| templates::spell_card(o, dummy))
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
