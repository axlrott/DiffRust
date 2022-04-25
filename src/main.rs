use lcs::LCSError;
mod lcs;

///Programa que pasados dos archivos por parametro realiza el diff del primero con el segundo
fn main() {
    match lcs::run_lcs() {
        Ok(()) => (),
        Err(LCSError::File(e)) => println!("[{:?}] {}", e, e),
        Err(LCSError::Matrix(e)) => println!("[{:?}] {}", e, e),
        Err(LCSError::Args(e)) => println!("[{:?}] {}", e, e),
    }
}
