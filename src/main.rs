/*
COSAS A CORREGIR:
- El output consiste en un archivo en el filesystem (con el mismo nombre que el 
    archivo de input), localizado en el segundo parametro de invocacion del programa,
     donde debera guardarse el estado del laberinto de luego de haber detonado la 
     bomba. En caso de no existir el archivo, este debera ser creado.
    En caso de que un error ocurriese, se deberá escribir en el archivo un mensaje de 
    error con el siguiente formato: ERROR: [descripcion_del_error].
- Las funciones y los tipos de datos (struct) deben estar documentados siguiendo 
    el estándar de cargo doc.
- El código debe formatearse utilizando cargo fmt.
- Cada tipo de dato implementado debe ser colocado en una unidad de compilación 
    (archivo fuente) independiente.
-Ver de modularizar mas y dividir en archivos
- Se deben implementar tests unitarios y de integración de las funcionalidades 
    que se consideren más importantes.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::io::Write;
use std::collections::HashSet;

#[derive(Clone)]
enum Objeto{
    Enemigo(i32, HashSet<(usize, usize)>),
    Bomba(bool, i32),
    Roca,
    Pared,
    Desvio (Direccion),
    Vacio,
}

#[derive(Clone)]
enum Direccion{
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

struct Tablero {
    cuadricula: Vec<Vec<Objeto>>,
    tamaño: i32,
}

impl Tablero {
    fn new(tamaño: i32) -> Tablero{
        let cuadricula = Vec::new();

        Tablero {cuadricula, tamaño}
    }

    fn obtener_objeto_en_posicion(&self, x: usize, y: usize) -> Option<&Objeto> {
        if y < self.tamaño as usize && x < self.tamaño as usize {
            Some(&self.cuadricula[y][x])
        } else {
            None
        }
    }

    fn detonar(&mut self, x: i32, y: i32) {    
        let x_usize = x as usize;
        let y_usize = y as usize;
    
        match self.cuadricula[y_usize][x_usize] {
            Objeto::Bomba(traspaso, alcance) =>{
                self.cuadricula[y_usize][x_usize] = Objeto::Vacio;
                self.detonar_hacia_arriba(x_usize, y_usize, alcance, traspaso);
                self.detonar_hacia_abajo(x_usize, y_usize, alcance, traspaso);
                self.detonar_hacia_izquierda(x_usize, y_usize, alcance, traspaso);
                self.detonar_hacia_derecha(x_usize, y_usize, alcance, traspaso);   
            }
            _ => {
                // No es una bomba, no se puede detonar.
            }
        }
    }

    fn detonar_hacia_arriba(&mut self, x_usize: usize, y_usize: usize, alcance: i32, traspaso: bool) {
        let mut seguir_detonando;

        for i in 1..=alcance {
            let i_usize = i as usize;
            if y_usize >= i_usize{
            match self.obtener_objeto_en_posicion(x_usize, y_usize - i_usize) {
                Some(Objeto::Desvio(dir)) => {
                    match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(x_usize, y_usize - i_usize, alcance - i, traspaso);
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(x_usize, y_usize - i_usize, alcance - i, traspaso);
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(x_usize, y_usize - i_usize, alcance - i, traspaso);
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(x_usize, y_usize - i_usize, alcance - i, traspaso);
                        }
                    }
                    
                }
                Some(_) => {
                    seguir_detonando=self.detonar_en_posicion(x_usize, y_usize - i_usize, traspaso, x_usize, y_usize);
                    if !seguir_detonando{
                        return;
                    }
                }
                None => return,
            }
        }
            
        
    }
    }
    
    fn detonar_hacia_abajo(&mut self, x_usize: usize, y_usize: usize, alcance: i32, traspaso: bool) {
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            match self.obtener_objeto_en_posicion(x_usize, y_usize + i_usize) {
                Some(Objeto::Desvio(dir)) => {
                    match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(x_usize, y_usize + i_usize, alcance - i, traspaso);
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(x_usize, y_usize + i_usize, alcance - i, traspaso);
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(x_usize, y_usize + i_usize, alcance - i, traspaso);
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(x_usize, y_usize + i_usize, alcance - i, traspaso);
                        }
                    }
                }
                Some(_) => {
                    seguir_detonando=self.detonar_en_posicion(x_usize, y_usize + i_usize, traspaso, x_usize, y_usize);
                    if !seguir_detonando{
                        return;
                    }
                }
                None => return,
            }
        }
    }
    
    fn detonar_hacia_izquierda(&mut self, x_usize: usize, y_usize: usize, alcance: i32, traspaso: bool){
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            if x_usize >= i_usize{
            match self.obtener_objeto_en_posicion(x_usize - i_usize, y_usize) {
                Some(Objeto::Desvio(dir)) => {
                    match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(x_usize - i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(x_usize - i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(x_usize - i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(x_usize - i_usize, y_usize, alcance - i, traspaso);
                        }
                    }
                }
                Some(_) => {
                    seguir_detonando= self.detonar_en_posicion(x_usize - i_usize, y_usize, traspaso, x_usize, y_usize);
                    if !seguir_detonando{
                        return;
                    }
                }
                None => return,
            }
        }
        }
    }
    
    fn detonar_hacia_derecha(&mut self, x_usize: usize, y_usize: usize, alcance: i32, traspaso: bool) {
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            match self.obtener_objeto_en_posicion(x_usize + i_usize, y_usize) {
                Some(Objeto::Desvio(dir)) => {
                    match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(x_usize + i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(x_usize + i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(x_usize + i_usize, y_usize, alcance - i, traspaso);
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(x_usize + i_usize, y_usize, alcance - i, traspaso);
                        }
                    }
                }
                Some(_) => {
                    seguir_detonando=self.detonar_en_posicion(x_usize + i_usize, y_usize, traspaso, x_usize, y_usize);
                    if !seguir_detonando{
                        return;
                    }
                }
                None => return,
            }
        }
    }
    
    
    fn detonar_en_posicion(&mut self, x: usize, y: usize, traspaso: bool, x_original: usize, y_original: usize) -> bool {
        match self.cuadricula[y][x] {
            Objeto::Enemigo(ref mut vida, ref mut bombas_afectadas) => {
                if bomba_no_afecto_al_enemigo(x_original, y_original, bombas_afectadas){
                    bombas_afectadas.insert((x_original, y_original));
                    if *vida > 1 {
                        *vida -= 1;
                    } else{
                        self.cuadricula[y][x] = Objeto::Vacio;
                    }
                }
                true
            }
            Objeto::Bomba(_, _) => {
                self.detonar(x as i32, y as i32);
                true
            }
            Objeto::Roca =>{
                !traspaso
            }
            Objeto::Pared => {
                false
            }
            _ => {
                true
            }
        }
    }
}

fn convertir_simbolos(simbolo: &str) -> Result<Objeto, &'static str> {
    let first_char = simbolo.chars().next().ok_or("El símbolo está vacío")?;

    match first_char {
        'F' => {
            if let Some(vida) = simbolo.chars().last().and_then(|c| c.to_digit(10)) {
                Ok(Objeto::Enemigo(vida as i32,HashSet::new()))
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

fn bomba_no_afecto_al_enemigo(x: usize, y: usize, bombas_afectadas: &HashSet<(usize, usize)>) -> bool{
    !bombas_afectadas.contains(&(x, y))
}

fn crear_tablero(input_file: &str) -> Result<Tablero, io::Error> {
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


fn guardar_tablero(output_dir: &str, tablero: &Tablero) -> Result<(), io::Error> {
    let output_path = Path::new(output_dir);
    if !output_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Directorio de salida no existe",
        ));
    }
    let output_file_name = "output.txt";
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

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Uso incorrecto. Ejemplo: cargo run -- maze.txt /path/to/output_dir/ x y");
        return;
    }

    let input_file = &args[1];
    let output_dir = &args[2];
    let x = args[3].parse::<i32>().expect("La coordenada X debe ser un número válido.");
    let y = args[4].parse::<i32>().expect("La coordenada Y debe ser un número válido.");

    // Crea el tablero a partir del archivo de entrada.
    let mut tablero = match crear_tablero(input_file) {
        Ok(t) => t,
        Err(e) => {
            println!("Error al crear el tablero: {}", e);
            return;
        }
    };

    // Detona la bomba en las coordenadas especificadas.
    tablero.detonar(x, y);

    // Guarda el estado del tablero en el archivo de salida.
    match guardar_tablero(output_dir, &tablero) {
        Ok(_) => println!("Juego guardado exitosamente en {}", output_dir),
        Err(e) => println!("Error al guardar el juego: {}", e),
    }
}
