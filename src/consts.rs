use crate::logic::{ButtonType, Operation};

pub const BUTTONS: [ButtonType; 19] = [
    ButtonType::Reset,
    ButtonType::Sign,
    ButtonType::Percent,
    ButtonType::Arithmetic(Operation::Division),
    ButtonType::Number(7),
    ButtonType::Number(8),
    ButtonType::Number(9),
    ButtonType::Arithmetic(Operation::Multiply),
    ButtonType::Number(4),
    ButtonType::Number(5),
    ButtonType::Number(6),
    ButtonType::Arithmetic(Operation::Subtract),
    ButtonType::Number(3),
    ButtonType::Number(2),
    ButtonType::Number(1),
    ButtonType::Arithmetic(Operation::Addition),
    ButtonType::Number(0),
    ButtonType::Comma,
    ButtonType::Equal,
];
