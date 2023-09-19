pub mod model;
use model::direccion::Direccion;
use model::objeto::{convertir_simbolos, Objeto};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Representa un tablero del juego que contiene una cuadrícula de objetos y su tamaño.
pub struct Tablero {
    pub cuadricula: Vec<Vec<Objeto>>,
    pub tamaño: i32,
}

impl Tablero {
    /// Crea un nuevo tablero con el tamaño especificado.
    ///
    /// # Argumentos
    ///
    /// * `tamaño`: Un entero que representa el tamaño del tablero.
    ///
    pub fn new(tamaño: i32) -> Tablero {
        let cuadricula = Vec::new();
        Tablero {
            cuadricula, tamaño
        }
    }

    fn obtener_objeto_en_posicion(&self, x: usize, y: usize) -> Option<&Objeto> {
        if y < self.tamaño as usize && x < self.tamaño as usize {
            Some(&self.cuadricula[y][x])
        } else {
            None
        }
    }

    /// Detona una bomba en las coordenadas especificadas.
    ///
    /// # Argumentos
    ///
    /// * `x` - La coordenada x en la cuadrícula donde se detonará la bomba.
    /// * `y` - La coordenada y en la cuadrícula donde se detonará la bomba.
    ///
    /// # Devuelve
    ///
    /// Devuelve `Ok(())` si la bomba se detonó correctamente, o `Err(String)` si no se pudo detonar
    /// porque no había una bomba en las coordenadas especificadas.
    pub fn detonar(&mut self, x: i32, y: i32) -> Result<(), String> {
        let x_usize = x as usize;
        let y_usize = y as usize;

        match self
            .cuadricula
            .get(y_usize)
            .and_then(|row| row.get(x_usize))
        {
            Some(&Objeto::Bomba(traspaso, alcance)) => {
                self.cuadricula[y_usize][x_usize] = Objeto::Vacio;
                self.detonar_en_direccion(
                    (x, y, x_usize, y_usize, traspaso),
                    Direccion::Arriba,
                    alcance,
                );
                self.detonar_en_direccion(
                    (x, y, x_usize, y_usize, traspaso),
                    Direccion::Abajo,
                    alcance,
                );
                self.detonar_en_direccion(
                    (x, y, x_usize, y_usize, traspaso),
                    Direccion::Izquierda,
                    alcance,
                );
                self.detonar_en_direccion(
                    (x, y, x_usize, y_usize, traspaso),
                    Direccion::Derecha,
                    alcance,
                );
                Ok(())
            }
            _ => Err("ERROR: No es una bomba, no se puede detonar.".to_string()),
        }
    }

    fn detonar_en_direccion(
        &mut self,
        args: (i32, i32, usize, usize, bool),
        direccion: Direccion,
        alcance: i32,
    ) {
        let (x, y, x_usize, y_usize, traspaso) = args;
        let mut i = 1;
        let mut seguir_detonando = true;
        while i <= alcance && seguir_detonando {
            let (nuevo_x, nuevo_y) =
                Self::calcular_nueva_posicion(x_usize, y_usize, direccion.clone(), i);
            if nuevo_x < self.tamaño as usize && nuevo_y < self.tamaño as usize {
                match self.obtener_objeto_en_posicion(nuevo_x, nuevo_y) {
                    Some(Objeto::Desvio(dir)) => {
                        self.detonar_en_direccion(
                            (x, y, nuevo_x, nuevo_y, traspaso),
                            dir.clone(),
                            alcance - i,
                        );
                    }
                    Some(_) => {
                        seguir_detonando =
                            self.detonar_en_posicion(nuevo_x, nuevo_y, traspaso, x, y);
                    }
                    None => seguir_detonando = false,
                }
            } else {
                seguir_detonando = false;
            }
            i += 1;
        }
    }

