/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::thread;

fn tarea(){
    for _i in 0..5 {
        println!("Hilo trabajando...");
    }
}

fn main(){
    
    println!("Hilo principal al comienzo");
    let manejador = thread::spawn(tarea);
    
    println!("Hilo principal en proceso");
    manejador.join().unwrap();
    
    println!("Hilo principal finalizando...")
}