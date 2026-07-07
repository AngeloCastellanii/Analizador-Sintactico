use analizador_lexico::AnalizadorLexico;
use analizador_sintactico::AnalizadorSintactico;

#[test]
fn parsea_main_simple() {
    let src = "int main() { return 0; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let ast = AnalizadorSintactico::new(tokens).parsear().expect("debe parsear");
    assert_eq!(ast.declaraciones.len(), 1);
}

#[test]
fn parsea_if_y_operadores() {
    let src = "int main() { int x = 1; if (x > 0) { x++; } return x != 0; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    AnalizadorSintactico::new(tokens).parsear().expect("debe parsear");
}

#[test]
fn parsea_archivo_ejemplo() {
    let ruta = concat!(env!("CARGO_MANIFEST_DIR"), "/ejemplos/ejemplo.c");
    let tokens = AnalizadorLexico::analizar_archivo(ruta).expect("lexico ok");
    AnalizadorSintactico::new(tokens).parsear().expect("debe parsear ejemplo.c");
}
