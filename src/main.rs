/*
COSAS A CORREGIR:
- Las funciones y los tipos de datos (struct) deben estar documentados siguiendo
    el estándar de cargo doc.
- Se deben implementar tests unitarios y de integración de las funcionalidades
    que se consideren más importantes.
*/

mod tablero;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tablero::{crear_tablero, guardar_tablero};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err_msg) = run(args) {
        println!("{}", err_msg);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() != 5 {
        return Err(
            "Uso incorrecto. Ejemplo: cargo run -- maze.txt /path/to/output_dir/ x y".to_string(),
        );
    }

    let input_file = &args[1];
    let output_dir = &args[2];
    let x = parse_coordenadas(&args[3], "X")?;
    let y = parse_coordenadas(&args[4], "Y")?;

    let mut tablero = match crear_tablero(input_file) {
        Ok(t) => t,
        Err(e) => {
            guardar_error(output_dir, &format!("ERROR: {}", e), input_file);
            return Ok(());
        }
    };

    match tablero.detonar(x, y) {
        Ok(_) => match guardar_tablero(output_dir, &tablero, input_file) {
            Ok(_) => Ok(()),
            Err(e) => {
                guardar_error(output_dir, &format!("ERROR: {}", e), input_file);
                Ok(())
            }
        },
        Err(err_msg) => {
            guardar_error(output_dir, &format!("ERROR: {}", err_msg), input_file);
            Ok(())
        }
    }
}

fn parse_coordenadas(coord_str: &str, coord_nombre: &str) -> Result<i32, String> {
    coord_str.parse::<i32>().map_err(|_| {
        format!(
            "ERROR: La coordenada {} debe ser un número válido.",
            coord_nombre
        )
    })
}

fn guardar_error(output_dir: &str, error_message: &str, input_file: &str) {
    let output_file_name = input_file;
    let output_file_path = Path::new(output_dir).join(output_file_name);
    match File::create(output_file_path) {
        Ok(mut file) => {
            if write!(file, "{}", error_message).is_err() {
                println!("Error al escribir el mensaje de error en el archivo.");
            }
        }
        Err(_) => {
            println!("Error al crear el archivo de salida.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::remove_dir_all;

    const TEMP_DIR_NAME: &str = "test_temp_dir";

    #[test]
    fn test_parse_coordenadas_valid() {
        // Prueba con una cadena que se puede analizar correctamente
        assert_eq!(parse_coordenadas("42", "X"), Ok(42));
    }

    #[test]
    fn test_parse_coordenadas_invalid() {
        // Prueba con una cadena no válida
        assert_eq!(
            parse_coordenadas("no_valido", "Y"),
            Err("ERROR: La coordenada Y debe ser un número válido.".to_string())
        );
    }

    #[test]
    fn test_guardar_error() {
        // Crear un directorio temporal para las pruebas
        fs::create_dir(TEMP_DIR_NAME).expect("Error al crear el directorio temporal");

        let temp_dir_path = &TEMP_DIR_NAME;
        let error_message = "Este es un mensaje de error de prueba";
        let input_file = "archivo_prueba.txt";

        guardar_error(temp_dir_path, error_message, input_file);

        // Leer el contenido del archivo y verificar si es igual al mensaje de error
        let output_file_path = Path::new(temp_dir_path).join(input_file);
        let contents =
            fs::read_to_string(output_file_path).expect("Error al leer el archivo de salida");

        assert_eq!(contents, error_message);

        // Eliminar el directorio temporal después de la prueba
        remove_dir_all(temp_dir_path).expect("Error al eliminar el directorio temporal");
    }
}
