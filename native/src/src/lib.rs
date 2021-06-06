use gdnative::prelude::*;
use gdnative::api::{Reference, File};
use keepass::{Database, Group};
use keepass::Value::{Bytes, Unprotected, Protected};
use std::str::from_utf8;
use std::io::Write;

#[derive(NativeClass)]
#[inherit(Reference)]
struct KeepassTotp;

#[methods]
impl KeepassTotp {
    fn new(_owner: TRef<Reference>) -> Self {
        KeepassTotp
    }

    #[export]
    fn open_keepass_db(&self,
                       _owner: TRef<Reference>,
                       db_path: GodotString,
                       pwd: Option<String>) -> String {
        let db = open_db(db_path, pwd);
        if db.0.is_err() {
            return format!("{:?}", db.0.unwrap_err());
        }

        let otps = iterate_group(&db.0.unwrap().root);

        return String::from(otps.join("\r\n"));
    }
}

// We can't implement a foreign trait on a foreign type, so we have to wrap it
struct FileWrapper(Ref<gdnative::api::File, Unique>);

impl std::io::Read for FileWrapper {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let read_size: i64 = std::cmp::min(buf.len() as i64,
                                           self.0.get_len() - self.0.get_position());

        let b = self.0.as_ref().get_buffer(read_size);
        let res = buf.write(b.read().as_slice());
        res
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<KeepassTotp>();
}

#[derive(Debug)]
pub struct ResultWrapper<T, U>(Result<T, U>);

#[derive(Debug)]
pub enum Error {
    Ok,
    StringError(String),
    GodotError(gdnative::core_types::GodotError),
    KeepassError(keepass::Error),
}

impl From<keepass::Result<Database>> for ResultWrapper<Database, Error> {
    fn from(r: keepass::Result<Database>) -> Self {
        ResultWrapper(match r {
            Ok(db) => Ok(db),
            Err(e) => Err(Error::KeepassError(e))
        })
    }
}

impl From<gdnative::GodotResult> for ResultWrapper<Database, Error> {
    fn from(r: gdnative::GodotResult) -> Self {
        ResultWrapper(match r {
            Ok(_) => Err(Error::Ok),
            Err(e) => Err(Error::GodotError(e))
        })
    }
}

fn open_db(path: GodotString, pwd: Option<String>) -> ResultWrapper<Database, Error> {
    let f = File::new();
    let r = f.open(path, File::READ);

    if r.is_err() {
        return ResultWrapper(Result::Err(Error::GodotError(r.unwrap_err())));
    }

    Database::open(
        &mut FileWrapper(f),
        pwd.as_deref(),
        None,
    ).into()
}

fn iterate_group(group: &Group) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

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

        if otp.is_some() {
            res.push(otp.unwrap().into())
        }

        godot_print!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
        godot_print!("\tTOTP is : {:?}", otp);
    }

    for (_, g) in &group.child_groups {
        godot_print!("Saw group '{0}'", g.name);
        res.extend(iterate_group(&g));
    }

    return res;
}

godot_init!(init);
