use {super::*, serde::Serialize};

#[derive(Serialize)]

pub(crate) struct InscriptionsAPI {
  pub(crate) inscriptions: Vec<InscriptionId>,
  pub(crate) prev: Option<u64>,
  pub(crate) next: Option<u64>,
}
