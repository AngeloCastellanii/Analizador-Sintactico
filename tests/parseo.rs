use analizador_lexico::AnalizadorLexico;
use analizador_sintactico::AnalizadorSintactico;

#[test]
fn parsea_main_simple() {
    let src = "int main() { return 0; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let ast = AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect("debe parsear");
    assert_eq!(ast.declaraciones.len(), 1);
}

#[test]
fn parsea_if_y_operadores() {
    let src = "int main() { int x = 1; if (x > 0) { x++; } return x != 0; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect("debe parsear");
}

#[test]
fn parsea_archivo_ejemplo() {
    let ruta = concat!(env!("CARGO_MANIFEST_DIR"), "/ejemplos/ejemplo.c");
    let fuente = std::fs::read_to_string(ruta).expect("leer ejemplo");
    let tokens = AnalizadorLexico::desde_texto(&fuente).tokenizar();
    AnalizadorSintactico::con_fuente(tokens, &fuente)
        .parsear()
        .expect("debe parsear ejemplo.c");
}

#[test]
fn imprime_arbol_ramificado_con_comillas() {
    let src = "int main() { int x = 1; if (x > 0) { x++; } return x != 0; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let ast = AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect("debe parsear");

    let salida = ast.to_string();
    assert!(salida.contains("Decl: x"));
    assert!(salida.contains(">"));
    assert!(salida.contains("!="));
    assert!(salida.contains("++"));
    assert!(!salida.contains("cond: x > 0"));
    assert!(!salida.contains("Bloque vacío"));
}

#[test]
fn imprime_cadenas_con_comillas() {
    let src = r#"int main() { const char *msg = "Hola, mundo"; return 0; }"#;
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let ast = AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect("debe parsear");

    let salida = ast.to_string();
    assert!(salida.contains(r#""Hola, mundo""#));
}

#[test]
fn error_cancela_ast_y_muestra_origen() {
    let src = "int main() { return 0 }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let err = AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect_err("debe fallar sin punto y coma");

    let msg = err.to_string();
    assert!(msg.contains("se esperaba"));
    assert!(msg.contains("se encontró"));
    assert!(msg.contains("return 0"));
    assert!(msg.contains("^"));
}

#[test]
fn error_muestra_lexema_encontrado() {
    let src = "int main() { int x = ; }";
    let tokens = AnalizadorLexico::desde_texto(src).tokenizar();
    let err = AnalizadorSintactico::con_fuente(tokens, src)
        .parsear()
        .expect_err("debe fallar en expresión vacía");

    let msg = err.to_string();
    assert!(msg.contains("se encontró"));
    assert!(msg.contains(";"));
}
