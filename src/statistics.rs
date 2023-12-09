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

#[derive(Clone)]
pub struct SystemStats {
    pub firewall: FirewallStats,
}

#[derive(Clone)]
pub struct FirewallStats {
    pub blocked: usize,
    pub whitelisted: Vec<String>,
    pub blacklisted: Vec<String>,
}

#[derive(Clone)]
pub struct DDoSStats {
    pub blocked: usize,
}

#[derive(Clone)]
pub struct DataUsage {
    pub upload: usize,
    pub download: usize,
}

#[derive(Clone)]
pub struct UserStats {
    pub total_users: usize,
    pub total_blocked: usize,
}

#[derive(Clone)]
pub struct ProxyDomainStats {
    pub total_connections: usize,
}
