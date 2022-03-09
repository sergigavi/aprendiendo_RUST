
use std::io;

fn main() {

    //inicializo una variable de tipo String 
    let mut entrada = String::new();

    entrada = leer_por_teclado("Tu nombre: ");

    println!("\nHola {}", entrada.trim());

    

}

fn leer_por_teclado(msj: &str) -> String{

    print!("{}", msj);

    let mut s = String::new();

    //para leer por teclado pongo io::stdin().read_line(&mut s)
    
    io::stdin().read_line(&mut s)
    .ok().expect("No se ha leido bien");
    

    return s;

}
