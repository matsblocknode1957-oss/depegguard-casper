#![cfg_attr(target_arch = "wasm32", no_std)]
use odra::prelude::*;

/// A single depeg signal event stored on-chain.
#[odra::odra_type]
pub struct SignalEntry {
    pub index: u64,
    pub coin: String,
    /// "HEDGE" or "EXIT"
    pub signal: String,
    /// Price deviation from peg in basis points (e.g. 150 = -1.5%)
    pub deviation_bps: u64,
    /// Unix timestamp in seconds (supplied by caller)
    pub timestamp: u64,
}

#[odra::module]
pub struct SignalLogger {
    count: Var<u64>,
    signals: Mapping<u64, SignalEntry>,
}

#[odra::module]
impl SignalLogger {
    /// Record a new depeg signal on-chain.
    pub fn log_signal(
        &mut self,
        coin: String,
        signal: String,
        deviation_bps: u64,
        timestamp: u64,
    ) {
        let idx = self.count.get_or_default();
        self.signals.set(
            &idx,
            SignalEntry {
                index: idx,
                coin,
                signal,
                deviation_bps,
                timestamp,
            },
        );
        self.count.set(idx + 1);
    }

    /// Total number of signals logged.
    pub fn get_count(&self) -> u64 {
        self.count.get_or_default()
    }

    /// Return the last `n` signals, newest first. Capped at total count.
    pub fn get_last_n(&self, n: u64) -> Vec<SignalEntry> {
        let count = self.count.get_or_default();
        let start = if count > n { count - n } else { 0 };
        (start..count)
            .rev()
            .map(|i| self.signals.get(&i).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, NoArgs};

    #[test]
    fn log_and_retrieve() {
        let test_env = odra_test::env();
        let mut contract = SignalLogger::deploy(&test_env, NoArgs);

        assert_eq!(contract.get_count(), 0);

        contract.log_signal(
            "USDC".to_string(),
            "HEDGE".to_string(),
            150,
            1_700_000_000,
        );
        contract.log_signal(
            "USDT".to_string(),
            "EXIT".to_string(),
            300,
            1_700_000_060,
        );

        assert_eq!(contract.get_count(), 2);

        let last = contract.get_last_n(1);
        assert_eq!(last.len(), 1);
        assert_eq!(last[0].coin, "USDT");
        assert_eq!(last[0].signal, "EXIT");
        assert_eq!(last[0].deviation_bps, 300);

        let all = contract.get_last_n(10);
        assert_eq!(all.len(), 2);
        // newest first
        assert_eq!(all[0].index, 1);
        assert_eq!(all[1].index, 0);
    }
}
