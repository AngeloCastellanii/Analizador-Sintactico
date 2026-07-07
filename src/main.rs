use analizador_lexico::AnalizadorLexico;
use analizador_sintactico::AnalizadorSintactico;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: analizador_sintactico <archivo.c>");
        process::exit(1);
    }

    let ruta = &args[1];
    let fuente = match fs::read_to_string(ruta) {
        Ok(contenido) => contenido,
        Err(e) => {
            eprintln!("No se pudo leer el archivo '{}': {}", ruta, e);
            process::exit(1);
        }
    };

    let tokens = AnalizadorLexico::desde_texto(&fuente).tokenizar();
    let mut parser = AnalizadorSintactico::con_fuente(tokens, &fuente);

    match parser.parsear() {
        Ok(ast) => {
            println!("AST generado correctamente\n");
            println!("{}", ast);
        }
        Err(e) => {
            eprint!("{}", e);
            process::exit(1);
        }
    }
}
