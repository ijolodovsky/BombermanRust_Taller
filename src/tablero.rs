use crate::direccion::Direccion;
use crate::objeto::Objeto;

pub struct Tablero {
    pub cuadricula: Vec<Vec<Objeto>>,
    pub tamaño: i32,
}

impl Tablero {
    pub fn new(tamaño: i32) -> Tablero {
        let cuadricula = Vec::new();
        Tablero {
            cuadricula, tamaño
        }
    }

    pub fn obtener_objeto_en_posicion(&self, x: usize, y: usize) -> Option<&Objeto> {
        if y < self.tamaño as usize && x < self.tamaño as usize {
            Some(&self.cuadricula[y][x])
        } else {
            None
        }
    }

    pub fn detonar(&mut self, x: i32, y: i32) {
        let x_usize = x as usize;
        let y_usize = y as usize;
        match self.cuadricula[y_usize][x_usize] {
            Objeto::Bomba(traspaso, alcance) => {
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
            }
            _ => {
                // No es una bomba, no se puede detonar.
            }
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

    pub fn calcular_nueva_posicion(
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

    pub fn detonar_en_posicion(
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
                self.detonar(x as i32, y as i32);
                true
            }
            Objeto::Roca => traspaso,
            Objeto::Pared => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direccion::Direccion;
    use crate::objeto::Objeto;
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

        tablero.detonar(0, 0);
        assert_eq!(tablero.cuadricula[0][0], Objeto::Vacio); //Porque explotò la bomba
        assert_eq!(tablero.cuadricula[1][0], Objeto::Vacio); //Porque exploto la bomba

        tablero.detonar(1, 1);
        assert_eq!(tablero.cuadricula[1][1], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[0][1], Objeto::Vacio);
        assert_eq!(tablero.cuadricula[2][1], Objeto::Roca);

        tablero.detonar(2, 2);
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
}
