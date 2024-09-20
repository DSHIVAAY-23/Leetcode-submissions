impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap(); // maximum pile size

        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::calculate_hours(&piles, mid) <= h {
                right = mid; // try smaller speeds
            } else {
                left = mid + 1; // try larger speeds
            }
        }
        left
    }

    // Helper function to calculate the total number of hours Koko needs at speed `k`
    fn calculate_hours(piles: &Vec<i32>, k: i32) -> i32 {
        let mut hours = 0;
        for &pile in piles {
            hours += (pile + k - 1) / k; // this is equivalent to ceil(pile / k)
        }
        hours
    }
}