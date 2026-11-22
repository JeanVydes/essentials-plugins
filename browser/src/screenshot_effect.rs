use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine as _;
use norte::domain::part::Part;
use norte::{
    AnyFunction, Error, MediaKind, MediaPart, MediaSource, MimeType, Result, ToolContext, ToolKind,
    ToolOutput,
};
use norte_code_plugin_host::{HostEffects, UserMessageDraft};
use schemars::Schema;
use serde_json::Value;

const SCREENSHOT_FIELD: &str = "screenshot_base64";

#[derive(Debug)]
pub(super) struct ScreenshotEffectTool {
    inner: Arc<dyn AnyFunction>,
}

impl ScreenshotEffectTool {
    pub(super) fn new(inner: Arc<dyn AnyFunction>) -> Self {
        Self { inner }
    }

    fn screenshot(output: &ToolOutput) -> Option<&str> {
        output.structured.as_ref()?.get(SCREENSHOT_FIELD)?.as_str()
    }

    fn image(data: &str) -> Result<Part> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|error| {
                Error::InvalidInput(format!("a browser screenshot is not base64: {error}"))
            })?;
        let mime = MimeType::new("image/png")?;
        Ok(Part::Media(MediaPart::new(
            MediaKind::Image,
            MediaSource::Inline(bytes),
            mime,
        )))
    }
}

#[async_trait]
impl AnyFunction for ScreenshotEffectTool {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn parameters_schema(&self) -> &Schema {
        self.inner.parameters_schema()
    }

    fn kind(&self) -> ToolKind {
        self.inner.kind()
    }

    fn declaration(&self) -> Option<Value> {
        self.inner.declaration()
    }

    async fn execute(&self, context: &ToolContext, arguments: &Value) -> Result<ToolOutput> {
        let output = self.inner.execute(context, arguments).await?;
        let Some(data) = Self::screenshot(&output) else {
            return Ok(output);
        };
        let image = match Self::image(data) {
            Ok(image) => image,
            Err(error) => {
                tracing::warn!(tool = self.name(), %error, "a browser screenshot was not attached");
                return Ok(output);
            }
        };
        let draft = UserMessageDraft::new(vec![image])?;
        HostEffects::user_message(draft).attach(output)
    }
}

#[cfg(test)]
#[path = "screenshot_effect_tests.rs"]
mod tests;
