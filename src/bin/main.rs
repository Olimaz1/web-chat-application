use rocket::{fs::{NamedFile, FileServer, relative}};

#[macro_use] extern crate rocket;

/*#[get("/")]
async fn index() -> Option<NamedFile> {
    // NamedFile::open("static/index.html").await.ok()
}*/

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        //.mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}