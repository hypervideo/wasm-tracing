#[doc(hidden)]
mod console;
pub use console::*;
use std::sync::{Arc, RwLock};
use tracing_subscriber::EnvFilter;
use wasm_bindgen::prelude::*;

#[deprecated(since = "1.0.0", note = "Rename WASMLayerConfig to WasmLayerConfig.")]
pub type WASMLayerConfig = WasmLayerConfig;

/// Dynamic log filter. Wraps a [`tracing_subscriber::filter::EnvFilter`], supports logging directives like `RUST_LOG`.
#[derive(Debug, Clone)]
pub struct LogFilter(Arc<RwLock<EnvFilter>>);

impl Default for LogFilter {
    fn default() -> Self {
        EnvFilter::builder()
            .with_default_directive("error".parse().unwrap())
            .parse_lossy("")
            .into()
    }
}

impl From<EnvFilter> for LogFilter {
    fn from(filter: EnvFilter) -> Self {
        LogFilter(Arc::new(RwLock::new(filter)))
    }
}

impl LogFilter {
    pub fn new(directives: impl AsRef<str>) -> Self {
        EnvFilter::builder().parse_lossy(directives).into()
    }

    pub(crate) fn read(&self) -> std::sync::RwLockReadGuard<'_, EnvFilter> {
        self.0.read().unwrap()
    }

    pub fn update(&self, directives: impl AsRef<str>) {
        *self.0.write().unwrap() = EnvFilter::builder().parse_lossy(directives)
    }

    pub fn max_level_hint(&self) -> Option<tracing::level_filters::LevelFilter> {
        self.read().max_level_hint()
    }
}

/// JavaScript logger callback function.
/// Takes: level (string), module_path (string), message (string)
#[wasm_bindgen]
extern "C" {
    pub type JsLogger;

    #[wasm_bindgen(structural, method, js_name = log)]
    pub fn log(this: &JsLogger, level: &str, module_path: &str, message: &str);
}

/// Wrapper for JsLogger that implements Send + Sync.
/// SAFETY: WASM is single-threaded, so Send and Sync are safe.
pub struct SafeJsLogger(JsLogger);

impl SafeJsLogger {
    pub fn new(logger: JsLogger) -> Self {
        Self(logger)
    }

    pub fn log(&self, level: &str, module_path: &str, message: &str) {
        self.0.log(level, module_path, message)
    }
}

// SAFETY: WASM runs in a single-threaded environment
unsafe impl Send for SafeJsLogger {}
unsafe impl Sync for SafeJsLogger {}

///Configuration parameters for the [WasmLayer](crate::prelude::WasmLayer).
pub struct WasmLayerConfig {
    /// In dev-tools, report timings of traces
    pub report_logs_in_timings: bool,
    /// See [ConsoleConfig]
    pub console: ConsoleConfig,
    /// Static maximum log level, [`Self::filter`] is dynamically evaluated.
    pub max_level: Option<tracing::Level>,
    /// Dynamic log filter, see [`LogFilter`];
    pub filter: LogFilter,
    /// Show/hide fields of types
    pub show_fields: bool,
    /// Show origin (line number, source)
    pub show_origin: bool,
    /// Optional URL to prepend to origins. E.g. to allow for showing full file paths that can be navigated when logged in the browser console.
    pub origin_base_url: Option<String>,
    /// Optional JavaScript logger to use instead of console
    pub js_logger: Option<Arc<SafeJsLogger>>,
}

impl Default for WasmLayerConfig {
    fn default() -> Self {
        WasmLayerConfig {
            report_logs_in_timings: true,
            console: ConsoleConfig::ReportWithConsoleColor,
            max_level: None,
            filter: LogFilter::new("trace"),
            show_fields: true,
            show_origin: true,
            origin_base_url: None,
            js_logger: None,
        }
    }
}

impl WasmLayerConfig {
    /// Create a default [WasmLayerConfig]
    pub fn new() -> WasmLayerConfig {
        WasmLayerConfig::default()
    }

    /// Set whether events should appear in performance Timings
    pub fn set_report_logs_in_timings(&mut self, report_logs_in_timings: bool) -> &mut Self {
        self.report_logs_in_timings = report_logs_in_timings;
        self
    }

    /// Set the maximal level on which events should be displayed
    pub fn set_max_level(&mut self, max_level: tracing::Level) -> &mut Self {
        self.max_level = Some(max_level);
        self
    }

    /// See [`LogFilter`]
    pub fn set_env_filter(&mut self, env_filter: impl Into<LogFilter>) -> &mut Self {
        self.filter = env_filter.into();
        self
    }

    /// Set if and how events should be displayed in the browser console
    pub fn set_console_config(&mut self, console_config: ConsoleConfig) -> &mut Self {
        self.console = console_config;
        self
    }

    pub fn set_show_origin(&mut self, show_origin: bool) -> &mut Self {
        self.show_origin = show_origin;
        self
    }

    /// Set if events will show additional fields, usually the file or line.
    pub fn set_show_fields(&mut self, show_fields: bool) -> &mut Self {
        self.show_fields = show_fields;
        self
    }

    /// Set the base URL for origins. This can be used to show full file paths in the browser console.
    pub fn set_origin_base_url(&mut self, origin_base_url: impl ToString) -> &mut Self {
        self.origin_base_url = Some(origin_base_url.to_string());
        self
    }

    /// Set a JavaScript logger to use instead of console logging
    pub fn set_js_logger(&mut self, logger: JsLogger) -> &mut Self {
        self.js_logger = Some(Arc::new(SafeJsLogger::new(logger)));
        self
    }

    /// True if the console reporting spans
    pub fn console_enabled(&self) -> bool {
        self.console.reporting_enabled()
    }
}

#[test]
fn test_default_built_config() {
    let config = WasmLayerConfig::new();

    assert!(config.report_logs_in_timings);
    assert_eq!(config.console, ConsoleConfig::ReportWithConsoleColor);
    assert_eq!(config.max_level, None);
    assert!(config.show_fields);
    assert!(config.show_origin);
    assert_eq!(config.origin_base_url, None);
    assert!(config.js_logger.is_none());
}

#[test]
fn test_set_report_logs_in_timings() {
    let mut config = WasmLayerConfig::new();
    config.set_report_logs_in_timings(false);

    assert!(!config.report_logs_in_timings);
}

#[test]
fn test_set_console_config_no_reporting() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::NoReporting);

    assert!(!config.console.reporting_enabled());
}

#[test]
fn test_set_console_config_without_color() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithoutConsoleColor);

    assert_eq!(config.console, ConsoleConfig::ReportWithoutConsoleColor);
}

#[test]
fn test_set_console_config_with_color() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithConsoleColor);

    assert_eq!(config.console, ConsoleConfig::ReportWithConsoleColor);
}

#[test]
fn test_set_config_log_level_warn() {
    let mut config = WasmLayerConfig::new();
    config.set_max_level(tracing::Level::WARN);

    assert_eq!(config.max_level, Some(tracing::Level::WARN));
}

#[test]
fn test_update_log_filter() {
    let config = WasmLayerConfig::new();
    let filter = config.filter;
    assert_eq!(
        filter.read().max_level_hint(),
        Some(tracing::level_filters::LevelFilter::TRACE)
    );

    filter.update("info");

    assert_eq!(
        filter.max_level_hint(),
        Some(tracing::level_filters::LevelFilter::INFO)
    );
}
