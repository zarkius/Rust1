use std::io;

fn main() {
    println!("Hello, world!");

    let x: i32 = 32;

    println!("The value of x is: {}", x);


    let mi_array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("El valor del array es: {:?}", mi_array);




    fn leer_numero(){
        println!("Introduce un numero");
        let mut numero = String::new();
        io::stdin()
        .read_line(&mut numero)
        .expect("Error al leer la linea");
        
        println!("El numero introducido es: {}", numero);
    }

    
    for _ in 0..5 {
        leer_numero();
    }
}