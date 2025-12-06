use std::{
    hash::Hash,
    fmt,
    rc::Rc,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Map { id: usize },
    Marker { parent: Rc<Id>, id: usize },
    Polygon { parent: Rc<Id>, id: usize },
    Popup { parent: Rc<Id>, id: usize },
}

impl Id {
    pub fn map(id: usize) -> Self {
        Id::Map { id }
    }

    pub fn marker(parent: &Rc<Id>, id: usize) -> Id {
        Id::Marker { parent: parent.clone(), id }
    }

    pub fn polygon(parent: &Rc<Id>, id: usize) -> Id {
        Id::Polygon { parent: parent.clone(), id }
    }

    pub fn popup(parent: &Rc<Id>, id: usize) -> Id {
        Id::Popup { parent: parent.clone(), id }
    }

    pub fn id(&self) -> usize {
        match self {
            Id::Map { id } 
                | Id::Marker { id, .. } 
                | Id::Polygon { id, .. } 
                | Id::Popup { id, .. } 
                => *id,
        }
    }

    pub fn parent(&self) -> Option<&Id> {
        match self {
            Id::Map { .. } => None,
            Id::Marker { parent, .. }
                | Id::Polygon { parent, .. } 
                | Id::Popup { parent, .. }
                => Some(parent.as_ref()),
        }
    }
}

impl Into<f64> for Id {
    fn into(self) -> f64 {
        match self {
            Id::Map { id } 
                | Id::Marker { id, .. } 
                | Id::Polygon { id, .. } 
                | Id::Popup { id, .. } 
                => id as f64,
        }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            Id::Map { id } 
                | Id::Marker { id, .. } 
                | Id::Polygon { id, .. } 
                | Id::Popup { id, .. } 
                => id.serialize(serializer),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::Map { id } => write!(f, "map-{id}"),
            Id::Marker { id, .. } => write!(f, "marker-{id}"),
            Id::Polygon { id, .. } => write!(f, "polygon-{id}"),
            Id::Popup { id, .. } => write!(f, "popup-{id}"),
        }
    }
}