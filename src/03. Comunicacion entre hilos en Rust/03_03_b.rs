/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::Arc;
use std::thread;

fn main(){
    
    let numeros = Arc::new(vec![4, 8, 15, 16, 23, 42]);
    let mut hilos = vec![];
    
    for i in 0..3 {
        let numeros_cloned = Arc::clone(&numeros);
        let hilo = thread::spawn(move || {
            println!("Hilo {} accediendo a los datos: {:?}", i, numeros_cloned);
        });
        hilos.push(hilo);
    }
    
    for hilo in hilos {
        hilo.join().unwrap();
    }
}