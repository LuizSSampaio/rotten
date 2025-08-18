use log::error;

pub fn error(line: usize, message: String) {
    report(line, String::new(), message);
}

fn report(line: usize, location: String, message: String) {
    let message = format!("[line {}] Error{}: {}", line, location, message);

    error!("{}", message);
    print!("{}", message);
}
