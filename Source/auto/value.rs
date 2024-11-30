// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

#[cfg(feature = "v2_38")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
use crate::TypedArrayType;
use crate::{Context, ValuePropertyFlags};

glib::wrapper! {
	#[doc(alias = "JSCValue")]
	pub struct Value(Object<ffi::JSCValue, ffi::JSCValueClass>);

	match fn {
		type_ => || ffi::jsc_value_get_type(),
	}
}

impl Value {
	pub const NONE:Option<&'static Value> = None;

	#[doc(alias = "jsc_value_new_array_from_garray")]
	pub fn new_array_from_garray(context:&impl IsA<Context>, array:&[Value]) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_array_from_garray(
				context.as_ref().to_glib_none().0,
				array.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_new_array_from_strv")]
	pub fn new_array_from_strv(context:&impl IsA<Context>, strv:&[&str]) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_array_from_strv(
				context.as_ref().to_glib_none().0,
				strv.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_new_boolean")]
	pub fn new_boolean(context:&impl IsA<Context>, value:bool) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_boolean(
				context.as_ref().to_glib_none().0,
				value.into_glib(),
			))
		}
	}

	#[cfg(feature = "v2_28")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
	#[doc(alias = "jsc_value_new_from_json")]
	#[doc(alias = "new_from_json")]
	pub fn from_json(context:&impl IsA<Context>, json:&str) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_from_json(
				context.as_ref().to_glib_none().0,
				json.to_glib_none().0,
			))
		}
	}

	//#[doc(alias = "jsc_value_new_function")]
	// pub fn new_function<P: Fn() + 'static>(context: &impl IsA<Context>, name:
	// Option<&str>, callback: P, user_data: /*Unimplemented*/Option<Basic:
	// Pointer>, return_type: glib::types::Type, n_params: u32, : /*Unknown
	// conversion*//*Unimplemented*/Basic: VarArgs) -> Value {    unsafe {
	// TODO: call ffi:jsc_value_new_function() }
	//}

	//#[doc(alias = "jsc_value_new_function_variadic")]
	// pub fn new_function_variadic<P: Fn() + 'static>(context: &impl
	// IsA<Context>, name: Option<&str>, callback: P, user_data:
	// /*Unimplemented*/Option<Basic: Pointer>, return_type: glib::types::Type)
	// -> Value {    unsafe { TODO: call ffi:jsc_value_new_function_variadic()
	// }
	//}

	//#[doc(alias = "jsc_value_new_functionv")]
	// pub fn new_functionv<P: Fn() + 'static>(context: &impl IsA<Context>,
	// name: Option<&str>, callback: P, user_data:
	// /*Unimplemented*/Option<Basic: Pointer>, return_type: glib::types::Type,
	// n_parameters: u32) -> Value {    unsafe { TODO: call
	// ffi:jsc_value_new_functionv() }
	//}

	#[doc(alias = "jsc_value_new_null")]
	pub fn new_null(context:&impl IsA<Context>) -> Value {
		unsafe { from_glib_full(ffi::jsc_value_new_null(context.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_new_number")]
	pub fn new_number(context:&impl IsA<Context>, number:f64) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_number(context.as_ref().to_glib_none().0, number))
		}
	}

	//#[doc(alias = "jsc_value_new_object")]
	// pub fn new_object(context: &impl IsA<Context>, instance:
	// /*Unimplemented*/Option<Basic: Pointer>, jsc_class: Option<&Class>) ->
	// Value {    unsafe { TODO: call ffi:jsc_value_new_object() }
	//}

	#[doc(alias = "jsc_value_new_string")]
	pub fn new_string(context:&impl IsA<Context>, string:Option<&str>) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_string(
				context.as_ref().to_glib_none().0,
				string.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_new_string_from_bytes")]
	pub fn new_string_from_bytes(context:&impl IsA<Context>, bytes:Option<&glib::Bytes>) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_string_from_bytes(
				context.as_ref().to_glib_none().0,
				bytes.to_glib_none().0,
			))
		}
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_new_typed_array")]
	pub fn new_typed_array(
		context:&impl IsA<Context>,
		type_:TypedArrayType,
		length:usize,
	) -> Value {
		unsafe {
			from_glib_full(ffi::jsc_value_new_typed_array(
				context.as_ref().to_glib_none().0,
				type_.into_glib(),
				length,
			))
		}
	}

	#[doc(alias = "jsc_value_new_undefined")]
	pub fn new_undefined(context:&impl IsA<Context>) -> Value {
		unsafe { from_glib_full(ffi::jsc_value_new_undefined(context.as_ref().to_glib_none().0)) }
	}

	// rustdoc-stripper-ignore-next
	/// Creates a new builder-pattern struct instance to construct [`Value`]
	/// objects.
	///
	/// This method returns an instance of
	/// [`ValueBuilder`](crate::builders::ValueBuilder) which can be used to
	/// create [`Value`] objects.
	pub fn builder() -> ValueBuilder { ValueBuilder::new() }
}

