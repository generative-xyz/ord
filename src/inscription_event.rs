use {super::*, serde::Serialize};

#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]

pub struct InscriptionEvent {
  pub(crate) event: String,
  pub(crate) inscription_id: InscriptionId,
}

// impl<'de> Deserialize<'de> for InscriptionEvent {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     Ok(DeserializeFromStr::deserialize(deserializer)?.0)
//   }
// }

// impl Serialize for InscriptionEvent {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     serializer.collect_str(self)
//   }
// }

impl Display for InscriptionEvent {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}/{}", self.inscription_id.to_string(), self.event)
  }
}

// #[derive(Debug)]
// pub enum ParseError {
//   Character(char),
//   Length(usize),
//   Separator(char),
//   Txid(bitcoin::hashes::hex::Error),
//   Index(std::num::ParseIntError),
// }

// impl Display for ParseError {
//   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//     match self {
//       Self::Character(c) => write!(f, "invalid character: '{c}'"),
//       Self::Length(len) => write!(f, "invalid length: {len}"),
//       Self::Separator(c) => write!(f, "invalid seprator: `{c}`"),
//       Self::Txid(err) => write!(f, "invalid txid: {err}"),
//       Self::Index(err) => write!(f, "invalid index: {err}"),
//     }
//   }
// }

// impl std::error::Error for ParseError {}

// impl FromStr for InscriptionEvent {
//   type Err = ParseError;

//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     if let Some(char) = s.chars().find(|char| !char.is_ascii()) {
//       return Err(ParseError::Character(char));
//     }

//     const TXID_LEN: usize = 64;
//     const MIN_LEN: usize = TXID_LEN + 2;

//     if s.len() < MIN_LEN {
//       return Err(ParseError::Length(s.len()));
//     }

//     let txid = &s[..TXID_LEN];

//     let separator = s.chars().nth(TXID_LEN).unwrap();

//     if separator != 'i' {
//       return Err(ParseError::Separator(separator));
//     }

//     let vout = &s[TXID_LEN + 1..];

//     Ok(Self {
//       txid: txid.parse().map_err(ParseError::Txid)?,
//       index: vout.parse().map_err(ParseError::Index)?,
//     })
//   }
// }

// impl From<Txid> for InscriptionEvent {
//   fn from(txid: Txid) -> Self {
//     Self { txid, index: 0 }
//   }
// }
