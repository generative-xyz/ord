use {super::*, serde::Serialize};

#[derive(Serialize)]
pub(crate) struct BlockEventAPI {
  pub(crate) height: u64,
  pub(crate) events: Vec<InscriptionEvent>,
}
