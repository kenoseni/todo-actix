mod login;
mod logout;

use actix_web::web::{get, scope, ServiceConfig};

// ServiceConfig struct enables us to define things such as views on the server in different fields.
pub fn auth_views_factory(app: &mut ServiceConfig) {
    // service enables us to define a block of views that all get populated with the prefix defined in the scope.
    app.service(
        scope("v1/auth")
            .route("login", get().to(login::login))
            .route("logout", get().to(logout::logout)),
    );
}