    fn calcular_nueva_posicion(
        x_usize: usize,
        y_usize: usize,
        direccion: Direccion,
        paso: i32,
    ) -> (usize, usize) {
        match direccion {
            Direccion::Arriba => (x_usize, y_usize.wrapping_sub(paso as usize)),
            Direccion::Abajo => (x_usize, y_usize + paso as usize),
            Direccion::Derecha => (x_usize + paso as usize, y_usize),
            Direccion::Izquierda => (x_usize.wrapping_sub(paso as usize), y_usize),
        }
    }

    fn detonar_en_posicion(
        &mut self,
        x: usize,
        y: usize,
        traspaso: bool,
        x_original: i32,
        y_original: i32,
    ) -> bool {
        match self.cuadricula[y][x] {
            Objeto::Enemigo(ref mut vida, ref mut bombas_afectadas) => {
                if !bombas_afectadas.contains(&(x_original, y_original)) {
                    bombas_afectadas.insert((x_original, y_original));
                    if *vida > 1 {
                        *vida -= 1;
                    } else {
                        self.cuadricula[y][x] = Objeto::Vacio;
                    }
                }
                true
            }
            Objeto::Bomba(_, _) => {
                let _some = self.detonar(x as i32, y as i32);
                true
            }
            Objeto::Roca => traspaso,
            Objeto::Pared => false,
            _ => true,
        }
    }
}

/// Crea un tablero a partir de un archivo de entrada.
///
/// # Argumentos
///
/// * `input_file`: Una cadena de texto que especifica la ubicación del archivo de entrada.
///
/// # Devoluciones
///
/// Devuelve un resultado que contiene el tablero creado o un error de E/S.
///
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

