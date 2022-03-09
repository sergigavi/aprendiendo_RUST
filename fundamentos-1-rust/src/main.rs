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

    miPersona.nombre;
    miPersona.edad;
    miPersona.le_gustan_las_naranjas;

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

    let e:EventoWeb = EventoWeb::Cargada;

    struct Click{
        x: i64,
        y: i64
    }

    //funciones

    funcion1();

    let f:bool = es_divisible_entre(100, 50);
    println!("{}", f);

    //colecciones

    // -> arrays, tienen un tamaño conocido e inmutable, son listas elementos del mismo tipo

    let mut diasDeLaSemana = ["lunes", "martes", "miércoles", "jueves", "viernes", "sábado", "domingo"];

    println!("Lunes->{}\nTamaño del array: {}", diasDeLaSemana[0], diasDeLaSemana.len());

    
    //vectores, arrays pero su tamaño puede aumentarse o reducirse en otro momento.

    let mut vector3Numeros = vec![1,2,3];
    println!("vector de 3 numeros: {:?}", vector3Numeros);

    let mut diezCeros = vec![0; 10];
    println!("Diez ceros: {:?}", diezCeros);

    //se pueden crear variables de tipos de varias formas, con macros: vec!, println!, casi todo lo que tiene ! son macros
    // yluego tambien con el Vec::new()

    //en lugar de hacer -> let mut v = vec![1]      //habria que pasarle un parametro que lo inicializara si o si con algun valor dentro
    //hago              -> let mut v = Vec::new()   //en este caso se inicia a empty

    //creo un vector vacío

    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);

    //para sacar el ultimo parametro se puede usar 
    let ultimo = v.pop();
    println!("{:?}", ultimo);

    
    //HashMaps -> clave: valor

    //std es la libreria estandar

        //imports
    use std::collections::HashMap;
    

    let mut miHashMap: HashMap<String, String> = HashMap::new();

    miHashMap.insert("clave1".to_string(), "valor1".to_string());
    miHashMap.insert("clave2".to_string(), "valor2".to_string());
    miHashMap.insert("clave3".to_string(), "valor3".to_string());
    miHashMap.insert("clave4".to_string(), "valor4".to_string());

    //el .to_string() transforma un valor literal de cadena (&str) en String
    //si fuese &str sería una coleccion de punteros, ya que guarda la referencia, una direccion de memoria, pero al hacerlo String lo guarda directamente el valor

    let claveABuscar = "clave1"; //no hace falta ponerle al tostring ya que pueden usar referencias para las consultas
    if !miHashMap.contains_key(claveABuscar)
    {
        println!("Si existe un elemento con clave: {}", claveABuscar)
    }
    else{
        println!("No existe un elemento con clave: {}", claveABuscar)
    }

    //se puede buscar por key
    println!("valor de {}: {}",claveABuscar, miHashMap[claveABuscar]);

    //basicamente si pongo miHashMap["clave"] me devuelve el valor de esa clave, siempre que exista, sino explota y panickea
    println!("{}", miHashMap["clave1"].to_string());

    //para evitar el panic se puede usar el .get() en los mapas
    
    miHashMap.get("clave1");


    //hashset
    //use std::collections::HashSet;


    //tambien se pueden usar los if como expresiones

    let b = true;

    let saludo = if b {
        "Buenas"
    } else{
        "No buenas"
    };

    println!("{}", saludo);


    //bucle infinito

    let mut i = 0;
    loop{
        
        i+=1;   //i = i + 1

        if i == 10{
            println!("i es: {}", i);
            break
        }
    }

    println!("he salido del bucle");

    //puedes asignar un loop a una variable y devolverle el valor que quieres que tome con el break

    i = 1;

    let algo = loop { //esto se podría pasar de linea perfectamente como en java
        i *= 2; //i = i * 2

        if i > 100 {
            break i
        }
    };

    println!("{}", algo);

    //bucles while

    let mut contador = 0;

    while contador < 10 {
        println!("hola contador {}", contador);
        contador += 1;
    }

    //bucles for

    //elemento iterable

    let a = [10, 20, 30, 40, 50]; //array

    for i in a.iter() {
        println!("El valor es: {}", i);
    }

    //o bien

    for i in 0..5 {
        println!("{}", i);
    }



}

//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------

fn funcion1()
{
    println!("funcion 1");
}

fn es_divisible_entre(dividendo: u32, divisor: u32) -> bool{

    let mut sino:bool = false;

    if dividendo % divisor == 0{
        sino = true;
    }

    return sino;   //esto al ser la última linea me la retorna como si llevase un return delante
}




