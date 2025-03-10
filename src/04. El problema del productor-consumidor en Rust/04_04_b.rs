/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn producir_mensajes(transmisor: mpsc::Sender<String>, numero_mensajes: usize){
    for numero_mensaje in 1..=numero_mensajes {
        let mensaje = format!("Mensaje {}", numero_mensaje);
        transmisor.send(mensaje.clone()).unwrap();
        println!("Productor envía: {}", mensaje);
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn main(){
    
    let (transmisor, receptor) = mpsc::channel();
    let receptor_compartido = Arc::new(Mutex::new(receptor));

    thread::spawn(move || {
        producir_mensajes(transmisor, 100);
    });
}