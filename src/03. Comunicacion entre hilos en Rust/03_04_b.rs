/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::Mutex;
use std::thread;

fn main(){

    let numeros = Mutex::new(vec![]);
    
    let hilo = thread::spawn(move || {
        
        let mut numeros_lock = numeros.lock().unwrap(); // MutexGuard<T>
        numeros_lock.push(4);
        numeros_lock.push(8);
        numeros_lock.push(15);
        numeros_lock.push(16);
        numeros_lock.push(23);
        numeros_lock.push(42);
        
        println!("Vector en el subhilo: {:?}", *numeros_lock);
    });
    
    hilo.join().unwrap();
    
    // println!("Vector en el hilo principal: {:?}", numeros);
}