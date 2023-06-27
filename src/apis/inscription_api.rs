use {super::*, serde::Serialize};

#[derive(Serialize)]
pub(crate) struct InscriptionAPI {
  pub(crate) chain: Chain,
  pub(crate) genesis_fee: u64,
  pub(crate) genesis_height: u64,
  pub(crate) address: String,
  // pub(crate) inscription: Inscription,
  pub(crate) content_type: Option<String>,
  pub(crate) inscription_id: InscriptionId,
  pub(crate) next: Option<InscriptionId>,
  pub(crate) number: i64,
  pub(crate) output: TxOut,
  pub(crate) previous: Option<InscriptionId>,
  pub(crate) sat: Option<Sat>,
  pub(crate) satpoint: SatPoint,
  pub(crate) timestamp: String,
}
