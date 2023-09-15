use crate::direccion::Direccion;
use crate::objeto::Objeto;
use crate::utils::bomba_no_afecto_al_enemigo;

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
                self.detonar_en_direccion(x_usize, y_usize, Direccion::Arriba, alcance, traspaso);
                self.detonar_en_direccion(x_usize, y_usize, Direccion::Abajo, alcance, traspaso);
                self.detonar_en_direccion(
                    x_usize,
                    y_usize,
                    Direccion::Izquierda,
                    alcance,
                    traspaso,
                );
                self.detonar_en_direccion(x_usize, y_usize, Direccion::Derecha, alcance, traspaso);
            }
            _ => {
                // No es una bomba, no se puede detonar.
            }
        }
    }

    pub fn detonar_en_direccion(
        &mut self,
        x_usize: usize,
        y_usize: usize,
        direccion: Direccion,
        alcance: i32,
        traspaso: bool,
    ) {
        let mut i = 1;
        let mut seguir_detonando = true;

        while i <= alcance && seguir_detonando {
            let (nuevo_x, nuevo_y) =
                Self::calcular_nueva_posicion(x_usize, y_usize, direccion.clone(), i);
            if nuevo_x < self.tamaño as usize && nuevo_y < self.tamaño as usize {
                match self.obtener_objeto_en_posicion(nuevo_x, nuevo_y) {
                    Some(Objeto::Desvio(dir)) => {
                        self.detonar_en_direccion(
                            nuevo_x,
                            nuevo_y,
                            dir.clone(),
                            alcance - i,
                            traspaso,
                        );
                    }
                    Some(_) => {
                        seguir_detonando =
                            self.detonar_en_posicion(nuevo_x, nuevo_y, traspaso, x_usize, y_usize);
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
        x_original: usize,
        y_original: usize,
    ) -> bool {
        match self.cuadricula[y][x] {
            Objeto::Enemigo(ref mut vida, ref mut bombas_afectadas) => {
                if bomba_no_afecto_al_enemigo(x_original, y_original, bombas_afectadas) {
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
