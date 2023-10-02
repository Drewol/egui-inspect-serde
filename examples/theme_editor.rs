fn main() -> Result<(), eframe::Error> {
    eframe::run_simple_native(
        "Theme editor",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut style = (*ctx.style()).clone();
                    if let Some(err) = egui_inspect_serde::inspect_mut(&mut style, ui).err() {
                        println!("Error: {err}");
                    };

                    if style != **ui.style() {
                        ctx.set_style(style);
                    }
                });
            });
        },
    )
}
