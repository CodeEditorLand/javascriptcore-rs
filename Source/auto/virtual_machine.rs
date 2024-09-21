// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
	#[doc(alias = "JSCVirtualMachine")]
	pub struct VirtualMachine(Object<ffi::JSCVirtualMachine, ffi::JSCVirtualMachineClass>);

	match fn {
		type_ => || ffi::jsc_virtual_machine_get_type(),
	}
}

impl VirtualMachine {
	pub const NONE: Option<&'static VirtualMachine> = None;

	#[doc(alias = "jsc_virtual_machine_new")]
	pub fn new() -> VirtualMachine {
		unsafe { from_glib_full(ffi::jsc_virtual_machine_new()) }
	}
}

impl Default for VirtualMachine {
	fn default() -> Self {
		Self::new()
	}
}
