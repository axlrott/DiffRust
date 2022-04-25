//! Modulo que va a servir como un contenedor de archivos perteneciente al modulo LCS
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

///Estructura que se utiliza para leer archivos con los cuales hacer diff mediante LCS
#[derive(Debug)]
pub struct FileLCS {
    reader: BufReader<File>,
}
///Enum de los errores posibles de la estructura FileLCS
#[derive(PartialEq, Debug)]
pub enum FileLCSError {
    Open(String),
}

impl fmt::Display for FileLCSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            FileLCSError::Open(_) => "Hubo un error al intentar abrir el archivo",
        };
        f.write_str(description)
    }
}

impl FileLCS {
    ///Crea el FileLCS dado un String con la direccion del archivo
    pub fn new(dir: &str) -> Result<Self, FileLCSError> {
        if let Ok(file) = File::open(dir) {
            let reader = BufReader::new(file);
            Ok(Self { reader })
        } else {
            Err(FileLCSError::Open(dir.to_owned()))
        }
    }
    ///Funcion que devuelve un string con la siguiente linea del archivo en minusculas,
    /// en caso de no haber mas lineas devuelve None
    pub fn read_line(&mut self) -> Option<String> {
        let mut buffer = String::new();
        match self.reader.read_line(&mut buffer) {
            Ok(a) if a > 0 => Some(buffer.trim().to_owned()),
            _ => None,
        }
    }
    ///Funcion que imprime el diff comparando con otro FileLCS que se la haya pasado,
    /// la comparacion va a ser linea por linea
    pub fn diff(&mut self, file_old: &mut FileLCS) -> super::matrix::MatrixResult<()> {
        while let Some(line_new) = self.read_line() {
            if let Some(line_old) = file_old.read_line() {
                let tuple_old: (&str, usize) = (&line_old, line_old.len());
                let tuple_new: (&str, usize) = (&line_new, line_new.len());
                let lcs_matrix = super::lcs(&line_new, &line_old)?;
                super::print_diff(lcs_matrix, tuple_old, tuple_new);
            } else {
                println!(">> {}", line_new);
            }
        }
        while let Some(line_old) = file_old.read_line() {
            println!("<< {}", line_old);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_lcs_ok() {
        let file_name = "test_file.txt";
        let file_lcs = FileLCS::new(file_name);
        assert!(file_lcs.is_ok())
    }
    #[test]
    fn create_file_lcs_error() {
        let file_lcs = FileLCS::new("no_file.txt");
        assert_eq!(
            FileLCSError::Open("no_file.txt".to_owned()),
            file_lcs.unwrap_err()
        )
    }
    #[test]
    fn read_line_file_lcs() {
        let file_name = "test_file.txt";
        let file_lcs = FileLCS::new(file_name);
        assert!(file_lcs.is_ok());
        let mut reader = file_lcs.unwrap();
        assert_eq!(Some("test1".to_owned()), reader.read_line());
        assert_eq!(Some("test2".to_owned()), reader.read_line());
        assert_eq!(Some("test3".to_owned()), reader.read_line());
        assert_eq!(None, reader.read_line());
    }
}
