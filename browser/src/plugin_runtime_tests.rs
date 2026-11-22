use tokio::runtime::Builder;

use super::PluginRuntime;

#[test]
fn building_a_tool_enters_the_plugin_runtime() {
    let runtime = PluginRuntime::new().expect("plugin runtime");

    let sees_runtime = runtime
        .build(|| tokio::runtime::Handle::try_current().is_ok())
        .expect("build");

    assert!(sees_runtime);
}

#[test]
fn polling_a_future_enters_the_plugin_runtime() {
    let plugin_runtime = PluginRuntime::new().expect("plugin runtime");
    let driver = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("driver runtime");

    let future = Box::pin(async { tokio::runtime::Handle::try_current().is_ok() });
    let sees_runtime = driver.block_on(plugin_runtime.wrap(future));

    assert!(sees_runtime);
}
