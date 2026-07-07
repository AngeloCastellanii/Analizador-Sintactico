use analizador_lexico::{Posicion, TipoToken, Token};
use crate::ast::*;
use crate::error::{ErrorParseo, Resultado};

pub struct AnalizadorSintactico {
    tokens: Vec<Token>,
    pos: usize,
}

impl AnalizadorSintactico {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parsear(&mut self) -> Resultado<Programa> {
        self.saltar_bom();
        self.saltar_preprocesador();
        let mut declaraciones = Vec::new();
        while !self.es_fin() {
            self.saltar_preprocesador();
            if self.es_fin() { break; }
            declaraciones.push(self.parse_decl_externa()?);
        }
        Ok(Programa { declaraciones })
    }

    fn actual(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len().saturating_sub(1))]
    }

    fn pos_actual(&self) -> Posicion {
        self.actual().posicion
    }

    fn es_fin(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn avanzar(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() { self.pos += 1; }
        t
    }

    fn mirar(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn coincide(&self, tipo: TipoToken) -> bool {
        !self.es_fin() && self.actual().tipo == tipo
    }

    fn esperar(&mut self, tipo: TipoToken) -> Resultado<Token> {
        if self.coincide(tipo.clone()) { Ok(self.avanzar()) }
        else {
            Err(ErrorParseo::token_inesperado(self.pos_actual(), &format!("{:?}", tipo), &self.actual().tipo))
        }
    }

    fn saltar_bom(&mut self) {
        if !self.es_fin()
            && self.actual().tipo == TipoToken::Desconocido
            && self.actual().valor.as_deref() == Some("\u{feff}")
        {
            self.avanzar();
        }
    }
    fn saltar_preprocesador(&mut self) {
        while !self.es_fin() {
            if self.actual().tipo == TipoToken::Desconocido
                && self.actual().valor.as_deref() == Some("#")
            {
                let linea = self.actual().posicion.linea;
                while !self.es_fin() && self.actual().posicion.linea == linea {
                    self.avanzar();
                }
            } else {
                break;
            }
        }
    }


    fn es_inicio_decl_bloque(&self) -> bool {
        matches!(
            self.actual().tipo,
            TipoToken::Auto | TipoToken::Register | TipoToken::Static | TipoToken::Extern
                | TipoToken::Typedef | TipoToken::Const | TipoToken::Volatile
                | TipoToken::Void | TipoToken::Char | TipoToken::Short | TipoToken::Int
                | TipoToken::Long | TipoToken::Float | TipoToken::Double | TipoToken::Signed
                | TipoToken::Unsigned | TipoToken::Struct | TipoToken::Union | TipoToken::Enum
        )
    }
    fn es_inicio_decl(&self) -> bool {
        matches!(
            self.actual().tipo,
            TipoToken::Auto | TipoToken::Register | TipoToken::Static | TipoToken::Extern
                | TipoToken::Typedef | TipoToken::Const | TipoToken::Volatile
                | TipoToken::Void | TipoToken::Char | TipoToken::Short | TipoToken::Int
                | TipoToken::Long | TipoToken::Float | TipoToken::Double | TipoToken::Signed
                | TipoToken::Unsigned | TipoToken::Struct | TipoToken::Union | TipoToken::Enum
                | TipoToken::Identificador
        )
    }

    fn parse_decl_externa(&mut self) -> Resultado<DeclExterna> {
        let pos = self.pos_actual();
        let especificadores = self.parse_especificadores()?;
        let declarador = self.parse_declarador()?;
        if self.coincide(TipoToken::LlaveIzq) {
            let cuerpo = self.parse_bloque()?;
            Ok(DeclExterna::Funcion(FuncionDef { posicion: pos, especificadores, declarador, cuerpo }))
        } else {
            let mut declaradores = vec![InitDeclarador {
                declarador,
                inicializador: self.parse_inicializador_opt()?,
            }];
            while self.coincide(TipoToken::Coma) {
                self.avanzar();
                declaradores.push(InitDeclarador {
                    declarador: self.parse_declarador()?,
                    inicializador: self.parse_inicializador_opt()?,
                });
            }
            self.esperar(TipoToken::PuntoYComa)?;
            Ok(DeclExterna::Declaracion(Declaracion { posicion: pos, especificadores, declaradores }))
        }
    }

    fn parse_especificadores(&mut self) -> Resultado<Especificadores> {
        let mut esp = Especificadores::default();
        loop {
            match self.actual().tipo {
                TipoToken::Auto => { esp.almacenamiento.push(Almacenamiento::Auto); self.avanzar(); }
                TipoToken::Register => { esp.almacenamiento.push(Almacenamiento::Register); self.avanzar(); }
                TipoToken::Static => { esp.almacenamiento.push(Almacenamiento::Static); self.avanzar(); }
                TipoToken::Extern => { esp.almacenamiento.push(Almacenamiento::Extern); self.avanzar(); }
                TipoToken::Typedef => { esp.almacenamiento.push(Almacenamiento::Typedef); self.avanzar(); }
                TipoToken::Const => { esp.calificadores.push(Calificador::Const); self.avanzar(); }
                TipoToken::Volatile => { esp.calificadores.push(Calificador::Volatile); self.avanzar(); }
                TipoToken::Struct => { esp.tipo = Some(self.parse_struct_union(true)?); }
                TipoToken::Union => { esp.tipo = Some(self.parse_struct_union(false)?); }
                TipoToken::Enum => { esp.tipo = Some(self.parse_enum()?); }
                TipoToken::Void | TipoToken::Char | TipoToken::Short | TipoToken::Int | TipoToken::Long
                | TipoToken::Float | TipoToken::Double | TipoToken::Signed | TipoToken::Unsigned
                | TipoToken::Identificador => {
                    esp.tipo = Some(self.parse_tipo()?);
                    break;
                }
                _ => break,
            }
        }
        Ok(esp)
    }

    fn parse_tipo(&mut self) -> Resultado<Tipo> {
        if self.coincide(TipoToken::Identificador) {
            let nombre = self.avanzar().valor.unwrap_or_default();
            return Ok(Tipo::Nombre(nombre));
        }
        let prim = match self.avanzar().tipo {
            TipoToken::Void => TipoPrimitivo::Void,
            TipoToken::Char => TipoPrimitivo::Char,
            TipoToken::Short => TipoPrimitivo::Short,
            TipoToken::Int => TipoPrimitivo::Int,
            TipoToken::Long => TipoPrimitivo::Long,
            TipoToken::Float => TipoPrimitivo::Float,
            TipoToken::Double => TipoPrimitivo::Double,
            TipoToken::Signed => TipoPrimitivo::Signed,
            TipoToken::Unsigned => TipoPrimitivo::Unsigned,
            other => return Err(ErrorParseo::token_inesperado(self.pos_actual(), "tipo primitivo", &other)),
        };
        let es_long_doble = matches!(prim, TipoPrimitivo::Long | TipoPrimitivo::Double);
        let mut tipo = Tipo::Primitivo(prim);
        if self.coincide(TipoToken::Long) && es_long_doble {
            self.avanzar();
        }
        while self.coincide(TipoToken::Asterisco) {
            self.avanzar();
            let mut cal = Vec::new();
            while self.coincide(TipoToken::Const) { cal.push(Calificador::Const); self.avanzar(); }
            while self.coincide(TipoToken::Volatile) { cal.push(Calificador::Volatile); self.avanzar(); }
            tipo = Tipo::Puntero { calificadores: cal, apunta_a: Box::new(tipo) };
        }
        Ok(tipo)
    }

    fn parse_struct_union(&mut self, es_struct: bool) -> Resultado<Tipo> {
        self.avanzar();
        let nombre = if self.coincide(TipoToken::Identificador) {
            Some(self.avanzar().valor.unwrap_or_default())
        } else { None };
        let mut miembros = Vec::new();
        if self.coincide(TipoToken::LlaveIzq) {
            self.avanzar();
            while !self.coincide(TipoToken::LlaveDer) && !self.es_fin() {
                miembros.push(self.parse_declaracion()?);
            }
            self.esperar(TipoToken::LlaveDer)?;
        }
        let spec = StructUnionSpec { nombre, miembros };
        Ok(if es_struct { Tipo::Struct(spec) } else { Tipo::Union(spec) })
    }

    fn parse_enum(&mut self) -> Resultado<Tipo> {
        self.avanzar();
        let nombre = if self.coincide(TipoToken::Identificador) {
            Some(self.avanzar().valor.unwrap_or_default())
        } else { None };
        let mut enumeradores = Vec::new();
        if self.coincide(TipoToken::LlaveIzq) {
            self.avanzar();
            while !self.coincide(TipoToken::LlaveDer) && !self.es_fin() {
                let n = self.esperar(TipoToken::Identificador)?.valor.unwrap_or_default();
                let valor = if self.coincide(TipoToken::Asignar) {
                    self.avanzar();
                    Some(Box::new(self.parse_expresion()?))
                } else { None };
                enumeradores.push(Enumerador { nombre: n, valor });
                if self.coincide(TipoToken::Coma) { self.avanzar(); }
            }
            self.esperar(TipoToken::LlaveDer)?;
        }
        Ok(Tipo::Enum(EnumSpec { nombre, enumeradores }))
    }

    fn parse_declaracion(&mut self) -> Resultado<Declaracion> {
        let pos = self.pos_actual();
        let especificadores = self.parse_especificadores()?;
        let mut declaradores = vec![InitDeclarador {
            declarador: self.parse_declarador()?,
            inicializador: self.parse_inicializador_opt()?,
        }];
        while self.coincide(TipoToken::Coma) {
            self.avanzar();
            declaradores.push(InitDeclarador {
                declarador: self.parse_declarador()?,
                inicializador: self.parse_inicializador_opt()?,
            });
        }
        self.esperar(TipoToken::PuntoYComa)?;
        Ok(Declaracion { posicion: pos, especificadores, declaradores })
    }

    fn parse_declarador(&mut self) -> Resultado<Declarador> {
        Ok(Declarador { punteros: self.parse_punteros()?, base: self.parse_declarador_directo()? })
    }

    fn parse_punteros(&mut self) -> Resultado<Vec<Puntero>> {
        let mut punteros = Vec::new();
        while self.coincide(TipoToken::Asterisco) {
            self.avanzar();
            let mut cal = Vec::new();
            while self.coincide(TipoToken::Const) { cal.push(Calificador::Const); self.avanzar(); }
            while self.coincide(TipoToken::Volatile) { cal.push(Calificador::Volatile); self.avanzar(); }
            punteros.push(Puntero { calificadores: cal });
        }
        Ok(punteros)
    }

    fn parse_declarador_directo(&mut self) -> Resultado<DeclaradorBase> {
        let base = if self.coincide(TipoToken::ParentesisIzq) {
            self.avanzar();
            let inner = self.parse_declarador()?;
            self.esperar(TipoToken::ParentesisDer)?;
            DeclaradorBase::Agrupado(Box::new(inner))
        } else {
            DeclaradorBase::Identificador(self.esperar(TipoToken::Identificador)?.valor.unwrap_or_default())
        };
        self.aplicar_sufijos(base)
    }

    fn aplicar_sufijos(&mut self, mut base: DeclaradorBase) -> Resultado<DeclaradorBase> {
        loop {
            if self.coincide(TipoToken::CorcheteIzq) {
                self.avanzar();
                let tam = if self.coincide(TipoToken::CorcheteDer) { None } else { Some(Box::new(self.parse_expresion()?)) };
                self.esperar(TipoToken::CorcheteDer)?;
                base = DeclaradorBase::Derivado { interno: Box::new(base), sufijo: SufijoDeclarador::Array(tam) };
            } else if self.coincide(TipoToken::ParentesisIzq) {
                self.avanzar();
                let params = self.parse_parametros()?;
                self.esperar(TipoToken::ParentesisDer)?;
                base = DeclaradorBase::Derivado { interno: Box::new(base), sufijo: SufijoDeclarador::Funcion(params) };
            } else { break; }
        }
        Ok(base)
    }

    fn parse_parametros(&mut self) -> Resultado<Vec<Parametro>> {
        if self.coincide(TipoToken::Void) {
            self.avanzar();
            return Ok(Vec::new());
        }
        let mut params = Vec::new();
        if self.coincide(TipoToken::ParentesisDer) { return Ok(params); }
        loop {
            let especificadores = self.parse_especificadores()?;
            let declarador = if self.es_inicio_declarador() { Some(self.parse_declarador()?) } else { None };
            params.push(Parametro { especificadores, declarador });
            if self.coincide(TipoToken::Coma) { self.avanzar(); } else { break; }
        }
        Ok(params)
    }

    fn es_inicio_declarador(&self) -> bool {
        self.coincide(TipoToken::Asterisco) || self.coincide(TipoToken::ParentesisIzq) || self.coincide(TipoToken::Identificador)
    }

    fn parse_inicializador_opt(&mut self) -> Resultado<Option<Inicializador>> {
        if !self.coincide(TipoToken::Asignar) { return Ok(None); }
        self.avanzar();
        Ok(Some(self.parse_inicializador()?))
    }

    fn parse_inicializador(&mut self) -> Resultado<Inicializador> {
        if self.coincide(TipoToken::LlaveIzq) {
            self.avanzar();
            let mut lista = Vec::new();
            if !self.coincide(TipoToken::LlaveDer) {
                loop {
                    lista.push(self.parse_inicializador()?);
                    if self.coincide(TipoToken::Coma) { self.avanzar(); } else { break; }
                }
            }
            self.esperar(TipoToken::LlaveDer)?;
            Ok(Inicializador::Lista(lista))
        } else {
            Ok(Inicializador::Expr(Box::new(self.parse_expresion()?)))
        }
    }

    fn parse_bloque(&mut self) -> Resultado<Bloque> {
        let pos = self.pos_actual();
        self.esperar(TipoToken::LlaveIzq)?;
        let mut items = Vec::new();
        while !self.coincide(TipoToken::LlaveDer) && !self.es_fin() {
            if self.es_inicio_decl_bloque() {
                items.push(ItemBloque::Declaracion(self.parse_declaracion()?));
            } else {
                items.push(ItemBloque::Sentencia(self.parse_sentencia()?));
            }
        }
        self.esperar(TipoToken::LlaveDer)?;
        Ok(Bloque { posicion: pos, items })
    }

    fn parse_sentencia(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        match self.actual().tipo {
            TipoToken::LlaveIzq => Ok(Sentencia::Compuesta(self.parse_bloque()?)),
            TipoToken::If => self.parse_if(),
            TipoToken::Switch => self.parse_switch(),
            TipoToken::While => self.parse_while(),
            TipoToken::Do => self.parse_do_while(),
            TipoToken::For => self.parse_for(),
            TipoToken::Goto => {
                self.avanzar();
                let et = self.esperar(TipoToken::Identificador)?.valor.unwrap_or_default();
                self.esperar(TipoToken::PuntoYComa)?;
                Ok(Sentencia::Salto(Salto::Goto { posicion: pos, etiqueta: et }))
            }
            TipoToken::Continue => { self.avanzar(); self.esperar(TipoToken::PuntoYComa)?; Ok(Sentencia::Salto(Salto::Continuar { posicion: pos })) }
            TipoToken::Break => { self.avanzar(); self.esperar(TipoToken::PuntoYComa)?; Ok(Sentencia::Salto(Salto::Romper { posicion: pos })) }
            TipoToken::Return => {
                self.avanzar();
                let valor = if self.coincide(TipoToken::PuntoYComa) { None } else {
                    let e = self.parse_expresion()?; self.esperar(TipoToken::PuntoYComa)?; Some(Box::new(e))
                };
                if valor.is_none() { self.esperar(TipoToken::PuntoYComa)?; }
                Ok(Sentencia::Salto(Salto::Retornar { posicion: pos, valor }))
            }
            TipoToken::Case => {
                self.avanzar();
                let expr = Box::new(self.parse_expresion()?);
                self.esperar(TipoToken::DosPuntos)?;
                let s = Box::new(self.parse_sentencia()?);
                Ok(Sentencia::Etiquetada { posicion: pos, etiqueta: Etiqueta::Case(expr), sentencia: s })
            }
            TipoToken::Default => {
                self.avanzar();
                self.esperar(TipoToken::DosPuntos)?;
                let s = Box::new(self.parse_sentencia()?);
                Ok(Sentencia::Etiquetada { posicion: pos, etiqueta: Etiqueta::Default, sentencia: s })
            }
            TipoToken::Identificador if self.mirar(1).map(|t| t.tipo == TipoToken::DosPuntos).unwrap_or(false) => {
                let et = self.avanzar().valor.unwrap_or_default();
                self.esperar(TipoToken::DosPuntos)?;
                let s = Box::new(self.parse_sentencia()?);
                Ok(Sentencia::Etiquetada { posicion: pos, etiqueta: Etiqueta::Identificador(et), sentencia: s })
            }
            _ => {
                let expr = if self.coincide(TipoToken::PuntoYComa) { None } else {
                    let e = self.parse_expresion()?; Some(Box::new(e))
                };
                self.esperar(TipoToken::PuntoYComa)?;
                Ok(Sentencia::Expr { posicion: pos, expr })
            }
        }
    }

    fn parse_if(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        self.avanzar();
        self.esperar(TipoToken::ParentesisIzq)?;
        let cond = Box::new(self.parse_expresion()?);
        self.esperar(TipoToken::ParentesisDer)?;
        let entonces = Box::new(self.parse_sentencia()?);
        let sino = if self.coincide(TipoToken::Else) {
            self.avanzar();
            Some(Box::new(self.parse_sentencia()?))
        } else { None };
        Ok(Sentencia::Seleccion(Seleccion::If { posicion: pos, condicion: cond, entonces, sino }))
    }

    fn parse_switch(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        self.avanzar();
        self.esperar(TipoToken::ParentesisIzq)?;
        let cond = Box::new(self.parse_expresion()?);
        self.esperar(TipoToken::ParentesisDer)?;
        let cuerpo = Box::new(self.parse_sentencia()?);
        Ok(Sentencia::Seleccion(Seleccion::Switch { posicion: pos, condicion: cond, cuerpo }))
    }

    fn parse_while(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        self.avanzar();
        self.esperar(TipoToken::ParentesisIzq)?;
        let cond = Box::new(self.parse_expresion()?);
        self.esperar(TipoToken::ParentesisDer)?;
        let cuerpo = Box::new(self.parse_sentencia()?);
        Ok(Sentencia::Iteracion(Iteracion::While { posicion: pos, condicion: cond, cuerpo }))
    }

    fn parse_do_while(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        self.avanzar();
        let cuerpo = Box::new(self.parse_sentencia()?);
        self.esperar(TipoToken::While)?;
        self.esperar(TipoToken::ParentesisIzq)?;
        let cond = Box::new(self.parse_expresion()?);
        self.esperar(TipoToken::ParentesisDer)?;
        self.esperar(TipoToken::PuntoYComa)?;
        Ok(Sentencia::Iteracion(Iteracion::DoWhile { posicion: pos, cuerpo, condicion: cond }))
    }

    fn parse_for(&mut self) -> Resultado<Sentencia> {
        let pos = self.pos_actual();
        self.avanzar();
        self.esperar(TipoToken::ParentesisIzq)?;
        let init = if self.coincide(TipoToken::PuntoYComa) {
            None
        } else if self.es_inicio_decl_bloque() {
            Some(ForInit::Declaracion(self.parse_declaracion()?))
        } else {
            let e = self.parse_expresion()?;
            self.esperar(TipoToken::PuntoYComa)?;
            Some(ForInit::Expr(Some(Box::new(e))))
        };
        if init.is_none() {
            self.esperar(TipoToken::PuntoYComa)?;
        }
        let condicion = if self.coincide(TipoToken::PuntoYComa) {
            None
        } else {
            let c = self.parse_expresion()?;
            self.esperar(TipoToken::PuntoYComa)?;
            Some(Box::new(c))
        };
        if condicion.is_none() {
            self.esperar(TipoToken::PuntoYComa)?;
        }
        let incremento = if self.coincide(TipoToken::ParentesisDer) {
            None
        } else {
            Some(Box::new(self.parse_expresion()?))
        };
        self.esperar(TipoToken::ParentesisDer)?;
        let cuerpo = Box::new(self.parse_sentencia()?);
        Ok(Sentencia::Iteracion(Iteracion::For { posicion: pos, init, condicion, incremento, cuerpo }))
    }

    fn parse_expresion(&mut self) -> Resultado<Expr> {
        self.parse_asignacion()
    }

    fn parse_asignacion(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_ternario()?;
        if let Some(op) = self.operador_asignacion() {
            let pos = self.pos_actual();
            self.avanzar();
            let valor = Box::new(self.parse_asignacion()?);
            expr = Expr::Asignacion { posicion: pos, operador: op, destino: Box::new(expr), valor };
        }
        Ok(expr)
    }

    fn operador_asignacion(&self) -> Option<OperadorAsignacion> {
        Some(match self.actual().tipo {
            TipoToken::Asignar => OperadorAsignacion::Asignar,
            TipoToken::MasAsignar => OperadorAsignacion::MasAsignar,
            TipoToken::MenosAsignar => OperadorAsignacion::MenosAsignar,
            TipoToken::AsteriscoAsignar => OperadorAsignacion::AsteriscoAsignar,
            TipoToken::BarraAsignar => OperadorAsignacion::BarraAsignar,
            TipoToken::PorcentajeAsignar => OperadorAsignacion::PorcentajeAsignar,
            _ => return None,
        })
    }

    fn parse_ternario(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_o_logico()?;
        if self.coincide(TipoToken::Interrogacion) {
            let pos = self.pos_actual();
            self.avanzar();
            let verdadero = Box::new(self.parse_expresion()?);
            self.esperar(TipoToken::DosPuntos)?;
            let falso = Box::new(self.parse_ternario()?);
            expr = Expr::Ternaria { posicion: pos, condicion: Box::new(expr), verdadero, falso };
        }
        Ok(expr)
    }

    fn parse_o_logico(&mut self) -> Resultado<Expr> { self.parse_binaria(|t| t == TipoToken::OLogico, OperadorBinario::OLogico, |s| s.parse_y_logico()) }
    fn parse_y_logico(&mut self) -> Resultado<Expr> { self.parse_binaria(|t| t == TipoToken::YLogico, OperadorBinario::YLogico, |s| s.parse_o_bit()) }
    fn parse_o_bit(&mut self) -> Resultado<Expr> { self.parse_binaria(|t| t == TipoToken::OBit, OperadorBinario::OBit, |s| s.parse_xor_bit()) }
    fn parse_xor_bit(&mut self) -> Resultado<Expr> { self.parse_binaria(|t| t == TipoToken::XorBit, OperadorBinario::XorBit, |s| s.parse_y_bit()) }
    fn parse_y_bit(&mut self) -> Resultado<Expr> { self.parse_binaria(|t| t == TipoToken::YBit, OperadorBinario::YBit, |s| s.parse_igualdad()) }

    fn parse_igualdad(&mut self) -> Resultado<Expr> {
        self.parse_binaria(
            |t| matches!(t, TipoToken::Igual | TipoToken::Distinto),
            OperadorBinario::Igual,
            |s| s.parse_relacional(),
        )
    }

    fn parse_relacional(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_desplazamiento()?;
        loop {
            let op = match self.actual().tipo {
                TipoToken::Menor => OperadorBinario::Menor,
                TipoToken::Mayor => OperadorBinario::Mayor,
                TipoToken::MenorIgual => OperadorBinario::MenorIgual,
                TipoToken::MayorIgual => OperadorBinario::MayorIgual,
                _ => break,
            };
            let pos = self.pos_actual();
            self.avanzar();
            expr = Expr::Binaria { posicion: pos, operador: op, izquierda: Box::new(expr), derecha: Box::new(self.parse_desplazamiento()?) };
        }
        Ok(expr)
    }

    fn parse_desplazamiento(&mut self) -> Resultado<Expr> {
        self.parse_binaria(
            |t| matches!(t, TipoToken::DesplazarIzq | TipoToken::DesplazarDer),
            OperadorBinario::DesplazarIzq,
            |s| s.parse_aditiva(),
        )
    }

    fn parse_aditiva(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_multiplicativa()?;
        loop {
            let op = match self.actual().tipo {
                TipoToken::Mas => OperadorBinario::Mas,
                TipoToken::Menos => OperadorBinario::Menos,
                _ => break,
            };
            let pos = self.pos_actual();
            self.avanzar();
            expr = Expr::Binaria { posicion: pos, operador: op, izquierda: Box::new(expr), derecha: Box::new(self.parse_multiplicativa()?) };
        }
        Ok(expr)
    }

    fn parse_multiplicativa(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_unaria()?;
        loop {
            let op = match self.actual().tipo {
                TipoToken::Asterisco => OperadorBinario::Asterisco,
                TipoToken::Barra => OperadorBinario::Barra,
                TipoToken::Porcentaje => OperadorBinario::Porcentaje,
                _ => break,
            };
            let pos = self.pos_actual();
            self.avanzar();
            expr = Expr::Binaria { posicion: pos, operador: op, izquierda: Box::new(expr), derecha: Box::new(self.parse_unaria()?) };
        }
        Ok(expr)
    }

    fn parse_binaria<F, G>(&mut self, pred: F, op_default: OperadorBinario, mut siguiente: G) -> Resultado<Expr>
    where
        F: Fn(TipoToken) -> bool,
        G: FnMut(&mut Self) -> Resultado<Expr>,
    {
        let mut expr = siguiente(self)?;
        while pred(self.actual().tipo.clone()) {
            let op = match self.actual().tipo {
                TipoToken::Igual => OperadorBinario::Igual,
                TipoToken::Distinto => OperadorBinario::Distinto,
                TipoToken::DesplazarIzq => OperadorBinario::DesplazarIzq,
                TipoToken::DesplazarDer => OperadorBinario::DesplazarDer,
                TipoToken::OLogico => OperadorBinario::OLogico,
                TipoToken::YLogico => OperadorBinario::YLogico,
                TipoToken::OBit => OperadorBinario::OBit,
                TipoToken::XorBit => OperadorBinario::XorBit,
                TipoToken::YBit => OperadorBinario::YBit,
                _ => op_default.clone(),
            };
            let pos = self.pos_actual();
            self.avanzar();
            expr = Expr::Binaria { posicion: pos, operador: op, izquierda: Box::new(expr), derecha: Box::new(siguiente(self)?) };
        }
        Ok(expr)
    }

    fn parse_unaria(&mut self) -> Resultado<Expr> {
        let pos = self.pos_actual();
        if self.coincide(TipoToken::Sizeof) {
            self.avanzar();
            let es_tipo = self.coincide(TipoToken::ParentesisIzq) && self.es_tipo_en_parentesis();
            if es_tipo {
                self.avanzar();
                let tipo = self.parse_tipo()?;
                self.esperar(TipoToken::ParentesisDer)?;
                return Ok(Expr::Sizeof { posicion: pos, es_tipo: true, objetivo: SizeofObjetivo::Tipo(tipo) });
            }
            let expr = self.parse_unaria()?;
            return Ok(Expr::Sizeof { posicion: pos, es_tipo: false, objetivo: SizeofObjetivo::Expr(Box::new(expr)) });
        }
        if let Some(op) = self.operador_unario() {
            self.avanzar();
            let operando = Box::new(self.parse_unaria()?);
            return Ok(Expr::Unaria { posicion: pos, operador: op, operando });
        }
        if self.es_cast() {
            self.avanzar();
            let tipo = self.parse_tipo()?;
            self.esperar(TipoToken::ParentesisDer)?;
            let expr = Box::new(self.parse_unaria()?);
            return Ok(Expr::Cast { posicion: pos, tipo, expr });
        }
        self.parse_postfija()
    }

    fn es_cast(&self) -> bool {
        if !self.coincide(TipoToken::ParentesisIzq) { return false; }
        // heuristica: ( tipo ) expr
        matches!(
            self.mirar(1).map(|t| t.tipo.clone()),
            Some(TipoToken::Void | TipoToken::Char | TipoToken::Short | TipoToken::Int | TipoToken::Long
                | TipoToken::Float | TipoToken::Double | TipoToken::Signed | TipoToken::Unsigned
                | TipoToken::Struct | TipoToken::Union | TipoToken::Enum | TipoToken::Identificador)
        )
    }

    fn es_tipo_en_parentesis(&self) -> bool {
        matches!(
            self.mirar(1).map(|t| t.tipo.clone()),
            Some(TipoToken::Void | TipoToken::Char | TipoToken::Short | TipoToken::Int | TipoToken::Long
                | TipoToken::Float | TipoToken::Double | TipoToken::Signed | TipoToken::Unsigned
                | TipoToken::Struct | TipoToken::Union | TipoToken::Enum | TipoToken::Identificador)
        )
    }

    fn operador_unario(&self) -> Option<OperadorUnario> {
        Some(match self.actual().tipo {
            TipoToken::Mas => OperadorUnario::Mas,
            TipoToken::Menos => OperadorUnario::Menos,
            TipoToken::Negacion => OperadorUnario::Negacion,
            TipoToken::Complemento => OperadorUnario::Complemento,
            TipoToken::Asterisco => OperadorUnario::Asterisco,
            TipoToken::YBit => OperadorUnario::YBit,
            TipoToken::Incremento => OperadorUnario::Incremento,
            TipoToken::Decremento => OperadorUnario::Decremento,
            _ => return None,
        })
    }

    fn parse_postfija(&mut self) -> Resultado<Expr> {
        let mut expr = self.parse_primaria()?;
        loop {
            let pos = self.pos_actual();
            match self.actual().tipo {
                TipoToken::CorcheteIzq => {
                    self.avanzar();
                    let indice = Box::new(self.parse_expresion()?);
                    self.esperar(TipoToken::CorcheteDer)?;
                    expr = Expr::Index { posicion: pos, arreglo: Box::new(expr), indice };
                }
                TipoToken::ParentesisIzq => {
                    self.avanzar();
                    let mut args = Vec::new();
                    if !self.coincide(TipoToken::ParentesisDer) {
                        loop {
                            args.push(self.parse_expresion()?);
                            if self.coincide(TipoToken::Coma) { self.avanzar(); } else { break; }
                        }
                    }
                    self.esperar(TipoToken::ParentesisDer)?;
                    expr = Expr::Llamada { posicion: pos, funcion: Box::new(expr), argumentos: args };
                }
                TipoToken::Punto => {
                    self.avanzar();
                    let m = self.esperar(TipoToken::Identificador)?.valor.unwrap_or_default();
                    expr = Expr::Miembro { posicion: pos, objeto: Box::new(expr), miembro: m, es_puntero: false };
                }
                TipoToken::Flecha => {
                    self.avanzar();
                    let m = self.esperar(TipoToken::Identificador)?.valor.unwrap_or_default();
                    expr = Expr::Miembro { posicion: pos, objeto: Box::new(expr), miembro: m, es_puntero: true };
                }
                TipoToken::Incremento => { self.avanzar(); expr = Expr::Unaria { posicion: pos, operador: OperadorUnario::Incremento, operando: Box::new(expr) }; }
                TipoToken::Decremento => { self.avanzar(); expr = Expr::Unaria { posicion: pos, operador: OperadorUnario::Decremento, operando: Box::new(expr) }; }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primaria(&mut self) -> Resultado<Expr> {
        let pos = self.pos_actual();
        match self.actual().tipo.clone() {
            TipoToken::Entero => {
                let v = self.avanzar().valor.unwrap_or_default();
                Ok(Expr::Literal { posicion: pos, valor: Literal::Entero(v) })
            }
            TipoToken::Flotante => {
                let v = self.avanzar().valor.unwrap_or_default();
                Ok(Expr::Literal { posicion: pos, valor: Literal::Flotante(v) })
            }
            TipoToken::Caracter => {
                let v = self.avanzar().valor.unwrap_or_default();
                Ok(Expr::Literal { posicion: pos, valor: Literal::Caracter(v) })
            }
            TipoToken::Cadena => {
                let v = self.avanzar().valor.unwrap_or_default();
                Ok(Expr::Literal { posicion: pos, valor: Literal::Cadena(v) })
            }
            TipoToken::Identificador => {
                let v = self.avanzar().valor.unwrap_or_default();
                Ok(Expr::Identificador { posicion: pos, nombre: v })
            }
            TipoToken::ParentesisIzq => {
                self.avanzar();
                let e = self.parse_expresion()?;
                self.esperar(TipoToken::ParentesisDer)?;
                Ok(e)
            }
            other => Err(ErrorParseo::token_inesperado(pos, "expresion primaria", &other)),
        }
    }
}





