use crate::{
    theme::Theme,
    window::get_window_options,
    workspace::{build_workspace_view, build_workspace_view_remote}
};
use std::env;

pub fn run_app(app: gpui::App) {

    let args: Vec<String> = env::args().collect();

    if args[1].contains("http") {
        app.run(move |cx| {
            Theme::init(cx);
            let bounds = cx.displays().first().unwrap().bounds();
            cx.open_window(get_window_options(bounds), build_workspace_view_remote);
        });
    } else {
        app.run(move |cx| {
            Theme::init(cx);
            let bounds = cx.displays().first().unwrap().bounds();
            cx.open_window(get_window_options(bounds), build_workspace_view);
        });
    }
}
