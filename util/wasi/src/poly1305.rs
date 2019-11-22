// Copyright 2015-2016 Brian Smith.
// Portions Copyright (c) 2014, 2015, Google Inc.
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
use wasmer_runtime_core::{vm::Ctx, types::ValueType, memory::ptr};
use super::deref;
use ring::aead::{
  Tag,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    subblocks: [u64; 2],
}

pub const BLOCK_LEN: usize = 16;

/// A Poly1305 key.
pub struct Key([Block; KEY_BLOCKS]);

pub const KEY_BLOCKS: usize = 2;

/// This is *not* an "AEAD nonce"; it's a Poly1305-specific nonce.
#[repr(C)]
struct Nonce(Block);

#[repr(C)]
struct Funcs {
    blocks_fn:
        unsafe extern "C" fn(&mut Opaque, input: *const u8, input_len: usize, should_pad: Pad),
    emit_fn: unsafe extern "C" fn(&mut Opaque, &mut Tag, nonce: &Nonce),
}

pub struct Context {
  opaque: Opaque,
  nonce: Nonce,
  func: Funcs,
}

/// The memory manipulated by the assembly.
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Opaque([u8; OPAQUE_LEN]);
const OPAQUE_LEN: usize = 192;

#[repr(u32)]
pub enum Pad {
    AlreadyPadded = 0,
    Pad = 1,
}

unsafe impl ValueType for Opaque {}

extern "C" {
    fn GFp_poly1305_blocks(
        state: &mut Opaque,
        input: *const u8,
        len: usize,
        should_pad: Pad,
    );
//    fn GFp_poly1305_emit(state: &mut Opaque, tag: &mut Tag, nonce: &Nonce);
}

pub fn __gfp_poly1305_blocks(ctx: &mut Ctx, state: WasmPtr<Opaque>, input: u32, len: u32, should_pad: u32) {
    let memory = ctx.memory(0);
    let state: &mut Opaque = unsafe { state.deref_mut(memory).unwrap().get_mut() };
    let input: *const u8 = deref(memory, input as usize).unwrap().get();
    let pad = match should_pad { 0 => Pad::AlreadyPadded, 1 => Pad::Pad, _ => Err(()).unwrap() };
    unsafe { GFp_poly1305_blocks(state, input, len as usize, pad) };
}
