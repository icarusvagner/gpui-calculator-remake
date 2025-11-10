#[derive(Debug, Clone)]
pub enum Operation {
    Division,
    Multiply,
    Addition,
    Subtract,
}

#[derive(Debug, Clone)]
pub enum ButtonType {
    Reset,
    Sign,
    Percent,
    Comma,
    Equal,
    Number(u8),
    Arithmetic(Operation),
}

pub struct Logic {
    first_value: f64,
    second_value: Option<f64>,
    operation: Option<Operation>,
    use_comma: bool,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            first_value: 0.,
            second_value: None,
            operation: None,
            use_comma: false,
        }
    }

    pub fn handle_key_input(&mut self, key_input: &str) {
        if let Ok(num) = key_input.parse::<u8>() {
            self.on_button_pressed(ButtonType::Number(num));
        } else {
            match key_input {
                "/" => self.on_button_pressed(ButtonType::Arithmetic(Operation::Division)),
                "*" => self.on_button_pressed(ButtonType::Arithmetic(Operation::Multiply)),
                "-" => self.on_button_pressed(ButtonType::Arithmetic(Operation::Subtract)),
                "+" => self.on_button_pressed(ButtonType::Arithmetic(Operation::Addition)),
                "enter" => self.on_button_pressed(ButtonType::Equal),
                "=" => self.on_button_pressed(ButtonType::Equal),
                "," => self.on_button_pressed(ButtonType::Comma),
                "." => self.on_button_pressed(ButtonType::Comma),
                "%" => self.on_button_pressed(ButtonType::Percent),
                "backspace" => self.on_button_pressed(ButtonType::Reset),
                _ => {}
            }
        }
    }

    pub fn get_display_value(&self) -> f64 {
        self.second_value.unwrap_or(self.first_value)
    }

    pub fn on_button_pressed(&mut self, button_type: ButtonType) {
        match button_type {
            ButtonType::Reset => self.set_result(0.),
            ButtonType::Sign => self.change_current_value(&|n| n * -1.),
            ButtonType::Percent => self.change_current_value(&|n| n / 100.),
            ButtonType::Comma => self.use_comma = true,
            ButtonType::Equal => self.get_result(),
            ButtonType::Number(num) => self.add_number(num),
            ButtonType::Arithmetic(operation) => {
                self.use_comma = false;
                self.operation = Some(operation);
            }
        }
    }

    fn append_digit(&self, value: f64, digit: u8) -> f64 {
        let digit_str = digit.to_string();
        let mut value_str = value.to_string();

        if !value_str.contains('.') & self.use_comma {
            value_str.push('.');
        }

        value_str.push_str(&digit_str);
        value_str.parse::<f64>().unwrap()
    }

    fn change_current_value(&mut self, f: &dyn Fn(f64) -> f64) {
        self.set_result(f(self.get_display_value()));
    }

    fn get_result(&mut self) {
        if let (Some(op), Some(second_value)) = (self.operation.clone(), self.second_value) {
            let result = match op {
                Operation::Division => self.first_value / second_value,
                Operation::Multiply => self.first_value * second_value,
                Operation::Addition => self.first_value + second_value,
                Operation::Subtract => self.first_value - second_value,
            };

            self.set_result(result);
        }
    }

    fn set_result(&mut self, n: f64) {
        self.first_value = n;
        self.second_value = None;
        self.operation = None;
        self.use_comma = n % 1.0 != 0.;
    }

    fn add_number(&mut self, n: u8) -> () {
        match self.operation {
            Some(_) => match self.second_value {
                Some(value) => self.second_value = Some(self.append_digit(value, n)),
                None => self.second_value = Some(n as f64),
            },
            None => self.first_value = self.append_digit(self.first_value, n),
        }
    }
}
