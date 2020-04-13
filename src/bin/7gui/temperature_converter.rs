use gtk::prelude::*;

fn main() -> Result<(), glib::error::BoolError> {
    gtk::init()?;
    let counter_ui = std::include_str!("temperature_converter.ui");
    let builder = gtk::Builder::new_from_string(counter_ui);
    let window: gtk::Window = builder.get_object("win").unwrap();
    window.connect_destroy(|_win| {
        gtk::main_quit();
    });
    window.show_all();
    let celsius: gtk::Entry = builder.get_object("celsius").unwrap();
    let fahrenheit: gtk::Entry = builder.get_object("fahrenheit").unwrap();
    celsius
        .bind_property("text", &fahrenheit, "text")
        .flags(glib::BindingFlags::BIDIRECTIONAL)
        .transform_to(|_binding, value| {
            if let Ok(Some(v)) = value.get::<String>() {
                if let Ok(celsius_i) = v.parse::<f64>() {
                    let fahrenheit_i = (celsius_i * (9. / 5.) + 32.) as i32;
                    return Some(fahrenheit_i.to_string().to_value());
                }
            }
            None
        })
        .transform_from(|_binding, value| {
            if let Ok(Some(v)) = value.get::<String>() {
                if let Ok(fahrenheit_i) = v.parse::<f64>() {
                    let celsius_i = ((fahrenheit_i - 32.) * (5. / 9.)) as i32;
                    return Some(celsius_i.to_string().to_value());
                }
            }
            None
        })
        .build();
    gtk::main();
    Ok(())
}

