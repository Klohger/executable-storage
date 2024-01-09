#![feature(generic_const_exprs)]
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
};

#[repr(C)]
pub struct Entry<T, const PREFIX_SIZE: usize> {
    pub prefix: [u8; PREFIX_SIZE],
    pub default_value: T,
}

pub struct ExecutableStorage<'a, const PREFIX_SIZE: usize, T: 'a>
where
    [u8; size_of::<T>()]:,
{
    entry: &'a mut Entry<T, PREFIX_SIZE>,
    file: File,
    location: u64,
    _phantom_data: PhantomData<T>,
}

impl<'a, const PREFIX_SIZE: usize, T: 'a> DerefMut for ExecutableStorage<'a, PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry.default_value
    }
}

impl<'a, const PREFIX_SIZE: usize, T: 'a> Deref for ExecutableStorage<'a, PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.entry.default_value
    }
}

impl<'a, const PREFIX_SIZE: usize, T: 'a> ExecutableStorage<'a, PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    pub fn new(entry: &'a mut Entry<T, PREFIX_SIZE>) -> io::Result<Self>
    where
        [u8; PREFIX_SIZE - 1]:,
    {
        let exe_path = std::env::current_exe()?;
        fs::rename(&exe_path, ".\\tmp")?;
        fs::copy(".\\tmp", &exe_path)?;

        let file = File::options().read(true).write(true).open(exe_path)?;

        let location = {
            let mut reader = BufReader::new(&file);
            loop {
                let mut buf = [0; PREFIX_SIZE];

                reader.read_exact(&mut buf[0..1])?;

                if buf[0] == entry.prefix[0] {
                    reader.read_exact(&mut buf[1..])?;
                    if buf[1..] == entry.prefix[1..] {
                        break reader.seek(std::io::SeekFrom::Current(0))?;
                    }
                }
            }
        };

        Ok(Self {
            entry,
            file,
            location,
            _phantom_data: PhantomData,
        })
    }
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.seek(io::SeekFrom::Start(self.location))?;

        let data = unsafe {
            (&mut self.entry.default_value as *mut _ as *mut [u8; size_of::<T>()])
                .as_mut()
                .unwrap_unchecked()
        };

        self.file.write_all(data)?;
        self.file.flush()?;
        Ok(())
    }
}

impl<'a, const PREFIX_SIZE: usize, T: 'a> Drop for ExecutableStorage<'a, PREFIX_SIZE, T>
where
    [u8; size_of::<T>()]:,
{
    fn drop(&mut self) {
        let _result = self.flush();
    }
}
