#![cfg(target_os = "macos")]

use std::convert::From;
use std::os::raw::c_void;
use cocoa::appkit::NSApplicationActivationPolicy;
use {MonitorId, Window, WindowBuilder};
use platform::EventsLoop;

/// Additional methods on `EventsLoop` that are specific to Linux.
pub trait EventsLoopExt {
    fn set_alt_as_meta(&mut self, bool);
}

impl EventsLoopExt for EventsLoop {
    fn set_alt_as_meta(&mut self, value: bool) {
        self.alt_as_meta = value;
    }
}

/// Additional methods on `Window` that are specific to MacOS.
pub trait WindowExt {
    /// Returns a pointer to the cocoa `NSWindow` that is used by this window.
    ///
    /// The pointer will become invalid when the `Window` is destroyed.
    fn get_nswindow(&self) -> *mut c_void;

    /// Returns a pointer to the cocoa `NSView` that is used by this window.
    ///
    /// The pointer will become invalid when the `Window` is destroyed.
    fn get_nsview(&self) -> *mut c_void;
}

impl WindowExt for Window {
    #[inline]
    fn get_nswindow(&self) -> *mut c_void {
        self.window.get_nswindow()
    }

    #[inline]
    fn get_nsview(&self) -> *mut c_void {
        self.window.get_nsview()
    }
}

/// Corresponds to `NSApplicationActivationPolicy`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationPolicy {
    /// Corresponds to `NSApplicationActivationPolicyRegular`.
    Regular,
    /// Corresponds to `NSApplicationActivationPolicyAccessory`.
    Accessory,
    /// Corresponds to `NSApplicationActivationPolicyProhibited`.
    Prohibited,
}

impl Default for ActivationPolicy {
    fn default() -> Self {
        ActivationPolicy::Regular
    }
}

impl From<ActivationPolicy> for NSApplicationActivationPolicy {
    fn from(activation_policy: ActivationPolicy) -> Self {
        match activation_policy {
            ActivationPolicy::Regular =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            ActivationPolicy::Accessory =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
            ActivationPolicy::Prohibited =>
                NSApplicationActivationPolicy::NSApplicationActivationPolicyProhibited,
        }
    }
}

/// Additional methods on `WindowBuilder` that are specific to MacOS.
pub trait WindowBuilderExt {
    fn with_activation_policy(self, activation_policy: ActivationPolicy) -> WindowBuilder;
}

impl WindowBuilderExt for WindowBuilder {
    /// Sets the activation policy for the window being built
    #[inline]
    fn with_activation_policy(mut self, activation_policy: ActivationPolicy) -> WindowBuilder {
        self.platform_specific.activation_policy = activation_policy;
        self
    }
}

/// Additional methods on `MonitorId` that are specific to MacOS.
pub trait MonitorIdExt {
    /// Returns the identifier of the monitor for Cocoa.
    fn native_id(&self) -> u32;
}

impl MonitorIdExt for MonitorId {
    #[inline]
    fn native_id(&self) -> u32 {
        self.inner.get_native_identifier()
    }
}
