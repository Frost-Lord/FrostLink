use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ProxyStatistics {
    pub system: SystemStats,
    pub ddos_attacks: DDoSStats,
    pub data_usage: DataUsage,
    pub users: UserStats,
    pub proxies: Arc<Mutex<HashMap<String, ProxyDomainStats>>>,
}

impl Default for ProxyStatistics {
    fn default() -> Self {
        ProxyStatistics {
            system: SystemStats::default(),
            ddos_attacks: DDoSStats::default(),
            data_usage: DataUsage::default(),
            users: UserStats::default(),
            proxies: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Clone, Default)]
pub struct SystemStats {
    pub firewall: FirewallStats,
}

#[derive(Clone, Default)]
pub struct FirewallStats {
    pub blocked: usize,
    pub whitelisted: Vec<String>,
    pub blacklisted: Vec<String>,
}

#[derive(Clone, Default)]
pub struct DDoSStats {
    pub blocked: usize,
}

#[derive(Clone, Default)]
pub struct DataUsage {
    pub upload: usize,
    pub download: usize,
}

#[derive(Clone, Default)]
pub struct UserStats {
    pub total_users: usize,
    pub total_blocked: usize,
}

#[derive(Clone, Default)]
pub struct ProxyDomainStats {
    pub total_connections: usize,
}
