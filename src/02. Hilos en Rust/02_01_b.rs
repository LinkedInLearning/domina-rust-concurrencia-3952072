/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::thread;

fn tarea(){
    for i in 0..5 {
        println!("Tarea en ejecución: {}", i);
    }
}

fn main(){
    
    let hilo = thread::spawn(tarea);
    
    hilo.join().unwrap();
}