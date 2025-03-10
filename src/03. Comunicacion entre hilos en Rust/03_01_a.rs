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
    
}
