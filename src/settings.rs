use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Settings {
    pub rpc: RPCSetting,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rpc: RPCSetting::default(),
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum RPCSetting {
    None,
    #[default]
    HideFilename,
    Full,
}

impl std::fmt::Debug for RPCSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "None",
            Self::HideFilename => "Hide Filename",
            Self::Full => "Full",
        })
    }
}
