/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::thread;

fn main() {

    let libro_recomendado = String::from("https://rust-lang.es/");
   
    let manejador = thread::spawn(move || {
        println!("{}", libro_recomendado);
    });

    println!("Manual de referencia: {}", libro_recomendado);

    manejador.join().unwrap();
}