
/*
 * @lc app=leetcode id=118 lang=golang
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
func generate(numRows int) [][]int {
    var result [][]int

	for i := 0; i < numRows; i++ {
		row := make([]int, i+1)
		row[0] = 1
		row[i] = 1

		for j := 1; j < i; j++ {
			row[j] = result[i-1][j-1] + result[i-1][j]
		}

		result = append(result, row)
	}
    return result
}

// @lc code=end
