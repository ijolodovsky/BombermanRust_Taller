use crate::direccion::Direccion;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub enum Objeto {
    Enemigo(i32, HashSet<(i32, i32)>),
    Bomba(bool, i32),
    Roca,
    Pared,
    Desvio(Direccion),
    Vacio,
}
