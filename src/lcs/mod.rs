//! Modulo de LCS que contiene una matriz sobre la cual se va a realizar LCS y un
//! contenedor de archivos para ir haciendo el diff de los mismos linea por linea
use std::cmp;
use std::env;
use std::fmt;

pub mod file;
pub mod matrix;

///Enum de los posibles errores de argumentos pasados por consola
#[derive(Debug)]
pub enum ArgsError {
    NoArgs,
    InvalidArgs,
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            ArgsError::NoArgs => "No se pasaron argumentos",
            ArgsError::InvalidArgs => "La cantidad de argumentos pasados es erronea",
        };
        f.write_str(description)
    }
}

///Enum de los distintos tipos de errores que puede dar run_lcs
#[derive(Debug)]
pub enum LCSError {
    Matrix(matrix::MatrixError),
    File(file::FileLCSError),
    Args(ArgsError),
}

///Funcion que devuelve una matriz de LCS de los dos Strings que se le hayan pasado
///
/// En caso de que haya un error al crear la matriz se devolvera el mismo
fn lcs(line_new: &str, line_old: &str) -> matrix::MatrixResult<matrix::Matrix> {
    let mut element: u32;
    let len_new = line_new.len();
    let len_old = line_old.len();

    let new_lower = line_new.to_lowercase();
    let old_lower = line_old.to_lowercase();

    let mut matrix = matrix::Matrix::new(len_old + 1, len_new + 1);
    let chars_new = new_lower.as_bytes();
    let chars_old = old_lower.as_bytes();

    for (i, value_new) in chars_old.iter().enumerate().take(len_old) {
        for (j, value_old) in chars_new.iter().enumerate().take(len_new) {
            if *value_new == *value_old {
                element = matrix.get_element(i, j)?;
                element += 1;
                matrix.set_element(i + 1, j + 1, element)?;
            } else {
                let elem1 = matrix.get_element(i, j + 1)?;
                let elem2 = matrix.get_element(i + 1, j)?;
                element = cmp::max(elem1, elem2);
                matrix.set_element(i + 1, j + 1, element)?;
            }
        }
    }
    Ok(matrix)
}
///Funcion que recibe dos tuplas del tipo (&str[linea de texto], usize[longitud de la linea de texto])
/// y su matriz de LCS, esta funcion va a imprimir por pantalla el diff entre las dos lineas de texto pasadas
/// siendo la primer tupla pasada la del texto anterior y la segunda la del texto nuevo
fn print_diff(matrix: matrix::Matrix, old_line: (&str, usize), new_line: (&str, usize)) {
    let (old_str, old_len) = old_line;
    let (new_str, new_len) = new_line;

    let same_len = old_len == new_len;
    if let Ok(cant_equals) = matrix.get_element(old_len, new_len) {
        if same_len && cant_equals == new_len as u32 {
            println!("{}", new_str);
        } else {
            println!(">> {}", new_str);
            println!("<< {}", old_str);
        }
    }
    println!();
}

///Esta funcion toma los dos argumentos pasados por consola, los pasa a FileLCS y luego realiza el diff de los mismos
/// En caso de que suceda algun error se devolvera el mismo que sera del tipo LCSError
pub fn run_lcs() -> Result<(), LCSError> {
    let new_file: String;
    let old_file: String;

    if let Some(first_file) = env::args().nth(1) {
        if let Some(second_file) = env::args().nth(2) {
            new_file = first_file;
            old_file = second_file;
        } else {
            return Err(LCSError::Args(ArgsError::InvalidArgs));
        }
    } else {
        return Err(LCSError::Args(ArgsError::NoArgs));
    }

    let first_file = file::FileLCS::new(&new_file);
    let second_file = file::FileLCS::new(&old_file);
    match first_file {
        Ok(mut new_file) => match second_file {
            Ok(mut old_file) => match new_file.diff(&mut old_file) {
                Ok(()) => Ok(()),
                Err(e) => Err(LCSError::Matrix(e)),
            },
            Err(e) => Err(LCSError::File(e)),
        },
        Err(e) => Err(LCSError::File(e)),
    }
}
