use gpui::*;

#[derive(Debug)]
pub struct Theme {
    pub bg: Hsla,
    pub fg: Hsla,
    pub pri: Hsla,
    pub sec: Hsla,
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(Theme::new())
    }

    fn new() -> Self {
        Self {
            bg: hsla(129. / 360., 0.47, 0.96, 1.),
            fg: white(),
            pri: hsla(123. / 360., 0.37, 0.89, 1.),
            sec: hsla(120. / 360., 0.13, 0.61, 1.),
        }
    }
}

impl Global for Theme {}