/// Guarda el contenido de un tablero en un archivo de salida en el directorio especificado.
///
/// # Argumentos
///
/// * `output_dir`: Una cadena de texto que especifica el directorio de salida.
/// * `tablero`: Una referencia al tablero que se va a guardar.
/// * `input_file`: Una cadena de texto que contiene el nombre del archivo de entrada (se usará como nombre de salida).
///
/// # Devoluciones
///
/// Devuelve un resultado que indica si la operación de guardar fue exitosa o si ocurrió un error de E/S.
///
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
    use std::collections::HashSet;

    #[test]
    fn test_tablero_new() {
        let tablero = Tablero::new(5);
        assert_eq!(tablero.tamaño, 5);
        assert_eq!(tablero.cuadricula.len(), 0);
    }

    #[test]
    fn test_obtener_objeto_en_posicion() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![Objeto::Vacio, Objeto::Roca, Objeto::Pared],
            vec![
                Objeto::Bomba(false, 2),
                Objeto::Desvio(Direccion::Arriba),
                Objeto::Enemigo(3, HashSet::new()),
            ],
            vec![Objeto::Bomba(true, 1), Objeto::Vacio, Objeto::Roca],
        ];

        assert_eq!(
            tablero.obtener_objeto_en_posicion(0, 0),
            Some(&Objeto::Vacio)
        );
        assert_eq!(
            tablero.obtener_objeto_en_posicion(1, 1),
            Some(&Objeto::Desvio(Direccion::Arriba))
        );
        assert_eq!(
            tablero.obtener_objeto_en_posicion(2, 2),
            Some(&Objeto::Roca)
        );
        assert_eq!(tablero.obtener_objeto_en_posicion(0, 3), None);
        assert_eq!(tablero.obtener_objeto_en_posicion(4, 0), None);
    }

    #[test]
    fn test_detonar() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![Objeto::Bomba(false, 1), Objeto::Vacio, Objeto::Vacio],
            vec![Objeto::Vacio, Objeto::Bomba(true, 2), Objeto::Vacio],
            vec![
                Objeto::Enemigo(2, HashSet::new()),
                Objeto::Roca,
                Objeto::Bomba(false, 1),
            ],
        ];

        // Detonar la bomba en (0, 0)
        let result = tablero.detonar(0, 0);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (0, 0): {:?}",
            result
        );
        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio); // Porque explotó la bomba
        assert_eq!(tablero.cuadricula[1][0], Objeto::Vacio); // Porque explotó la bomba

        // Detonar la bomba en (1, 1)
        let result = tablero.detonar(1, 1);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (1, 1): {:?}",
            result
        );
        assert_eq!(tablero.cuadricula[1][1], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[0][1], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[2][1], Objeto::Roca);

        // Detonar la bomba en (2, 2)
        let result = tablero.detonar(2, 2);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (2, 2): {:?}",
            result
        );
        assert_eq!(tablero.cuadricula[2][2], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[1][2], Objeto::Vacio);
    }

    #[test]
    fn test_detonar_en_posicion() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![
                Objeto::Vacio,
                Objeto::Enemigo(3, HashSet::new()),
                Objeto::Bomba(false, 2),
            ],
            vec![Objeto::Bomba(true, 1), Objeto::Roca, Objeto::Pared],
            vec![
                Objeto::Desvio(Direccion::Derecha),
                Objeto::Vacio,
                Objeto::Bomba(false, 3),
            ],
        ];

        let mut set = HashSet::new();
        set.insert((0, 0));

        let resultado1 = tablero.detonar_en_posicion(1, 0, false, 0, 0);
        assert!(resultado1);
        assert_eq!(tablero.cuadricula[0][1], Objeto::Enemigo(2, set));

        let resultado2 = tablero.detonar_en_posicion(0, 1, true, 1, 1);
        assert!(resultado2);
        assert_eq!(tablero.cuadricula[0][2], Objeto::Bomba(false, 2));
        assert_eq!(tablero.cuadricula[2][0], Objeto::Desvio(Direccion::Derecha));

        let resultado3 = tablero.detonar_en_posicion(2, 2, false, 2, 0);
        assert!(resultado3);
        assert_eq!(tablero.cuadricula[2][2], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[1][2], Objeto::Pared);
    }

    #[test]
    fn test_detonar_bomba_con_diferente_alcance() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![Objeto::Bomba(false, 2), Objeto::Vacio, Objeto::Vacio],
            vec![
                Objeto::Bomba(true, 1),
                Objeto::Bomba(false, 3),
                Objeto::Vacio,
            ],
            vec![
                Objeto::Bomba(true, 1),
                Objeto::Vacio,
                Objeto::Bomba(false, 4),
            ],
        ];

        // Detonar la bomba en (0, 0)
        let result = tablero.detonar(0, 0);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (0, 0): {:?}",
            result
        );
        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[1][0], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[2][0], Objeto::Vacio);

        // Detonar la bomba en (2, 2)
        let result = tablero.detonar(2, 2);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (2, 2): {:?}",
            result
        );
        assert_eq!(tablero.cuadricula[2][2], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[1][2], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[0][2], Objeto::Vacio);
    }

    #[test]
    fn test_detonar_in_direction() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![Objeto::Bomba(false, 2), Objeto::Vacio, Objeto::Vacio],
            vec![
                Objeto::Bomba(true, 2),
                Objeto::Enemigo(3, HashSet::new()),
                Objeto::Roca,
            ],
            vec![
                Objeto::Desvio(Direccion::Derecha),
                Objeto::Enemigo(2, HashSet::new()),
                Objeto::Bomba(false, 2),
            ],
        ];

        tablero.detonar_en_direccion((0, 1, 0, 1, true), Direccion::Derecha, 2);
        let mut set_uno = HashSet::new();
        set_uno.insert((0, 1));
        assert_eq!(tablero.cuadricula[1][1], Objeto::Enemigo(2, set_uno));
        assert_eq!(tablero.cuadricula[1][2], Objeto::Roca);

        tablero.detonar_en_direccion((0, 1, 0, 1, true), Direccion::Abajo, 2);
        let mut set_dos = HashSet::new();
        set_dos.insert((0, 1));
        assert_eq!(tablero.cuadricula[2][0], Objeto::Desvio(Direccion::Derecha));
        assert_eq!(tablero.cuadricula[2][1], Objeto::Enemigo(1, set_dos));
    }

    #[test]
    fn test_acceder_posiciones_fuera_del_limite() {
        let tablero = Tablero::new(2);
        assert_eq!(tablero.obtener_objeto_en_posicion(2, 1), None);
        assert_eq!(tablero.obtener_objeto_en_posicion(1, 2), None);
    }

    #[test]
    fn test_dos_explosiones_en_mismo_enemigo() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![
                Objeto::Bomba(false, 3),
                Objeto::Enemigo(2, HashSet::new()),
                Objeto::Vacio,
            ],
            vec![
                Objeto::Desvio(Direccion::Derecha),
                Objeto::Desvio(Direccion::Arriba),
                Objeto::Roca,
            ],
            vec![Objeto::Pared, Objeto::Vacio, Objeto::Pared],
        ];

        // Detonar la bomba en (0, 0)
        let result = tablero.detonar(0, 0);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (0, 0): {:?}",
            result
        );
        let mut set = HashSet::new();
        set.insert((0, 0));
        assert_eq!(tablero.cuadricula[0][1], Objeto::Enemigo(1, set));
    }

    #[test]
    fn test_detonar_con_pared() {
        let mut tablero = Tablero::new(5);
        tablero.cuadricula = vec![
            vec![
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
            ],
            vec![
                Objeto::Bomba(false, 1),
                Objeto::Bomba(false, 2),
                Objeto::Pared,
                Objeto::Enemigo(1, HashSet::new()),
            ],
            vec![
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
            ],
            vec![
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
            ],
            vec![
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
                Objeto::Vacio,
            ],
        ];

        // Detonar la bomba en (1, 1)
        let result = tablero.detonar(1, 1);
        assert!(
            result.is_ok(),
            "Error al detonar la bomba en (1, 1): {:?}",
            result
        );

        assert_eq!(tablero.cuadricula[1][0], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[1][1], Objeto::Vacio); // Bomba explotó, pero detenida por la pared.
        assert_eq!(tablero.cuadricula[1][2], Objeto::Pared); // La pared detuvo la explosión.
        let set = HashSet::new();
        assert_eq!(tablero.cuadricula[1][3], Objeto::Enemigo(1, set));
    }

    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to create a temporary test file with specified content
    fn create_test_file(file_path: &str, content: &str) -> Result<(), io::Error> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_crear_tablero() {
        // Crear un archivo de prueba 'test_maze.txt' con contenido válido.
        let content = "F2 R\nW DU";
        let file_path = "test_maze.txt";
        let _ = create_test_file(file_path, content).expect("Failed to create test file");

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

    #[test]
    fn test_detonar_en_posicion_no_es_bomba() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![
                Objeto::Vacio,
                Objeto::Enemigo(3, HashSet::new()),
                Objeto::Roca,
            ],
            vec![
                Objeto::Desvio(Direccion::Derecha),
                Objeto::Vacio,
                Objeto::Enemigo(2, HashSet::new()),
            ],
            vec![
                Objeto::Bomba(true, 2),
                Objeto::Pared,
                Objeto::Bomba(false, 2),
            ],
        ];

        // Detonar en una posición que no es una bomba (0, 1).
        let result = tablero.detonar(0, 1);
        assert!(result.is_err(), "Se esperaba un error al detonar en (0, 1)");
        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[0][1], Objeto::Enemigo(3, HashSet::new()));
        assert_eq!(tablero.cuadricula[0][2], Objeto::Roca);
    }

    #[test]
    fn test_detonar_fuera_de_limites() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![
                Objeto::Bomba(true, 2),
                Objeto::Roca,
                Objeto::Enemigo(2, HashSet::new()),
            ],
            vec![
                Objeto::Desvio(Direccion::Izquierda),
                Objeto::Bomba(false, 1),
                Objeto::Vacio,
            ],
            vec![Objeto::Vacio, Objeto::Pared, Objeto::Bomba(true, 1)],
        ];

        // Detonar fuera de los límites del tablero (3, 2).
        let result = tablero.detonar(3, 2);
        assert!(
            result.is_err(),
            "Se esperaba un error al detonar fuera de límites"
        );
        assert_eq!(tablero.cuadricula[2][0], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[2][1], Objeto::Pared);
        assert_eq!(tablero.cuadricula[2][2], Objeto::Bomba(true, 1));
    }
}
