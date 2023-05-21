mod create;
mod get;

use actix_web::web::{get, post, scope, ServiceConfig};

pub fn todo_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/items")
            .route("create/{title}", post().to(create::create))
            .route("", get().to(get::get)),
    );
}
