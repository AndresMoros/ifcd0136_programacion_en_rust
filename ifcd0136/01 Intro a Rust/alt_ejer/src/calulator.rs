// 1. Recibo por teclado 2 números y un operador que podrá ser (+, - , *, /)
// Realizar la operación que corresponda y calcular el resultado.
#[derive(Debug)]
pub struct Calculator {
    x: f64,
    y: f64,
    operator: String,
}

impl Calculator {
    /// Crea un nuevo Struct Calculator
    /// # Parametros
    /// - `x`: i32
    /// - `y`: i32
    /// - `op`: String | Operador para hacer los cálculos
    pub fn new(x: f64, y: f64, op: String) -> Self {
        Calculator {
            x: x,
            y: y,
            operator: op,
        }
    }

    /// Devuelve la el resultado la operación entre dos números: `x` & `y`
    /// # Parametros
    /// - `y`: i32
    /// - `x`: i32
    /// # Devuelve
    /// `(Resultado en i32`, `Estado de la operación`)
    pub fn calc(&self) -> (f64, &str) {
        match self.operator.as_str() {
            "+" => (self.x + self.y, "Ok"),
            "-" => (self.x - self.y, "Ok"),
            "*" => (self.x * self.y, "Ok"),
            "/" => {
                if self.y == 0.0 {
                    (0.0, "No se pueden dividir números entre 0")
                } else {
                    (self.x / self.y, "Ok")
                }
            }
            _ => (0.0, "Error, operador NO válido"),
        }
    }
}
