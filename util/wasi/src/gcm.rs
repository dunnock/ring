// Copyright 2018 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use wasmer_runtime::WasmPtr;
use wasmer_runtime_core::{vm::Ctx, types::ValueType};
use super::deref;

// Had to copy from ring/src/aead/gcm.rs since it is private
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Key(GCM128_KEY);
// Keep in sync with `GCM128_KEY` in modes/internal.h.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
struct GCM128_KEY {
    Htable: [u128; GCM128_HTABLE_LEN],
}
#[derive(Clone, Copy)]
#[repr(C)]
struct u128 {
    hi: u64,
    lo: u64,
}
const GCM128_HTABLE_LEN: usize = 16;
unsafe impl ValueType for Key {}

extern "C" {
    fn GFp_gcm_init_avx(key: &mut Key, h: &[u64; 2]);
}
pub fn __gfp_gcm_init_avx(ctx: &mut Ctx, key: WasmPtr<Key>, hptr: u32) {
    let memory = ctx.memory(0);
    let key: &mut Key = unsafe { key.deref_mut(memory).unwrap().get_mut() };
    let hs: [u64; 2] = deref(memory, hptr as usize).unwrap().get();
    unsafe { GFp_gcm_init_avx(key, &hs) };
}
