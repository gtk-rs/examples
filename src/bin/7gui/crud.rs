use glib::clone;
use gtk::prelude::*;

enum Message {
    CreateItem(String, String),
    RemoveItem(i32),
    SelectItem(i32),
    UpdateItem(i32, String, String),
}

struct ItemRow {
    name: String,
    surname: String,
    row: gtk::ListBoxRow,
}

impl ItemRow {
    fn new(name: &str, surname: &str) -> Self {
        let row = gtk::ListBoxRow::new();
        row.add(&gtk::Label::new(Some(&format!("{}, {}", surname, name))));
        row.show_all();
        Self {
            name: name.to_string(),
            surname: surname.to_string(),
            row: row,
        }
    }

    fn update(&mut self, name: &str, surname: &str) {
        self.name = name.to_string();
        self.surname = surname.to_string();
        self.row.get_child().unwrap().destroy();
        self.row
            .add(&gtk::Label::new(Some(&format!("{}, {}", surname, name))));
        self.row.show_all();
    }
}

struct List {
    list: Vec<ItemRow>,
    listbox: gtk::ListBox,
}

impl List {
    pub fn new(listbox: gtk::ListBox) -> Self {
        Self {
            list: Vec::new(),
            listbox: listbox,
        }
    }

    pub fn add_bottom(&mut self, row: ItemRow) {
        self.listbox.insert(&row.row, -1);
        self.list.push(row);
    }

    pub fn get_item(&self, index: usize) -> &ItemRow {
        self.list.get(index).expect("ItemRow not found")
    }

    pub fn get_item_mut(&mut self, index: usize) -> &mut ItemRow {
        self.list.get_mut(index).expect("ItemRow not found")
    }

    pub fn remove(&mut self, index: i32) {
        let row = self.listbox.get_row_at_index(index).unwrap();
        self.listbox.remove(&row);
        self.list.remove(index as usize);
    }
}

fn main() -> Result<(), glib::error::BoolError> {
    gtk::init()?;

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let crud_ui = std::include_str!("crud.ui");
    let builder = gtk::Builder::new_from_string(crud_ui);

    let window: gtk::Window = builder.get_object("win").unwrap();
    window.connect_destroy(|_win| {
        gtk::main_quit();
    });
    window.show_all();

    let name_entry: gtk::Entry = builder.get_object("name_entry").unwrap();
    let surname_entry: gtk::Entry =
        builder.get_object("surname_entry").unwrap();
    let listbox: gtk::ListBox = builder.get_object("listbox").unwrap();

    let mut list = List::new(listbox.clone());

    listbox.connect_row_selected(clone!(@strong sender => move |_, row| {
        if let Some(row) = row {
            sender.send(Message::SelectItem(row.get_index())).expect("cannot send Message");
        }
    }));

    let create_btn: gtk::Button = builder.get_object("create_btn").unwrap();
    create_btn.connect_clicked(
        clone!(@strong sender, @strong name_entry, @strong surname_entry => move |_btn| {
            let name = name_entry.get_text().unwrap();
            let surname = surname_entry.get_text().unwrap();
            sender.send(Message::CreateItem(name.to_string(), surname.to_string())).expect("Cant send Message");
        }),
    );

    let delete_btn: gtk::Button = builder.get_object("delete_btn").unwrap();
    delete_btn.connect_clicked(clone!(@strong listbox, @strong sender => move |_btn| {
        if let Some(row) = listbox.get_selected_row() {
            sender.send(Message::RemoveItem(row.get_index())).expect("Cannot send Message");
            //listbox.remove(&row);
        }
    }));

    let update_btn: gtk::Button = builder.get_object("update_btn").unwrap();
    update_btn.connect_clicked(
        clone!(@strong listbox, @strong sender, @strong name_entry, @strong surname_entry => move |_btn| {
            if let Some(row) = listbox.get_selected_row() {
            let name = name_entry.get_text().unwrap();
            let surname = surname_entry.get_text().unwrap();
                sender.send(Message::UpdateItem(row.get_index(), name.to_string(), surname.to_string())).expect("Cannot send Message");
            }
        }),
    );

    // Filter
    // let filter_entry: gtk::Entry = builder.get_object("filter_entry").unwrap();
    // listbox.set_filter_func(Some(Box::new(
    //     clone!(@strong filter_entry => move |row| {
    //         let val = list.borrow();
    //         let itemrow = val.get_item(row.get_index() as usize);
    //         let filter = filter_entry.get_text().unwrap();
    //         itemrow.surname.contains(filter.as_str())
    //     }
    //     ),
    // )));

    receiver.attach(None, move |msg| {
        match msg {
            Message::CreateItem(name, surname) => {
                let itemrow = ItemRow::new(&name, &surname);
                list.add_bottom(itemrow);
            }
            Message::RemoveItem(index) => {
                list.remove(index);
            }
            Message::SelectItem(index) => {
                let row = list.get_item(index as usize);
                name_entry.set_text(&row.name);
                surname_entry.set_text(&row.surname);
            }
            Message::UpdateItem(index, name, surname) => {
                let row = list.get_item_mut(index as usize);
                row.update(&name, &surname);
            }
        }
        glib::Continue(true)
    });

    gtk::main();

    Ok(())
}
