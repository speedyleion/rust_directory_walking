/*
 *          Copyright Nick G. 2021.
 * Distributed under the Boost Software License, Version 1.0.
 *    (See accompanying file LICENSE or copy at
 *          https://www.boost.org/LICENSE_1_0.txt)
 */

use memoffset::offset_of;
use ntapi::ntioapi::{
    FileFullDirectoryInformation, NtQueryDirectoryFile, FILE_FULL_DIR_INFORMATION, IO_STATUS_BLOCK,
};
use std::ffi::CString;
use std::path::Path;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
use winapi::um::winnt::{
    FILE_ATTRIBUTE_DIRECTORY, FILE_LIST_DIRECTORY, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, HANDLE
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DirEntry{
    pub name: String,
    pub is_dir: bool,
}

pub fn dir_walk(directory: &str) -> usize {
    let mut files = get_dir_stats(Path::new(directory));
    files.sort();
    files.len()
}

fn get_dir_stats(path: &Path) -> Vec<DirEntry> {
    let mut files = vec![];
    let handle = get_directory_handle(path);
    let mut io_block: IO_STATUS_BLOCK = unsafe { std::mem::zeroed() };
    let io_ptr: *mut IO_STATUS_BLOCK = &mut io_block as *mut _;
    let mut buffer: [u8; 1000] = [0; 1000];
    let name_member_offset = offset_of!(FILE_FULL_DIR_INFORMATION, FileName);
    loop {
        let mut offset = 0;
        let result = unsafe {
            NtQueryDirectoryFile(
                handle,
                std::ptr::null_mut(),
                None,
                std::ptr::null_mut(),
                io_ptr,
                buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
                buffer.len() as u32,
                FileFullDirectoryInformation,
                0,
                std::ptr::null_mut(),
                0,
            )
        };
        if result < 0 {
            break;
        }

        loop {
            let (_head, body, _tail) =
                unsafe { buffer[offset..].align_to::<FILE_FULL_DIR_INFORMATION>() };
            let file_info = &body[0];
            let name_offset = name_member_offset + offset;
            offset += file_info.NextEntryOffset as usize;
            let is_dir = file_info.FileAttributes & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY;
            let name = read_string(&buffer[name_offset..], file_info.FileNameLength as usize).unwrap();
            if !(is_dir && name.starts_with(".")) {
                files.push(DirEntry{name, is_dir});
            }
            if file_info.NextEntryOffset == 0 {
                break;
            }
        }
    }
    // TODO look at making a wrapper object and use drop.
    unsafe {
        CloseHandle(handle);
    }
    let nested_files: Vec<Vec<DirEntry>> = files.iter().filter(|s| s.is_dir).map(|s| get_dir_stats(&path.join(&s.name))).collect();
    for nested in nested_files {
        files.extend(nested);
    }

    files
}

fn get_directory_handle(path: &Path) -> HANDLE {
    let name = CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        CreateFileA(
            name.as_ptr(),
            FILE_LIST_DIRECTORY,
            FILE_SHARE_WRITE | FILE_SHARE_READ | FILE_SHARE_DELETE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            std::ptr::null_mut(),
        )
    }
}

fn read_string(slice: &[u8], size: usize) -> Option<String> {
    let (_front, slice, _back) = unsafe { slice.align_to::<u16>() };
    String::from_utf16(&slice[..size / 2]).ok()
}