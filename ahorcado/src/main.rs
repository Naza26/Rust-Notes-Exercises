use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn crear_palabra_oculta(largo_palabra: i32) -> String {
    let mut palabra = String::new();

    let mut i: i32 = 0;

    while i < largo_palabra {
        palabra.push('_');
        palabra.push(' ');
        i = i + 1;
    }

    palabra
}

fn leer_palabra_ingresada() -> String {
    let mut letra_ingresada = String::new();

    io::stdin()
        .read_line(&mut letra_ingresada)
        .expect("Falló al leer linea");
    
    letra_ingresada
}

fn mostrar_mensajes(largo_palabra: i32) -> String {
    let word_to_guess = crear_palabra_oculta(largo_palabra);
    println!("La palabra hasta el momento es: {word_to_guess}");
    println!("Adivinaste las siguientes letras: ");
    println!("Te quedan {largo_palabra} intentos");

    println!("Ingrese una letra");

    word_to_guess
}

fn encontrar_indice(palabra: String, letra: String) -> i32 {
    let mut indice: i32 = 0;
    for c in palabra.chars() {
        if c == letra.parse().unwrap() {
            return indice;
        }
        indice = indice + 1;
    }
    -1
}

fn main() -> std::io::Result<()> {

    println!("Bienvenido al Ahorcado de FIUBA!");

    let mut palabra_a_adivinar = String::new();
    let mut largo_palabra = 0;

    let file = File::open("archivo.txt")
        .expect("file not found!");
    let  buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        palabra_a_adivinar = line?.parse().unwrap();
        largo_palabra = palabra_a_adivinar.len();
        // println!("{}", palabra_a_adivinar);
        // println!("{}", largo_palabra);
        let casillero_palabra = mostrar_mensajes(largo_palabra.try_into().unwrap());

        while largo_palabra > 0 {

            let letra_ingresada = leer_palabra_ingresada();
        
            // let indice = palabra_a_adivinar.find(&letra_ingresada);

            let i: i32 = encontrar_indice(palabra_a_adivinar.clone(), letra_ingresada);
    
            if i == -1 {
                largo_palabra = largo_palabra - 1;
                // println!("Se ha ingresado incorrectamente la letra: {}", letra_ingresada);
            } else {
                println!("{}", casillero_palabra);
                println!("La palabra hasta el momento es: {casillero_palabra}");
            }
        }
    }

    Ok(())


}

