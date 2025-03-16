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
    
    for i in 0..3 {
        let acceso_seguro = Arc::clone(&vector);
        let hilo = thread::spawn(move || {
            let mut generador = rand::thread_rng();
            loop {
                let mut escritura_exclusiva = acceso_seguro.write().unwrap();
                let nuevo_valor = generador.gen_range(0..=108);
                escritura_exclusiva.push(nuevo_valor);
                println!("Escritor {} añade {} al vector", i, nuevo_valor);
                thread::sleep(Duration::from_millis(60));
            }
        });

        hilos.push(hilo);
    }

    for hilo in hilos {
        hilo.join().unwrap();
    }
}