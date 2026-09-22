use std::fmt;

use glam::Vec3;
use tiger_pkg::TagHash;

use crate::world::node_filter::NodeFilter;

#[derive(Debug, Clone)]
pub struct Label {
    pub label: String,
    pub kind: NodeFilter,
    pub default: bool,
    pub offset: Vec3,
    pub entity_hash: Option<TagHash>,
    pub related_hashes: Vec<TagHash>,
}

impl Label {
    pub fn default_for(kind: NodeFilter) -> Self {
        Self {
            label: kind.name().to_string(),
            kind,
            default: true,
            offset: Vec3::ZERO,
            entity_hash: None,
            related_hashes: Vec::new(),
        }
    }

    pub fn named(name: impl Into<String>, kind: NodeFilter) -> Self {
        Self {
            label: name.into(),
            kind,
            default: false,
            offset: Vec3::ZERO,
            entity_hash: None,
            related_hashes: Vec::new(),
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(hash) = self.entity_hash {
            write!(f, "{} [{:?}]", self.label, hash)
        } else {
            f.write_str(&self.label)
        }
    }
}
