#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
mod repository;
mod models;
mod routes;

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;
// use lazy_static::lazy_static;
use log::{error, info, warn};
use log4rs;
use axum;
use std::net::SocketAddr;
use rbatis::rbatis::Rbatis;
use models::*;
use rbatis::crud::CRUD;
use crate::repository::DbRepository;



//mysql driver url
const MYSQL_URL: &'static str = "mysql://root:test@123456.@192.168.8.234:3306/demo";

lazy_static! {
   pub static ref REPO:DbRepository = DbRepository::new();
}



#[tokio::main]
async fn main() {
    REPO.link(MYSQL_URL).await;
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
  
    let routes = self::routes::router();
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
        
}
