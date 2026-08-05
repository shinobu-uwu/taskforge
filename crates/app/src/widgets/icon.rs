use std::borrow::Cow;

use gpui::{
    AnyElement, App, AssetSource, Entity, IntoElement, RenderOnce, Result, SharedString, Window,
};
use gpui_component::{Icon, IconNamed, icon_named};
use rust_embed::RustEmbed;

icon_named!(FontAwesomeIconName, "../../assets/icons");

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow::anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

impl FontAwesomeIconName {
    /// Return the icon as a Entity<Icon>
    pub fn view(self, cx: &mut App) -> Entity<Icon> {
        Icon::new(self).view(cx)
    }
}

impl From<FontAwesomeIconName> for AnyElement {
    fn from(val: FontAwesomeIconName) -> Self {
        Icon::new(val).into_any_element()
    }
}

impl RenderOnce for FontAwesomeIconName {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}
