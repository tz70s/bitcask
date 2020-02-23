//! Server implements the grpc based apis for bitcask.

use tonic::{transport::Server, Request, Response, Status};

use bitcaskapi::bitcasker_server::{Bitcasker, BitcaskerServer};
use bitcaskapi::{
    DelReply, DelRequest, GetReply, GetRequest, ListReply, ListRequest, SetReply, SetRequest,
};

use super::engine;
use super::logger;
use super::Config;
use slog::debug;

pub mod bitcaskapi {
    tonic::include_proto!("bitcaskapi");
}

pub struct BitcaskServer {
    logger: logger::Logger,
    engine: engine::Engine,
}

impl BitcaskServer {
    pub fn new(logger: logger::Logger) -> BitcaskServer {
        BitcaskServer {
            logger,
            engine: engine::Engine::new(),
        }
    }
}

pub async fn run(logger: logger::Logger, config: Config) -> Result<(), failure::Error> {
    let addr = format!("{}:{}", config.host, config.port).parse()?;
    let server = BitcaskServer::new(logger);

    Server::builder()
        .add_service(BitcaskerServer::new(server))
        .serve(addr)
        .await
        .map_err(|e| e.into())
}

#[tonic::async_trait]
impl Bitcasker for BitcaskServer {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "get", "request" => ?request);

        let key = request.into_inner().key;

        let reply = bitcaskapi::GetReply {
            entry: Some(bitcaskapi::Entry {
                key,
                val: "456".to_string(),
            }),
        };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "set", "request" => ?request);
        let entry = request.into_inner().entry;
        let reply = bitcaskapi::SetReply {};

        Ok(Response::new(reply))
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "list", "request" => ?request);
        let reply = bitcaskapi::ListReply { entry: vec![] };

        Ok(Response::new(reply))
    }

    async fn del(&self, request: Request<DelRequest>) -> Result<Response<DelReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "del", "request" => ?request);
        let key = request.into_inner().key;

        let reply = bitcaskapi::DelReply {};

        Ok(Response::new(reply))
    }
}
