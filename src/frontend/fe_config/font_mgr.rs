use std::sync::Arc;
use egui::FontData;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FontConfig {
    roots: Vec<String>, // 扫描的根路径
    default_font_bin: Option<Vec<u8>>,
}

impl Default for FontConfig {
    fn default() -> FontConfig {
        FontConfig{
            roots: Vec::new(),
            default_font_bin: Some(include_bytes!("../../../assets/fonts/LXGWBrightCodeTC-Regular.ttf").to_vec()),
        }
    }
}

pub struct FontManager {
    // config: FontConfig,
    changed: bool,
    default_font_data: Arc<FontData>
}
impl FontManager {
    pub fn new(config: FontConfig) -> FontManager {
        let font_data = Arc::new(FontData::from_owned(config.default_font_bin.clone().unwrap()));
        FontManager{
            // config,
            changed: true,
            default_font_data: font_data,
        }
    }

    pub fn inject(&mut self, ctx: &egui::Context) {
        if !self.changed {
            return;
        }

        tracing::info!("Injecting font");
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("default".to_string(), self.default_font_data.clone());
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "default".to_string());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "default".to_string());
        ctx.set_fonts(fonts);
        self.changed = false;
    }
}