use gdnative::prelude::*;
use gdnative::api::{EditorPlugin, Resource, Script, Texture, ImageTexture, Image};
use gdnative::prelude::Null;

#[derive(NativeClass)]
struct KeepassTotp {
}

#[methods]
#[no_constructor]
impl KeepassTotp {
    #[export]
    fn test() -> String {
        "Hello World"
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<KeepassTotp>();
}

godot_init!(init);
