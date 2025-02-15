#[doc(hidden)]
mod console;
pub use console::*;

#[deprecated(since = "1.0.0", note = "Rename WASMLayerConfig to WasmLayerConfig.")]
pub type WASMLayerConfig = WasmLayerConfig;

///Configuration parameters for the [WasmLayer](crate::prelude::WasmLayer).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WasmLayerConfig {
    /// In dev-tools, report timings of traces
    pub report_logs_in_timings: bool,
    /// See [ConsoleConfig]
    pub console: ConsoleConfig,
    /// Maximum log level
    pub max_level: tracing::Level,
    /// Show/hide fields of types
    pub show_fields: bool,
    /// Show origin (line number, source)
    pub show_origin: bool,
    /// Optional URL to prepend to origins. E.g. to allow for showing full file paths that can be navigated when logged in the browser console.
    pub origin_base_url: Option<String>,
}

impl Default for WasmLayerConfig {
    fn default() -> Self {
        WasmLayerConfig {
            report_logs_in_timings: true,
            console: ConsoleConfig::ReportWithConsoleColor,
            max_level: tracing::Level::TRACE,
            show_fields: true,
            show_origin: true,
            origin_base_url: None,
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
        self.max_level = max_level;
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

    /// True if the console reporting spans
    pub fn console_enabled(&self) -> bool {
        self.console.reporting_enabled()
    }
}

#[test]
fn test_default_built_config() {
    let config = WasmLayerConfig::new();

    assert_eq!(
        config,
        WasmLayerConfig {
            report_logs_in_timings: true,
            console: ConsoleConfig::ReportWithConsoleColor,
            max_level: tracing::Level::TRACE,
            show_fields: true,
            show_origin: true,
            origin_base_url: None
        }
    )
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

    assert_eq!(config.max_level, tracing::Level::WARN);
}
