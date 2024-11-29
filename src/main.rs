use eframe::egui::{self, CentralPanel, Context, TextEdit, TopBottomPanel};

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
    error_message: Option<String>,
    history: Vec<String>,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Калькулятор на Rust");
        });

        CentralPanel::default().show(ctx, |ui| {
            // Ввід виразу
            ui.horizontal(|ui| {
                ui.label("Введіть вираз:");
                ui.add(TextEdit::singleline(&mut self.expression).hint_text("Наприклад, 4 + 5"));
            });

            // Кнопка обчислення
            if ui.button("Обчислити").clicked() {
                self.calculate_result();
            }

            // Відображення результату
            if let Some(result) = self.result {
                ui.label(format!("Результат: {}", result));
            } else {
                ui.label("Результат: -");
            }

            // Відображення повідомлення про помилку
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }

            // Відображення історії
            if !self.history.is_empty() {
                ui.separator();
                ui.label("Історія обчислень:");
                for (i, entry) in self.history.iter().enumerate() {
                    ui.label(format!("{}. {}", i + 1, entry));
                }
            }
        });
    }
}

impl CalculatorApp {
    fn calculate_result(&mut self) {
        self.error_message = None;

        let tokens: Vec<&str> = self.expression.split_whitespace().collect();
        if tokens.len() != 3 {
            self.error_message =
                Some("Неправильний формат. Використовуйте: число оператор число".to_string());
            self.result = None;
            return;
        }

        let num1: f64 = match tokens[0].parse() {
            Ok(n) => n,
            Err(_) => {
                self.error_message = Some("Перше значення не є числом".to_string());
                return;
            }
        };

        let num2: f64 = match tokens[2].parse() {
            Ok(n) => n,
            Err(_) => {
                self.error_message = Some("Друге значення не є числом".to_string());
                return;
            }
        };

        self.result = match tokens[1] {
            "+" => Some(num1 + num2),
            "-" => Some(num1 - num2),
            "*" => Some(num1 * num2),
            "/" => {
                if num2 == 0.0 {
                    self.error_message = Some("Ділення на нуль".to_string());
                    None
                } else {
                    Some(num1 / num2)
                }
            }
            _ => {
                self.error_message =
                    Some("Невідомий оператор. Використовуйте +, -, *, /".to_string());
                None
            }
        };

        // Якщо немає помилок, додаємо вираз і результат до історії
        if let Some(result) = self.result {
            let entry = format!("{} = {}", self.expression, result);
            self.history.push(entry);
            if self.history.len() > 3 {
                self.history.remove(0); // Зберігаємо тільки останні три результати
            }
        }
    }
}
