
use actix_web::{Error, get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use sqlite::{open};
use serde::{Deserialize};
use chrono;

//get user from db and sent a json 
#[get("/users/{user_id}/{friend}")]
async fn index(req: HttpRequest) -> Result<String, Error> {
    let name: String = req.match_info().get("friend").unwrap().to_string();
    let userid: i32 = req.match_info().get("user_id").unwrap().parse().unwrap();
    let connection = open(":memory").unwrap();
    
    let query  = format!(" SELECT {} FROM TASK", name);


    let qeury_result = connection.execute(query);
    let query =  match &qeury_result{
       Ok(format!(""[{:?}] : GET user {}, {}", chrono::offset::Local::now(),name, userid ")) => test,
        None  =  Err(actix_web::error::ErrorBadRequest("Name is missing")),
    } 
    
    println!("[{:?}] : GET user {}, {}", chrono::offset::Local::now(),name, userid );

    return Ok(format!("Welcome {}, user_id {}!", userid, name))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct Task {
    name: Option<String>,
}

//post a new user to the databases. 
#[post("/submit")]
async fn submit(info: web::Json<Task>) -> Result<String, Error> {

    let connection = open(":memory").unwrap();

    let qeury = "   
                                // CREATE TABLE task(name TEXT, timestap INTERGER);
                                INSERT INTO task VALUES ('sleep', 80);
                                INSERT INTO task VALUES ('eat' , 1);
                                ";   

    connection.execute(qeury).unwrap();

    return match &info.name {
        None => Err(actix_web::error::ErrorBadRequest("Name is missing")),
        Some(name) => Ok(format!("Welcome {}!", name)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(submit)
            .service(index)
         
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}