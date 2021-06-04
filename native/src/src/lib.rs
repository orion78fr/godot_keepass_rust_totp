use gdnative::prelude::*;
use gdnative::api::{Reference};
use keepass::{Database, Group};
use std::fs::File;
use std::path::Path;
use keepass::Value::{Bytes, Unprotected, Protected};
use std::str::from_utf8;

#[derive(NativeClass)]
#[inherit(Reference)]
struct KeepassTotp;

#[methods]
impl KeepassTotp {
    fn new(_owner: TRef<Reference>) -> Self {
        KeepassTotp
    }
    #[export]
    fn test(&self, _owner: TRef<Reference>) -> String {
        "Hello World from Rust !".to_string()
    }
    #[export]
    fn open_keepass_db(&self, _owner: TRef<Reference>, db_path: String, pwd: Option<String>) -> () {
        let db = open_db(db_path, pwd).unwrap();

        iterate_group(&db.root);
    }


}

fn init(handle: InitHandle) {
    handle.add_class::<KeepassTotp>();
}

fn open_db(path: String, pwd: Option<String>) -> keepass::Result<Database> {
    Database::open(
        &mut File::open(Path::new(&path))?,
        Some(pwd.unwrap().as_str()),
        None
    )
}

fn iterate_group(group: &Group) {
    // Iterate over all Groups and Nodes
    for (_, node) in &group.entries {
        let title = node.get_title().unwrap();
        let user = node.get_username().unwrap();
        let pass = node.get_password().unwrap();
        let otp = node.fields.get("otp").map(|v| {
            match v {
                Bytes(b) => from_utf8(b.as_slice()).unwrap(),
                Unprotected(str) => str,
                Protected(sstr) => from_utf8(sstr.unsecure()).unwrap()
            }
        });

        godot_print!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
        godot_print!("\tTOTP is : {:?}", otp);
    }

    for (_, g) in &group.child_groups {
        godot_print!("Saw group '{0}'", g.name);
        iterate_group(&g);
    }
}

godot_init!(init);
