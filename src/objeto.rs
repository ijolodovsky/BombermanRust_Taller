use std::collections::HashSet;
use crate::direccion::Direccion;

#[derive(Clone)]
pub enum Objeto {
    Enemigo(i32, HashSet<(usize, usize)>),
    Bomba(bool, i32),
    Roca,
    Pared,
    Desvio(Direccion),
    Vacio,
}
