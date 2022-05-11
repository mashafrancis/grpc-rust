use tonic::{transport::Server, Response, Request, Status};

use bookstore::bookstore_server::{Bookstore, BookstoreServer};
use bookstore::{GetBookRequest, GetBookResponse};

mod bookstore {
    include!("bookstore.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");
}

#[derive(Default)]
pub struct BookStoreImpl {}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    async fn get_book(&self, request: Request<GetBookRequest>) -> Result<Response<GetBookResponse>, Status> {
        println!("get_book: {:?}", request.remote_addr());

        let response = GetBookResponse {
            id: request.into_inner().id,
            name: "Blah Blah".to_owned(),
            author: "Masha".to_owned(),
            year: 2020,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let bookstore = BookStoreImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(bookstore::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Bookstore server is listening on {}", addr);

    Server::builder()
        .add_service(BookstoreServer::new(bookstore))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
