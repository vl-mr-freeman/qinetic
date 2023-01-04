//! User-interface [`Plugin`]s functionality.

use crate::{
    components::{Button, Image, RectTransform, Scrollbar, Slider, Text},
    resources::UiResource,
    stages::{UiStage, UiStageGroup},
    systems::UiSystem,
};

use qinetic_app::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// User-interface [`Plugin`]
///
/// [`Component`]s:
/// * [`RectTransform`]
/// * [`Button`]
/// * [`Image`]
/// * [`Text`]
/// * [`Slider`]
/// * [`Scrollbar`]
///
/// [`Stage`]s:
/// * [`UiStage`]
///
/// [`Resource`]:
/// * [`UiResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_plugin(UiPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct UiPlugin {}

impl Plugin for UiPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(RectTransform::default())
            .with_component(Button::default())
            .with_component(Image::default())
            .with_component(Text::default())
            .with_component(Slider::default())
            .with_component(Scrollbar::default())
            .with_resource(UiResource::default())
            .with_stage_group(UiStageGroup::default())
            .with_system(UiStage::default(), UiSystem::default());
    }
}
