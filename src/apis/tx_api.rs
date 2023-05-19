use {super::*, serde::Serialize};

#[derive(Serialize)]
pub(crate) struct TxAPI {
  pub(crate) blockhash: Option<BlockHash>,
  pub(crate) chain: Chain,
  pub(crate) inscription: Option<InscriptionId>,
  pub(crate) transaction: Transaction,
  pub(crate) txid: Txid,
}
