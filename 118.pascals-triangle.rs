/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (70.20%)
 * Likes:    10415
 * Dislikes: 336
 * Total Accepted:    1.3M
 * Total Submissions: 1.8M
 * Testcase Example:  '5'
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it as shown:
 * 
 * 
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= numRows <= 30
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..num_rows {
        let mut row: Vec<i32> = Vec::with_capacity((i + 1) as usize);
        row.push(1);

        for j in 1..i {
            let value = result[(i - 1) as usize][(j - 1) as usize] + result[(i - 1) as usize][j as usize];
            row.push(value);
        }

        if i > 0 {
            row.push(1);
        }

        result.push(row);
    }

    result
}

}
// @lc code=end

