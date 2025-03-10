/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    
    let numeros = Arc::new(Mutex::new(vec![]));
    
    let mut hilos = vec![];
    
    for i in 0..5{
        let numeros_clone = Arc::clone(&numeros);
        
        let hilo = thread::spawn(move || {
            let mut guarda = numeros_clone.lock().unwrap();
            guarda.push(i);
            println!("Hilo {:?} añade {}", thread::current().id(), i);
        });
        
        hilos.push(hilo);
    }
    
    for hilo in hilos {
        hilo.join().unwrap();
    }
    
    println!("Vector final: {:?}", *numeros.lock().unwrap());
}