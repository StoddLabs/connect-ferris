use eframe::egui;
use egui::Response;
use egui::WidgetText::RichText;
use egui::WidgetType::Button;
use std::borrow::{Borrow, BorrowMut};

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

struct Board {
    board_layout: Vec<BoardSlot>,

}

struct BoardSlot {
    id: i32,
    x_coordinate: i32,
    y_coordinate: i32,
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Board::new();
        let mut curr_id = 0;
        for y_cord in 0..9 {
            for x_cord in 0..9 {
                let bl = BoardSlot {
                    id: curr_id,
                    x_coordinate: x_cord,
                    y_coordinate: y_cord,
                };
                curr_id += 1;
                b.board_layout.push(bl);
            }
        }
        b
    }
}

impl Board {
    fn new() -> Self {
        Board {
            board_layout: Vec::new(),
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            let mut curr_count = 0;
            for _ in 0..9 {
                ui.horizontal(|ui| {
                    for _ in 0..9 {
                        let curr = self.board_layout.get(curr_count).unwrap();
                        if ui
                            .button("|      |")
                            .on_hover_text(format!(
                                "x: {}, y: {}",
                                curr.x_coordinate, curr.y_coordinate
                            ))
                            .clicked()
                        {
                            println!("{},{}", curr.x_coordinate, curr.y_coordinate);
                        };
                        curr_count += 1;
                    }
                });
            }
        });
    }
}
