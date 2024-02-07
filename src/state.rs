use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::DerefMut;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct State {
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}

fn get_state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(State::default()))
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChangeStatus<T> {
    Unchanged(T),
    Changed { old: T, new: T },
    Unknown(T),
}

impl<T> ChangeStatus<T> where T: Eq + Clone {
    pub fn into_inner(self) -> T {
        match self {
            ChangeStatus::Unchanged(ip) => ip,
            ChangeStatus::Changed { new, .. } => new,
            ChangeStatus::Unknown(ip) => ip,
        }
    }

    fn check_and_replace(new: T, option_ref: &mut Option<T>) -> ChangeStatus<T> {
        match option_ref.take() {
            Some(old_value) if old_value == new => ChangeStatus::Unchanged(new),
            Some(old_value) => {
                *option_ref = Some(new.clone());
                ChangeStatus::Changed { old: old_value, new }
            }
            None => {
                *option_ref = Some(new.clone());
                ChangeStatus::Unknown(new)
            }
        }
    }
}

impl ChangeStatus<Ipv4Addr> {
    pub async fn check_ipv4_changed(ip: Ipv4Addr) -> ChangeStatus<Ipv4Addr> {
        ChangeStatus::check_and_replace(ip, &mut get_state().lock().await.deref_mut().ipv4)
    }
}


impl ChangeStatus<Ipv6Addr> {
    pub async fn check_ipv6_changed(ip: Ipv6Addr) -> ChangeStatus<Ipv6Addr> {
        ChangeStatus::check_and_replace(ip, &mut get_state().lock().await.deref_mut().ipv6)
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use serial_test::serial;

    use super::*;

    async fn reset_state() {
        *get_state().lock().await.deref_mut() = State::default();
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv4_initially_unknown() {
        reset_state().await;

        let ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
        assert_eq!(ChangeStatus::check_ipv4_changed(ip).await, ChangeStatus::Unknown(ip));
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv4_remembers_change() {
        reset_state().await;

        let ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
        ChangeStatus::check_ipv4_changed(ip).await;
        assert_eq!(ChangeStatus::check_ipv4_changed(ip).await, ChangeStatus::Unchanged(ip));
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv4_changes() {
        reset_state().await;

        let old_ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
        ChangeStatus::check_ipv4_changed(old_ip).await;

        let new_ip: Ipv4Addr = "127.0.0.2".parse().unwrap();
        assert_eq!(ChangeStatus::check_ipv4_changed(new_ip).await, ChangeStatus::Changed { old: old_ip, new: new_ip });
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv6_initially_unknown() {
        reset_state().await;

        let ip: Ipv6Addr = "::".parse().unwrap();
        assert_eq!(ChangeStatus::check_ipv6_changed(ip).await, ChangeStatus::Unknown(ip));
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv6_remebers_change() {
        reset_state().await;

        let ip: Ipv6Addr = "::".parse().unwrap();
        ChangeStatus::check_ipv6_changed(ip).await;

        assert_eq!(ChangeStatus::check_ipv6_changed(ip).await, ChangeStatus::Unchanged(ip));
    }

    #[tokio::test]
    #[serial]
    async fn test_check_ipv6_changes() {
        reset_state().await;

        let old: Ipv6Addr = "::".parse().unwrap();
        ChangeStatus::check_ipv6_changed(old).await;

        let new: Ipv6Addr = "::2".parse().unwrap();
        assert_eq!(ChangeStatus::check_ipv6_changed(new).await, ChangeStatus::Changed { old, new })
    }
}