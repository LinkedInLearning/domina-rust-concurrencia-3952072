/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {

    let numero = Arc::new(RwLock::new(108));
    let mut hilos = vec![];
    for _ in 0..5 {
        let lectura_dato = Arc::clone(&numero);
        let hilo = thread::spawn(move || {
            let bloqueo_lectura = lectura_dato.read().unwrap();
            println!("{}", *bloqueo_lectura);
        });

        hilos.push(hilo);
    }

    for hilo in hilos {
        hilo.join().unwrap();
    }
}