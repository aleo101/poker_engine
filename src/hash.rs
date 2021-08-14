use crate::dptables::DP;

pub fn hash_quinary(q: &[u8], mut k: i32) -> u32 {
    let mut sum: u32 = 0;
    const LEN: i32 = 13;

    for i in 0..LEN {
        sum += DP[q[i as usize] as usize][(LEN - i - 1) as usize][k as usize];

        k -= q[i as usize] as i32;

        if k <= 0 {
            break;
        }
    }

    return sum;
}
