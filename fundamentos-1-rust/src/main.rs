fn main() {
    println!("Fundamentos rust y primeros pasos"); //obviamente esto tambien es un proyecto cargo pa que se me quede mejor organizado

    //con el let asigno valor a constantes (sí, por defecto todas son variable inmutables (constantes))
    let a_number = 10;
    let a_boolean = true;

    println!("el numero es {}", a_number);
    println!("el booleano es {}", a_boolean);

    //para que una variable sea mutable (no sea constante) debemos indicarla como let mut

    let mut numero = 0;
    println!("variable numero mutable es {}", numero);

    numero = 1;
    println!("variable numero mutable es {}", numero);

    //puedes crear varias variables con el mismo nombre y se crearán varios enlaces a memoria -> esto se llama Shadowing (propiedad de reemplazo) -> se sobreescribe basicamente
    //la variable nueva prevalece sobre la anterior, la antigua sigue existiendo pero ya desde ahí no se puede acceder a ella con punteros ni nada en ese ambito
    let num = 5;
    let num = num +5;
    let num = num * 2;
    println!("El numero es; {}", num);   

    //hay que indicar los tipos, el compilador puede deducir algunos valores pero mejor tiparlo
    let numero: u32 = "42".parse().expect("No es un numero!");  //u32 -> numero de 32 bits
    //el parse lo parsea al tipo que le he indicado a la variable, en este caso lo parsea a u32
    //con el expect mostramos un mensaje de error si falla
    println!("Numero: {}", numero);
    //sin signo u32 //desde el 0 hasta el +32bits
    //con signo i32 //desde el -16b hasta el +16b tanto

    let x = 2.0; //float 64 bits por defecto
    let y: f32; //coma flotante, float de 32 bits

    


}
