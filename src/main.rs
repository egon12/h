use ntex::web::ErrorRenderer;
use ntex::{self, web};
use ntex_files;
use ntex_files::Files;

#[web::get("/ping")]
async fn ping() -> &'static str {
    "pong"
}

fn create_file_server<Err: ErrorRenderer>() -> Files<Err> {
    Files::new("/", ".").show_files_listing()
}

#[ntex::main]
async fn main() -> std::io::Result<()>{
    let addr = "[::]:8080";
    //let port = 8080;

    println!("Starting server at {}", addr);

    web::HttpServer::new(move || {
        web::App::new()
                .service(ping)
                .service(create_file_server())
    }).bind(addr)?
    .run().await
}
