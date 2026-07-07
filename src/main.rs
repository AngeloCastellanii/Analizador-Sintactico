use analizador_lexico::AnalizadorLexico;
use analizador_sintactico::AnalizadorSintactico;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: analizador_sintactico <archivo.c>");
        process::exit(1);
    }

    let ruta = &args[1];
    let tokens = match AnalizadorLexico::analizar_archivo(ruta) {
        Ok(t) => t,
        Err(e) => { eprintln!("Error lexico: {}", e); process::exit(1); }
    };

    let mut parser = AnalizadorSintactico::new(tokens);
    match parser.parsear() {
        Ok(ast) => {
            println!("AST generado correctamente\n");
            println!("{}", ast);
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

