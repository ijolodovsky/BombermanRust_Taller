#[cfg(test)]
mod integration_tests {
    use std::collections::HashSet;

    use tp_individual::tablero::model::direccion::Direccion;
    use tp_individual::tablero::model::objeto::Objeto;
    use tp_individual::tablero::{crear_tablero, guardar_tablero, Tablero};

    #[test]
fn test_detonacion_de_bomba() {
    let maze_content = "B1 R B1 \nW F2 W\n_ W R";

    let maze_file = "test_maze1.txt";
    if let Err(err) = std::fs::write(maze_file, maze_content) {
        assert!(false, "Fallo la creacion del archivo del tablero: {:?}", err);
        return;
    }

    // Crear el tablero desde el archivo de prueba
    let mut tablero = match crear_tablero(maze_file) {
        Ok(tab) => tab,
        Err(err) => {
            println!("Fallo la creacion del tablero: {}", err);
            return;
        }
    };

    if let Objeto::Bomba(false, 1) = tablero.cuadricula[0][0] {
        // Detonar la bomba en la posición (0, 0)
        if let Ok(_) = tablero.detonar(0, 0) {
            // Verificar que la bomba haya sido reemplazada por un espacio vacío después de la detonación
            if let Objeto::Vacio = tablero.cuadricula[0][0] {
                assert_eq!(tablero.cuadricula[0][1], Objeto::Roca);
                assert_eq!(tablero.cuadricula[1][2], Objeto::Pared);
            } else {
                assert!(false, "No pudo detonar la bomba");
            }
        } else {
            assert!(false, "No pudo detonar la bombas");
        }
    } else {
        assert!(false, "Estado inicial de la bomba incorrecto");
    }
}

#[test]
fn test_detonacion_con_desvio() {
    let maze_content = "B2 DD _\nW F2 W\nS3 W DU";

    let maze_file = "test_maze2.txt";
    if let Err(err) = std::fs::write(maze_file, maze_content) {
        assert!(false, "Fallo la creacion del archivo del tablero: {:?}", err);
        return;
    }

    let mut tablero = match crear_tablero(maze_file) {
        Ok(tab) => tab,
        Err(err) => {
            println!("Fallo la creacion del tablero: {}", err);
            return;
        }
    };

    // Verificar que la bomba esté en la posición deseada antes de la detonación
    if let Objeto::Bomba(false, 2) = tablero.cuadricula[0][0] {

        if let Ok(_) = tablero.detonar(0, 0) {

            if let Objeto::Vacio = tablero.cuadricula[0][0] {
                assert_eq!(tablero.cuadricula[1][0], Objeto::Pared);
                if let Objeto::Desvio(Direccion::Abajo) = tablero.cuadricula[0][1] {
                    assert_eq!(tablero.cuadricula[0][2], Objeto::Vacio);
                    if let Objeto::Enemigo(1, set) = &tablero.cuadricula[1][1] {
                        assert!(set.contains(&(0, 0)));
                    } else {
                        assert!(false, "Se esperaba un enemigo en (1, 1)");
                    }
                } else {
                    assert!(false, "Se esperaba DD en (0, 1)");
                }

                if let Objeto::Bomba(true, 3) = tablero.cuadricula[2][0] {
                    return;
                }
            } else {
                assert!(false, "Fallo la detonacion");
            }
        } else {
            assert!(false, "Fallo la detonacion");
        }
    } else {
        assert!(false, "Estado inicial de la bomba incorrecto");
    }
}

#[test]
fn test_errores_detonacion() {
    let maze_content = "B2 _ _\n_ _ _\n_ _ _";
    let maze_file = "test_maze3.txt";
    if let Err(err) = std::fs::write(maze_file, maze_content) {
        assert!(false, "Fallo la creacion del archivo: {:?}", err);
        return;
    }

    let mut tablero = match crear_tablero(maze_file) {
        Ok(tab) => tab,
        Err(err) => {
            println!("Fallo la creacion del tablero: {}", err);
            return;
        }
    };

    // Intentar detonar una bomba en (0, 0), lo cual debe ser exitoso
    if let Ok(_) = tablero.detonar(0, 0) {
        // Intentar detonar una bomba en (1, 1), que no es una bomba (debe generar un error)
        if let Err(err) = tablero.detonar(1, 1) {
            assert!(true, "Se esperaba un error al detonar en (1, 1): {:?}", err);
            assert_eq!(
                err,
                "ERROR: No es una bomba, no se puede detonar.".to_string()
            );
            return;
        } else {
            assert!(false, "Fallo al generar un error en (1, 1)");
        }
    } else {
        assert!(false, "Fallo la detonacion at (0, 0)");
    }
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

    if let Err(err) = tablero.detonar(x, y) {
        assert!(false, "Fallo al detonar la bomba: {:?}", err);
        return;
    }

    let output_dir = ".";
    let output_file = "test_output.txt";
    if let Err(err) = guardar_tablero(output_dir, &tablero, output_file) {
        assert!(false, "Fallo el guardado del tablero: {:?}", err);
        return;
    }

    // Leer el contenido del archivo de salida generado por el programa
    let expected_output = "_ DD _ \n_ F2 R \nW _ B1 \n"; // Define el estado final esperado

    match std::fs::read_to_string(output_file) {
        Ok(actual_output) => {
            // Verificar que el contenido del archivo de salida sea correcto
            assert_eq!(
                expected_output, actual_output,
                "El archivo de salida no coincide con el estado final esperado"
            );

            // Limpiar el archivo de salida después de la prueba
            if let Err(err) = std::fs::remove_file(output_file) {
                assert!(false, "Fallo la eliminacion del output: {:?}", err);
            }
        }
        Err(err) => {
            assert!(false, "fallo la lectura del output: {:?}", err);
        }
    }
}

}
