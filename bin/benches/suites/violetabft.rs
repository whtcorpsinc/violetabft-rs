// Copyright 2021 EinsteinDB Project Authors. Licensed under Apache-2.0.

use crate::DEFAULT_RAFT_SETS;
use criterion::Criterion;
use violetabft::evioletabftpb::ConfState;
use violetabft::{storage::MemStorage, Config, VioletaBFT};

pub fn bench_violetabft(c: &mut Criterion) {
    bench_violetabft_new(c);
    bench_violetabft_campaign(c);
}

fn new_storage(voters: usize, learners: usize) -> MemStorage {
    let mut cc = ConfState::default();
    for i in 1..=voters {
        cc.voters.push(i as u64);
    }
    for i in 1..=learners {
        cc.learners.push(voters as u64 + i as u64);
    }
    MemStorage::new_with_conf_state(cc)
}

fn quick_violetabft(storage: MemStorage, logger: &slog::Logger) -> VioletaBFT<MemStorage> {
    let id = 1;
    let config = Config::new(id);
    VioletaBFT::new(&config, storage, logger).unwrap()
}

pub fn bench_violetabft_new(c: &mut Criterion) {
    DEFAULT_RAFT_SETS.iter().for_each(|(voters, learners)| {
        c.bench_function(&format!("VioletaBFT::new ({}, {})", voters, learners), move |b| {
            let logger = violetabft::default_logger();
            let storage = new_storage(*voters, *learners);
            b.iter(|| quick_violetabft(storage.clone(), &logger))
        });
    });
}

pub fn bench_violetabft_campaign(c: &mut Criterion) {
    DEFAULT_RAFT_SETS
        .iter()
        .skip(1)
        .for_each(|(voters, learners)| {
            // We don't want to make `violetabft::violetabft` public at this point.
            let msgs = &[
                "CampaignPreElection",
                "CampaignElection",
                "CampaignTransfer",
            ];
            // Skip the first since it's 0,0
            for msg in msgs {
                c.bench_function(
                    &format!("VioletaBFT::campaign ({}, {}, {})", voters, learners, msg),
                    move |b| {
                        let logger = violetabft::default_logger();
                        let storage = new_storage(*voters, *learners);
                        b.iter(|| {
                            let mut violetabft = quick_violetabft(storage.clone(), &logger);
                            violetabft.campaign(msg.as_bytes());
                        })
                    },
                );
            }
        });
}
