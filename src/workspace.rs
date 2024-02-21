use gpui::*;

use std::sync::Arc;
use crate::ui::{
    ImageShowcase,
    ImageShowcaseRemote
};
use std::path::PathBuf;

pub fn build_workspace_view(cx: &mut WindowContext<'_>) -> View<ImageShowcase> {
    let args: Vec<String> = std::env::args().collect();
    cx.new_view(|_cx| ImageShowcase {
        local_resource: Arc::new(
            PathBuf::from(args[1].clone()),
        ),
    })
}

pub fn build_workspace_view_remote(cx: &mut WindowContext<'_>) -> View<ImageShowcaseRemote> {
    let args: Vec<String> = std::env::args().collect();

    cx.new_view(|_cx| ImageShowcaseRemote {
        remote_resource: args[1].clone(),
        // remote_resource: url.into(),
    })
}
