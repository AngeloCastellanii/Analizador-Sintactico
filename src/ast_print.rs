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
            let mut n = soporte::structures::tree::Node::new(format!("Función: {}", fmt_declarador(&f.declarador)));
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("Especificadores: {}", fmt_especificadores(&f.especificadores))));
            if let Some(cuerpo) = nodo_bloque_opt(&f.cuerpo) {
                n.agregar_hijo(cuerpo);
            }
            n
        }
        DeclExterna::Declaracion(d) => {
            let mut n = soporte::structures::tree::Node::new(format!("Declaración: {}", fmt_especificadores(&d.especificadores)));
            for init in &d.declaradores {
                n.agregar_hijo(soporte::structures::tree::Node::new(format!("Decl: {}", fmt_init_declarador(init))));
            }
            n
        }
    }
}

fn nodo_bloque_opt(b: &Bloque) -> Option<soporte::structures::tree::Node<String>> {
    if b.items.is_empty() {
        return None;
    }

    let mut n = soporte::structures::tree::Node::new("Bloque".to_string());
    for item in &b.items {
        match item {
            ItemBloque::Declaracion(d) => {
                for init in &d.declaradores {
                    n.agregar_hijo(soporte::structures::tree::Node::new(format!("Decl: {}", fmt_init_declarador(init))));
                }
            }
            ItemBloque::Sentencia(s) => {
                if let Some(hijo) = nodo_sentencia(s) {
                    n.agregar_hijo(hijo);
                }
            }
        }
    }
    Some(n)
}

