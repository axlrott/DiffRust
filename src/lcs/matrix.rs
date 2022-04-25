//! Modulo de matriz perteneciente a LCS
use std::fmt;
pub type MatrixResult<T> = Result<T, MatrixError>;

///Estructura que va a representar a una matriz
#[derive(Debug)]
pub struct Matrix {
    fil: usize,
    col: usize,
    pub vector: Vec<u32>,
}
///Enum de los errores posibles de la estructura Matrix
#[derive(PartialEq, Debug)]
pub enum MatrixError {
    OutOfIndex,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            MatrixError::OutOfIndex => "La posicion ingresada esta fuera de rango",
        };
        f.write_str(description)
    }
}

impl Matrix {
    ///Crea una nueva matriz con el numero de filas y el numero de columnas que se le pase
    /// # Ejemplo
    ///
    /// ```
    /// let matrix = Matrix::new(3,3);
    /// ```
    /// Esto crearia una matriz de 3 filas y 3 columnas inicializadas en ceros
    pub fn new(fil: usize, col: usize) -> Self {
        let len = fil * col;
        Self {
            fil,
            col,
            vector: vec![0; len],
        }
    }
    ///Valida si la posicion dada es valida en la matriz
    fn pos_valid(&self, pos_fil: usize, pos_col: usize) -> bool {
        !(pos_fil >= self.fil || pos_col >= self.col)
    }
    ///Setea el elemento dado en la posicion ingresada en la matriz
    ///
    /// En caso de ingresar una posicion no valida de la matriz devolvera el error OutOfIndex
    pub fn set_element(&mut self, fil: usize, col: usize, element: u32) -> MatrixResult<()> {
        if self.pos_valid(fil, col) {
            let pos = (self.col * fil) + col;
            self.vector[pos as usize] = element;
            Ok(())
        } else {
            Err(MatrixError::OutOfIndex)
        }
    }
    ///Devuelve el elemento perteneciente a la posicion dada
    ///
    /// En caso de ingresar una posicion no valida se devolvera None
    pub fn get_element(&self, fil: usize, col: usize) -> MatrixResult<u32> {
        if self.pos_valid(fil, col) {
            let pos = (self.col * fil) + col;
            Ok(self.vector[pos as usize])
        } else {
            Err(MatrixError::OutOfIndex)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_matrix_ok() {
        let vector_init = vec![0; 9];
        let matrix = Matrix::new(3, 3);
        assert_eq!(matrix.vector, vector_init)
    }
    #[test]
    fn set_value_matrix_ok() {
        let mut matrix = Matrix::new(3, 3);
        assert_eq!(Ok(()), matrix.set_element(0, 0, 5))
    }
    #[test]
    fn set_value_matrix_err() {
        let mut matrix = Matrix::new(2, 2);
        assert_eq!(Err(MatrixError::OutOfIndex), matrix.set_element(3, 3, 5))
    }
    #[test]
    fn get_value_matrix_error_ok() {
        let mut matrix = Matrix::new(3, 3);
        let res = matrix.set_element(0, 0, 5);
        assert_eq!(Ok(()), res);
        assert_eq!(matrix.get_element(0, 0), Ok(5))
    }
    #[test]
    fn get_value_matrix_error() {
        let matrix = Matrix::new(2, 2);
        assert_eq!(matrix.get_element(5, 5), Err(MatrixError::OutOfIndex))
    }
}
