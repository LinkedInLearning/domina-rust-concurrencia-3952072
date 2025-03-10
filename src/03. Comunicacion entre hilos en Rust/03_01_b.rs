/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::mpsc;
use std::thread;

fn productor(transmisor: mpsc::Sender<&'static str>){
    let palabras = vec!["Bazinga", "Galaxia", "Exterminar"];
    for palabra in palabras {
        transmisor.send(palabra).expect("Error al enviar la palabra");
    }
}

fn consumidor(receptor: mpsc::Receiver<&'static str>){
    for palabra in receptor {
        println!("Palabra recibida: {}", palabra);
    }
}

fn main() {
    
    let (transmisor, receptor) = mpsc::channel();
    
    let hilo_productor = thread::spawn(move || {
        productor(transmisor);
    });
    
    let hilo_consumidor = thread::spawn(move || {
        consumidor(receptor);
    });
    
    hilo_productor.join().expect("Error al esperar el hilo productor");
    hilo_consumidor.join().expect("Error al esperar el hilo consumidor");
}
