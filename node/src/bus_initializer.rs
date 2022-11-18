use bus_rabbitmq::RabbitmqBus;
use graph::components::bus::Bus;
use graph::slog::warn;
use regex::Regex;

pub struct BusInitializer;

pub enum BusScheme {
    RabbitMQ,
}

impl BusInitializer {
    pub fn new(uri: Option<String>, logger: graph::slog::Logger) -> Option<impl Bus> {
        match BusInitializer::get_bus_scheme(&uri) {
            Some(BusScheme::RabbitMQ) => {
                warn!(logger, "Starting Bus of RabbitMQ";);
                Some(RabbitmqBus::new(uri.unwrap(), logger))
            }
            _ => {
                warn!(logger, "No bus at work";);
                None
            }
        }
    }
    pub fn get_bus_scheme(uri: &Option<String>) -> Option<BusScheme> {
        if uri.is_none() {
            return None;
        }

        let re = Regex::new(r"^\w+").unwrap();
        let scheme = uri.clone().and_then(|text| {
            re.find(text.as_str())
                .and_then(|regex_match| match regex_match.as_str() {
                    "amqp" => Some(BusScheme::RabbitMQ),
                    _ => None,
                })
        });
        return scheme;
    }
}
