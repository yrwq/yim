use gpui::*;

fn load_fonts(cx: &mut AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx.asset_source().load(&font_path)?;
            embedded_fonts.push(font_bytes);
        }
    }
    cx.text_system().add_fonts(embedded_fonts)
}

#[derive(Debug)]
pub struct Theme {
    pub font: SharedString,
    pub bg: Rgba,
    pub bg_sec: Rgba,
    pub fg: Hsla,
    pub pri: Hsla,
    pub sec: Hsla,
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        load_fonts(cx).expect("Failed to load fonts");
        cx.set_global(Theme::new())
    }

    fn new() -> Self {
        Self {
            font: "Iosevka Custom".into(),
            bg: rgb(0x121212),
            bg_sec: rgb(0x282828),
            fg: white(),
            pri: hsla(123. / 360., 0.37, 0.89, 1.),
            sec: hsla(120. / 360., 0.13, 0.61, 1.),
        }
    }
}

impl Global for Theme {}
