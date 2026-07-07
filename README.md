# Analizador Sintáctico (C-like)

Analizador sintáctico en Rust que consume la salida del [analizador léxico](https://github.com/AngeloCastellanii/Analizador-Lexico) y genera un **árbol sintáctico abstracto (AST)** para un subconjunto de C.

## Requisitos

- [Rust](https://www.rust-lang.org/) (edition 2021)
- Proyectos hermanos en el mismo directorio padre:
  - `Analizador Lexico` → dependencia `analizador_lexico`
  - `soporte` → librería de utilidades del curso

Estructura esperada:

```
Diseño de compiladores/
├── Analizador Lexico/
├── Analizador sintactico/   ← este repo
└── soporte/
```

## Uso

```bash
cargo run -- ejemplos/ejemplo.c
cargo test
```

## Flujo

1. **Léxico:** el archivo `.c` se tokeniza (`Vec<Token>`).
2. **Sintáctico:** el parser construye el AST (`Programa`).
3. **Salida:** se imprime el árbol en consola.

## Estructura del proyecto

| Archivo | Descripción |
|---------|-------------|
| `src/ast.rs` | Definición de nodos del AST |
| `src/parser.rs` | Parser por descenso recursivo |
| `src/ast_print.rs` | Visualización del árbol |
| `src/main.rs` | CLI |
| `ejemplos/ejemplo.c` | Programa de prueba |

## Curso

Proyecto de **Diseño de Compiladores**.
