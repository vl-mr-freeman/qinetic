//! Input [`Event`]s functionality.

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

pub use crate::{
    keyboard::KeyboardEvent,
    mouse::{
        MouseButtonEvent, MouseButtonEventBuilder, MouseButtonEventBuilderError, MouseMotionEvent,
        MouseMotionEventBuilder, MouseMotionEventBuilderError, MouseWheelEvent,
        MouseWheelEventBuilder, MouseWheelEventBuilderError,
    },
};
