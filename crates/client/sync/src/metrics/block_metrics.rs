use mc_analytics::{register_counter_metric_instrument, register_histogram_metric_instrument};
use opentelemetry::{
    global::{self, Error},
    metrics::{Counter as OtelCounter, Histogram as OtelHistogram}, KeyValue,
};

#[derive(Clone, Debug)]
pub struct BlockMetrics {
    // L2 network metrics
    pub l2_block_number: OtelHistogram<f64>,
    pub l2_sync_time: OtelHistogram<f64>,
    pub l2_avg_sync_time: OtelHistogram<f64>,
    pub l2_latest_sync_time: OtelHistogram<f64>,
    pub l2_state_size: OtelHistogram<f64>,
    pub transaction_count: OtelCounter<u64>,
    pub event_count: OtelCounter<u64>,
    // L1 network metrics
    // gas price is also define in eth/client.rs but this would be the gas used in the block and it's price
    pub l1_gas_price_wei: OtelHistogram<f64>,
    pub l1_gas_price_strk: OtelHistogram<f64>,
}

impl BlockMetrics {
    pub fn register() -> Result<Self, Error> {
        tracing::trace!("Registering Block metrics.");
        // TODO: Remove this println
        println!("Registering Block metrics.");

        let common_scope_attributes = vec![KeyValue::new("crate", "block")];
        let block_meter = global::meter_with_version(
            "crates.block.opentelemetry",
            // TODO: Unsure of these settings, come back
            Some("0.17"),
            Some("https://opentelemetry.io/schemas/1.2.0"),
            Some(common_scope_attributes.clone()),
        );

        let l2_block_number = register_histogram_metric_instrument(
            &block_meter,
            "l2_block_number".to_string(),
            "Gauge for madara L2 block number".to_string(),
            "".to_string(),
        );

        let l2_sync_time = register_histogram_metric_instrument(
            &block_meter,
            "l2_sync_time".to_string(),
            "Gauge for madara L2 sync time".to_string(),
            "".to_string(),
        );

        let l2_avg_sync_time = register_histogram_metric_instrument(
            &block_meter,
            "l2_avg_sync_time".to_string(),
            "Gauge for madara L2 average sync time".to_string(),
            "".to_string(),
        );

        let l2_latest_sync_time = register_histogram_metric_instrument(
            &block_meter,
            "l2_latest_sync_time".to_string(),
            "Gauge for madara L2 latest sync time".to_string(),
            "".to_string(),
        );

        let l2_state_size = register_histogram_metric_instrument(
            &block_meter,
            "l2_state_size".to_string(),
            "Gauge for madara L2 state size".to_string(),
            "".to_string(),
        );

        let transaction_count = register_counter_metric_instrument(
            &block_meter,
            "transaction_count".to_string(),
            "Gauge for madara transaction count".to_string(),
            "".to_string(),
        );

        let event_count = register_counter_metric_instrument(
            &block_meter,
            "event_count".to_string(),
            "Gauge for madara event count".to_string(),
            "".to_string(),
        );

        let l1_gas_price_wei = register_histogram_metric_instrument(
            &block_meter,
            "l1_gas_price_wei".to_string(),
            "Gauge for madara L1 gas price in wei".to_string(),
            "".to_string(),
        );

        let l1_gas_price_strk = register_histogram_metric_instrument(
            &block_meter,
            "l1_gas_price_strk".to_string(),
            "Gauge for madara L1 gas price in strk".to_string(),
            "".to_string(),
        );

        Ok(Self {
            l2_block_number: l2_block_number,
            l2_sync_time: l2_sync_time,
            l2_avg_sync_time: l2_avg_sync_time,
            l2_latest_sync_time: l2_latest_sync_time,
            l2_state_size: l2_state_size,
            transaction_count: transaction_count,
            event_count: event_count,
            l1_gas_price_wei: l1_gas_price_wei,
            l1_gas_price_strk: l1_gas_price_strk,
        })
    }
}
