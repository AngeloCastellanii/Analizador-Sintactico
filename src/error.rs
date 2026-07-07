use analizador_lexico::Posicion;
use analizador_lexico::TipoToken;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorParseo {
    pub posicion: Posicion,
    pub mensaje: String,
}

impl ErrorParseo {
    pub fn nuevo(posicion: Posicion, mensaje: impl Into<String>) -> Self {
        Self { posicion, mensaje: mensaje.into() }
    }

    pub fn token_inesperado(posicion: Posicion, esperado: &str, encontrado: &TipoToken) -> Self {
        Self::nuevo(posicion, format!("se esperaba {}, se encontro {}", esperado, encontrado))
    }
}

impl fmt::Display for ErrorParseo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error de parseo en {}:{}: {}", self.posicion.linea, self.posicion.columna, self.mensaje)
    }
}

impl std::error::Error for ErrorParseo {}
pub type Resultado<T> = Result<T, ErrorParseo>;
