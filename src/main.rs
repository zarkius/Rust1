extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType};

fn main() {
    // Inicializar GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crear una nueva ventana
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ventana GTK");
    window.set_default_size(350, 70);


    //    DEFINIENDO LOS WIDGETS


    // Crear un botón
    let button = gtk::Button::new();
    button.set_label("¡Hola, mundo!");

    let texto1 = gtk::Label::new(Some("Texto 1"));



    //  A PARTIR DE ESTE CONTENEDOR AÑADIMOS LOS WIDGETS PREVIAMENTE DEFINIDOS!

    // Crear un contenedor vertical
    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);



    //  AÑADIMOS LOS WIDGETS AL CONTENEDOR
    

    container.add(&texto1);

    // Agregar el botón al contenedor
    container.add(&button);

    //    AÑADIMOS EL CONTENEDOR A LA VENTANA PRINCIPAL

    // Agregar el contenedor a la ventana
    window.add(&container);

    // Mostrar todos los widgets
    window.show_all();

    // Iniciar el bucle principal de GTK
    gtk::main();
}
