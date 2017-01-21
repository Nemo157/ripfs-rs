extern crate maddr;
extern crate libp2p;
extern crate futures;
extern crate tokio_core;

use std::str::FromStr;
use maddr::MultiAddr;
use libp2p::{ PeerInfo, Swarm };
use libp2p::identity::HostId;
use tokio_core::reactor::Core;
use futures::future;

const BOOTSTRAP_ADDRESSES: &'static [&'static str] = &[
    "/ip4/127.0.0.1/tcp/4001/ipfs/QmcD3Pzo3kwvuZYNcxwEbefhmhR8s2ftd7zMkAWBwMhjax",
    // "/ip4/104.131.131.82/tcp/4001/ipfs/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
    /*
    "/ip4/104.236.176.52/tcp/4001/ipfs/QmSoLnSGccFuZQJzRadHn95W2CrSFmZuTdDWP8HXaHca9z",
    "/ip4/104.236.179.241/tcp/4001/ipfs/QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM",
    "/ip4/104.236.76.40/tcp/4001/ipfs/QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64",
    "/ip4/178.62.61.185/tcp/4001/ipfs/QmSoLMeWqB7YGVLJN3pNLQpmmEk35v6wYtsMGLzSr5QBU3",
    "/ip4/104.236.176.52/tcp/4001/ipfs/QmSoLnSGccFuZQJzRadHn95W2CrSFmZuTdDWP8HXaHca9z",
    "/ip4/104.236.179.241/tcp/4001/ipfs/QmSoLpPVmHKQ4XTPdz8tjDFgdeRFkpV8JgYq8JVJ69RrZm",
    "/ip4/162.243.248.213/tcp/4001/ipfs/QmSoLueR4xBeUbY9WZ9xGUUxunbKWcrNFTDAadQJmocnWm",
    "/ip4/128.199.219.111/tcp/4001/ipfs/QmSoLSafTMBsPKadTEgaXctDQVcqN88CNLHXMkTNwMKPnu",
    "/ip4/104.236.76.40/tcp/4001/ipfs/QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64",
    "/ip4/178.62.158.247/tcp/4001/ipfs/QmSoLer265NRgSp2LA3dPaeykiS1J6DifTC88f5uVQKNAd",
    "/ip4/178.62.61.185/tcp/4001/ipfs/QmSoLMeWqB7YGVLJN3pNLQpmmEk35v6wYtsMGLzSr5QBU3",
    "/ip4/104.236.151.122/tcp/4001/ipfs/QmSoLju6m7xTh3DuokvT3886QRYqxAzb1kShaanJgW36yx",
    */
];

fn main() {
    let host_id = HostId::from_der(
        include_bytes!("private_key.der").as_ref().to_owned(),
        include_bytes!("public_key.der").as_ref().to_owned()).unwrap();

    let bootstrap_peers = BOOTSTRAP_ADDRESSES
        .into_iter()
        .map(|addr| MultiAddr::from_str(addr).unwrap())
        .map(|addr| PeerInfo::from_addr(addr).unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", bootstrap_peers);

    let mut core = Core::new().unwrap();

    let mut swarm = {
        let mut swarm = Swarm::new(host_id, true, core.handle());
        core.run(swarm.add_peers(bootstrap_peers)).unwrap();
        swarm
    };

    println!("{:?}", core.run(swarm.pre_connect_all()));
    core.run(future::empty::<(), ()>()).unwrap();
}
