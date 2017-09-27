#![feature(proc_macro)]

extern crate bytes;
extern crate futures;
extern crate libp2p;
extern crate maddr;
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate tokio_core;
extern crate tokio_io;

mod identity;

use std::mem;
use std::str::FromStr;
use std::time::Duration;

use maddr::{MultiAddr, Segment};
use libp2p::{ PeerInfo, Swarm };
use libp2p::identity::HostId;
use tokio_core::reactor::{Core, Timeout};
use futures::{Future, Sink, Stream};
use futures::future::Either;
use tokio_io::codec::Framed;
use slog::{b, log, kv, record, record_static};
use slog::{info, o, Drain, Logger};
use slog_term::TermDecorator;
use slog_term::CompactFormat;
use slog_async::Async;



use identity::Identity;

fn main() {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let (drain, guard) = Async::new(drain).build_with_guard();

    let logger = Logger::root(drain.fuse(), o!());

    let host = HostId::from_der(
        include_bytes!("private_key.der").as_ref().to_owned(),
        include_bytes!("public_key.der").as_ref().to_owned()).unwrap();

    info!(logger, "host: {:#?}", host);

    let arg = std::env::args().skip(1).next();
    info!(logger, "arg: {:#?}", arg);

    let mut core = Core::new().unwrap();

    fn ping(logger: Logger, core: &mut Core, mut swarm: Swarm, peer: PeerInfo) {
        info!(logger, "going to ping {:#?}", peer);
        swarm.add_peers(vec![peer.clone()]);
        info!(logger, "added peer");
        let id = peer.id().clone();
        let parts = core.run(swarm.open_stream(id, "/ipfs/ping/1.0.0")).unwrap();
        info!(logger, "opened stream");
        let stream = Framed::from_parts(parts, Identity);
        let stream = core.run(stream.send(b"1234567890ABCDEF1234567890ABCDEF"[..].into())).unwrap();
        info!(logger, "sent ping");
        let (result, _) = core.run(stream.into_future()).unwrap();
        let result = result.unwrap();
        info!(logger, "ping result {:#?}", String::from_utf8(result.to_vec()).unwrap());
    }

    if arg == Some("listen".to_owned()) {
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4002").unwrap();
        info!(logger, "listening at: {}", addr.clone() + Segment::Ipfs(host.hash().clone()));
        let swarm = Swarm::new(logger.clone(), host, "ripfs/0.0.0".to_owned(), vec![addr], core.handle()).unwrap();
        info!(logger, "result: {:#?}", core.run(swarm));
    } else if arg == Some("self".to_owned()) {
        let swarm = Swarm::new(logger.clone(), host, "ripfs/0.0.0".to_owned(), vec![], core.handle()).unwrap();
        {
            let logger = logger.clone();
            core.handle().spawn(swarm.clone().map_err(move |err| info!(logger, "Swarm error {:#?}", err)));
        }
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4002/ipfs/QmdxDB3FVu9JoUtSgaVTaCQpswH7ghfRsM3wGYWFSqW7Gb").unwrap();
        let peer = PeerInfo::from_addr(addr).unwrap();
        ping(logger, &mut core, swarm, peer);
    } else {
        let swarm = Swarm::new(logger.clone(), host, "ripfs/0.0.0".to_owned(), vec![], core.handle()).unwrap();
        {
            let logger = logger.clone();
            core.handle().spawn(swarm.clone().map_err(move |err| info!(logger, "Swarm error {:#?}", err)));
        }
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4001/ipfs/QmcD3Pzo3kwvuZYNcxwEbefhmhR8s2ftd7zMkAWBwMhjax").unwrap();
        let peer = PeerInfo::from_addr(addr).unwrap();
        ping(logger.clone(), &mut core, swarm.clone(), peer);
        info!(logger, "Running swarm for 5 seconds");
        let timeout = Timeout::new(Duration::from_secs(5), &core.handle()).unwrap();
        let result = core.run(swarm.select2(timeout)).map_err(|_| ()).unwrap();
        match result {
            Either::A((swarm, _)) => info!(logger, "Swarm ended: {:#?}", swarm),
            Either::B((_, swarm)) => info!(logger, "Swarm still running: {:#?}", swarm),
        }
    }

    mem::drop(guard);
}
