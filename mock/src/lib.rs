mod metrics_registry;

pub use self::metrics_registry::MockMetricsRegistry;
use graph::components::store::DeploymentLocator;
use graph::prelude::Logger;
use graph::prelude::StopwatchMetrics;
use std::sync::Arc;

pub fn create_stopwatch(deployment: &DeploymentLocator, logger: Logger) -> StopwatchMetrics {
    let metrics_registry = Arc::new(MockMetricsRegistry::new());
    StopwatchMetrics::new(logger, deployment, "transact", metrics_registry.clone())
}
