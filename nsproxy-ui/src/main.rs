#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use egui_node_graph_example::NodeGraphExample;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "nsproxy",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            #[cfg(feature = "persistence")]
            {
                Box::new(NodeGraphExample::new(cc))
            }
            #[cfg(not(feature = "persistence"))]
            Box::<NodeGraphExample>::default()
        }),
    )
    .expect("Failed to run native example");
}
