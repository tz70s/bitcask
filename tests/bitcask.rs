//! Integration tests for bitcask operations.

use bitcask::logger::Logger;
use bitcask::proto::bitcaskapi::bitcasker_client::BitcaskerClient;
use bitcask::proto::bitcaskapi::{DelRequest, Entry, GetReply, GetRequest, SetRequest};
use bitcask::Config;

// Remember to allow incoming connection before running tests on Mac.
// Temporarily disable firewall.

#[tokio::test]
async fn test_bitcask_get_set_apis() {
    setup_server().await;

    let mut client = BitcaskerClient::connect("http://127.0.0.1:5232")
        .await
        .unwrap();

    let req = tonic::Request::new(SetRequest {
        entry: Some(Entry {
            key: "123".to_string(),
            val: "223".to_string(),
        }),
    });

    client.set(req).await.unwrap();

    let req = tonic::Request::new(GetRequest {
        key: "123".to_string(),
    });

    let resp = client.get(req).await.unwrap();

    assert_eq!(
        resp.into_inner(),
        GetReply {
            entry: Some(Entry {
                key: "123".to_string(),
                val: "223".to_string(),
            })
        }
    )
}

#[tokio::test]
async fn test_bitcask_del_apis() {
    setup_server().await;

    let mut client = BitcaskerClient::connect("http://127.0.0.1:5232")
        .await
        .unwrap();

    let req = tonic::Request::new(SetRequest {
        entry: Some(Entry {
            key: "123".to_string(),
            val: "223".to_string(),
        }),
    });

    client.set(req).await.unwrap();

    let req = tonic::Request::new(DelRequest {
        key: "123".to_string(),
    });

    client.del(req).await.unwrap();

    let req = tonic::Request::new(GetRequest {
        key: "123".to_string(),
    });

    let resp = client.get(req).await.unwrap();

    assert_eq!(resp.into_inner(), GetReply { entry: None });
}

#[tokio::test]
async fn test_bitcask_multi_set() {
    setup_server().await;

    let mut client = BitcaskerClient::connect("http://127.0.0.1:5232")
        .await
        .unwrap();

    let entry0 = Entry {
        key: "123".to_string(),
        val: "223".to_string(),
    };

    let req = tonic::Request::new(SetRequest {
        entry: Some(entry0.clone()),
    });

    client.set(req).await.unwrap();

    let entry1 = Entry {
        key: "223".to_string(),
        val: "323".to_string(),
    };

    let req = tonic::Request::new(SetRequest {
        entry: Some(entry1.clone()),
    });

    client.set(req).await.unwrap();

    let req = tonic::Request::new(GetRequest {
        key: "123".to_string(),
    });

    let resp = client.get(req).await.unwrap();

    assert_eq!(
        resp.into_inner(),
        GetReply {
            entry: Some(entry0)
        }
    );

    let req = tonic::Request::new(GetRequest {
        key: "223".to_string(),
    });

    let resp = client.get(req).await.unwrap();

    assert_eq!(
        resp.into_inner(),
        GetReply {
            entry: Some(entry1)
        }
    );
}

async fn setup_server() {
    let logger = Logger::new();
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 5232,
    };
    // Spawn server in background.
    tokio::spawn(bitcask::server::run(logger, config));

    tokio::time::delay_for(std::time::Duration::from_secs(3)).await;
}
