use crate::{Direccion, Objeto, Tablero};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn convertir_simbolos(simbolo: &str) -> Result<Objeto, &'static str> {
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
        Ok(Objeto::Enemigo(vida as i32, HashSet::new()))
    } else {
        Err("Valor de vida de enemigo no válido")
    }
}

fn convertir_bomba(simbolo: &str) -> Result<Objeto, &'static str> {
    let alcance_str = &simbolo[1..];
    if let Ok(alcance) = alcance_str.parse::<i32>() {
        Ok(Objeto::Bomba(false, alcance))
    } else {
        Err("Valor de alcance de bomba no válido")
    }
}

fn convertir_bomba_traspaso(simbolo: &str) -> Result<Objeto, &'static str> {
    let alcance_str = &simbolo[1..];
    if let Ok(alcance) = alcance_str.parse::<i32>() {
        Ok(Objeto::Bomba(true, alcance))
    } else {
        Err("Valor de alcance de bomba de traspaso no válido")
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

    tablero.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No se pudo crear el tablero"))
}

pub fn guardar_tablero(
    output_dir: &str,
    tablero: &Tablero,
    input_file: &str,
) -> Result<(), io::Error> {
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
        esribir_linea_en_archivo(&mut file, row)?;
    }

    Ok(())
}

fn esribir_linea_en_archivo(file: &mut File, row: &[Objeto]) -> Result<(), io::Error> {
    for objeto in row {
        let simbolo = match objeto {
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
        write!(file, "{} ", simbolo)?;
    }
    writeln!(file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to create a temporary test file with specified content
    fn create_test_file(file_path: &str, content: &str) -> Result<(), io::Error> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_convertir_simbolos() {
        // Prueba para convertir un símbolo en un Objeto válido.
        assert_eq!(
            convertir_simbolos("F3"),
            Ok(Objeto::Enemigo(3, HashSet::new()))
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
    }

    #[test]
    fn test_crear_tablero() {
        // Crear un archivo de prueba 'test_maze.txt' con contenido válido.
        let content = "F2 R\nW DU";
        let file_path = "test_maze.txt";
        create_test_file(file_path, content).expect("Failed to create test file");

        // Prueba para crear un tablero a partir del archivo de prueba.
        let resultado = crear_tablero(file_path);
        assert!(resultado.is_ok(), "La creación del tablero falló");

        // Verificar que el tablero tenga el tamaño correcto.
        if let Ok(tablero) = resultado {
            assert_eq!(tablero.tamaño, 2, "El tamaño del tablero es incorrecto");
        }

        // Prueba para crear un tablero a partir de un archivo inexistente.
        let resultado = crear_tablero("archivo_inexistente.txt");
        assert!(
            resultado.is_err(),
            "Se esperaba un error al crear el tablero"
        );

        // Limpiar el archivo de prueba después de usarlo.
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_guardar_tablero() {
        // Crear un tablero de prueba.
        let mut tablero = Tablero::new(2);
        tablero.cuadricula = vec![
            vec![Objeto::Bomba(false, 1), Objeto::Enemigo(2, HashSet::new())],
            vec![Objeto::Roca, Objeto::Pared],
        ];

        // Crear un nombre de archivo temporal único.
        let output_dir = ".";
        let output_file = "test_output.txt";

        // Guardar el tablero en el archivo temporal.
        let resultado = guardar_tablero(output_dir, &tablero, output_file);
        assert!(resultado.is_ok(), "La operación de guardar tablero falló");

        // Verificar que el archivo se haya creado.
        let output_file_path = format!("{}/{}", output_dir, output_file);
        assert!(
            Path::new(&output_file_path).exists(),
            "El archivo de salida no existe"
        );

        // Leer el contenido del archivo y verificar que coincida con el tablero original.
        let file_content =
            fs::read_to_string(output_file_path.clone()).expect("Failed to read output file");
        assert_eq!(
            file_content, "B1 F2 \nR W \n",
            "El contenido del archivo no coincide"
        );

        // Eliminar el archivo temporal después de usarlo.
        fs::remove_file(output_file_path).expect("Failed to remove output file");
    }
}
