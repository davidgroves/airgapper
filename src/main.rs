use clap::Parser;
use qrcodegen::{QrCode, QrCodeEcc};
use std::io::Read;

fn print_qr(qr: &QrCode) {
    let border = 4;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let c = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}

fn get_stdin_contents() -> String {
    let mut bytes = Vec::new();

    std::io::stdin()
        .read_to_end(&mut bytes)
        .expect("Failed to read from stdin");
    return String::from_utf8(bytes).expect("Invalid UTF-8");
}

fn get_file_contents(path: &str) -> String {
    let mut file = std::fs::File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read from file");
    return contents;
}

fn generate_qr(text: &str, error_correction: QrCodeEcc) -> QrCode {
    return QrCode::encode_text(text, error_correction).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_qr_medium() {
        let qr = generate_qr("Hello, world!", QrCodeEcc::Medium);
        assert_eq!(qr.size(), 21);
        assert_eq!(qr.get_module(0, 0), true);
        assert_eq!(qr.get_module(20, 0), true);
        assert_eq!(qr.get_module(10, 10), true);
        assert_eq!(qr.get_module(10, 12), false);
    }

    #[test]
    fn test_generate_qr_high() {
        let qr = generate_qr("Hello, world!", QrCodeEcc::High);
        assert_eq!(qr.size(), 25);
        assert_eq!(qr.get_module(0, 0), true);
        assert_eq!(qr.get_module(25, 0), false);
        assert_eq!(qr.get_module(9, 9), false);
        assert_eq!(qr.get_module(11, 15), true);
    }

    #[test]
    #[should_panic(expected = "Input too long: 2954 > 2953")]
    fn test_generate_qr_too_long() {
        let input = (0..2954).map(|_| "X").collect::<String>();
        check_input_length(&input);
    }
}

#[derive(clap::Parser)]
#[clap(value_enum)]
#[derive(Debug, Clone)]
enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl From<ErrorCorrectionLevel> for QrCodeEcc {
    fn from(level: ErrorCorrectionLevel) -> Self {
        match level {
            ErrorCorrectionLevel::Low => QrCodeEcc::Low,
            ErrorCorrectionLevel::Medium => QrCodeEcc::Medium,
            ErrorCorrectionLevel::Quartile => QrCodeEcc::Quartile,
            ErrorCorrectionLevel::High => QrCodeEcc::High,
        }
    }
}

impl From<ErrorCorrectionLevel> for String {
    fn from(level: ErrorCorrectionLevel) -> Self {
        match level {
            ErrorCorrectionLevel::Low => "low".to_string(),
            ErrorCorrectionLevel::Medium => "medium".to_string(),
            ErrorCorrectionLevel::Quartile => "quartile".to_string(),
            ErrorCorrectionLevel::High => "high".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional input file
    #[clap(short, long)]
    input: Option<String>,

    /// Error correction level to use
    #[clap(short, long, default_value = "medium", value_parser = ["low", "medium", "quartile", "high"])]
    ec: String,
}

fn parse_error_correction_level(s: &str) -> ErrorCorrectionLevel {
    match s {
        "low" => ErrorCorrectionLevel::Low,
        "medium" => ErrorCorrectionLevel::Medium,
        "quartile" => ErrorCorrectionLevel::Quartile,
        "high" => ErrorCorrectionLevel::High,
        _ => ErrorCorrectionLevel::Medium, // Default to Medium if invalid input
    }
}

fn check_input_length(input: &str) {
    if input.chars().count() > 2953 {
        panic!("Input too long: {} > 2953", input.chars().count());
    }
}

fn main() {
    let args = Args::parse();

    let input = match args.input {
        Some(path) => get_file_contents(&path),
        None => get_stdin_contents(),
    };

    check_input_length(&input);

    let ec_level = parse_error_correction_level(&args.ec);
    let qr = generate_qr(&input, ec_level.into());
    print_qr(&qr);
}
