use gpui::*;

use std::path::PathBuf;
use std::sync::Arc;

use crate::theme::Theme;

#[derive(IntoElement)]
struct ImageContainer {
    src: ImageSource,
}

impl ImageContainer {
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Self {
            src: src.into(),
        }
    }
}

impl RenderOnce for ImageContainer {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        div().child(
            img(self.src).w(px(800.0)).h(px(600.0)),
        )
    }
}

pub struct ImageShowcase {
    pub local_resource: Arc<PathBuf>,

}

impl Render for ImageShowcase {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .font(theme.font.clone())
            .size_full()
            .justify_center()
            .items_center()
            .bg(theme.bg)
            .child(ImageContainer::new(
                self.local_resource.clone(),
            ))
    }
}

pub struct ImageShowcaseRemote {
    pub remote_resource: String,
}

impl Render for ImageShowcaseRemote {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .font(theme.font.clone())
            .size_full()
            .justify_center()
            .items_center()
            .bg(theme.bg)
            .child(ImageContainer::new(
                self.remote_resource.clone(),
            ))
    }
}
