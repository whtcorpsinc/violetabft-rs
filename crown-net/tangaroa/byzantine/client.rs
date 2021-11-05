use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::JoinHandle;
use std::thread::spawn;
use std::thread::sleep;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use criterion::Criterion;
use violetabft::evioletabftpb::ConfState;
use violetabft::{storage::MemStorage, Config, VioletaBFT};

// TODO:
// - add a timer for the heartbeat timeout
// - add a timer for the election timeout
// - add a timer for the client timeout
// - add a timer for the election timeout
// - add a timer for the client timeout
pub fn raft_client(raft: &mut Raft, use_result: fn(&Raft, &CommandResponse)) {
    let nodes = raft.cfg.other_nodes.clone();
    if nodes.is_empty() {
        error!("The client has no nodes to send requests to.");
    }
    raft.current_leader = Some(nodes.iter().next().unwrap().clone());
    raft.pending_requests = HashMap::new();
    raft.num_timeouts = 0;
    raft.client_handle_events(use_result);
}

fn raft_client_handle_events(raft: &mut Raft, use_result: fn(&Raft, &CommandResponse)) {
    loop {
        match raft.dequeue_event() {
            Some(ERPC(CMD(cmd))) => {
                raft.client_send_command(cmd);
            }
            Some(ERPC(CMDR(cmdr))) => {
                raft.client_handle_command_response(use_result, cmdr);
            }
            Some(HeartbeatTimeout(node_id)) => {
                let mut timeouts = raft.num_timeouts;
                let limit = raft.cfg.client_timeout_limit;
                if timeouts < limit {
                    debug!("choosing a new leader and resending commands");
                    raft.set_leader_to_next();
                    let reqs = raft.pending_requests.clone();
                    raft.pending_requests = HashMap::new(); // this will reset the timer on resend
                    for req in reqs.values() {
                        raft.client_send_command(req);
                    }
                    raft.num_timeouts += 1;
                } else {
                    debug!("starting a revolution");
                    let nid = raft.cfg.node_id.clone();
                    let mlid = raft.current_leader.clone();
                    match mlid {
                        Some(lid) => {
                            let rid = raft.next_request_id();
                            raft.cfg.other_nodes.iter().for_each(|n| {
                                raft.send_signed_rpc(n, REVOLUTION(Revolution {
                                    node_id: nid.clone(),
                                    leader_id: lid.clone(),
                                    request_id: rid.clone(),
                                    data: vec![],
                                }));
                            });
                            raft.num_timeouts = 0;
                            raft.reset_heartbeat_timer();
                        }
                        _ => {
                            raft.set_leader_to_first();
                            raft.reset_heartbeat_timer();
                        }
                    }
                }
            }
            _ => {
                return;
            }
        }
    }
}

fn raft_client_send_command(raft: &mut Raft, cmd: Command) {
    let nid = raft.cfg.node_id.clone();
    let rid = raft.next_request_id();
    raft.cfg.other_nodes.iter().for_each(|n| {
        raft.send_signed_rpc(n, REQUEST(Request {
            node_id: nid.clone(),
            request_id: rid.clone(),
            data: cmd.data,
        }));
    });
}
