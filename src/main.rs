// SPDX-License-Identifier: Apache-2.0

#![no_main]
#![no_std]

use core::arch::asm;
use core::arch::x86_64::{__cpuid, __get_cpuid_max};
use r_efi::{base, efi, protocols};

const CPUID_ECX_FEAT_X2APIC: u32 = 1 << 21;

const MSR_IA32_APIC_BASE:               u32 = 0x1b;
const          APIC_BASE_X2APIC_ENABLE: u64 = 1 << 10;
const          APIC_BASE_GLOBAL_ENABLE: u64 = 1 << 11;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe fn rdmsr(msr: u32) -> u64 {
    let (hi, lo): (u32, u32);
    asm!("rdmsr", in("ecx") msr, out("edx") hi, out("eax") lo);
    (hi as u64) << 32 | lo as u64
}

unsafe fn wrmsr(msr: u32, val: u64) {
    let hi: u32 = (val >> 32) as u32;
    let lo: u32 = val as u32;
    asm!("wrmsr", in("ecx") msr, in("edx") hi, in("eax") lo);
}

extern "efiapi" fn enable_x2apic(_: *mut core::ffi::c_void) {
    let mut apic_base: u64 = unsafe { rdmsr(MSR_IA32_APIC_BASE) };
    if apic_base & (APIC_BASE_GLOBAL_ENABLE | APIC_BASE_X2APIC_ENABLE)
        != APIC_BASE_GLOBAL_ENABLE | APIC_BASE_X2APIC_ENABLE
    {
        apic_base |= APIC_BASE_GLOBAL_ENABLE | APIC_BASE_X2APIC_ENABLE;
        unsafe { wrmsr(MSR_IA32_APIC_BASE, apic_base) };
    }
}

#[export_name = "efi_main"]
pub extern "efiapi" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let (max_leaf, _) = unsafe { __get_cpuid_max(0) };
    if max_leaf < 1 {
        return efi::Status::UNSUPPORTED;
    }

    let result = unsafe { __cpuid(1) };
    if result.ecx & CPUID_ECX_FEAT_X2APIC == 0 {
        return efi::Status::UNSUPPORTED;
    }

    let mut mp_services: *mut protocols::mp_services::Protocol = core::ptr::null_mut();
    let mut status = (unsafe { (*((*st).boot_services)).locate_protocol })(
        &mut protocols::mp_services::PROTOCOL_GUID,
        core::ptr::null_mut(),
        &mut mp_services as *mut _ as *mut *mut core::ffi::c_void,
    );

    if status != efi::Status::SUCCESS {
        return status;
    }

    status = (unsafe { (*mp_services).startup_all_aps })(
        mp_services,
        enable_x2apic,
        base::Boolean::FALSE,
        core::ptr::null_mut(),
        10_000,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );

    if status != efi::Status::SUCCESS {
        return status;
    }

    enable_x2apic(core::ptr::null_mut());
    efi::Status::SUCCESS
}
