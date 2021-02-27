use wasmer::{Array, WasmPtr};

pub trait WasmPtrExt {
    fn add(self, offset: usize) -> Self;
}

impl<T, Ty> WasmPtrExt for WasmPtr<T, Ty>
where
    T: Copy,
{
    fn add(self, offset: usize) -> Self {
        WasmPtr::new(self.offset() + offset as u32)
    }
}

pub trait WasmPtrIntoArray<T> {
    fn array(self) -> WasmPtr<T, Array>
    where
        T: Copy;
}

impl<T> WasmPtrIntoArray<T> for WasmPtr<T>
where
    T: Copy,
{
    fn array(self) -> WasmPtr<T, Array> {
        WasmPtr::new(self.offset())
    }
}
