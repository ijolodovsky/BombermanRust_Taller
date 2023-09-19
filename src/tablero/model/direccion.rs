#[derive(PartialEq, Debug, Clone)]
/// Enumeración que representa las direcciones posibles.
pub enum Direccion {
    /// Dirección hacia arriba.
    Arriba,
    /// Dirección hacia abajo.
    Abajo,
    /// Dirección hacia la izquierda.
    Izquierda,
    /// Dirección hacia la derecha.
    Derecha,
}
