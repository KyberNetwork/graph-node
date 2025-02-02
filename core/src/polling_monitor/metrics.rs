use std::sync::Arc;

use graph::{
    components::store::DeploymentLocator,
    prelude::MetricsRegistry,
    prometheus::{Counter, Gauge},
};

pub struct PollingMonitorMetrics {
    pub requests: Counter,
    pub errors: Counter,
    pub not_found: Counter,
    pub queue_depth: Gauge,
}

impl PollingMonitorMetrics {
    pub fn new(registry: Arc<dyn MetricsRegistry>, deployment: &DeploymentLocator) -> Self {
        let requests = registry
            .new_deployment_counter(
                "polling_monitor_requests",
                "counts the total requests made to the service being polled",
                deployment,
            )
            .unwrap();
        let not_found = registry
            .new_deployment_counter(
                "polling_monitor_not_found",
                "counts 'not found' responses returned from the service being polled",
                deployment,
            )
            .unwrap();
        let errors = registry
            .new_deployment_counter(
                "polling_monitor_errors",
                "counts errors returned from the service being polled",
                deployment,
            )
            .unwrap();
        let queue_depth = registry
            .new_deployment_gauge(
                "polling_monitor_queue_depth",
                "size of the queue of polling requests",
                deployment,
            )
            .unwrap();
        Self {
            requests,
            errors,
            not_found,
            queue_depth,
        }
    }

    #[cfg(test)]
    pub(crate) fn mock() -> Self {
        Self {
            requests: Counter::new("x", " ").unwrap(),
            errors: Counter::new("y", " ").unwrap(),
            not_found: Counter::new("z", " ").unwrap(),
            queue_depth: Gauge::new("w", " ").unwrap(),
        }
    }
}
