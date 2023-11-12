use {super::*, serde::Serialize};

#[derive(Serialize)]
pub(crate) struct OutputAPI {
  pub(crate) outpoint: OutPoint,
  // pub(crate) list: Option<List>,
  // pub(crate) chain: Chain,
  pub(crate) output: TxOut,
  pub(crate) inscriptions: Vec<InscriptionId>,
}
