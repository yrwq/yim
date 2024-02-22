use gpui::{
    WindowOptions,
    WindowBounds,
    Bounds,
    Point,
    Size,
    GlobalPixels,
    TitlebarOptions,
    WindowKind,
    point,
    px,
};

pub static WIDTH: f64 = 800.0;
pub static HEIGHT: f64 = 600.0;

pub fn get_window_options(bounds: Bounds<GlobalPixels>) -> WindowOptions {
    let width = GlobalPixels::from(WIDTH);
    let height = GlobalPixels::from(HEIGHT);
    return WindowOptions {
        bounds: WindowBounds::Fixed(Bounds {
            origin: Point {
                x: GlobalPixels::from(bounds.center().x - width / 2.0),
                y: GlobalPixels::from(bounds.center().y - height / 2.0),
            },
            size: Size {
                width: GlobalPixels::from(800.),
                height: GlobalPixels::from(600.),
            },
        }),
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(8.), px(8.))),
            // traffic_light_position: Some(point,
        }),
        center: true,
        focus: true,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: true,
        display_id: None,
    };
}
