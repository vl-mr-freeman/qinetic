use crate::entity::*;

/// A data conteiner for an [`Entity`].
pub trait Component: Send + Sync + 'static {}
