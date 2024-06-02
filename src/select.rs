use std::fmt::Display;

use inquire::Select;

pub fn interactive_select<S: Display>(data: Vec<S>) -> Result<S, inquire::InquireError> {
    Select::new("Select Row", data).prompt()
}
