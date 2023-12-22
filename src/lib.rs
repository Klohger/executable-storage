#![feature(generic_const_exprs)]
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
};

#[repr(C)]
pub struct StaticStorage<T, const PREFIX_SIZE: usize> {
    pub prefix: [u8; PREFIX_SIZE],
    pub value: T,
}

pub struct ExecutableStorage<const PREFIX_SIZE: usize, T: 'static>
where
    [u8; size_of::<T>()]:,
{
    static_storage: &'static mut StaticStorage<T, PREFIX_SIZE>,
    file: BufWriter<File>,
    location: u64,
    _phantom_data: PhantomData<T>,
}

impl<const PREFIX_SIZE: usize, T: 'static> DerefMut for ExecutableStorage<PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.static_storage.value
    }
}

impl<const PREFIX_SIZE: usize, T: 'static> Deref for ExecutableStorage<PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.static_storage.value
    }
}

impl<const PREFIX_SIZE: usize, T: 'static> ExecutableStorage<PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    pub fn new(static_storage: &'static mut StaticStorage<T, PREFIX_SIZE>) -> io::Result<Self>
    where
        [u8; PREFIX_SIZE - 1]:,
    {
        let exe_path = std::env::current_exe()?;
        fs::copy(".\\tmp", &exe_path)?;

        let file = File::options().read(true).write(true).open(exe_path)?;

        let location = {
            let mut reader = BufReader::new(&file);
            loop {
                let mut buf = [0; PREFIX_SIZE];

                reader.read_exact(&mut buf[0..1])?;

                if buf[0] == static_storage.prefix[0] {
                    reader.read_exact(&mut buf[1..])?;
                    if buf[1..] == static_storage.prefix[1..] {
                        break reader.seek(std::io::SeekFrom::Current(0))?;
                    }
                }
            }
        };

        Ok(Self {
            static_storage,
            file: BufWriter::new(file),
            location,
            _phantom_data: PhantomData,
        })
    }
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.seek(io::SeekFrom::Start(self.location))?;

        let data = unsafe {
            (&mut self.static_storage.value as *mut _ as *mut [u8; size_of::<T>()])
                .as_mut()
                .unwrap_unchecked()
        };

        self.file.write(data)?;

        Ok(())
    }
}

impl<const PREFIX_SIZE: usize, T: 'static> Drop for ExecutableStorage<PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    fn drop(&mut self) {
        let _result = self.flush();
    }
}
