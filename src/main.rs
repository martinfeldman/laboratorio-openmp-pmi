use std::sync::{Arc, Mutex};
use std::thread;
use reqwest;
use serde_json::Value;
use std::time::Instant;
use tokio::time::{sleep, Duration};


fn main() {

    ejecucion_sin_hilos();
    ejecucion_con_hilos();

}



#[tokio::main]
async fn ejecucion_sin_hilos() {

    // Realizar la solicitud HTTP en el programa principal
    let response = reqwest::blocking::get("https://catfact.ninja/breeds")
        .expect("Error al llamar a la API")
        .text()
        .expect("Error al obtener el cuerpo de la respuesta");


    // Parsear el JSON en un valor genérico y clonarlo para cada hilo
    let json_data: Value = serde_json::from_str(&response).expect("Error al parsear el JSON");


    // Registra el tiempo de inicio
    let start_time = Instant::now();

    println!("\nComenzando ejecución sin hilos\n");


    // Imprimimos por pantalla cada elemento del array data con un segundo de pausa
    for breed_info in json_data["data"].as_array().unwrap_or(&Vec::new()) {
        println!("{:?}", breed_info);
        let _ = sleep(Duration::from_secs(1)).await;
    }


    // Registra el tiempo de finalización
    let end_time = Instant::now();

    // Calcula la duración de ejecución
    let duration = end_time.duration_since(start_time);

    // Convierte la duración en milisegundos (puedes usar as_secs para segundos, etc.)
    let milliseconds = duration.as_millis();

    // Imprime el tiempo de ejecución
    println!("\n\nTiempo de ejecución sin hilos: {} milisegundos\n", milliseconds);

}



#[tokio::main]
async fn ejecucion_con_hilos() {

    // Crear un valor compartido para almacenar la respuesta de la API
    let shared_response = Arc::new(Mutex::new(String::new()));
    
    // Crear un valor compartido para el índice actual
    let shared_index = Arc::new(Mutex::new(0));

    // Realizar la solicitud HTTP en el programa principal
    let response = reqwest::blocking::get("https://catfact.ninja/breeds")
        .expect("Error al llamar a la API")
        .text()
        .expect("Error al obtener el cuerpo de la respuesta");

    // Almacenar la respuesta en la variable compartida
    {
        let mut shared_response = shared_response.lock().unwrap();
        *shared_response = response.clone();
    }


    // Parsear el JSON en un valor genérico y clonarlo para cada hilo
    let json_data: Value = serde_json::from_str(&response).expect("Error al parsear el JSON");
    let shared_json_data = Arc::new(json_data);


    // Obtener el número total de elementos en "data"
    let data_len = shared_json_data["data"].as_array().unwrap().len();


    // Registra el tiempo de inicio
    let start_time = Instant::now();


    println!("Comenzando ejecución con hilos\n");

    // Define the number of threads you want to create
    let num_threads = 7;
    let mut threads = Vec::with_capacity(num_threads);

    for i in 1..=num_threads {

        let shared_index = Arc::clone(&shared_index);
        let shared_json_data = Arc::clone(&shared_json_data);

        let thread = thread::spawn(move || {
            while let Some(breed_info) = get_next_element(&shared_json_data, &shared_index, data_len) {
                println!("Proceso {}: {:?}", i, breed_info);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        threads.push(thread);

    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }



    // Registra el tiempo de finalización
    let end_time = Instant::now();

    // Calcula la duración de ejecución
    let duration = end_time.duration_since(start_time);

    // Convierte la duración en milisegundos (puedes usar as_secs para segundos, etc.)
    let milliseconds = duration.as_millis();

    // Imprime el tiempo de ejecución
    println!("\n\nTiempo de ejecución con {} hilos: {} milisegundos", num_threads, milliseconds);

}




fn get_next_element(json_data: &Value, shared_index: &Mutex<i32>, data_len: usize) -> Option<Value> {

    let mut index = shared_index.lock().unwrap();
    if *index < data_len as i32 {
        let element = json_data["data"][*index as usize].clone();
        *index += 1;
        Some(element)
    } else {
        None
    }

}
