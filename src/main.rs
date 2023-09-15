/*
COSAS A CORREGIR:
- Las funciones y los tipos de datos (struct) deben estar documentados siguiendo
    el estándar de cargo doc.
-Ver de modularizar mas y dividir en archivos
- Se deben implementar tests unitarios y de integración de las funcionalidades
    que se consideren más importantes.
*/

mod utils;
mod objeto;
mod direccion;
mod tablero;

use objeto::Objeto;
use direccion::Direccion;
use tablero::Tablero;

use utils::{crear_tablero, guardar_tablero};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Uso incorrecto. Ejemplo: cargo run -- maze.txt /path/to/output_dir/ x y");
        return;
    }

    let input_file = &args[1];
    let output_dir = &args[2];
    let x = match args[3].parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            guardar_error(
                output_dir,
                "ERROR: La coordenada X debe ser un número válido.",
                input_file,
            );
            return;
        }
    };
    let y = match args[4].parse::<i32>() {
        Ok(y) => y,
        Err(_) => {
            guardar_error(
                output_dir,
                "ERROR: La coordenada Y debe ser un número válido.",
                input_file,
            );
            return;
        }
    };
    // Crea el tablero a partir del archivo de entrada.
    let mut tablero = match crear_tablero(input_file) {
        Ok(t) => t,
        Err(e) => {
            guardar_error(output_dir, &format!("ERROR: {}", e), input_file);
            return;
        }
    };

    // Detona la bomba en las coordenadas especificadas.
    tablero.detonar(x, y);

    // Guarda el estado del tablero en el archivo de salida.
    match guardar_tablero(output_dir, &tablero, input_file) {
        Ok(_) => {}
        Err(e) => guardar_error(output_dir, &format!("ERROR: {}", e), input_file),
    }
}

fn guardar_error(output_dir: &str, error_message: &str, input_file: &str) {
    let output_file_name = input_file;
    let output_file_path = Path::new(output_dir).join(output_file_name);
    match File::create(output_file_path) {
        Ok(mut file) => {
            if writeln!(file, "{}", error_message).is_err() {
                println!("Error al escribir el mensaje de error en el archivo.");
            }
        }
        Err(_) => {
            println!("Error al crear el archivo de salida.");
        }
    }
}
