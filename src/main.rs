/*
COSAS A CORREGIR:
- Las funciones y los tipos de datos (struct) deben estar documentados siguiendo
    el estándar de cargo doc.
- Se deben implementar tests unitarios y de integración de las funcionalidades
    que se consideren más importantes.
*/
mod direccion;
mod objeto;
mod tablero;
mod utils;

use direccion::Direccion;
use objeto::Objeto;
use tablero::Tablero;
use utils::{crear_tablero, guardar_tablero};

use std::env;

fn main() {
    if let Err(error_message) = run() {
        println!("{}", error_message);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(
            "Uso incorrecto. Ejemplo: cargo run -- maze.txt /path/to/output_dir/ x y".to_string(),
        );
    }

    let input_file = &args[1];
    let output_dir = &args[2];
    let x = args[3].parse::<i32>().map_err(|_| {
        format!(
            "ERROR: La coordenada X debe ser un número válido. {}",
            input_file
        )
    })?;
    let y = args[4].parse::<i32>().map_err(|_| {
        format!(
            "ERROR: La coordenada Y debe ser un número válido. {}",
            input_file
        )
    })?;

    let mut tablero = crear_tablero(input_file).map_err(|e| format!("ERROR: {}", e))?;
    tablero.detonar(x, y);

    guardar_tablero(output_dir, &tablero, input_file).map_err(|e| format!("ERROR: {}", e))?;
    Ok(())
}
