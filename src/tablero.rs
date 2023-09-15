use crate::objeto::Objeto;
use crate::direccion::Direccion;
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

    pub fn detonar_hacia_arriba(
        &mut self,
        x_usize: usize,
        y_usize: usize,
        alcance: i32,
        traspaso: bool,
    ) {
        let mut seguir_detonando;

        for i in 1..=alcance {
            let i_usize = i as usize;
            if y_usize >= i_usize {
                match self.obtener_objeto_en_posicion(x_usize, y_usize - i_usize) {
                    Some(Objeto::Desvio(dir)) => match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(
                                x_usize,
                                y_usize - i_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(
                                x_usize,
                                y_usize - i_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(
                                x_usize,
                                y_usize - i_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(
                                x_usize,
                                y_usize - i_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                    },
                    Some(_) => {
                        seguir_detonando = self.detonar_en_posicion(
                            x_usize,
                            y_usize - i_usize,
                            traspaso,
                            x_usize,
                            y_usize,
                        );
                        if !seguir_detonando {
                            return;
                        }
                    }
                    None => return,
                }
            }
        }
    }

    pub fn detonar_hacia_abajo(
        &mut self,
        x_usize: usize,
        y_usize: usize,
        alcance: i32,
        traspaso: bool,
    ) {
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            match self.obtener_objeto_en_posicion(x_usize, y_usize + i_usize) {
                Some(Objeto::Desvio(dir)) => match dir {
                    Direccion::Abajo => {
                        self.detonar_hacia_abajo(x_usize, y_usize + i_usize, alcance - i, traspaso);
                    }
                    Direccion::Arriba => {
                        self.detonar_hacia_arriba(
                            x_usize,
                            y_usize + i_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                    Direccion::Izquierda => {
                        self.detonar_hacia_izquierda(
                            x_usize,
                            y_usize + i_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                    Direccion::Derecha => {
                        self.detonar_hacia_derecha(
                            x_usize,
                            y_usize + i_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                },
                Some(_) => {
                    seguir_detonando = self.detonar_en_posicion(
                        x_usize,
                        y_usize + i_usize,
                        traspaso,
                        x_usize,
                        y_usize,
                    );
                    if !seguir_detonando {
                        return;
                    }
                }
                None => return,
            }
        }
    }

    pub fn detonar_hacia_izquierda(
        &mut self,
        x_usize: usize,
        y_usize: usize,
        alcance: i32,
        traspaso: bool,
    ) {
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            if x_usize >= i_usize {
                match self.obtener_objeto_en_posicion(x_usize - i_usize, y_usize) {
                    Some(Objeto::Desvio(dir)) => match dir {
                        Direccion::Abajo => {
                            self.detonar_hacia_abajo(
                                x_usize - i_usize,
                                y_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Arriba => {
                            self.detonar_hacia_arriba(
                                x_usize - i_usize,
                                y_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Izquierda => {
                            self.detonar_hacia_izquierda(
                                x_usize - i_usize,
                                y_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                        Direccion::Derecha => {
                            self.detonar_hacia_derecha(
                                x_usize - i_usize,
                                y_usize,
                                alcance - i,
                                traspaso,
                            );
                        }
                    },
                    Some(_) => {
                        seguir_detonando = self.detonar_en_posicion(
                            x_usize - i_usize,
                            y_usize,
                            traspaso,
                            x_usize,
                            y_usize,
                        );
                        if !seguir_detonando {
                            return;
                        }
                    }
                    None => return,
                }
            }
        }
    }

    pub fn detonar_hacia_derecha(
        &mut self,
        x_usize: usize,
        y_usize: usize,
        alcance: i32,
        traspaso: bool,
    ) {
        let mut seguir_detonando;
        for i in 1..=alcance {
            let i_usize = i as usize;
            match self.obtener_objeto_en_posicion(x_usize + i_usize, y_usize) {
                Some(Objeto::Desvio(dir)) => match dir {
                    Direccion::Abajo => {
                        self.detonar_hacia_abajo(x_usize + i_usize, y_usize, alcance - i, traspaso);
                    }
                    Direccion::Arriba => {
                        self.detonar_hacia_arriba(
                            x_usize + i_usize,
                            y_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                    Direccion::Izquierda => {
                        self.detonar_hacia_izquierda(
                            x_usize + i_usize,
                            y_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                    Direccion::Derecha => {
                        self.detonar_hacia_derecha(
                            x_usize + i_usize,
                            y_usize,
                            alcance - i,
                            traspaso,
                        );
                    }
                },
                Some(_) => {
                    seguir_detonando = self.detonar_en_posicion(
                        x_usize + i_usize,
                        y_usize,
                        traspaso,
                        x_usize,
                        y_usize,
                    );
                    if !seguir_detonando {
                        return;
                    }
                }
                None => return,
            }
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