impl std::fmt::Display for Value {
	#[inline]
	fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(&ValueExt::to_str(self))
	}
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Value`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ValueBuilder {
	builder:glib::object::ObjectBuilder<'static, Value>,
}

impl ValueBuilder {
	fn new() -> Self { Self { builder:glib::object::Object::builder() } }

	pub fn context(self, context:&impl IsA<Context>) -> Self {
		Self { builder:self.builder.property("context", context.clone().upcast()) }
	}

	// rustdoc-stripper-ignore-next
	/// Build the [`Value`].
	#[must_use = "Building the object from the builder is usually expensive and is not expected to \
	              have side effects"]
	pub fn build(self) -> Value { self.builder.build() }
}

mod sealed {
	pub trait Sealed {}

	impl<T:super::IsA<super::Value>> Sealed for T {}
}

pub trait ValueExt: IsA<Value> + sealed::Sealed + 'static {
	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_array_buffer_get_size")]
	fn array_buffer_get_size(&self) -> usize {
		unsafe { ffi::jsc_value_array_buffer_get_size(self.as_ref().to_glib_none().0) }
	}

	//#[doc(alias = "jsc_value_constructor_call")]
	//#[must_use]
	// fn constructor_call(&self, first_parameter_type: glib::types::Type, :
	// /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Value> {
	//    unsafe { TODO: call ffi:jsc_value_constructor_call() }
	//}

	#[doc(alias = "jsc_value_constructor_callv")]
	#[must_use]
	fn constructor_callv(&self, parameters:&[Value]) -> Option<Value> {
		let n_parameters = parameters.len() as _;

		unsafe {
			from_glib_full(ffi::jsc_value_constructor_callv(
				self.as_ref().to_glib_none().0,
				n_parameters,
				parameters.to_glib_none().0,
			))
		}
	}

	//#[doc(alias = "jsc_value_function_call")]
	//#[must_use]
	// fn function_call(&self, first_parameter_type: glib::types::Type, :
	// /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Value> {
	//    unsafe { TODO: call ffi:jsc_value_function_call() }
	//}

	#[doc(alias = "jsc_value_function_callv")]
	#[must_use]
	fn function_callv(&self, parameters:&[Value]) -> Option<Value> {
		let n_parameters = parameters.len() as _;

		unsafe {
			from_glib_full(ffi::jsc_value_function_callv(
				self.as_ref().to_glib_none().0,
				n_parameters,
				parameters.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_get_context")]
	#[doc(alias = "get_context")]
	fn context(&self) -> Option<Context> {
		unsafe { from_glib_none(ffi::jsc_value_get_context(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_array")]
	fn is_array(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_array(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_is_array_buffer")]
	fn is_array_buffer(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_array_buffer(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_boolean")]
	fn is_boolean(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_boolean(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_constructor")]
	fn is_constructor(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_constructor(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_function")]
	fn is_function(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_function(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_null")]
	fn is_null(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_null(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_number")]
	fn is_number(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_number(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_object")]
	fn is_object(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_object(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_string")]
	fn is_string(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_string(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_is_typed_array")]
	fn is_typed_array(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_typed_array(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_is_undefined")]
	fn is_undefined(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_is_undefined(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_new_typed_array_with_buffer")]
	#[must_use]
	fn new_typed_array_with_buffer(
		&self,
		type_:TypedArrayType,
		offset:usize,
		length:isize,
	) -> Option<Value> {
		unsafe {
			from_glib_full(ffi::jsc_value_new_typed_array_with_buffer(
				self.as_ref().to_glib_none().0,
				type_.into_glib(),
				offset,
				length,
			))
		}
	}

	//#[doc(alias = "jsc_value_object_define_property_accessor")]
	// fn object_define_property_accessor(&self, property_name: &str, flags:
	// ValuePropertyFlags, property_type: glib::types::Type, getter:
	// Option<Box_<dyn FnOnce() + 'static>>, setter: Option<Box_<dyn Fn() +
	// 'static>>, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
	//    unsafe { TODO: call ffi:jsc_value_object_define_property_accessor() }
	//}

	#[doc(alias = "jsc_value_object_define_property_data")]
	fn object_define_property_data(
		&self,
		property_name:&str,
		flags:ValuePropertyFlags,
		property_value:Option<&impl IsA<Value>>,
	) {
		unsafe {
			ffi::jsc_value_object_define_property_data(
				self.as_ref().to_glib_none().0,
				property_name.to_glib_none().0,
				flags.into_glib(),
				property_value.map(|p| p.as_ref()).to_glib_none().0,
			);
		}
	}

	#[doc(alias = "jsc_value_object_delete_property")]
	fn object_delete_property(&self, name:&str) -> bool {
		unsafe {
			from_glib(ffi::jsc_value_object_delete_property(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_object_enumerate_properties")]
	fn object_enumerate_properties(&self) -> Vec<glib::GString> {
		unsafe {
			FromGlibPtrContainer::from_glib_full(ffi::jsc_value_object_enumerate_properties(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_object_get_property")]
	#[must_use]
	fn object_get_property(&self, name:&str) -> Option<Value> {
		unsafe {
			from_glib_full(ffi::jsc_value_object_get_property(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_object_get_property_at_index")]
	#[must_use]
	fn object_get_property_at_index(&self, index:u32) -> Option<Value> {
		unsafe {
			from_glib_full(ffi::jsc_value_object_get_property_at_index(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	#[doc(alias = "jsc_value_object_has_property")]
	fn object_has_property(&self, name:&str) -> bool {
		unsafe {
			from_glib(ffi::jsc_value_object_has_property(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	//#[doc(alias = "jsc_value_object_invoke_method")]
	//#[must_use]
	// fn object_invoke_method(&self, name: &str, first_parameter_type:
	// glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Basic:
	// VarArgs) -> Option<Value> {    unsafe { TODO: call
	// ffi:jsc_value_object_invoke_method() }
	//}

	#[doc(alias = "jsc_value_object_invoke_methodv")]
	#[must_use]
	fn object_invoke_methodv(&self, name:&str, parameters:&[Value]) -> Option<Value> {
		let n_parameters = parameters.len() as _;

		unsafe {
			from_glib_full(ffi::jsc_value_object_invoke_methodv(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
				n_parameters,
				parameters.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_object_is_instance_of")]
	fn object_is_instance_of(&self, name:&str) -> bool {
		unsafe {
			from_glib(ffi::jsc_value_object_is_instance_of(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "jsc_value_object_set_property")]
	fn object_set_property(&self, name:&str, property:&impl IsA<Value>) {
		unsafe {
			ffi::jsc_value_object_set_property(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
				property.as_ref().to_glib_none().0,
			);
		}
	}

	#[doc(alias = "jsc_value_object_set_property_at_index")]
	fn object_set_property_at_index(&self, index:u32, property:&impl IsA<Value>) {
		unsafe {
			ffi::jsc_value_object_set_property_at_index(
				self.as_ref().to_glib_none().0,
				index,
				property.as_ref().to_glib_none().0,
			);
		}
	}

	#[doc(alias = "jsc_value_to_boolean")]
	fn to_boolean(&self) -> bool {
		unsafe { from_glib(ffi::jsc_value_to_boolean(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_to_double")]
	fn to_double(&self) -> f64 {
		unsafe { ffi::jsc_value_to_double(self.as_ref().to_glib_none().0) }
	}

	#[doc(alias = "jsc_value_to_int32")]
	fn to_int32(&self) -> i32 { unsafe { ffi::jsc_value_to_int32(self.as_ref().to_glib_none().0) } }

	#[cfg(feature = "v2_28")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
	#[doc(alias = "jsc_value_to_json")]
	fn to_json(&self, indent:u32) -> Option<glib::GString> {
		unsafe { from_glib_full(ffi::jsc_value_to_json(self.as_ref().to_glib_none().0, indent)) }
	}

	#[doc(alias = "jsc_value_to_string")]
	#[doc(alias = "to_string")]
	fn to_str(&self) -> glib::GString {
		unsafe { from_glib_full(ffi::jsc_value_to_string(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "jsc_value_to_string_as_bytes")]
	fn to_string_as_bytes(&self) -> Option<glib::Bytes> {
		unsafe { from_glib_full(ffi::jsc_value_to_string_as_bytes(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_typed_array_get_buffer")]
	#[must_use]
	fn typed_array_get_buffer(&self) -> Option<Value> {
		unsafe {
			from_glib_full(ffi::jsc_value_typed_array_get_buffer(self.as_ref().to_glib_none().0))
		}
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_typed_array_get_length")]
	fn typed_array_get_length(&self) -> usize {
		unsafe { ffi::jsc_value_typed_array_get_length(self.as_ref().to_glib_none().0) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_typed_array_get_offset")]
	fn typed_array_get_offset(&self) -> usize {
		unsafe { ffi::jsc_value_typed_array_get_offset(self.as_ref().to_glib_none().0) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_typed_array_get_size")]
	fn typed_array_get_size(&self) -> usize {
		unsafe { ffi::jsc_value_typed_array_get_size(self.as_ref().to_glib_none().0) }
	}

	#[cfg(feature = "v2_38")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
	#[doc(alias = "jsc_value_typed_array_get_type")]
	fn typed_array_get_type(&self) -> TypedArrayType {
		unsafe { from_glib(ffi::jsc_value_typed_array_get_type(self.as_ref().to_glib_none().0)) }
	}
}

impl<O:IsA<Value>> ValueExt for O {}
