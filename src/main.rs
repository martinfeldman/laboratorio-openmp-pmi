use std::sync::{Arc, Mutex};
use std::thread;
use reqwest;
use serde_json::Value;

fn main() {

    // Registra el tiempo de inicio
    let start_time = Instant::now();



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


    // Crear el primer proceso
    let thread1 = thread::spawn({
        let _shared_response = Arc::clone(&shared_response);
        let shared_index = Arc::clone(&shared_index);
        let shared_json_data = Arc::clone(&shared_json_data);
        move || {
            while let Some(breed_info) = get_next_element(&shared_json_data, &shared_index, data_len) {
                println!("Proceso 1: {:?}", breed_info);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });


    // Crear el segundo proceso
    let thread2 = thread::spawn({
        let _shared_response = Arc::clone(&shared_response);
        let shared_index = Arc::clone(&shared_index);
        let shared_json_data = Arc::clone(&shared_json_data);
        move || {
            while let Some(breed_info) = get_next_element(&shared_json_data, &shared_index, data_len) {
                println!("Proceso 2: {:?}", breed_info);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });


    // Esperar a que ambos procesos terminen
    thread1.join().unwrap();
    thread2.join().unwrap();



    // Coloca aquí el código de tu programa

    // Registra el tiempo de finalización
    let end_time = Instant::now();

    // Calcula la duración de ejecución
    let duration = end_time.duration_since(start_time);

    // Convierte la duración en milisegundos (puedes usar as_secs para segundos, etc.)
    let milliseconds = duration.as_millis();

    // Imprime el tiempo de ejecución
    println!("Tiempo de ejecución: {} milisegundos", milliseconds);

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
