use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::{Objeto, Tablero, Direccion};
use std::io::Write;

pub fn convertir_simbolos(simbolo: &str) -> Result<Objeto, &'static str> {
    let first_char = simbolo.chars().next().ok_or("El símbolo está vacío")?;

    match first_char {
        'F' => {
            if let Some(vida) = simbolo.chars().last().and_then(|c| c.to_digit(10)) {
                Ok(Objeto::Enemigo(vida as i32, HashSet::new()))
            } else {
                Err("Valor de vida de enemigo no válido")
            }
        }
        'B' => {
            let alcance_str = &simbolo[1..];
            if let Ok(alcance) = alcance_str.parse::<i32>() {
                Ok(Objeto::Bomba(false, alcance))
            } else {
                Err("Valor de alcance de bomba no válido")
            }
        }
        'S' => {
            let alcance_str = &simbolo[1..];
            if let Ok(alcance) = alcance_str.parse::<i32>() {
                Ok(Objeto::Bomba(true, alcance))
            } else {
                Err("Valor de alcance de bomba de traspaso no válido")
            }
        }
        'R' => Ok(Objeto::Roca),
        'W' => Ok(Objeto::Pared),
        'D' => {
            let direccion = match &simbolo[1..] {
                "U" => Direccion::Arriba,
                "D" => Direccion::Abajo,
                "L" => Direccion::Izquierda,
                "R" => Direccion::Derecha,
                _ => return Err("Dirección de desvío no válida"),
            };
            Ok(Objeto::Desvio(direccion))
        }
        '_' => Ok(Objeto::Vacio),
        _ => Err("Símbolo no válido en el laberinto"),
    }
}

pub fn bomba_no_afecto_al_enemigo(
    x: usize,
    y: usize,
    bombas_afectadas: &HashSet<(usize, usize)>,
) -> bool {
    !bombas_afectadas.contains(&(x, y))
}

pub fn crear_tablero(input_file: &str) -> Result<Tablero, io::Error> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut tablero = None;

    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();

        for simbolo in line.split_whitespace() {
            match convertir_simbolos(simbolo) {
                Ok(objeto) => row.push(objeto.clone()),
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            }
        }

        if tablero.is_none() {
            let size = row.len() as i32;
            tablero = Some(Tablero::new(size));
        }

        if let Some(ref mut t) = tablero {
            t.cuadricula.push(row);
        }
    }

    if let Some(t) = tablero {
        Ok(t)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No se pudo crear el tablero",
        ))
    }
}

pub fn guardar_tablero(output_dir: &str, tablero: &Tablero, input_file: &str) -> Result<(), io::Error> {
    let output_path = Path::new(output_dir);
    if !output_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Directorio de salida no existe",
        ));
    }
    let output_file_name = input_file;
    let output_file_path = output_path.join(output_file_name);
    let mut file = File::create(&output_file_path)?;

    for row in &tablero.cuadricula {
        for objeto in row {
            let symbol = match objeto {
                Objeto::Enemigo(vida, _) => format!("F{}", vida),
                Objeto::Bomba(false, alcance) => format!("B{}", alcance),
                Objeto::Bomba(true, alcance) => format!("S{}", alcance),
                Objeto::Roca => "R".to_string(),
                Objeto::Pared => "W".to_string(),
                Objeto::Desvio(direccion) => match direccion {
                    Direccion::Arriba => "DU".to_string(),
                    Direccion::Abajo => "DD".to_string(),
                    Direccion::Izquierda => "DL".to_string(),
                    Direccion::Derecha => "DR".to_string(),
                },
                Objeto::Vacio => "_".to_string(),
            };
            write!(file, "{} ", symbol)?;
        }
        writeln!(file)?;
    }
    Ok(())
}