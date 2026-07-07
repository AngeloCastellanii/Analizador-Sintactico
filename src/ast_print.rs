use crate::ast::*;
use soporte::structures::{Printable, Tree};

pub fn ast_a_arbol(programa: &Programa) -> Tree<String> {
    let mut arbol = Tree::with_root("Programa".to_string());
    for decl in &programa.declaraciones {
        if let Ok(raiz) = arbol.raiz_mut() {
            raiz.agregar_hijo(nodo_decl_externa(decl));
        }
    }
    arbol
}

fn nodo_decl_externa(decl: &DeclExterna) -> soporte::structures::tree::Node<String> {
    match decl {
        DeclExterna::Funcion(f) => {
            let mut n = soporte::structures::tree::Node::new(format!("Funcion: {}", nombre_decl(&f.declarador)));
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("Especificadores: {:?}", f.especificadores)));
            n.agregar_hijo(nodo_bloque(&f.cuerpo));
            n
        }
        DeclExterna::Declaracion(d) => {
            let mut n = soporte::structures::tree::Node::new("Declaracion".to_string());
            for init in &d.declaradores {
                n.agregar_hijo(soporte::structures::tree::Node::new(format!("{} {:?}", nombre_decl(&init.declarador), init.inicializador)));
            }
            n
        }
    }
}

fn nodo_bloque(b: &Bloque) -> soporte::structures::tree::Node<String> {
    let mut n = soporte::structures::tree::Node::new("Bloque".to_string());
    for item in &b.items {
        match item {
            ItemBloque::Declaracion(d) => {
                for init in &d.declaradores {
                    n.agregar_hijo(soporte::structures::tree::Node::new(format!("Decl: {:?}", init.declarador)));
                }
            }
            ItemBloque::Sentencia(s) => n.agregar_hijo(nodo_sentencia(s)),
        }
    }
    n
}

fn nodo_sentencia(s: &Sentencia) -> soporte::structures::tree::Node<String> {
    match s {
        Sentencia::Compuesta(b) => nodo_bloque(b),
        Sentencia::Expr { expr, .. } => soporte::structures::tree::Node::new(format!("ExprStmt: {:?}", expr)),
        Sentencia::Seleccion(Seleccion::If { condicion, entonces, sino, .. }) => {
            let mut n = soporte::structures::tree::Node::new("If".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n.agregar_hijo(nodo_sentencia(entonces));
            if let Some(s) = sino { n.agregar_hijo(nodo_sentencia(s)); }
            n
        }
        Sentencia::Seleccion(Seleccion::Switch { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("Switch".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n.agregar_hijo(nodo_sentencia(cuerpo));
            n
        }
        Sentencia::Iteracion(Iteracion::While { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("While".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n.agregar_hijo(nodo_sentencia(cuerpo));
            n
        }
        Sentencia::Iteracion(Iteracion::DoWhile { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("DoWhile".to_string());
            n.agregar_hijo(nodo_sentencia(cuerpo));
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n
        }
        Sentencia::Iteracion(Iteracion::For { init, condicion, incremento, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("For".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("init: {:?}", init)));
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("inc: {:?}", incremento)));
            n.agregar_hijo(nodo_sentencia(cuerpo));
            n
        }
        Sentencia::Salto(Salto::Retornar { valor, .. }) => soporte::structures::tree::Node::new(format!("Return: {:?}", valor)),
        Sentencia::Salto(Salto::Romper { .. }) => soporte::structures::tree::Node::new("Break".to_string()),
        Sentencia::Salto(Salto::Continuar { .. }) => soporte::structures::tree::Node::new("Continue".to_string()),
        Sentencia::Salto(Salto::Goto { etiqueta, .. }) => soporte::structures::tree::Node::new(format!("Goto: {}", etiqueta)),
        Sentencia::Etiquetada { etiqueta, sentencia, .. } => {
            let mut n = soporte::structures::tree::Node::new(format!("Etiqueta: {:?}", etiqueta));
            n.agregar_hijo(nodo_sentencia(sentencia));
            n
        }
    }
}

fn nombre_decl(d: &Declarador) -> String {
    nombre_base(&d.base)
}

fn nombre_base(b: &DeclaradorBase) -> String {
    match b {
        DeclaradorBase::Identificador(n) => n.clone(),
        DeclaradorBase::Agrupado(inner) => nombre_decl(inner),
        DeclaradorBase::Derivado { interno, .. } => nombre_base(interno),
    }
}

impl std::fmt::Display for Programa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ast_a_arbol(self).to_repr())
    }
}
