use super::direccion::Direccion;
use std::collections::HashSet;

/// Enumeración que representa los diferentes tipos de objetos en el juego.
#[derive(PartialEq, Debug, Clone)]
pub enum Objeto {
    /// Representa un enemigo con una cantidad de vida y las bombas afectadas.
    Enemigo(i32, HashSet<(i32, i32)>),
    /// Representa una bomba, con una indicación de si es de traspaso y su alcance.
    Bomba(bool, i32),
    /// Representa una roca en el tablero.
    Roca,
    /// Representa una pared en el tablero.
    Pared,
    /// Representa un desvío con una dirección específica.
    Desvio(Direccion),
    /// Representa una casilla vacía en el tablero.
    Vacio,
}

/// Convierte un símbolo en un objeto del juego.
///
/// # Argumentos
///
/// * `simbolo`: Un `&str` que contiene el símbolo a convertir.
///
pub fn convertir_simbolos(simbolo: &str) -> Result<Objeto, &'static str> {
    let first_char = simbolo.chars().next().ok_or("El símbolo está vacío")?;

    match first_char {
        'F' => convertir_enemigo(simbolo),
        'B' => convertir_bomba(simbolo),
        'S' => convertir_bomba_traspaso(simbolo),
        'R' => Ok(Objeto::Roca),
        'W' => Ok(Objeto::Pared),
        'D' => convertir_desvio(simbolo),
        '_' => Ok(Objeto::Vacio),
        _ => Err("Símbolo no válido en el laberinto"),
    }
}

fn convertir_enemigo(simbolo: &str) -> Result<Objeto, &'static str> {
    if let Some(vida) = simbolo.chars().last().and_then(|c| c.to_digit(10)) {
        if (1..=2).contains(&vida) {
            Ok(Objeto::Enemigo(vida as i32, HashSet::new()))
        } else {
            Err("Valor de vida de enemigo no válido")
        }
    } else {
        Err("No se pudo obtener un valor de vida válido")
    }
}


fn convertir_bomba(simbolo: &str) -> Result<Objeto, &'static str> {
    let alcance_str = &simbolo[1..];
    if let Ok(alcance) = alcance_str.parse::<i32>() {
        if alcance > 0 {
            Ok(Objeto::Bomba(false, alcance))
        } else {
            Err("Valor de alcance de bomba no válido")
        }
    } else {
        Err("No se pudo parsear el valor de alcance de bomba")
    }
}


fn convertir_bomba_traspaso(simbolo: &str) -> Result<Objeto, &'static str> {
    let alcance_str = &simbolo[1..];
    if let Ok(alcance) = alcance_str.parse::<i32>() {
        if alcance > 0 {
            Ok(Objeto::Bomba(true, alcance))
        } else {
            Err("Valor de alcance de bomba de traspaso no válido")
        }
    } else {
        Err("No se pudo parsear el valor de alcance de bomba de traspaso")
    }
}


fn convertir_desvio(simbolo: &str) -> Result<Objeto, &'static str> {
    let direccion = match &simbolo[1..] {
        "U" => Direccion::Arriba,
        "D" => Direccion::Abajo,
        "L" => Direccion::Izquierda,
        "R" => Direccion::Derecha,
        _ => return Err("Dirección de desvío no válida"),
    };
    Ok(Objeto::Desvio(direccion))
}

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn test_convertir_simbolos() {
            // Prueba para convertir un símbolo en un Objeto válido.
            assert_eq!(
                convertir_simbolos("F2"),
                Ok(Objeto::Enemigo(2, HashSet::new()))
            );
            assert_eq!(convertir_simbolos("R"), Ok(Objeto::Roca));
            assert_eq!(convertir_simbolos("W"), Ok(Objeto::Pared));

            // Prueba para convertir un símbolo en una Bomba y Bomba de Traspaso.
            assert_eq!(convertir_simbolos("B2"), Ok(Objeto::Bomba(false, 2)));
            assert_eq!(convertir_simbolos("S1"), Ok(Objeto::Bomba(true, 1)));

            // Prueba para convertir un símbolo en un Desvío.
            assert_eq!(
                convertir_simbolos("DU"),
                Ok(Objeto::Desvio(Direccion::Arriba))
            );
            assert_eq!(
                convertir_simbolos("DL"),
                Ok(Objeto::Desvio(Direccion::Izquierda))
            );

            // Prueba para un símbolo no válido.
            assert_eq!(
                convertir_simbolos("X"),
                Err("Símbolo no válido en el laberinto")
            );

            // Prueba para valores inválidos de enemigo.
            assert_eq!(
                convertir_simbolos("F0"),
                Err("Valor de vida de enemigo no válido")
            );
            assert_eq!(
                convertir_simbolos("F5"),
                Err("Valor de vida de enemigo no válido")
            );

            // Prueba para valores inválidos de alcance de bomba.
            assert_eq!(
                convertir_simbolos("B0"),
                Err("Valor de alcance de bomba no válido")
            );

            // Prueba para valores inválidos de alcance de bomba de traspaso.
            assert_eq!(
                convertir_simbolos("S0"),
                Err("Valor de alcance de bomba de traspaso no válido")
            );
        }
    }
