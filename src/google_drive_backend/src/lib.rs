#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{
    vec, BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct File {
    id: u64,
    folder_id: u64,
    file_name: String,
    mime_type: String,
    content: String,
    updated_at: Option<u64>,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for File {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for File {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Folder {
    id: u64,
    folder_name: String,
    updated_at: Option<u64>,
}

impl Storable for Folder {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for Folder {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static FILE_STORAGE: RefCell<StableBTreeMap<u64, File, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static FOLDER_STORAGE: RefCell<StableBTreeMap<u64, Folder, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FilePayload {
    folder_id: u64,
    file_name: String,
    mime_type: String,
    content: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FolderPayload {
    folder_name: String,
}

#[ic_cdk::query]
fn get_file(id: u64) -> Result<File, Error> {
    match _get_file(&id) {
        Some(file) => Ok(file),
        None => Err(Error::NotFound {
            msg: format!("a file with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_all_files() -> Result<Vec<File>, Error> {
    let files_map: Vec<(u64, File)> =
        FILE_STORAGE.with(|service| service.borrow().iter().collect());
    let files: Vec<File> = files_map.into_iter().map(|(_, file)| file).collect();

    if !files.is_empty() {
        Ok(files)
    } else {
        Err(Error::NotFound {
            msg: "No file found.".to_string(),
        })
    }
}

// get all files by folder id
#[ic_cdk::query]
fn get_all_files_by_folder_id(folder_id: u64) -> Result<Vec<File>, Error> {
    match _get_folder(&folder_id) {
        Some(folder) => {
            let files_map: Vec<(u64, File)> =
                FILE_STORAGE.with(|service| service.borrow().iter().collect());
            let files: Vec<File> = files_map.into_iter().map(|(_, file)| file).collect();
            let mut result: Vec<File> = vec![];

            if !files.is_empty() {
                for file in files {
                    if file.folder_id == folder.id {
                        result.push(file.clone());
                    }
                }
                Ok(result)
            } else {
                Err(Error::NotFound {
                    msg: "No file found.".to_string(),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("a folder with id={} not found", folder_id),
        }),
    }
}

// get all files by folder name
#[ic_cdk::query]
fn get_all_files_by_folder_name(folder_name: String) -> Result<Vec<File>, Error> {
    let folders_map: Vec<(u64, Folder)> =
        FOLDER_STORAGE.with(|service| service.borrow().iter().collect());
    let folders: Vec<Folder> = folders_map.into_iter().map(|(_, folder)| folder).collect();
    let found_folder: Folder;

    if !folders.is_empty() {
        let mut result: Vec<File> = vec![];
        for folder in folders {
            if folder.folder_name == folder_name {
                found_folder = folder;
                let files_map: Vec<(u64, File)> =
                    FILE_STORAGE.with(|service| service.borrow().iter().collect());
                let files: Vec<File> = files_map.into_iter().map(|(_, file)| file).collect();
                if !files.is_empty() {
                    for file in files {
                        if file.folder_id == found_folder.id {
                            result.push(file.clone());
                        }
                    }
                } else {
                    return Err(Error::NotFound {
                        msg: "No file found.".to_string(),
                    });
                }
                break;
            }
        }
        Ok(result)
    } else {
        Err(Error::NotFound {
            msg: "No folder found.".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_folder(id: u64) -> Result<Folder, Error> {
    match _get_folder(&id) {
        Some(folder) => Ok(folder),
        None => Err(Error::NotFound {
            msg: format!("a folder with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_folder_by_name(folder_name: String) -> Result<Folder, Error> {
    let folders_map: Vec<(u64, Folder)> =
        FOLDER_STORAGE.with(|service| service.borrow().iter().collect());
    let folders: Vec<Folder> = folders_map.into_iter().map(|(_, folder)| folder).collect();

    let mut result: Option<Folder> = None;
    if !folders.is_empty() {
        for folder in folders {
            if folder.folder_name == folder_name {
                result = Some(folder);
                break;
            }
        }
        if result.is_none() {
            Err(Error::NotFound {
                msg: "No folder found.".to_string(),
            })
        } else {
            Ok(result.unwrap())
        }
    } else {
        Err(Error::NotFound {
            msg: "No folder found.".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_all_folders() -> Result<Vec<Folder>, Error> {
    let folders_map: Vec<(u64, Folder)> =
        FOLDER_STORAGE.with(|service| service.borrow().iter().collect());
    let folders: Vec<Folder> = folders_map.into_iter().map(|(_, folder)| folder).collect();

    if !folders.is_empty() {
        Ok(folders)
    } else {
        Err(Error::NotFound {
            msg: "No folder found.".to_string(),
        })
    }
}

#[ic_cdk::update]
fn create_file(payload: FilePayload) -> Result<File, Error> {
    if payload.file_name.is_empty() {
        return Err(Error::CreateFail {
            msg: String::from("Invalid file name"),
        });
    };

    if payload.mime_type.is_empty() {
        return Err(Error::CreateFail {
            msg: String::from("Invalid mime type"),
        });
    };

    if payload.content.is_empty() {
        return Err(Error::CreateFail {
            msg: String::from("Invalid content"),
        });
    };

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let file = File {
        id,
        folder_id: payload.folder_id,
        file_name: payload.file_name,
        mime_type: payload.mime_type,
        content: payload.content,
        updated_at: Some(time()),
    };
    do_insert_file(&file);
    Ok(file)
}

#[ic_cdk::update]
fn update_file(id: u64, payload: FilePayload) -> Result<File, Error> {
    if payload.file_name.is_empty() {
        return Err(Error::UpdateFail {
            msg: String::from("Invalid file name"),
        });
    };

    if payload.mime_type.is_empty() {
        return Err(Error::CreateFail {
            msg: String::from("Invalid mime type"),
        });
    };

    if payload.content.is_empty() {
        return Err(Error::UpdateFail {
            msg: String::from("Invalid content"),
        });
    };

    match FILE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut file) => {
            file.folder_id = payload.folder_id;
            file.file_name = payload.file_name;
            file.mime_type = payload.mime_type;
            file.content = payload.content;
            file.updated_at = Some(time());
            do_insert_file(&file);
            Ok(file)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a file with id={}. file not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_file_name(id: u64, file_name: String) -> Result<File, Error> {
    if file_name.is_empty() {
        return Err(Error::UpdateFail {
            msg: String::from("Invalid file name"),
        });
    };
    match FILE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut file) => {
            file.file_name = file_name;
            file.updated_at = Some(time());
            do_insert_file(&file);
            Ok(file)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a file with id={}. file not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_file(id: u64) -> Result<File, Error> {
    match FILE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(file) => Ok(file),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a file with id={}. file not found.", id),
        }),
    }
}

#[ic_cdk::update]
fn create_folder(payload: FolderPayload) -> Result<Folder, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let folder = Folder {
        id,
        folder_name: payload.folder_name,
        updated_at: Some(time()),
    };
    do_insert_folder(&folder);
    Ok(folder)
}

#[ic_cdk::update]
fn update_folder(id: u64, payload: FolderPayload) -> Result<Folder, Error> {
    let folder_option: Option<Folder> = FOLDER_STORAGE.with(|service| service.borrow().get(&id));

    match folder_option {
        Some(mut folder) => {
            folder.folder_name = payload.folder_name;
            folder.updated_at = Some(time());
            do_insert_folder(&folder);
            Ok(folder)
        }
        None => Err(Error::NotFound {
            msg: format!("Folder with id={} not found.", id),
        }),
    }
}

// helper method to perform insert.
fn do_insert_file(file: &File) {
    FILE_STORAGE.with(|service| service.borrow_mut().insert(file.id, file.clone()));
}

fn do_insert_folder(folder: &Folder) {
    FOLDER_STORAGE.with(|service| service.borrow_mut().insert(folder.id, folder.clone()));
}

// a helper method to get a file by id
fn _get_file(id: &u64) -> Option<File> {
    FILE_STORAGE.with(|service| service.borrow().get(id))
}

// a helper method to get a folder by id
fn _get_folder(id: &u64) -> Option<Folder> {
    FOLDER_STORAGE.with(|service| service.borrow().get(id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    CreateFail { msg: String },
    UpdateFail { msg: String },
}

// generate candid
ic_cdk::export_candid!();
