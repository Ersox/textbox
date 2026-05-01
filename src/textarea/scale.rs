use std::ops::Deref;

use ab_glyph::PxScale;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Deref for Scale {
    type Target = PxScale;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `PxScale` is a transparent wrapper around two `f32` values, so this transmute is safe.
        unsafe { std::mem::transmute(self) }
    }
}