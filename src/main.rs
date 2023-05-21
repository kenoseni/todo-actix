mod playground;
mod processes;
mod state;
mod todo;
mod views;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
// use futures::executor::block_on;

// use playground::{
//     evaluate_dosomething_asynchronous, evaluate_dosomething_futures,
//     evaluate_dosomething_futures_with_async_task, evaluate_dosomething_synchronous,
// };

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_web::main] // a macro that marks our async main function as the Actix web system entry point
                   // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Executor
    HttpServer::new(|| {
        // closure returning an App struct
        let app = App::new().configure(views::views_factory);
        return app;
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
        // .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8711")?
    .workers(3)
    .run()
    .await
}

// fn main() {
//     // evaluate_dosomething_synchronous()
//     // evaluate_dosomething_asynchronous()
//     // let future_one = evaluate_dosomething_futures();

//     let result = block_on(evaluate_dosomething_futures_with_async_task());

//     println!("Here is the result: {}", result);
// }
