use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::components::bus::BusMessage;
use graph::prelude::async_trait;
use graph::prelude::Logger;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RabbitmqBus {
    pub name: String,
    pub logger: Logger,
    connection: Arc<Mutex<Connection>>,
}

#[async_trait]
impl Bus for RabbitmqBus {
    async fn new(connection_uri: String, logger: Logger) -> RabbitmqBus {
        let connection = Connection::insecure_open(&connection_uri).unwrap();

        RabbitmqBus {
            name: String::from("my rabbit store"),
            connection: Arc::new(Mutex::new(connection)),
            logger,
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn send_plain_text(&self, message: BusMessage) -> Result<(), BusError> {
        // NOTE: this is very UGLY, but we are doing POC, so its ok for now
        let join_str = message.value.join(",");
        let data_as_bytes = join_str.as_bytes();
        let routing_key = message.subgraph_id.as_str();
        let exchange_name = message.subgraph_id.as_str();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        return match self.connection.lock() {
            Ok(mut conn) => match conn.open_channel(None) {
                Ok(channel) => {
                    match channel.exchange_declare(
                        ExchangeType::Topic,
                        exchange_name,
                        exchange_opts.clone(),
                    ) {
                        Ok(exchange) => {
                            match exchange.publish(Publish::new(data_as_bytes.clone(), routing_key))
                            {
                                Ok(()) => Ok(()),
                                Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
                            }
                        }
                        Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
                    }
                }
                Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
            },
            Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
        };
    }

    async fn start(&self, _r: UnboundedReceiver<BusMessage>) {}
}
