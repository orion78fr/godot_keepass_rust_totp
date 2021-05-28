use gdnative::prelude::*;
use gdnative::api::{Reference};

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
}

fn init(handle: InitHandle) {
    handle.add_class::<KeepassTotp>();
}

godot_init!(init);
