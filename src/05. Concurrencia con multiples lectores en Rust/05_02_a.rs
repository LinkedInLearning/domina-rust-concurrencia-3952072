/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, Mutex};
use std::thread;

fn main(){

    let recurso1 = Arc::new(Mutex::new(0));
    let recurso2 = Arc::new(Mutex::new(0));

    let referencia1 = Arc::clone(&recurso1);
    let referencia2 = Arc::clone(&recurso2);

    let hilo1 = thread::spawn(move || {
        let _bloqueo1 = referencia1.lock().unwrap();
        println!("El hilo 1 bloquea el recurso 1");

        std::thread::sleep(std::time::Duration::from_millis(50));

        let _bloqueo2 = referencia2.lock().unwrap();
        println!("El hilo 1 bloquea el recurso 2");
    });

    let referencia1 = Arc::clone(&recurso1);
    let referencia2 = Arc::clone(&recurso2);

    let hilo2 = thread::spawn(move || {
        let _bloqueo2 = referencia2.lock().unwrap();
        println!("El hilo 2 bloquea el recurso 2");

        std::thread::sleep(std::time::Duration::from_millis(50));

        let _bloqueo1 = referencia1.lock().unwrap();
        println!("El hilo 2 bloquea el recurso 1");
    });

    hilo1.join().unwrap();
    hilo2.join().unwrap();
}