pub struct KeyExchangeManager {
    rep_id: u32,
    cluster_size: u32,
    quorum_size: u32,
    public_keys: Vec<PublicKey>,
    private_keys: PlainSecretsManager,
    clients_public_keys: ClientPublicKeys,
    client: InternalBFTClient,
    multi_sig_key_hdlr: MultiSigKeyHandler,
    client_public_key_store: ClientPublicKeyStore,
    timers: Timers,
    secrets_mgr: SecretsManager,
    initial_exchange: bool,
    metrics: Option<Metrics>,
    metrics_timer: Option<Timers::Handle>,
}

impl KeyExchangeManager {
    pub fn new(id: InitData) -> Self {
        let rep_id = ReplicaConfig::instance().getreplicaId();
        let cluster_size = ReplicaConfig::instance().getnumReplicas();
        let quorum_size = (2 * ReplicaConfig::instance().fVal + ReplicaConfig::instance().cVal) as u32;
        let public_keys = vec![];
        let private_keys = PlainSecretsManager::new();
        let clients_public_keys = ClientPublicKeys::new();
        let client = InternalBFTClient::new();
        let multi_sig_key_hdlr = MultiSigKeyHandler::new();
        let client_public_key_store = ClientPublicKeyStore::new();
        let timers = Timers::new();
        let secrets_mgr = SecretsManager::new();
        let initial_exchange = ReplicaConfig::instance().getkeyExchangeOnStart();
        let metrics = None;
        let metrics_timer = None;
        KeyExchangeManager {
            rep_id,
            cluster_size,
            quorum_size,
            public_keys,
            private_keys,
            clients_public_keys,
            client,
            multi_sig_key_hdlr,
            client_public_key_store,
            timers,
            secrets_mgr,
            initial_exchange,
            metrics,
            metrics_timer,
        }
    }

    pub fn init_metrics(&mut self, a: std::sync::Arc<concordMetrics::Aggregator>, interval: std::time::Duration) {
        self.metrics = Some(Metrics::new(a, interval));
        self.metrics_timer = Some(self.timers.add(
            std::time::Duration::from_millis(100),
            concordUtil::Timers::Timer::RECURRING,
            move |h| {
                self.metrics.as_ref().unwrap().component.update_aggregator();
                let curr_time = std::time::Duration::from_secs(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
                if curr_time - self.metrics.as_ref().unwrap().last_metrics_dump_time >= self.metrics.as_ref().unwrap().metrics_dump_interval_in_sec {
                    self.metrics.as_ref().unwrap().last_metrics_dump_time = curr_time;
                    info!(KEY_EX_LOG, "-- KeyExchangeManager metrics