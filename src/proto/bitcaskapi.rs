#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReply {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRequest {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReply {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelRequest {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    #[prost(string, tag = "2")]
    pub val: std::string::String,
}
/// Status code from reply.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Http 200
    Ok = 0,
    /// Http 404
    NotFound = 1,
    /// Http 500
    InternalServerError = 2,
    /// Http 503
    ServiceUnavailable = 3,
}
#[doc = r" Generated client implementations."]
pub mod bitcasker_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct BitcaskerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BitcaskerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BitcaskerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bitcaskapi.Bitcasker/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bitcaskapi.Bitcasker/Set");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ListReply>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bitcaskapi.Bitcasker/List");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn del(
            &mut self,
            request: impl tonic::IntoRequest<super::DelRequest>,
        ) -> Result<tonic::Response<super::DelReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bitcaskapi.Bitcasker/Del");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BitcaskerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bitcasker_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BitcaskerServer."]
    #[async_trait]
    pub trait Bitcasker: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetReply>, tonic::Status>;
        async fn set(
            &self,
            request: tonic::Request<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetReply>, tonic::Status>;
        #[doc = "Server streaming response type for the List method."]
        type ListStream: Stream<Item = Result<super::ListReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn list(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<Self::ListStream>, tonic::Status>;
        async fn del(
            &self,
            request: tonic::Request<super::DelRequest>,
        ) -> Result<tonic::Response<super::DelReply>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct BitcaskerServer<T: Bitcasker> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Bitcasker> BitcaskerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T: Bitcasker> Service<http::Request<HyperBody>> for BitcaskerServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/bitcaskapi.Bitcasker/Get" => {
                    struct GetSvc<T: Bitcasker>(pub Arc<T>);
                    impl<T: Bitcasker> tonic::server::UnaryService<super::GetRequest> for GetSvc<T> {
                        type Response = super::GetReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bitcaskapi.Bitcasker/Set" => {
                    struct SetSvc<T: Bitcasker>(pub Arc<T>);
                    impl<T: Bitcasker> tonic::server::UnaryService<super::SetRequest> for SetSvc<T> {
                        type Response = super::SetReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bitcaskapi.Bitcasker/List" => {
                    struct ListSvc<T: Bitcasker>(pub Arc<T>);
                    impl<T: Bitcasker> tonic::server::ServerStreamingService<super::ListRequest> for ListSvc<T> {
                        type Response = super::ListReply;
                        type ResponseStream = T::ListStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bitcaskapi.Bitcasker/Del" => {
                    struct DelSvc<T: Bitcasker>(pub Arc<T>);
                    impl<T: Bitcasker> tonic::server::UnaryService<super::DelRequest> for DelSvc<T> {
                        type Response = super::DelReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.del(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Bitcasker> Clone for BitcaskerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Bitcasker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Bitcasker> tonic::transport::NamedService for BitcaskerServer<T> {
        const NAME: &'static str = "bitcaskapi.Bitcasker";
    }
}
