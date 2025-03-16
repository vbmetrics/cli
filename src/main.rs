use parser::parser::parse_line;

fn main() {
    let parsed: parser::parser::ParsedAction = parse_line("L09S-1A6AFXT%Ngapeth Paris first serve");
    println!("{:?}", parsed);
}
