/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;

fn main(){
    let contador = Arc::new(AtomicUsize::new(0));
    let mut hilos = vec![];

    for _ in 0..4{
        let contador = Arc::clone(&contador);
        let hilo = thread::spawn(move || {
            for _ in 0..8 {
                contador.fetch_add(1, Ordering::Relaxed);
                thread::sleep(Duration::from_millis(10));
            }
        });
        hilos.push(hilo);
    }

    for _ in 0..4 {
        let contador = Arc::clone(&contador);
        let hilo = thread::spawn(move || {
            for _ in 0..8 {
                let numero = contador.load(Ordering::Relaxed);
                println!("Valor del contador: {}", numero);
                thread::sleep(Duration::from_millis(10));
            }
        });
        hilos.push(hilo);
    }
}