#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{RichText, FontId, Vec2};
use f64;
use cpython::{Python, PyDict, PyResult, PyErr};
use std::error::Error;

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(200.0, 260.0)),
        ..Default::default()
    };
    eframe::run_native(
        "",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    button_font: FontId,
    expression: String,
    ans: bool,
}

impl MyApp {
    fn add_val(&mut self, val: String) {
        if self.ans {
            self.expression = "".to_string();
            self.ans = false;
        }

        println!("{}", &self.expression);
        match val.as_str() {
            "C" => {
                self.expression = "".to_string();
            }
            
            exp @ ("=" | "√") => {
                if self.expression.len() == 0 {
                    return;
                }
                self.expression = self.expression.replace("÷", "/").replace("x", "*");
                let gil = Python::acquire_gil();
                for c in ["*", "-", "+"] {
                    if self.expression.chars().last().unwrap().to_string() == c {
                        self.expression = "Syntax error!".to_string();
                        self.ans = true;
                        return;
                    }
                    self.expression = self.expression.trim_end_matches(c).to_string();
                }
                for c in ["*", "+"] {
                    if self.expression.chars().nth(0).unwrap().to_string() == c {
                        self.expression = "Syntax error!".to_string();
                        self.ans = true;
                        return;
                    }
                    self.expression = self.expression.trim_start_matches(c).to_string();
                }
                
                let val = {
                if self.expression.contains("/") || self.expression.contains(".") {
                    match eval_float(gil.python(), &self.expression) {
                        Ok(val) => {
                            val.to_string()
                        }
                        _ => {
                            "Error".to_string()
                        }
                    }
                } else {
                    match eval(gil.python(), &self.expression) {
                        Ok(val) => {
                            val.to_string()
                        } 
                        _ => {
                            "Error".to_string()
                        }
                }}};

                match exp {
                    "=" => {
                        self.expression = val;
                    } 
                    "√" => {
                        self.expression = f64::sqrt(val.parse::<f64>().unwrap()).to_string();
                    }
                    _ => {}
                }
                self.ans = true;
            }
            _ => {
                self.expression += &val;
            }
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            button_font: FontId::proportional(30.0),
            expression: "".to_string(),
            ans: false
        }
    }
}

fn eval(py: Python, exp: &String) -> Result<i64, PyErr> {
    let val = py.eval(&exp, None, None)?;
    println!("Val: {}", val);
    val.extract(py)
}

fn eval_float(py: Python, exp: &String) -> Result<f64, PyErr> {
    let val = py.eval(&exp, None, None)?;
    println!("Val: {}", val);
    val.extract(py)
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
            ui.add(egui::widgets::Label::new(egui::RichText::new(&self.expression).size(30.0)).wrap(false));
            ui.horizontal(|ui| {
                let vals = ["C", "√"];
                for val in vals {
                    if ui.add_sized([40.0, 40.0], egui::widgets::Button::new(egui::RichText::new(val.to_string()).font(self.button_font.clone()))).clicked() {
                    self.add_val(val.to_string());
                    };
                }
            });
            for i in 0..3 {
                    ui.horizontal(|ui| {
                        for j in 1..4 {
                            let val = i*3+j;
                            if ui.add_sized([40.0, 40.0], egui::widgets::Button::new(egui::RichText::new(&val.to_string()).font(self.button_font.clone()))).clicked() {
                                self.add_val(val.to_string());
                            };
                        }
                        let val: Option<String>;
                        match i {
                            0 => {
                                val = Some("÷".to_string());
                            }
                            1 => {
                                val = Some("x".to_string());
                            }
                            2 => {
                                val = Some("-".to_string());
                                
                            }
                            _=> { val = None}
                        }
                        match val {
                            Some(v) => {
                                if ui.add_sized([40.0, 40.0], egui::widgets::Button::new(egui::RichText::new(&v).font(self.button_font.clone()))).clicked() {
                                    self.add_val(v);
                                };
                            } 
                            None => {

                            }
                            
                        }
                        
                    });
                };
            ui.horizontal(|ui| {
                let vals = ["0", ".", "=", "+"];
                for val in vals {
                    if ui.add_sized([40.0, 40.0], egui::widgets::Button::new(egui::RichText::new(val.to_string()).font(self.button_font.clone()))).clicked() {
                    self.add_val(val.to_string());
                    };
                }
            });
        });
        });
    }
}