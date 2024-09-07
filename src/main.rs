use futures::StreamExt;
use litep2p::{
    config::ConfigBuilder,
    protocol::{libp2p::ping, request_response::ConfigBuilder as ReqRespConfigBuilder},
    transport::{quic::config::Config as QuicConfig, tcp::config::Config as TcpConfig},
    Litep2p, ProtocolName,
};

#[tokio::main]
async fn main() {
    // Enable IPFS PING protocol
    let (ping_config, mut ping_event_stream) = ping::Config::default();

    // Enable `/request/1` request-response protocol
    let (req_resp_config, mut req_resp_handle) =
        ReqRespConfigBuilder::new(ProtocolName::from("/request/1"))
            .with_max_size(1024)
            .build();

    // Build `Litep2pConfig` object with TCP and QUIC transports
    let config = ConfigBuilder::new()
        .with_tcp(TcpConfig {
            listen_addresses: vec![
                "/ip6/::1/tcp/0".parse().expect("valid address"),
                "/ip4/127.0.0.1/tcp/0".parse().expect("valid address"),
            ],
            ..Default::default()
        })
        .with_quic(QuicConfig {
            listen_addresses: vec![
                "/ip4/127.0.0.1/udp/0/quic-v1".parse().expect("valid address")
            ],
        })
        .with_libp2p_ping(ping_config)
        .with_request_response_protocol(req_resp_config)
        .build();

    // Create the `Litep2p` object
    let mut litep2p = Litep2p::new(config).await.unwrap();

    // Event loop for the P2P node
    loop {
        tokio::select! {
            event = litep2p.next_event() => {
                if let Some(event) = event {
                    println!("P2P Event: {:?}", event);
                }
            },
            req_event = req_resp_handle.next() => {
                if let Some(req_event) = req_event {
                    println!("Request-Response Event: {:?}", req_event);
                }
            },
            ping_event = ping_event_stream.next() => {
                if let Some(ping_event) = ping_event {
                    println!("Ping Event: {:?}", ping_event);
                }
            },
        }
    }
}
