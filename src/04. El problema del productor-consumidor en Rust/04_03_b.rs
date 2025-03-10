/*
 * Curso: Domina Rust: Concurrencia
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn main(){
    
    let (transmisor, receptor) = mpsc::channel();
    let receptor_compartido = Arc::new(Mutex::new(receptor));
}