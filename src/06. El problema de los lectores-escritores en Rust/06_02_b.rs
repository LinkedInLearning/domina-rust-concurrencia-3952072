/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main(){
 
    let vector = Arc::new(RwLock::new(vec![4, 8]));
    let mut hilos = vec![];
     
    for i in 0..10 {
        let acceso_seguro = Arc::clone(&vector);
        let hilo = thread::spawn(move || {
            loop {
                let lectura = acceso_seguro.read().unwrap();
                println!("Lector {} lee. Lectura actual: {:?}", i, *lectura);
                thread::sleep(Duration::from_millis(50));
            }
        });
         
        hilos.push(hilo);
    }
     
    for hilo in hilos {
        hilo.join().unwrap();
    }
}