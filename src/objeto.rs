use crate::direccion::Direccion;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Objeto {
    Enemigo(i32, HashSet<(usize, usize)>),
    Bomba(bool, i32),
    Roca,
    Pared,
    Desvio(Direccion),
    Vacio,
}
