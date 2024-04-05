use crate::statistics::{LogEntry, ProxyDomainStats};
use chrono::Local;
use std::time::Instant;

pub mod global {
    use super::*;

    pub fn logger(
        domain: &str,
        ip: std::net::IpAddr,
        path: Option<String>,
        event:&'static str,
        domain_stats: &mut ProxyDomainStats,
        start_time: Instant,
    ) {
        domain_stats.last_active = Local::now().format("%d/%m/%Y, %H:%M:%S").to_string();
        let formatted_date = Local::now().format("%d/%m/%Y, %H:%M:%S").to_string();
        domain_stats.log.push(LogEntry {
            domain: domain.to_string(),
            ip,
            path,
            event,
            time: format!("{} {}", start_time.elapsed().as_millis(), formatted_date),
        });
    }    
}
