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
    let y: f32 = 2.0; //coma flotante, float de 32 bits

    println!("{}{}", x, y);

    //le indico el tipo directamente a un valor y al operar con él ya el resultado sabe de qué tipo es
    println!("Adición de tipo -> 1 + 2 = {}", 1u32 + 2); //el resultado lo muestra del tipo u32

    //en rust hay tipos tanto cadena(String) como caracter 
    //Caracter -> char //para un caracter solo
    //Cadena -> str (aunque realmente se pone como &str), String
    //un string directo es tipo &str, "mistring" sería tipo &str
    let mut string ="mi string aunque es del tipo &str";
    println!("{}", string);
    string =  "mi string aunque es del tipo &str mutado";
    println!("{}", string);

    let mut hello = String::from("Hello, ");
    hello.push('w'); //añado un caracter a mi string
    hello.push_str("orld!"); //añado una cadena a mi string
    println!("{}", hello);

    //&str tiene un tamaño conocido e inmutable
    //String tiene un tamaño desconocido y mutable, puede cambiar en tiempo de ejecucion y hacerse más grande

    //Tuplas , array de longitud fija
    let array = ("hello", 5i32, 'c');

    //para acceder a elementos del array se usa array.0 ... sería como array[0] pero con array.0

    //assert_eq es como una funcion que recibe argumentos, devuelve true o false dependiendo si las dos expresiones de su interior son iguales
    assert_eq!(array.0, "hello");
    assert_eq!(array.1, 5);
    assert_eq!(array.2, 'c');

    println!("{}", array.0);
    //las tuplas permiten combinar en un solo valor varios tipos, en este caso estamos guardando tipo char, str y tipo i32

    //estructuras: similar a objeto
    //3 tipos

    //1 -> Con campos con nombre
    struct Persona {
        nombre: String,
        edad: u8,
        le_gustan_las_naranjas: bool
    }

    //2 -> Estructura en tupla, sus campos no tienen nombre //es como tipar una tupla con un nombre
    struct Punto2d(u32, u32);

    //3 -> Unitaria //uso como marcadores
    struct PUnitaria;

    //creo un objeto del tipo
    //asigno las variables de mi tipo de objeto

    let miPersona = Persona{nombre: String::from("Sergio"), le_gustan_las_naranjas:true, edad:19};

    let miPunto2d = Punto2d(0,0);

    let miUnitaria = PUnitaria;

    //enumerados - enums

    enum EventoWeb{

        Cargada, //Unitarios
        NoCargada,
        TeclaPulsada(char), //caracter
        PegarTexto(String), //String
        Click{x: i64, y: i64} //estructura (objeto) tipo tupla con coordenadas
    }






}
