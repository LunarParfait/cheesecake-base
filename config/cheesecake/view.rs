use axum::response::Html;
#[cfg(debug_assertions)]
use hotwatch::{Event, EventKind, Hotwatch};
use serde::Serialize;
use std::convert::identity;
use std::sync::LazyLock;
#[cfg(debug_assertions)]
use std::sync::RwLock;
use tera::Tera;
#[cfg(debug_assertions)]
use tokio::sync::watch;

#[cfg(debug_assertions)]
static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    Tera::new(concat!("resources/templates", "**/*.html"))
        .unwrap()
        .into()
});

#[cfg(not(debug_assertions))]
static TERA: LazyLock<Tera> = LazyLock::new(|| {
    Tera::new(concat!("dist/templates", "**/*.html")).unwrap()
});

#[cfg(debug_assertions)]
pub static HOTWATCH_CHANNEL: LazyLock<(
    watch::Sender<()>,
    watch::Receiver<()>,
)> = LazyLock::new(|| watch::channel(()));

#[cfg(debug_assertions)]
static HOTWATCH: LazyLock<Hotwatch> = LazyLock::new(|| {
    use std::time::Duration;

    let mut hotwatch =
        Hotwatch::new_with_custom_delay(Duration::new(0, 300000000)).unwrap();
    hotwatch
        .watch("resources/templates", |event: Event| {
            match event.kind {
                EventKind::Any | EventKind::Other => (),
                _ => {
                    HOTWATCH_CHANNEL.0.send(()).unwrap();
                    TERA.write().unwrap().full_reload().unwrap();
                }
            };
        })
        .unwrap();

    hotwatch
});

#[cfg(debug_assertions)]
pub fn setup_hotwatch() {
    let _ = &*HOTWATCH;
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Tera: {0}")]
    Tera(tera::Error),
    #[error("Serde: {0}")]
    Serde(serde_json::Error),
}

pub type RenderResult = Result<Html<String>, anyhow::Error>;

pub trait AppTemplate: Serialize + Default {
    /// Renders the template with given path/name
    ///
    /// # Errors
    /// This method errors if Tera fails to render the template,
    /// or if the serialization with serde fails, or if
    /// (dev-only) tera full reload fails
    fn render(self, path: &'static str) -> RenderResult;
}

#[cfg(debug_assertions)]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> anyhow::Result<String> {
    ctx.insert("env_is_dev", &true);
    let mteradev = TERA.read().unwrap();
    let raw = mteradev.render(path, &ctx)?;

    Ok(raw)
}

#[cfg(not(debug_assertions))]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> anyhow::Result<String> {
    ctx.insert("env_is_dev", &false);
    let raw = TERA.render(path, &ctx)?;

    Ok(raw)
}

impl<T: Serialize + Default> AppTemplate for T {
    fn render(self, path: &'static str) -> RenderResult {
        let ctx_json =
            serde_json::to_value(self).map_err(RenderError::Serde)?;
        let ctx = tera::Context::from_value(ctx_json)
            .map_or_else(|_| tera::Context::new(), identity);

        render_internal(path, ctx).map(Html)
    }
}
