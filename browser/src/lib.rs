//! Browser automation plugin for Norte Code.
//!
//! The CDP executor stays in Norte's `norte-tools` crate. This plugin only
//! hands its tool to the host, which registers it like any other tool.

use std::io;
use std::sync::Arc;

use norte::AnyFunction;
use norte_code_plugin_host::plugins::{PLUGIN_ABI_VERSION, Plugin};
use norte_tools::browser_tool;

mod plugin_runtime;
mod screenshot_effect;

use plugin_runtime::PluginRuntime;
use screenshot_effect::ScreenshotEffectTool;

struct BrowserPlugin {
    runtime: PluginRuntime,
}

impl BrowserPlugin {
    fn new() -> io::Result<Self> {
        Ok(Self {
            runtime: PluginRuntime::new()?,
        })
    }
}

impl Plugin for BrowserPlugin {
    fn name(&self) -> &'static str {
        "browser"
    }

    fn tools(&self) -> Vec<Arc<dyn AnyFunction>> {
        let Some(inner) = self.runtime.build(browser_tool) else {
            eprintln!("warning: the browser tool could not be built");
            return Vec::new();
        };
        vec![Arc::new(ScreenshotEffectTool::new(
            inner,
            self.runtime.clone(),
        ))]
    }
}

/// Exports the plugin interface version expected by Norte Code.
#[unsafe(no_mangle)]
pub extern "C" fn norte_plugin_abi_version() -> u32 {
    PLUGIN_ABI_VERSION
}

/// Constructs the plugin for the host loader.
#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    match BrowserPlugin::new() {
        Ok(plugin) => Box::into_raw(Box::new(plugin)),
        Err(error) => {
            eprintln!("browser plugin could not start its runtime: {error}");
            let null: *mut BrowserPlugin = std::ptr::null_mut();
            null
        }
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
