use std::{alloc::Layout, cell::Cell};

use anyhow::Context;
use wasmer::{NativeFunc, WasmPtr};

const INITIAL_CHUNK_SIZE: usize = 2048;

const CHUNK_ALIGN: usize = 16;

fn round_up_to(n: usize, divisor: usize) -> Option<usize> {
    debug_assert!(divisor > 0);
    debug_assert!(divisor.is_power_of_two());
    Some(n.checked_add(divisor - 1)? & !(divisor - 1))
}

/// Host-controlled bump allocator for a WASM plugin.
/// See the Quill docs for why we use this.
///
/// The implementation is mostly ported from the `bumpalo`
/// crate.
pub struct WasmBump {
    /// Plugin function to allocate memory. Used
    /// for the slow path when the current chunk
    /// is exhausted.
    allocate_function: NativeFunc<(u32, u32), u32>,
    /// Plugin function to deallocate memory.
    deallocate_function: NativeFunc<(u32, u32, u32)>,
    /// Allocated chunks.
    chunks: Vec<Chunk>,
}

impl WasmBump {
    /// Creates a new bump allocator.
    pub fn new(
        allocate_function: NativeFunc<(u32, u32), u32>,
        deallocate_function: NativeFunc<(u32, u32, u32)>,
    ) -> anyhow::Result<Self> {
        let mut this = Self {
            allocate_function,
            deallocate_function,
            chunks: Vec::new(),
        };
        Ok(this)
    }

    /// Allocates memory of the given layout
    /// within the plugin's linear memory.
    pub fn alloc(&mut self, layout: Layout) -> anyhow::Result<WasmPtr<u8>> {
        let offset = match self.alloc_fast_path(layout) {
            Some(offset) => offset,
            None => self.alloc_slow_path(layout)?,
        };
        Ok(WasmPtr::new(offset))
    }

    fn alloc_fast_path(&self, layout: Layout) -> Option<u32> {
        let chunk = self.chunks.last().expect(">0 chunks");
        let ptr = chunk.ptr.get();
        let start = chunk.start;
        debug_assert!(start <= ptr);

        let ptr = ptr.checked_sub(layout.size() as u32)?;
        let aligned_ptr = ptr & !(layout.align() as u32 - 1);

        if aligned_ptr >= start {
            chunk.ptr.set(aligned_ptr);
            Some(aligned_ptr)
        } else {
            None
        }
    }

    /// Slow path for allocation where we need to allocate
    /// a new chunk.
    fn alloc_slow_path(&mut self, layout: Layout) -> anyhow::Result<u32> {
        let previous_size = self.chunks.last().expect(">0 chunks").layout.size();
        let new_chunk = self.allocate_chunk(Some(layout), Some(previous_size))?;
        self.chunks.push(new_chunk);
        Ok(self
            .alloc_fast_path(layout)
            .expect("new chunk can fit layout"))
    }

    fn allocate_chunk(
        &self,
        min_layout: Option<Layout>,
        previous_size: Option<usize>,
    ) -> anyhow::Result<Chunk> {
        let mut new_size = match previous_size {
            Some(previous_size) => previous_size
                .checked_mul(2)
                .context("chunk overflows usize")?,
            None => INITIAL_CHUNK_SIZE,
        };
        let mut align = CHUNK_ALIGN;
        if let Some(min_layout) = min_layout {
            align = align.max(min_layout.align());
            let requested_size =
                round_up_to(min_layout.size(), align).context("allocation too large")?;
            new_size = new_size.max(requested_size);
        }
        assert_eq!(align % CHUNK_ALIGN, 0);
        assert_eq!(new_size % CHUNK_ALIGN, 0);
        let layout = Layout::from_size_align(new_size, align).context("size or align is 0")?;
        assert!(new_size >= previous_size.unwrap_or(0) * 2);
        let start = self
            .allocate_function
            .call(layout.size() as u32, layout.align() as u32)?;
        Ok(Chunk {
            start,
            layout,
            ptr: Cell::new(start + new_size as u32),
        })
    }

    /// Resets the bump allocator, freeing
    /// all allocated memory.
    pub fn reset(&mut self) -> anyhow::Result<()> {
        // Free all but the last chunk.
        for chunk in self.chunks.drain(..self.chunks.len()) {
            self.deallocate_function.call(
                chunk.start,
                chunk.layout.size() as u32,
                chunk.layout.align() as u32,
            )?;
        }

        // Allocate initial chunk
        let chunk = self.allocate_chunk(None, None)?;
        self.chunks.push(chunk);

        Ok(())
    }
}

/// A chunk of memory in the bump allocator.
struct Chunk {
    /// Offset into linear memory of the start
    /// of this chunk.
    start: u32,
    /// Layout of the chunk.
    layout: Layout,
    /// Pointer to the next available byte plus one
    /// in the chunk. Starts at the end of the chunk.
    ptr: Cell<u32>,
}
