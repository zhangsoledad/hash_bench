extern "C" {
    /// Hashes input. Returns -1 if either out or input does not exist. Otherwise returns 0.
    pub fn keccak_256(out: *mut u8, outlen: usize, input: *const u8, inputlen: usize) -> i32;
    /// Hashes input. Returns -1 if either out or input does not exist. Otherwise returns 0.
    pub fn keccak_512(out: *mut u8, outlen: usize, input: *const u8, inputlen: usize) -> i32;
}
