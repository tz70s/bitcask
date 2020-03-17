//! Server implements the grpc based apis for bitcask.

use tonic::{transport::Server, Request, Response, Status};

use crate::proto::bitcaskapi::{
    self,
    bitcasker_server::{Bitcasker, BitcaskerServer},
    DelReply, DelRequest, GetReply, GetRequest, ListReply, ListRequest, SetReply, SetRequest,
};

use super::engine;
use super::logger;
use super::Config;
use futures::Stream;
use slog::debug;
use std::pin::Pin;

pub struct BitcaskServer {
    logger: logger::Logger,
    engine: engine::Engine,
}

impl BitcaskServer {
    pub async fn new(logger: logger::Logger) -> BitcaskServer {
        let engine = engine::Engine::new().await;
        BitcaskServer { logger, engine }
    }
}

pub async fn run(logger: logger::Logger, config: Config) -> Result<(), failure::Error> {
    let addr = format!("{}:{}", config.host, config.port).parse()?;
    let server = BitcaskServer::new(logger).await;

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

        let record = match self.engine.get(&key).await {
            Ok(record) => record,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let entry = record.map(|record| bitcaskapi::Entry {
            key: record.key.into_owned(),
            val: record.val,
        });

        debug!(self.logger.log, "Query entry result"; "key" => key, "entry" => ?entry);

        let reply = bitcaskapi::GetReply { entry };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "set", "request" => ?request);
        let entry = if let Some(entry) = request.into_inner().entry {
            entry
        } else {
            return Err(Status::invalid_argument("should pass entry value for set"));
        };

        self.engine
            .set(entry.key, entry.val)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let reply = bitcaskapi::SetReply {};

        Ok(Response::new(reply))
    }

    type ListStream =
        Pin<Box<dyn Stream<Item = Result<ListReply, Status>> + Send + Sync + 'static>>;

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "list", "request" => ?request);

        let records = self
            .engine
            .list()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let entries = records.into_iter().map(|r| {
            let e = bitcaskapi::Entry {
                key: r.key.into_owned(),
                val: r.val,
            };
            Ok(ListReply { entry: Some(e) })
        });

        let reply = futures::stream::iter(entries);

        Ok(Response::new(Box::pin(reply)))
    }

    async fn del(&self, request: Request<DelRequest>) -> Result<Response<DelReply>, Status> {
        debug!(self.logger.log, "Got incoming request"; "method" => "del", "request" => ?request);
        let key = request.into_inner().key;

        self.engine
            .rm(&key)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let reply = bitcaskapi::DelReply {};

        Ok(Response::new(reply))
    }
}
