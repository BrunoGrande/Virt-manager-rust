//! GTK4 desktop GUI entry point.
//!
//! Screen tickets resolved so far (`.scratch/rust-conversion/issues/`):
//! 08 (Create VM wizard), 10 (VM details window), 11 (Snapshot
//! management), 12 (Add Hardware wizard), 13 (VM Manager main window),
//! 14 (console-viewer specifics). Remaining screens: see the map's
//! "Not yet specified" section — ticket 07's inventory has the full list.
//!
//! The shared async-job wrapper (ticket 09) lives in this crate, not
//! `virtinst-core` — it shows a modal, `virtinst-core` has no GTK
//! dependency at all.

fn main() {
    // TODO: gtk4::Application bootstrap, LibvirtGLib event-loop hookup
    // (ticket 09), vmmEngine-equivalent app lifecycle.
}
