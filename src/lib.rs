use std::{
    ffi::c_void,
    mem::{size_of, MaybeUninit},
};
use windows::{
    core::PCWSTR,
    Win32::{
        System::{
            IO::IO_STATUS_BLOCK,
            Memory::{GetProcessHeap, HeapAlloc,HeapFree, HEAP_ZERO_MEMORY},
        },
        Foundation::{GetLastError, CloseHandle},
        Storage::FileSystem::{
            CreateFileW,
            SetFileInformationByHandle,
            FILE_RENAME_INFO,
            FILE_SHARE_READ,
            OPEN_EXISTING,
            DELETE,
            SYNCHRONIZE,
            FILE_FLAGS_AND_ATTRIBUTES,
            FileRenameInfo
        }
    },
    Wdk::Storage::FileSystem::{
        FILE_DISPOSITION_INFORMATION_EX,
        FILE_DISPOSITION_INFORMATION_EX_FLAGS,
        FILE_DISPOSITION_DELETE,
        FILE_DISPOSITION_POSIX_SEMANTICS,
        NtSetInformationFile,
        FileDispositionInformationEx
    }
};

pub fn delete_self() -> Result<(), Box<dyn std::error::Error>> {
    // The new data stream name
    let stream = ":pengrey";
    let new_stream: Vec<u16> = stream.encode_utf16().chain(std::iter::once(0)).collect();
    let stream_length = new_stream.len();
    let s_rename = size_of::<FILE_RENAME_INFO>() + (stream_length * size_of::<u16>());

    unsafe {
        // allocating enough buffer for the 'FILE_RENAME_INFO' structure
        let p_rename = HeapAlloc(GetProcessHeap()?, HEAP_ZERO_MEMORY, s_rename).cast::<FILE_RENAME_INFO>();

        //--------------------------------------------------------------------------------------------------------------------------
        // marking the file for deletion (used in the SetFileInformationEx call)
        // more info here: https://tkyn.dev/2025-6-8-The-Not-So-Self-Deleting-Executable-on-24h2/
        let file_disp_info_ex = FILE_DISPOSITION_INFORMATION_EX {
            Flags: {
                FILE_DISPOSITION_INFORMATION_EX_FLAGS(
                    FILE_DISPOSITION_DELETE.0 | FILE_DISPOSITION_POSIX_SEMANTICS.0
                )
            }
        };
        // setting the new data stream name buffer and size in the 'FILE_RENAME_INFO' structure
        (*p_rename).FileNameLength = (stream_length * size_of::<u16>()) as u32 - 2;
        std::ptr::copy_nonoverlapping(
            new_stream.as_ptr(),
            (*p_rename).FileName.as_mut_ptr(),
            stream_length,
        );

        //--------------------------------------------------------------------------------------------------------------------------
        // used to get the current file name
        let path = std::env::current_exe()?;
        let path_str = path.to_str().unwrap_or("");
        let mut full_path: Vec<u16> = path_str.encode_utf16().collect();
        full_path.push(0);

        //--------------------------------------------------------------------------------------------------------------------------
        // RENAMING

        // openning a handle to the current file
        let handle = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        ).map_err(|_| {
            let error_code = GetLastError();
            format!("[!] CreateFileW [R] Failed With Error : {:?}", error_code)
        })?;

        println!("[i] Renaming :$DATA ...");

        // renaming the data stream
        SetFileInformationByHandle(
            handle,
            FileRenameInfo,
            p_rename as *const c_void,
            s_rename as u32
        ).map_err(|_| {
            let error_code = GetLastError();
            format!("[!] SetFileInformationByHandle failed with error: {:?}", error_code)
        })?;

        println!("[+] DONE");

        CloseHandle(handle).map_err(|_| {
            let error_code = GetLastError();
            format!("[!] CloseHandle [R] failed with error: {:?}", error_code)
        })?;

    	//--------------------------------------------------------------------------------------------------------------------------
	    // DELETING

	    // openning a new handle to the current file
        let handle = CreateFileW(
            PCWSTR(full_path.as_ptr()),
            DELETE.0 | SYNCHRONIZE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        ).map_err(|_| {
            let error_code = GetLastError();
            format!("[!] CreateFileW [D] Failed With Error : {:?}", error_code)
        })?;

        println!("[i] DELETING ...");

    	// marking for deletion after the file's handle is closed
        let mut iosb: MaybeUninit<IO_STATUS_BLOCK> = MaybeUninit::uninit();
        let status = NtSetInformationFile(
            handle,
            iosb.as_mut_ptr(),
            &file_disp_info_ex as *const _ as *const c_void,
            size_of::<FILE_DISPOSITION_INFORMATION_EX>() as u32,
            FileDispositionInformationEx,
        );

        if !status.is_ok() {
            return Err(Box::from("[!] NtSetInformationFile failed"));
        }


        CloseHandle(handle).map_err(|_| {
            let error_code = GetLastError();
            format!("[!] CloseHandle [D] failed with error: {:?}", error_code)
        })?;

        //--------------------------------------------------------------------------------------------------------------------------

        // freeing the allocated buffer
        HeapFree(
            GetProcessHeap()?,
            HEAP_ZERO_MEMORY,
            Some(p_rename as *const c_void),
        )?;

        Ok(())
    }
}
