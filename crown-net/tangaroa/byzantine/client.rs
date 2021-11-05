
use criterion::Criterion;
use violetabft::evioletabftpb::ConfState;
use violetabft::{storage::MemStorage, Config, VioletaBFT};

// TODO:
// - add a timer for the heartbeat timeout
// - add a timer for the election timeout
// - add a timer for the client timeout
// - add a timer for the election timeout
// - add a timer for the client timeout

fn run_raft_client<'a>(get_entry: Box<Fn(u64) -> Vec<u8> + 'a>, use_result: Box<Fn(Vec<u8>) + 'a>, rconf: RaftConfig, spec: RaftSpec<'a>) {
    let qsize = get_quorum_size(rconf.other_nodes.len());
    let (ein, eout) = channel();

fn run_raft_client<F, G>(
    raft_client: F,
    raft_env: RaftEnv,
    initial_raft_state: RaftState,
    spec: RaftSpec,
) -> RaftState
where
    F: Fn(RaftClient) -> G,
    G: Future<Output = Result<RaftClientResponse, RaftClientError>>,
{
    let mut raft_client = raft_client(RaftClient::new(
        raft_env.clone(),
        initial_raft_state.clone(),
        spec.clone(),
    ));

    loop {
        let result = raft_client.poll();
        match result {
            Ok(Async::Ready(RaftClientResponse::ClientResponse(client_response))) => {
                return client_response.into();
            }
            Ok(Async::Ready(RaftClientResponse::ClientRequest(client_request))) => {
                let client_request = client_request.into();