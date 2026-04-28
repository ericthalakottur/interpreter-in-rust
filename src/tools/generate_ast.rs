use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} <output_directory>", args[1]);
    }

    let expressions: Vec<&str> = vec![
        "Binary : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal : Object value",
        "Unary : Token operator, Expr right",
    ];

    let _ = fs::write(args[1].clone() + "/expr.rs", generate_ast(&expressions));
    Ok(())
}

fn generate_ast(expressions: &Vec<&str>) -> String {
    let mut ast = String::new();
    for expression in expressions {
        let t: Vec<&str> = expression.split(":").collect();
        let class_name: &str = t[0].trim();
        let fields: &str = t[1].trim();
        ast += &generate_type_string(class_name, fields);
    }
    ast
}

fn generate_type_string(class_name: &str, fields_str: &str) -> String {
    let mut type_string = String::new();
    let fields: Vec<&str> = fields_str.split(",").collect();
    type_string += &format!("pub struct {} {{\n", class_name).to_string();
    for i in 0..fields.len() {
        let t: Vec<&str> = fields[i].trim().split(" ").collect();
        let datatype = t[0].trim();
        let variable_name = t[1].trim();
        type_string += &format!("\t{}: {}", variable_name, datatype).to_string();
        if i < fields.len() - 1 {
            type_string += ","
        }
        type_string += "\n"
    }
    type_string += &format!("}}\n").to_string();
    type_string
}
