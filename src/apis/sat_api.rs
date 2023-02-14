use {super::*, serde::Serialize};

#[derive(Serialize)]
pub(crate) struct SatAPI {
  pub(crate) sat: Sat,
  pub(crate) satpoint: Option<SatPoint>,
  pub(crate) block: String,
  // pub(crate) blocktime: Blocktime,
  pub(crate) inscription: Option<InscriptionId>,
}