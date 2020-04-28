/// Name of app stage that runs once when an app starts up
pub const STARTUP: &str = "startup";

/// Name of app stage that runs before all other app stages
pub const FIRST: &str = "first";

/// Name of app stage that updates events. Generally this should run before UPDATE
pub const EVENT_UPDATE: &str = "event_update";

/// Name of app stage responsible for doing most app logic. Systems should be registered here by default.
pub const UPDATE: &str = "update";

/// Name of app stage responsible for processing the results of UPDATE. Runs immediately after UPDATE.
pub const POST_UPDATE: &str = "post_update";

/// Name of app stage that runs after all other app stages
pub const LAST: &str = "last";