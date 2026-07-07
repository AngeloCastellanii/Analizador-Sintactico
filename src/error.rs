use analizador_lexico::{Posicion, Token};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorParseo {
    pub posicion: Posicion,
    pub esperado: String,
    pub encontrado: String,
    pub linea_fuente: Option<String>,
}

impl ErrorParseo {
    pub fn nuevo(
        posicion: Posicion,
        esperado: impl Into<String>,
        encontrado: impl Into<String>,
        linea_fuente: Option<String>,
    ) -> Self {
        Self {
            posicion,
            esperado: esperado.into(),
            encontrado: encontrado.into(),
            linea_fuente,
        }
    }

    pub fn token_inesperado(
        posicion: Posicion,
        esperado: impl Into<String>,
        token: &Token,
        linea_fuente: Option<String>,
    ) -> Self {
        Self::nuevo(posicion, esperado, describir_token(token), linea_fuente)
    }

    fn indicador_columna(&self) -> String {
        let offset = self.posicion.columna.saturating_sub(1);
        format!("{}^", " ".repeat(offset))
    }
}

/// Describe un token para el mensaje de error (lexema o tipo).
pub fn describir_token(token: &Token) -> String {
    match &token.valor {
        Some(v) => format!("`{}`", v),
        None => format!("`{}`", token.tipo),
    }
}

impl fmt::Display for ErrorParseo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Error de parseo en línea {}, columna {}",
            self.posicion.linea, self.posicion.columna
        )?;
        writeln!(f, "  se esperaba: {}", self.esperado)?;
        writeln!(f, "  se encontró:  {}", self.encontrado)?;

        if let Some(linea) = &self.linea_fuente {
            writeln!(f)?;
            writeln!(f, "  {} | {}", self.posicion.linea, linea)?;
            writeln!(f, "  {} | {}", " ".repeat(self.posicion.linea.to_string().len()), self.indicador_columna())?;
        }

        Ok(())
    }
}

impl std::error::Error for ErrorParseo {}

pub type Resultado<T> = Result<T, ErrorParseo>;
