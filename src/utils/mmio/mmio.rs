use super::config::MmioConfig;
use super::error::MmioError;
use libc::MAP_FAILED;
use libc::MAP_SHARED;
use libc::PROT_READ;
use libc::PROT_WRITE;
use libc::c_void;
use libc::mmap;
use libc::munmap;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::ptr;

pub struct Mmio {
    config: MmioConfig,
    region: *mut u8,
}

impl Mmio {
    pub fn open(config: MmioConfig) -> Result<Self, MmioError> {
        let fd = OpenOptions::new().read(true).write(true).open(&config.path).map_err(|e| MmioError::Open {
            path: config.path.display().to_string(),
            source: e,
        })?;

        let region = unsafe { mmap(ptr::null_mut(), config.size, PROT_READ | PROT_WRITE, MAP_SHARED, fd.as_raw_fd(), config.offset) };

        if region == MAP_FAILED {
            return Err(MmioError::Mmap);
        }

        Ok(Self { config, region: region as *mut u8 })
    }

    fn check_bounds(&self, address: usize, size: usize) -> Result<(), MmioError> {
        if self.region.is_null() {
            return Err(MmioError::AlreadyClosed);
        }

        if address.checked_add(size).ok_or(MmioError::OutOfBounds { address, size: self.config.size })? > self.config.size {
            return Err(MmioError::OutOfBounds { address, size: self.config.size });
        }
        Ok(())
    }

    fn check_alignment(&self, address: usize, alignment: usize) -> Result<(), MmioError> {
        if address % alignment != 0 {
            return Err(MmioError::UnalignedAccess { address, alignment });
        }
        Ok(())
    }

    pub fn read32(&self, address: usize) -> Result<u32, MmioError> {
        self.check_alignment(address, 4)?;
        self.check_bounds(address, 4)?;

        let ptr = unsafe { self.region.add(address) as *const u32 };
        Ok(unsafe { ptr::read_volatile(ptr) })
    }

    pub fn write32(&self, address: usize, value: u32) -> Result<(), MmioError> {
        self.check_alignment(address, 4)?;
        self.check_bounds(address, 4)?;

        let ptr = unsafe { self.region.add(address) as *mut u32 };
        unsafe { ptr::write_volatile(ptr, value) };
        Ok(())
    }

    pub fn read64(&self, address: usize) -> Result<u64, MmioError> {
        let low = self.read32(address)? as u64;
        let high = self.read32(address + 4)? as u64;
        Ok(low | (high << 32))
    }

    pub fn write64(&self, address: usize, value: u64) -> Result<(), MmioError> {
        let low = (value & 0xFFFFFFFF) as u32;
        let high = ((value >> 32) & 0xFFFFFFFF) as u32;
        self.write32(address, low)?;
        self.write32(address + 4, high)?;
        Ok(())
    }

    pub fn read128(&self, address: usize) -> Result<u128, MmioError> {
        let word0 = self.read32(address)? as u128;
        let word1 = self.read32(address + 4)? as u128;
        let word2 = self.read32(address + 8)? as u128;
        let word3 = self.read32(address + 12)? as u128;
        Ok(word0 | (word1 << 32) | (word2 << 64) | (word3 << 96))
    }

    pub fn write128(&self, address: usize, value: u128) -> Result<(), MmioError> {
        let word0 = (value & 0xFFFFFFFF) as u32;
        let word1 = ((value >> 32) & 0xFFFFFFFF) as u32;
        let word2 = ((value >> 64) & 0xFFFFFFFF) as u32;
        let word3 = ((value >> 96) & 0xFFFFFFFF) as u32;
        self.write32(address, word0)?;
        self.write32(address + 4, word1)?;
        self.write32(address + 8, word2)?;
        self.write32(address + 12, word3)?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), MmioError> {
        if self.region.is_null() {
            return Err(MmioError::AlreadyClosed);
        }

        let result = unsafe { munmap(self.region as *mut c_void, self.config.size) };
        if result < 0 {
            return Err(MmioError::Munmap);
        }

        self.region = ptr::null_mut();
        Ok(())
    }
}

impl Drop for Mmio {
    fn drop(&mut self) {
        if !self.region.is_null() {
            let _ = self.close();
        }
    }
}

unsafe impl Send for Mmio {}
unsafe impl Sync for Mmio {}
