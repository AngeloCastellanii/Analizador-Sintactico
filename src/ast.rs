use analizador_lexico::Posicion;

#[derive(Debug, Clone, PartialEq)]
pub struct Programa {
    pub declaraciones: Vec<DeclExterna>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclExterna {
    Funcion(FuncionDef),
    Declaracion(Declaracion),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncionDef {
    pub posicion: Posicion,
    pub especificadores: Especificadores,
    pub declarador: Declarador,
    pub cuerpo: Bloque,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaracion {
    pub posicion: Posicion,
    pub especificadores: Especificadores,
    pub declaradores: Vec<InitDeclarador>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Especificadores {
    pub almacenamiento: Vec<Almacenamiento>,
    pub calificadores: Vec<Calificador>,
    pub tipo: Option<Tipo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Almacenamiento { Auto, Register, Static, Extern, Typedef }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Calificador { Const, Volatile }

#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Primitivo(TipoPrimitivo),
    Puntero { calificadores: Vec<Calificador>, apunta_a: Box<Tipo> },
    Array { elemento: Box<Tipo>, tamano: Option<Box<Expr>> },
    Funcion { retorno: Box<Tipo>, parametros: Vec<Parametro> },
    Struct(StructUnionSpec),
    Union(StructUnionSpec),
    Enum(EnumSpec),
    Nombre(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoPrimitivo { Void, Char, Short, Int, Long, Float, Double, Signed, Unsigned }

#[derive(Debug, Clone, PartialEq)]
pub struct StructUnionSpec {
    pub nombre: Option<String>,
    pub miembros: Vec<Declaracion>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumSpec {
    pub nombre: Option<String>,
    pub enumeradores: Vec<Enumerador>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enumerador { pub nombre: String, pub valor: Option<Box<Expr>> }

#[derive(Debug, Clone, PartialEq)]
pub struct Declarador { pub punteros: Vec<Puntero>, pub base: DeclaradorBase }

#[derive(Debug, Clone, PartialEq)]
pub struct Puntero { pub calificadores: Vec<Calificador> }

#[derive(Debug, Clone, PartialEq)]
pub enum DeclaradorBase {
    Identificador(String),
    Derivado { interno: Box<DeclaradorBase>, sufijo: SufijoDeclarador },
    Agrupado(Box<Declarador>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SufijoDeclarador {
    Array(Option<Box<Expr>>),
    Funcion(Vec<Parametro>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parametro { pub especificadores: Especificadores, pub declarador: Option<Declarador> }

#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarador { pub declarador: Declarador, pub inicializador: Option<Inicializador> }

#[derive(Debug, Clone, PartialEq)]
pub enum Inicializador { Expr(Box<Expr>), Lista(Vec<Inicializador>) }

#[derive(Debug, Clone, PartialEq)]
pub struct Bloque { pub posicion: Posicion, pub items: Vec<ItemBloque> }

#[derive(Debug, Clone, PartialEq)]
pub enum ItemBloque { Declaracion(Declaracion), Sentencia(Sentencia) }

#[derive(Debug, Clone, PartialEq)]
pub enum Sentencia {
    Etiquetada { posicion: Posicion, etiqueta: Etiqueta, sentencia: Box<Sentencia> },
    Compuesta(Bloque),
    Expr { posicion: Posicion, expr: Option<Box<Expr>> },
    Seleccion(Seleccion),
    Iteracion(Iteracion),
    Salto(Salto),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Etiqueta { Identificador(String), Case(Box<Expr>), Default }

#[derive(Debug, Clone, PartialEq)]
pub enum Seleccion {
    If { posicion: Posicion, condicion: Box<Expr>, entonces: Box<Sentencia>, sino: Option<Box<Sentencia>> },
    Switch { posicion: Posicion, condicion: Box<Expr>, cuerpo: Box<Sentencia> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Iteracion {
    While { posicion: Posicion, condicion: Box<Expr>, cuerpo: Box<Sentencia> },
    DoWhile { posicion: Posicion, cuerpo: Box<Sentencia>, condicion: Box<Expr> },
    For { posicion: Posicion, init: Option<ForInit>, condicion: Option<Box<Expr>>, incremento: Option<Box<Expr>>, cuerpo: Box<Sentencia> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInit { Expr(Option<Box<Expr>>), Declaracion(Declaracion) }

#[derive(Debug, Clone, PartialEq)]
pub enum Salto {
    Goto { posicion: Posicion, etiqueta: String },
    Continuar { posicion: Posicion },
    Romper { posicion: Posicion },
    Retornar { posicion: Posicion, valor: Option<Box<Expr>> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal { posicion: Posicion, valor: Literal },
    Identificador { posicion: Posicion, nombre: String },
    Binaria { posicion: Posicion, operador: OperadorBinario, izquierda: Box<Expr>, derecha: Box<Expr> },
    Unaria { posicion: Posicion, operador: OperadorUnario, operando: Box<Expr> },
    Ternaria { posicion: Posicion, condicion: Box<Expr>, verdadero: Box<Expr>, falso: Box<Expr> },
    Asignacion { posicion: Posicion, operador: OperadorAsignacion, destino: Box<Expr>, valor: Box<Expr> },
    Llamada { posicion: Posicion, funcion: Box<Expr>, argumentos: Vec<Expr> },
    Index { posicion: Posicion, arreglo: Box<Expr>, indice: Box<Expr> },
    Miembro { posicion: Posicion, objeto: Box<Expr>, miembro: String, es_puntero: bool },
    Cast { posicion: Posicion, tipo: Tipo, expr: Box<Expr> },
    Sizeof { posicion: Posicion, es_tipo: bool, objetivo: SizeofObjetivo },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SizeofObjetivo { Expr(Box<Expr>), Tipo(Tipo) }

#[derive(Debug, Clone, PartialEq)]
pub enum Literal { Entero(String), Flotante(String), Caracter(String), Cadena(String) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperadorBinario {
    Mas, Menos, Asterisco, Barra, Porcentaje, Menor, Mayor, MenorIgual, MayorIgual,
    Igual, Distinto, YLogico, OLogico, YBit, OBit, XorBit, DesplazarIzq, DesplazarDer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperadorUnario { Mas, Menos, Negacion, Complemento, Asterisco, YBit, Incremento, Decremento }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperadorAsignacion { Asignar, MasAsignar, MenosAsignar, AsteriscoAsignar, BarraAsignar, PorcentajeAsignar }

