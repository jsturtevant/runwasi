use std::os::windows::prelude::AsRawHandle;
use std::os::windows::raw::HANDLE;
use std::{ptr, mem};

use anyhow::{Error, Context};
use windows::Win32::Foundation::HANDLE as Win32Handle;
use windows::Win32::System::JobObjects::{ CreateJobObjectW, SetInformationJobObject,QueryInformationJobObject, JOBOBJECTINFOCLASS, 
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION,   
    JobObjectCreateSilo, JobObjectExtendedLimitInformation, 
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE };
use windows::Win32::System::Threading::{WaitForSingleObjectEx, self, ProcThreadAttributeIdealProcessor};
use windows::core::w;
use windows::core::{Error as Win32Error, PCWSTR};


pub struct JobOject(Win32Handle);

impl AsRawHandle for JobOject {
    fn as_raw_handle(&self) -> HANDLE {
        self.0.0 as HANDLE
    }
}

pub fn new_job() -> Result<JobOject, Error> {
    let job_handle = unsafe { CreateJobObjectW(None, w!("test-wasm")) }.context(format!("CreateJobObjectW windows error: {}", Win32Error::from_win32()))?;

    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    unsafe { QueryInformationJobObject(job_handle, JobObjectExtendedLimitInformation, ptr::addr_of_mut!(info) as *mut _, mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32, None) }.context(format!("QueryInformationJobObject windows error: {}", Win32Error::from_win32()) );

    // required for silo
    info.BasicLimitInformation.LimitFlags |= JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    unsafe { SetInformationJobObject(job_handle, JobObjectExtendedLimitInformation, std::ptr::addr_of!(info) as *const _, mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32)}.context(format!("SetInformationJobObject windows error: {}", Win32Error::from_win32()) );
    
    unsafe { SetInformationJobObject(job_handle, JobObjectCreateSilo, std::ptr::null_mut(), 0) }.context(format!("SetInformationJobObject windows error: {}", Win32Error::from_win32()) );

    Ok(JobOject(job_handle))
}

