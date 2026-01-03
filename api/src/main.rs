use poem::{
    Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}
};

use crate::{request_inputs::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use store::Store;
pub mod request_inputs;
pub mod request_output;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_website(Json( data) : Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store{};
    s.create_website();
    let url = data.url;
    let response = CreateWebsiteOutput{
        id : url
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
            .at("/website/:name", get(get_website))
            .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}