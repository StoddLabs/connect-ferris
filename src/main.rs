use eframe::egui;
use egui::{Response, Vec2};
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
    turn: i32,

}

struct BoardSlot {
    id: i32,
    x_coordinate: i32,
    y_coordinate: i32,
    slot_value: String,


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
                    slot_value: String::from("  "),
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
            turn: 0,
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
                        let mut curr = self.board_layout.get_mut(curr_count).unwrap();
                        let b = egui::Button::new(&curr.slot_value).min_size(Vec2::new(50.0, 50.0));
                        if ui
                            //.button(&curr.slot_value)
                            .add(b)
                            .on_hover_text(format!(
                                "x: {}, y: {}",
                                curr.x_coordinate, curr.y_coordinate
                            ))
                            .clicked()
                        {
                            println!("{},{}", curr.x_coordinate, curr.y_coordinate);
                            if self.turn == 0 {
                                curr.slot_value = String::from("X");
                                self.turn = 1;
                            } else {
                                curr.slot_value = String::from("O");
                                self.turn = 0;
                            }


                        };
                        curr_count += 1;
                    }
                });
            }
        });
    }
}
