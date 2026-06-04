use egui::Key::F;
use egui_plot::{Heatmap, Line, Plot, PlotPoints};
use astroimsim_geometry::grid2d::PlotPoint;

pub fn run(plots:Vec<Vec<Vec<f64>>>){
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(PlotWindow::new(cc,plots)))));
}


pub fn display(matrix:Vec<Vec<f64>>){
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(DetectorWindow::new(cc,matrix)))));


}



#[derive(Default)]
pub struct PlotWindow {
    plots: Vec<Vec<Vec<f64>>>
}

pub struct DetectorWindow{
    matrix:Vec<Vec<f64>>
}



impl PlotWindow {
    fn new(cc: &eframe::CreationContext<'_>,plots:Vec<Vec<Vec<f64>>>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        PlotWindow{
            plots
        }
    }
}

impl DetectorWindow{
    fn new(cc: &eframe::CreationContext<'_>,matrix:Vec<Vec<f64>>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        DetectorWindow{
            matrix
        }
    }

}


impl eframe::App for DetectorWindow {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {


            let cols = self.matrix[0].len();


            let heatmap = Heatmap::new(self.matrix.iter().flatten().map(|a|*a).collect(), cols)
                .show_labels(false);


            Plot::new("Heatmap Demo")
                .allow_zoom(false)
                .allow_scroll(false)
                .allow_drag(false)
                .allow_axis_zoom_drag(false)
                .allow_boxed_zoom(false)
                .show(ui, |plot_ui| {
                    plot_ui.heatmap(heatmap);
                })
                .response





        });
    }
}


impl eframe::App for PlotWindow {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let height = ui.available_height();

            for plot in &self.plots {
                ui.horizontal(|ui|{
                    let graph: PlotPoints = plot.iter()
                        .map(|i| {
                            [i[0] as f64,i[1] as f64]
                        })
                        .collect();
                    let line = Line::new("sin", graph);
                    Plot::new("my_plot")
                        .view_aspect(2.0)
                        .height(height as f32/(self.plots.len() as f32))
                        .allow_scroll(false)
                        .show(ui, |plot_ui| plot_ui.line(line));

                });

            }





        });
    }
}