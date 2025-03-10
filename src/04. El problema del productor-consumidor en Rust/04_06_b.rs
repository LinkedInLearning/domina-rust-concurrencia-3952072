/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn producir_mensajes(transmisor: mpsc::Sender<String>, numero_mensajes: usize) {
    for numero_mensaje in 1..=numero_mensajes {
        let mensaje = format!("Mensaje {}", numero_mensaje);
        transmisor.send(mensaje.clone()).unwrap();
        println!("Productor envía: {}", mensaje);
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn consumir_mensajes(id_consumidor: usize, receptor: Arc<Mutex<mpsc::Receiver<String>>>) {
    loop {
        let mensaje = receptor.lock().unwrap().recv();
        match mensaje {
            Ok(mensaje_recibido) => {
                println!("Consumidor {} recibe: {}", id_consumidor, mensaje_recibido);
                thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => break,
        }
    }
}

fn main() {

    let (transmisor, receptor) = mpsc::channel();
    let receptor_compartido = Arc::new(Mutex::new(receptor));

    let mut manejadores = vec![];
    for id_consumidor in 1..=30 {
        let receptor_clonado = Arc::clone(&receptor_compartido);
        let manejador = thread::spawn(move || {
            consumir_mensajes(id_consumidor, receptor_clonado);
        });
        manejadores.push(manejador);
    }

    thread::spawn(move || {
        producir_mensajes(transmisor, 100);
    });

    for manejador in manejadores {
        manejador.join().unwrap();
    }
}