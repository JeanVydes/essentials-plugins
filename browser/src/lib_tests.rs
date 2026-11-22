use super::*;

#[test]
fn the_plugin_declares_the_browser_tool() {
    let plugin = BrowserPlugin::new().expect("plugin");
    let tools = plugin.tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name(), "browser_use");

    let schema = serde_json::to_value(tools[0].parameters_schema()).expect("schema");
    let variants = schema["$defs"]["BrowserAction"]["oneOf"]
        .as_array()
        .expect("action variants");
    assert!(
        variants.iter().any(|variant| {
            variant["properties"]["type"]["const"].as_str() == Some("screenshot")
        })
    );
}

#[test]
fn the_plugin_reports_the_host_abi_version() {
    assert_eq!(norte_plugin_abi_version(), PLUGIN_ABI_VERSION);
}

#[test]
fn the_loader_can_construct_the_plugin() {
    let raw = create_plugin();
    assert!(!raw.is_null());
    // The pointer crossed back into this crate's allocator, so the test owns it.
    unsafe {
        drop(Box::from_raw(raw));
    }
}
