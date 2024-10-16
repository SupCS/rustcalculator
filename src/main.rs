use eframe::egui::{self, CentralPanel, Context, TextEdit, TopBottomPanel};
use meval::eval_str;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Калькулятор на Rust",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    )
}

#[derive(Default)]
struct CalculatorApp {
    expression: String,
    result: Option<f64>,
    previous_results: Vec<f64>, // Зберігає до 3 попередніх результатів
    error_message: Option<String>,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Калькулятор на Rust");
        });

        CentralPanel::default().show(ctx, |ui| {
            // Відображення попередніх результатів
            if !self.previous_results.is_empty() {
                ui.label("Попередні результати:");
                for (i, &res) in self.previous_results.iter().enumerate() {
                    ui.label(format!("{}. {}", i + 1, res));
                }
            } else {
                ui.label("Попередні результати: -");
            }

            ui.horizontal(|ui| {
                ui.label("Введіть вираз:");
                ui.add(
                    TextEdit::singleline(&mut self.expression).hint_text("Наприклад, 4+5*(2-3)"),
                );
            });

            if ui.button("Обчислити").clicked() {
                self.calculate_result();
            }

            // Відображення результату
            if let Some(result) = self.result {
                ui.label(format!("Результат: {}", result));
            } else {
                ui.label("Результат: -");
            }

            // Відображення повідомлення про помилку, якщо є
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }
}

impl CalculatorApp {
    fn calculate_result(&mut self) {
        self.error_message = None; // Скидання попередньої помилки

        // Спроба обчислення виразу
        match eval_str(&self.expression) {
            Ok(res) => {
                self.result = Some(res);
                self.add_to_previous_results(res); // Додавання результату до списку попередніх
            }
            Err(e) => {
                self.error_message = Some(format!("Помилка: {}", e));
                self.result = None;
            }
        }
    }

    fn add_to_previous_results(&mut self, result: f64) {
        // Додаємо новий результат у вектор
        self.previous_results.push(result);
        // Якщо кількість результатів перевищує 3, видаляємо найстаріший
        if self.previous_results.len() > 3 {
            self.previous_results.remove(0);
        }
    }
}
