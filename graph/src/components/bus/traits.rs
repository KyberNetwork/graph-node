use super::*;
use crate::prelude::BlockPtr;
use crate::prelude::EntityModification;
use crate::prelude::Logger;
use async_trait::async_trait;

#[async_trait]
pub trait Bus: Send + Sync + 'static {
    fn new(connection_uri: String, logger: Logger) -> Self;
    async fn send_trigger_data(&self) -> Result<(), BusError>;
    async fn send_modification_data(
        &self,
        block_ptr: BlockPtr,
        mods: Vec<EntityModification>,
        manifest_idx_and_names: Vec<(u32, String)>,
    ) -> Result<(), BusError>;
}