fn nodo_sentencia(s: &Sentencia) -> Option<soporte::structures::tree::Node<String>> {
    match s {
<<<<<<< Updated upstream
        Sentencia::Compuesta(b) => nodo_bloque_opt(b),
        Sentencia::Expr { expr, .. } => Some(soporte::structures::tree::Node::new(format!("ExprStmt: {}", fmt_expr_opt(expr.as_deref())))),
        Sentencia::Seleccion(Seleccion::If { condicion, entonces, sino, .. }) => {
            let mut n = soporte::structures::tree::Node::new("If".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {}", fmt_expr(condicion))));
            if let Some(then_node) = nodo_sentencia(entonces) {
                n.agregar_hijo(then_node);
            }
            if let Some(s) = sino {
                if let Some(else_node) = nodo_sentencia(s) {
                    n.agregar_hijo(else_node);
                }
            }
            Some(n)
=======
        Sentencia::Compuesta(b) => nodo_bloque(b),
        Sentencia::Expr { expr, .. } => {
            soporte::structures::tree::Node::new(format!("ExprStmt: {:?}", expr))
        }
        Sentencia::Seleccion(Seleccion::If { condicion, entonces, sino, .. }) => {
            let mut n = soporte::structures::tree::Node::new("If".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {:?}", condicion)));
            n.agregar_hijo(nodo_sentencia(entonces));
            if let Some(s) = sino {
                n.agregar_hijo(nodo_sentencia(s));
            }
            n
>>>>>>> Stashed changes
        }
        Sentencia::Seleccion(Seleccion::Switch { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("Switch".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {}", fmt_expr(condicion))));
            if let Some(cuerpo_node) = nodo_sentencia(cuerpo) {
                n.agregar_hijo(cuerpo_node);
            }
            Some(n)
        }
        Sentencia::Iteracion(Iteracion::While { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("While".to_string());
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {}", fmt_expr(condicion))));
            if let Some(cuerpo_node) = nodo_sentencia(cuerpo) {
                n.agregar_hijo(cuerpo_node);
            }
            Some(n)
        }
        Sentencia::Iteracion(Iteracion::DoWhile { condicion, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("DoWhile".to_string());
            if let Some(cuerpo_node) = nodo_sentencia(cuerpo) {
                n.agregar_hijo(cuerpo_node);
            }
            n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {}", fmt_expr(condicion))));
            Some(n)
        }
        Sentencia::Iteracion(Iteracion::For { init, condicion, incremento, cuerpo, .. }) => {
            let mut n = soporte::structures::tree::Node::new("For".to_string());
            if let Some(init_expr) = init {
                n.agregar_hijo(soporte::structures::tree::Node::new(format!("init: {}", fmt_for_init(init_expr))));
            }
            if let Some(cond) = condicion {
                n.agregar_hijo(soporte::structures::tree::Node::new(format!("cond: {}", fmt_expr(cond))));
            }
            if let Some(inc) = incremento {
                n.agregar_hijo(soporte::structures::tree::Node::new(format!("inc: {}", fmt_expr(inc))));
            }
            if let Some(cuerpo_node) = nodo_sentencia(cuerpo) {
                n.agregar_hijo(cuerpo_node);
            }
            Some(n)
        }
<<<<<<< Updated upstream
        Sentencia::Salto(Salto::Retornar { valor, .. }) => Some(soporte::structures::tree::Node::new(format!("Return: {}", fmt_expr_opt(valor.as_deref())))),
        Sentencia::Salto(Salto::Romper { .. }) => Some(soporte::structures::tree::Node::new("Break".to_string())),
        Sentencia::Salto(Salto::Continuar { .. }) => Some(soporte::structures::tree::Node::new("Continue".to_string())),
        Sentencia::Salto(Salto::Goto { etiqueta, .. }) => Some(soporte::structures::tree::Node::new(format!("Goto: {}", etiqueta))),
=======
        Sentencia::Salto(Salto::Retornar { valor, .. }) => {
            soporte::structures::tree::Node::new(format!("Return: {:?}", valor))
        }
        Sentencia::Salto(Salto::Romper { .. }) => soporte::structures::tree::Node::new("Break".to_string()),
        Sentencia::Salto(Salto::Continuar { .. }) => soporte::structures::tree::Node::new("Continue".to_string()),
        Sentencia::Salto(Salto::Goto { etiqueta, .. }) => {
            soporte::structures::tree::Node::new(format!("Goto: {}", etiqueta))
        }
>>>>>>> Stashed changes
        Sentencia::Etiquetada { etiqueta, sentencia, .. } => {
            let mut n = soporte::structures::tree::Node::new(format!("Etiqueta: {}", fmt_etiqueta(etiqueta)));
            if let Some(hijo) = nodo_sentencia(sentencia) {
                n.agregar_hijo(hijo);
            }
            Some(n)
        }
    }
}

fn fmt_especificadores(e: &Especificadores) -> String {
    let mut partes = Vec::new();
    for a in &e.almacenamiento {
        partes.push(fmt_almacenamiento(a));
    }
    for c in &e.calificadores {
        partes.push(fmt_calificador(c));
    }
    if let Some(tipo) = &e.tipo {
        partes.push(fmt_tipo(tipo));
    }
    partes.join(" ")
}

fn fmt_almacenamiento(a: &Almacenamiento) -> String {
    match a {
        Almacenamiento::Auto => "auto".to_string(),
        Almacenamiento::Register => "register".to_string(),
        Almacenamiento::Static => "static".to_string(),
        Almacenamiento::Extern => "extern".to_string(),
        Almacenamiento::Typedef => "typedef".to_string(),
    }
}

fn fmt_calificador(c: &Calificador) -> String {
    match c {
        Calificador::Const => "const".to_string(),
        Calificador::Volatile => "volatile".to_string(),
    }
}

fn fmt_tipo(t: &Tipo) -> String {
    match t {
        Tipo::Primitivo(p) => fmt_tipo_primitivo(*p),
        Tipo::Puntero { calificadores, apunta_a } => {
            let cal = if calificadores.is_empty() {
                String::new()
            } else {
                let calificados = calificadores.iter().map(fmt_calificador).collect::<Vec<_>>().join(" ");
                format!("{} ", calificados)
            };
            format!("{}{}*", cal, fmt_tipo(apunta_a))
        }
        Tipo::Array { elemento, tamano } => {
            let tam = tamano.as_ref().map(|expr| format!("[{}]", fmt_expr(expr))).unwrap_or_else(|| "[]".to_string());
            format!("{}{}", fmt_tipo(elemento), tam)
        }
        Tipo::Funcion { retorno, parametros } => {
            let params = parametros.iter().map(fmt_parametro).collect::<Vec<_>>().join(", ");
            format!("{}({})", fmt_tipo(retorno), params)
        }
        Tipo::Struct(spec) => {
            if let Some(nombre) = &spec.nombre {
                format!("struct {}", nombre)
            } else {
                "struct { ... }".to_string()
            }
        }
        Tipo::Union(spec) => {
            if let Some(nombre) = &spec.nombre {
                format!("union {}", nombre)
            } else {
                "union { ... }".to_string()
            }
        }
        Tipo::Enum(spec) => {
            if let Some(nombre) = &spec.nombre {
                format!("enum {}", nombre)
            } else {
                "enum { ... }".to_string()
            }
        }
        Tipo::Nombre(nombre) => nombre.clone(),
    }
}

fn fmt_tipo_primitivo(p: TipoPrimitivo) -> String {
    match p {
        TipoPrimitivo::Void => "void".to_string(),
        TipoPrimitivo::Char => "char".to_string(),
        TipoPrimitivo::Short => "short".to_string(),
        TipoPrimitivo::Int => "int".to_string(),
        TipoPrimitivo::Long => "long".to_string(),
        TipoPrimitivo::Float => "float".to_string(),
        TipoPrimitivo::Double => "double".to_string(),
        TipoPrimitivo::Signed => "signed".to_string(),
        TipoPrimitivo::Unsigned => "unsigned".to_string(),
    }
}

fn fmt_declarador(d: &Declarador) -> String {
    let mut out = fmt_declarador_base(&d.base);
    for puntero in &d.punteros {
        out = format!("{} {}", fmt_puntero(puntero), out);
    }
    out
}

fn fmt_puntero(p: &Puntero) -> String {
    if p.calificadores.is_empty() {
        "*".to_string()
    } else {
        let calificados = p.calificadores.iter().map(fmt_calificador).collect::<Vec<_>>().join(" ");
        format!("{} *", calificados)
    }
}

fn fmt_declarador_base(b: &DeclaradorBase) -> String {
    match b {
        DeclaradorBase::Identificador(nombre) => nombre.clone(),
        DeclaradorBase::Agrupado(inner) => format!("({})", fmt_declarador(inner)),
        DeclaradorBase::Derivado { interno, sufijo } => match sufijo {
            SufijoDeclarador::Array(tamano) => {
                let tam = tamano.as_ref().map(|expr| format!("{}", fmt_expr(expr))).unwrap_or_default();
                format!("{}[{}]", fmt_declarador_base(interno), tam)
            }
            SufijoDeclarador::Funcion(params) => {
                let ps = params.iter().map(fmt_parametro).collect::<Vec<_>>().join(", ");
                format!("{}({})", fmt_declarador_base(interno), ps)
            }
        },
    }
}

fn fmt_init_declarador(init: &InitDeclarador) -> String {
    let mut out = fmt_declarador(&init.declarador);
    if let Some(inicializador) = &init.inicializador {
        out.push_str(&format!(" = {}", fmt_inicializador(inicializador)));
    }
    out
}

fn fmt_inicializador(init: &Inicializador) -> String {
    match init {
        Inicializador::Expr(expr) => fmt_expr(expr),
        Inicializador::Lista(items) => {
            let inner = items.iter().map(fmt_inicializador).collect::<Vec<_>>().join(", ");
            format!("{{ {} }}", inner)
        }
    }
}

fn fmt_parametro(p: &Parametro) -> String {
    let mut partes = vec![fmt_especificadores(&p.especificadores)];
    if let Some(declarador) = &p.declarador {
        partes.push(fmt_declarador(declarador));
    }
    partes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
}

fn fmt_expr(e: &Expr) -> String {
    match e {
        Expr::Literal { valor, .. } => fmt_literal(valor),
        Expr::Identificador { nombre, .. } => nombre.clone(),
        Expr::Binaria { operador, izquierda, derecha, .. } => format!("{} {} {}", fmt_expr(izquierda), fmt_operador_binario(*operador), fmt_expr(derecha)),
        Expr::Unaria { operador, operando, .. } => match operador {
            OperadorUnario::Incremento => format!("{}++", fmt_expr(operando)),
            OperadorUnario::Decremento => format!("{}--", fmt_expr(operando)),
            OperadorUnario::Mas => format!("+{}", fmt_expr(operando)),
            OperadorUnario::Menos => format!("-{}", fmt_expr(operando)),
            OperadorUnario::Negacion => format!("!{}", fmt_expr(operando)),
            OperadorUnario::Complemento => format!("~{}", fmt_expr(operando)),
            OperadorUnario::Asterisco => format!("*{}", fmt_expr(operando)),
            OperadorUnario::YBit => format!("&{}", fmt_expr(operando)),
        },
        Expr::Ternaria { condicion, verdadero, falso, .. } => format!("{} ? {} : {}", fmt_expr(condicion), fmt_expr(verdadero), fmt_expr(falso)),
        Expr::Asignacion { operador, destino, valor, .. } => format!("{} {} {}", fmt_expr(destino), fmt_operador_asignacion(operador.clone()), fmt_expr(valor)),
        Expr::Llamada { funcion, argumentos, .. } => {
            let args = argumentos.iter().map(fmt_expr).collect::<Vec<_>>().join(", ");
            format!("{}({})", fmt_expr(funcion), args)
        }
        Expr::Index { arreglo, indice, .. } => format!("{}[{}]", fmt_expr(arreglo), fmt_expr(indice)),
        Expr::Miembro { objeto, miembro, es_puntero, .. } => {
            if *es_puntero {
                format!("{}->{}", fmt_expr(objeto), miembro)
            } else {
                format!("{}.{}", fmt_expr(objeto), miembro)
            }
        }
        Expr::Cast { tipo, expr, .. } => format!("({}) {}", fmt_tipo(tipo), fmt_expr(expr)),
        Expr::Sizeof { es_tipo, objetivo, .. } => {
            let objetivo_s = match objetivo {
                SizeofObjetivo::Expr(expr) => fmt_expr(expr),
                SizeofObjetivo::Tipo(tipo) => fmt_tipo(tipo),
            };
            if *es_tipo {
                format!("sizeof({})", objetivo_s)
            } else {
                format!("sizeof {}", objetivo_s)
            }
        }
    }
}

fn fmt_expr_opt(e: Option<&Expr>) -> String {
    e.map(|expr| fmt_expr(expr)).unwrap_or_else(|| "<vacío>".to_string())
}

fn fmt_literal(l: &Literal) -> String {
    match l {
        Literal::Entero(v) => v.clone(),
        Literal::Flotante(v) => v.clone(),
        Literal::Caracter(v) => v.clone(),
        Literal::Cadena(v) => v.clone(),
    }
}

fn fmt_operador_binario(o: OperadorBinario) -> String {
    match o {
        OperadorBinario::Mas => "+".to_string(),
        OperadorBinario::Menos => "-".to_string(),
        OperadorBinario::Asterisco => "*".to_string(),
        OperadorBinario::Barra => "/".to_string(),
        OperadorBinario::Porcentaje => "%".to_string(),
        OperadorBinario::Menor => "<".to_string(),
        OperadorBinario::Mayor => ">".to_string(),
        OperadorBinario::MenorIgual => "<=".to_string(),
        OperadorBinario::MayorIgual => ">=".to_string(),
        OperadorBinario::Igual => "==".to_string(),
        OperadorBinario::Distinto => "!=".to_string(),
        OperadorBinario::YLogico => "&&".to_string(),
        OperadorBinario::OLogico => "||".to_string(),
        OperadorBinario::YBit => "&".to_string(),
        OperadorBinario::OBit => "|".to_string(),
        OperadorBinario::XorBit => "^".to_string(),
        OperadorBinario::DesplazarIzq => "<<".to_string(),
        OperadorBinario::DesplazarDer => ">>".to_string(),
    }
}

fn fmt_operador_asignacion(o: OperadorAsignacion) -> String {
    match o {
        OperadorAsignacion::Asignar => "=".to_string(),
        OperadorAsignacion::MasAsignar => "+=".to_string(),
        OperadorAsignacion::MenosAsignar => "-=".to_string(),
        OperadorAsignacion::AsteriscoAsignar => "*=".to_string(),
        OperadorAsignacion::BarraAsignar => "/=".to_string(),
        OperadorAsignacion::PorcentajeAsignar => "%=".to_string(),
    }
}

fn fmt_for_init(init: &ForInit) -> String {
    match init {
        ForInit::Expr(expr) => fmt_expr_opt(expr.as_deref()),
        ForInit::Declaracion(decl) => format!("Decl: {}", fmt_especificadores(&decl.especificadores)),
    }
}

fn fmt_etiqueta(e: &Etiqueta) -> String {
    match e {
        Etiqueta::Identificador(nombre) => nombre.clone(),
        Etiqueta::Case(expr) => fmt_expr(expr),
        Etiqueta::Default => "default".to_string(),
    }
}

impl std::fmt::Display for Programa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ast_a_arbol(self).to_repr())
    }
}
