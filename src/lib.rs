#![feature(proc_macro_hygiene)]

#![feature(allocator_api)]

extern crate tinytemplate;

#[macro_use]
extern crate lazy_static;

use skyline_web::Webpage;
use tinytemplate::TinyTemplate;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use percent_encoding::percent_decode_str;
use nn_fuse::{AccessorResult, DAccessor, DirectoryAccessor, FAccessor, FileAccessor, FileSystemAccessor, FsAccessor, FsEntryType};
use smash_arc::{ArcFile, ArcLookup, Hash40, FileNode, Region};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Event {
    ArcFilesystemMounted,
    ModFilesystemMounted,
}

pub type EventCallbackFn = extern "C" fn(Event);

extern "C" {
    fn arcrop_register_event_callback(ty: Event, callback: EventCallbackFn);
}


#[global_allocator]
static UNIX_ALLOCATOR: skyline::unix_alloc::UnixAllocator = skyline::unix_alloc::UnixAllocator;

lazy_static! {
    static ref ARC_FILE: ArcFile = ArcFile::open("rom:/data.arc").unwrap();
}

pub fn get_arc_results() {
    // println!("{}", ARC_FILE.patch_section)
    for node in ARC_FILE.get_dir_listing("/").unwrap() {
        match node {
            FileNode::Dir(dir) => {
                // print out name of directory
                println!("directory: {}", dir.global_label().unwrap());
            }
            FileNode::File(file) => {
                // extract file
                let path = file.global_label().unwrap();
                println!("{}", &path)
            }
        }
    }
}


extern "C" fn main_real(event: Event) {
    Hash40::set_global_labels_file("sd:/ultimate/arcropolis/hashes.txt").unwrap();
    get_arc_results();
}
#[skyline::main(name = "file-explorer")]
pub fn main() {
    unsafe{
        arcrop_register_event_callback(Event::ArcFilesystemMounted, main_real);
    }
}