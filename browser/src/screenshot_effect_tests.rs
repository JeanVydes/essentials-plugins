use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine as _;
use norte::domain::part::Part;
use norte::{AnyFunction, MediaSource, Result, ToolContext, ToolKind, ToolOutput};
use norte_code_plugin_host::HostEffects;
use schemars::Schema;
use serde_json::{Value, json};

use super::ScreenshotEffectTool;

#[derive(schemars::JsonSchema)]
struct Arguments {}

#[derive(Debug)]
struct FakeTool {
    output: ToolOutput,
    schema: Schema,
}

#[async_trait]
impl AnyFunction for FakeTool {
    fn name(&self) -> &'static str {
        "browser_use"
    }

    fn description(&self) -> &'static str {
        "a fake browser"
    }

    fn parameters_schema(&self) -> &Schema {
        &self.schema
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Function
    }

    async fn execute(&self, _context: &ToolContext, _arguments: &Value) -> Result<ToolOutput> {
        Ok(self.output.clone())
    }
}

fn wrapped(output: ToolOutput) -> ScreenshotEffectTool {
    ScreenshotEffectTool::new(Arc::new(FakeTool {
        output,
        schema: schemars::schema_for!(Arguments),
    }))
}

#[tokio::test]
async fn a_screenshot_becomes_an_inline_image_effect() {
    let bytes = vec![1, 2, 3, 4];
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let tool = wrapped(ToolOutput {
        parts: vec![Part::text("screenshot: 4 bytes of base64 PNG")],
        structured: Some(json!({"screenshot_base64": encoded})),
        is_error: false,
    });
    assert_eq!(tool.name(), "browser_use");

    let mut output = tool
        .execute(&ToolContext::new(), &json!({}))
        .await
        .expect("execute");
    let effects = HostEffects::extract(&mut output)
        .expect("envelope")
        .expect("one effect");
    let draft = effects.user_messages.into_iter().next().expect("draft");
    let parts = draft.into_parts();
    let Some(Part::Media(media)) = parts.first() else {
        panic!("expected an image part");
    };
    assert_eq!(media.mime.as_str(), "image/png");
    assert_eq!(media.source, MediaSource::Inline(bytes));
    assert!(
        output
            .structured
            .expect("structured")
            .get("screenshot_base64")
            .is_some()
    );
}

#[tokio::test]
async fn an_output_without_a_screenshot_keeps_no_effect() {
    let original = json!({"message": "clicked"});
    let tool = wrapped(ToolOutput {
        parts: vec![Part::text("clicked")],
        structured: Some(original.clone()),
        is_error: false,
    });

    let mut output = tool
        .execute(&ToolContext::new(), &json!({}))
        .await
        .expect("execute");
    assert!(
        HostEffects::extract(&mut output)
            .expect("no envelope")
            .is_none()
    );
    assert_eq!(output.structured, Some(original));
}

#[tokio::test]
async fn a_bad_screenshot_keeps_the_tool_result_without_an_effect() {
    let original = json!({"screenshot_base64": "not base64!"});
    let tool = wrapped(ToolOutput {
        parts: vec![Part::text("screenshot failed")],
        structured: Some(original.clone()),
        is_error: false,
    });

    let mut output = tool
        .execute(&ToolContext::new(), &json!({}))
        .await
        .expect("execute");
    assert!(
        HostEffects::extract(&mut output)
            .expect("no envelope")
            .is_none()
    );
    assert_eq!(output.structured, Some(original));
}
