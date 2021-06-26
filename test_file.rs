pub fn sum(n: u32) -> u32 {
    let mut total: u32 = 0;
    let mut i: u32 = 0;
    while(i < n) {
        total += i;
        i += 2;
    }
    return total
}
fn main() {
	
}