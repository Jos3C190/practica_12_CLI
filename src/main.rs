use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    loop {
        println!("Menú:");
        println!("1. Añadir archivo de texto");
        println!("2. Visualizar archivo de texto");
        println!("3. Editar archivo de texto");
        println!("4. Verificar si existe archivo");
        println!("5. Eliminar archivo");
        println!("6. Salir");
        println!("Elija una opción:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada.");

        match input.trim() {
            "1" => {
                println!("Nombre del archivo de texto:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Error al leer la entrada.");
                let name = name.trim();

                println!("Contenido del archivo de texto:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Error al leer la entrada.");

                if create_file(name, &content).is_ok() {
                    println!("El archivo ha sido creado.");
                } else {
                    println!("Error al crear el archivo.");
                }
            }
            "2" => {
                println!("Nombre del archivo de texto a visualizar:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Error al leer la entrada");
                let name = name.trim();

                if let Some(content) = read_file(name) {
                    println!("Contenido del archivo de texto \n{}", content);
                } else {
                    println!("El archivo no existe");
                }
            }
            "3" => {
                println!("Nombre del archivo de texto a editar:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Error al leer la entrada.");
                let name = name.trim();

                if file_exists(name) {
                    println!("Nuevo contenido del archivo de texto:");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("Error al leer la entrada");

                    if edit_file(name, &content).is_ok() {
                        println!("El contenido del archivo ha sido actualizado.");
                    } else {
                        println!("Error al editar el archivo.");
                    }
                } else {
                    println!("El archivo no existe.");
                }
            }
            "4" => {
                println!("Nombre del archivo de texto a verificar:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Error al leer la entrada.");
                let name = name.trim();

                if file_exists(name) {
                    println!("El archivo existe en el sistema.");
                } else {
                    println!("El archivo no existe.");
                }
            }
            "5" => {
                println!("Nombre del archivo de texto a eliminar:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Error al leer la entrada");
                let name = name.trim();

                if delete_file(name).is_ok() {
                    println!("El archivo ha sido eliminado.");
                } else {
                    println!("Error al eliminar el archivo.");
                }
            }
            "6" => break,
            _ => println!("Opción no válida."),
        }
    }

    fn create_file(name: &str, content: &str) -> io::Result<()> {
        let file_name = format!("{}.txt", name);
        let mut file = File::create(&file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn read_file(name: &str) -> Option<String> {
        let file_name = format!("{}.txt", name);
        if let Ok(mut file) = File::open(&file_name) {
            let mut content = String::new();
            if let Ok(_) = file.read_to_string(&mut content) {
                Some(content)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn edit_file(name: &str, content: &str) -> io::Result<()> {
        let file_name = format!("{}.txt",name);
        let mut file = File::create(&file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn file_exists(name: &str) -> bool {
        let file_name = format!("{}.txt", name);
        Path::new(&file_name).exists()
    }

    fn delete_file(name: &str) -> io::Result<()> {
        let file_name = format!("{}.txt", name);
        fs::remove_file(&file_name)
    }

}
