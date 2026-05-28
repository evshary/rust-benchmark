pub fn prepare_numbers() -> Vec<u32> {
    (0..10_000).rev().collect()
}

pub fn sort_numbers(data: &mut [u32]) {
    data.sort_unstable();
}
