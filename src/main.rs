use std::borrow::Borrow;
use eframe::egui;
use egui::Response;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(Board::default())),
    )
}

#[derive(Clone)]
struct Board {
    board_layout: Vec<BoardSlot>,
}
#[derive(Clone)]
struct BoardSlot {
    x_coordinate: i32,
    y_coordinate: i32,
    button: Response,
}


impl Default for Board {
    fn default() -> Self {
        Self {
            board_layout: Vec::new(),
        }
    }
}

impl eframe::App for Board {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            for y_cord in 0..9 {
                    ui.horizontal(|ui| {
                        for x_cord in 0..9 {
                            let b = BoardSlot{x_coordinate: x_cord, y_coordinate: y_cord, button: ui.button("| |")};
                            self.board_layout.push(b.clone());
                            if b.clone().button.clicked() {
                               println!("{},{}", b.borrow().x_coordinate, b.borrow().y_coordinate);
                            }
                        }
                    });
                }
        });


    }
}
