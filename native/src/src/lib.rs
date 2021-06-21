use gdnative::prelude::*;
use gdnative::api::{Reference, File};
use keepass::{Database, Group};
use std::io::Write;
use std::time::SystemTime;

use otpauth_uri::parser::parse_otpauth_uri;
use otpauth_uri::types::{OTPGenerator};

#[derive(NativeClass)]
#[inherit(Reference)]
struct KeepassTotp;

#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
struct TOTPEntry {
    otp: Option<OTPGenerator>,

    #[property]
    name: String,
    #[property]
    user: String,
    #[property]
    pass: String,
    #[property]
    url: String,

    icon: Icon,
}

enum Icon {
    None,
    Id(u8),
    CustomRef(String),
    Custom(Vec<u8>),
}

#[methods]
impl KeepassTotp {
    fn new(_owner: TRef<Reference>) -> Self {
        KeepassTotp {}
    }

    #[export]
    fn open_keepass_db(&mut self,
                       _owner: TRef<Reference>,
                       db_path: GodotString,
                       pwd: Option<String>) -> Result<Vec<Variant>, String> {
        let db = open_db(db_path, pwd);
        if db.0.is_err() {
            return Err(format!("{:?}", db.0.unwrap_err()));
        }

        let Database { root, metadata, .. } = db.0.unwrap();

        return Ok(iterate_group(&root)
            .into_iter()
            .map(|mut e| {
                if let Icon::CustomRef(uuid) = &e.icon {
                    // TODO
                    e.icon = Icon::Custom(base64::decode(metadata.as_ref().unwrap().custom_icons.get(uuid).unwrap()).unwrap())
                }
                e
            })
            .map(Instance::emplace)
            .map(|i| i.owned_to_variant())
            .collect());
    }

    #[export]
    fn android_test(&mut self,
                    _owner: TRef<Reference>) -> String {
        return test_fun();
    }
}


#[cfg(target_os = "android")]
fn test_fun() -> String {
    unsafe{
        if jni::VM.is_none() {
            return "No JNI".to_string();
        }

        use jni_android_sys::android::content::Intent;

        return jni_glue::VM::from_jni_local(&(*jni::VM.unwrap())).with_env(|env| {
            let intent = Intent::new(env).unwrap();

            return format!("We are in Android ! {:?}", intent.toString().unwrap());
        });
    }

}

#[cfg(not(target_os = "android"))]
fn test_fun() -> String {
    return "Not Android".to_string();
}


#[methods]
impl TOTPEntry {
    #[export]
    fn generate(&self,
                _owner: TRef<Reference>) -> Option<String> {
        match self.otp.as_ref().unwrap() {
            OTPGenerator::TOTPGenerator(g) => {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap()
                    .as_secs();

                Some(g.generate(now))
            }
            OTPGenerator::HOTPGenerator(_) => None
        }
    }

    #[export]
    fn get_custom_icon(&self,
                       _owner: TRef<Reference>) -> Option<Vec<u8>> {
        match &self.icon {
            Icon::Custom(bytes) => Some(bytes.clone()),
            _ => None
        }
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
    handle.add_class::<TOTPEntry>();
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

fn iterate_group(group: &Group) -> Vec<TOTPEntry> {
    let mut res: Vec<TOTPEntry> = Vec::new();

    // Iterate over all Groups and Nodes
    for node in &group.children {
        match node {
            keepass::Node::Group(group) => {
                res.extend(iterate_group(&group));
            }
            keepass::Node::Entry(entry) => {
                let otp = entry.get("otp");

                if otp.is_some() {
                    let title = entry.get_title().unwrap();
                    let user = entry.get_username().unwrap();
                    let pass = entry.get_password().unwrap();
                    let url = entry.get_url().unwrap();

                    let icon = &entry.icon;

                    res.push(TOTPEntry {
                        otp: parse_otpauth_uri(otp.unwrap()).map(OTPGenerator::from).ok(),
                        name: title.to_string(),
                        user: user.to_string(),
                        pass: pass.to_string(),
                        url: url.to_string(),
                        icon: match icon {
                            keepass::Icon::None => Icon::None,
                            keepass::Icon::IconID(id) => Icon::Id(*id),
                            keepass::Icon::CustomIcon(uuid) => Icon::CustomRef(uuid.clone())
                        },
                    });
                }
            }
        }
    }

    return res;
}

godot_init!(init);

#[test]
fn test() {
    use std::fs::File;
    use std::path::Path;

    let db = Database::open(
        &mut File::open(Path::new("../../test/totp_test.kdbx")).unwrap(),
        Some("azerty"),
        None,
    ).unwrap();

    db.header;
}

#[cfg(target_os = "android")]
/// Methods automatically invoked by the JVM.
mod jni {
    use lazy_static::lazy_static;
    use jni_glue::jni_sys::{JavaVM, jint};
    use jni_glue::std::ffi::c_void;
    /*use std::sync::RwLock;

    pub(crate) struct JavaVMWrapper {
        vm: Option<jni_glue::VM>,
    }

    impl JavaVMWrapper {
        fn new() -> Self {
            JavaVMWrapper {
                vm: None
            }
        }

        pub(crate) fn get_vm(&self) -> &Option<jni_glue::VM> {
            &self.vm
        }

        fn set_vm(&mut self, vm: jni_glue::VM) {
            self.vm = Some(vm)
        }

        fn unset_vm(&mut self) {
            self.vm = None
        }
    }

    lazy_static! { // RwLock::new is not const
        pub(crate) static ref VM : RwLock<JavaVMWrapper> = RwLock::new(JavaVMWrapper::new());
    }*/

    pub(crate) static mut VM: Option<*const JavaVM> = None;

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "system" fn JNI_OnLoad(vm: *const JavaVM, reserved: *const c_void) -> jint {
        //VM.write().unwrap().set_vm(*jni_glue::VM::from_jni_local(&*vm));
        VM = Some(vm);
        jni_glue::on_load(vm, reserved)
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "system" fn JNI_OnUnload(vm: *const JavaVM, reserved: *const c_void) {
        //VM.write().unwrap().unset_vm();
        VM = None;
        jni_glue::on_unload(vm, reserved)
    }
}
