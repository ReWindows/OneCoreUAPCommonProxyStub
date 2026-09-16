//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const type_info::`vftable'", rva: 0xA9018, first_slot: 0, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const std::exception::`vftable'", rva: 0xA9028, first_slot: 1, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_alloc::`vftable'", rva: 0xA9040, first_slot: 3, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_array_new_length::`vftable'", rva: 0xA9058, first_slot: 5, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const ContextMenuRemoteProxyProvider::`vftable'", rva: 0xB2508, first_slot: 7, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const wil::TraceLoggingProvider::`vftable'", rva: 0xB2528, first_slot: 11, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const wil::ResultException::`vftable'", rva: 0xB2548, first_slot: 15, slot_count: 2, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0xA9018, slot: 0, byte_offset: 0, target_rva: 0x4D10, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xA9028, slot: 0, byte_offset: 0, target_rva: 0x7B40, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xA9028, slot: 1, byte_offset: 8, target_rva: 0xA520, target_id: Some("?what@exception@std@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl std::exception::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0xA9040, slot: 0, byte_offset: 0, target_rva: 0x7B40, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xA9040, slot: 1, byte_offset: 8, target_rva: 0xA520, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xA9058, slot: 0, byte_offset: 0, target_rva: 0x7B40, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xA9058, slot: 1, byte_offset: 8, target_rva: 0xA520, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2508, slot: 0, byte_offset: 0, target_rva: 0x8CC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2508, slot: 1, byte_offset: 8, target_rva: 0x89B0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2508, slot: 2, byte_offset: 16, target_rva: 0x8D00, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2508, slot: 3, byte_offset: 24, target_rva: 0x7A80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2528, slot: 0, byte_offset: 0, target_rva: 0x8CC0, target_id: Some("?NotifyFailure@TraceLoggingProvider@wil@@EEAA_NAEBUFailureInfo@2@@Z"), target_name: Some("private: virtual bool __cdecl wil::TraceLoggingProvider::NotifyFailure(struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0xB2528, slot: 1, byte_offset: 8, target_rva: 0x89B0, target_id: Some("?Initialize@TraceLoggingProvider@wil@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::Initialize(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0xB2528, slot: 2, byte_offset: 16, target_rva: 0x8D00, target_id: Some("?OnErrorReported@TraceLoggingProvider@wil@@MEAAX_NAEBUFailureInfo@2@@Z"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::OnErrorReported(bool, struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0xB2528, slot: 3, byte_offset: 24, target_rva: 0x7B00, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2548, slot: 0, byte_offset: 0, target_rva: 0x7AC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0xB2548, slot: 1, byte_offset: 8, target_rva: 0xA3C0, target_id: Some("?what@ResultException@wil@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl wil::ResultException::what(void) const"), ambiguous: false, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
