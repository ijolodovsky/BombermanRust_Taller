#[cfg(test)]
mod integration_tests {
    use std::collections::HashSet;

    use tp_individual::tablero::model::direccion::Direccion;
    use tp_individual::tablero::model::objeto::Objeto;
    use tp_individual::tablero::{crear_tablero, guardar_tablero, Tablero};

    #[test]
    fn test_detonacion_de_bomba() {
        // Define el contenido del laberinto de prueba
        let maze_content = "B1 R B1 \nW F2 W\n_ W R";

        // Crear un archivo temporal para el laberinto de prueba
        let maze_file = "test_maze1.txt";
        std::fs::write(maze_file, maze_content).expect("Failed to create test maze file");

        // Crear el tablero desde el archivo de prueba
        let mut tablero = match crear_tablero(maze_file) {
            Ok(tab) => tab,
            Err(err) => {
                println!("Failed to create test maze: {}", err);
                return;
            }
        };

        // Verificar que la bomba esté en la posición deseada antes de la detonación
        assert_eq!(tablero.cuadricula[0][0], Objeto::Bomba(false, 1));

        // Detonar la bomba en la posición (0, 0)
        let result = tablero.detonar(0, 0);

        // Verificar que la detonación fue exitosa
        assert!(result.is_ok(), "Detonation failed: {:?}", result);

        // Verificar que la bomba haya sido reemplazada por un espacio vacío después de la detonación
        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio);

        assert_eq!(tablero.cuadricula[0][1], Objeto::Roca); // Verificar que el objeto en (1, 0) sea una roca
        assert_eq!(tablero.cuadricula[1][2], Objeto::Pared); // Verificar que el objeto en (2, 1) sea una pared
    }

    #[test]
    fn test_detonacion_con_desvio() {
        let maze_content = "B2 DD _\nW F2 W\nS3 W DU";

        let maze_file = "test_maze2.txt";
        std::fs::write(maze_file, maze_content).expect("Failed to create test maze file");

        let mut tablero = match crear_tablero(maze_file) {
            Ok(tab) => tab,
            Err(err) => {
                println!("Failed to create test maze: {}", err);
                return;
            }
        };

        // Verificar que la bomba esté en la posición deseada antes de la detonación
        assert_eq!(tablero.cuadricula[0][0], Objeto::Bomba(false, 2));

        // Detonar la bomba en la posición (0, 0)
        let result = tablero.detonar(0, 0);

        // Verificar que la detonación fue exitosa
        assert!(result.is_ok(), "ERROR: {:?}", result);

        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio);

        assert_eq!(tablero.cuadricula[1][0], Objeto::Pared);
        assert_eq!(tablero.cuadricula[0][1], Objeto::Desvio(Direccion::Abajo));
        assert_eq!(tablero.cuadricula[0][2], Objeto::Vacio);
        let mut set = HashSet::new();
        set.insert((0, 0));
        assert_eq!(tablero.cuadricula[1][1], Objeto::Enemigo(1, set));

        assert_eq!(tablero.cuadricula[2][0], Objeto::Bomba(true, 3))
    }

    #[test]
    fn test_errores_detonacion() {
        let maze_content = "B2 _ _\n_ _ _\n_ _ _";
        let maze_file = "test_maze3.txt";
        std::fs::write(maze_file, maze_content).expect("Fallo al crear el archivo");

        let mut tablero = match crear_tablero(maze_file) {
            Ok(tab) => tab,
            Err(err) => {
                println!("Fallo al crear el tablero: {}", err);
                return;
            }
        };

        // Intentar detonar una bomba en (0, 0), lo cual debe ser exitoso
        let result = tablero.detonar(0, 0);
        assert!(result.is_ok(), "ERROR: {:?}", result);

        // Intentar detonar una bomba en (1, 1), que no es una bomba (debe generar un error)
        let result = tablero.detonar(1, 1);
        assert!(
            result.is_err(),
            "Se esperaba un error al detonar (1, 1): {:?}",
            result
        );
        assert_eq!(
            result.err(),
            Some("ERROR: No es una bomba, no se puede detonar.".to_string())
        );
    }

    #[test]
    fn test_archivos_salida() {
        let mut tablero = Tablero::new(3);
        tablero.cuadricula = vec![
            vec![
                Objeto::Bomba(false, 2),
                Objeto::Desvio(Direccion::Abajo),
                Objeto::Vacio,
            ],
            vec![
                Objeto::Vacio,
                Objeto::Enemigo(3, HashSet::new()),
                Objeto::Roca,
            ],
            vec![Objeto::Pared, Objeto::Vacio, Objeto::Bomba(false, 1)],
        ];

        let x = 0;
        let y = 0;

        let resultado = tablero.detonar(x, y);
        assert!(resultado.is_ok(), "La detonación de la bomba falló");

        let output_dir = ".";
        let output_file = "test_output.txt";
        let resultado_guardado = guardar_tablero(output_dir, &tablero, output_file);
        assert!(resultado_guardado.is_ok(), "El guardado del tablero falló");

        // Leer el contenido del archivo de salida generado por el programa
        let expected_output = "_ DD _ \n_ F2 R \nW _ B1 \n"; // Define el estado final esperado

        let actual_output =
            std::fs::read_to_string(output_file).expect("Fallo la lectura del output file");

        // Verificar que el contenido del archivo de salida sea correcto
        assert_eq!(
            expected_output, actual_output,
            "El archivo de salida no coincide con el estado final esperado"
        );

        // Limpiar el archivo de salida después de la prueba
        std::fs::remove_file(output_file).expect("Fallo el borrado del output file");
    }
}
