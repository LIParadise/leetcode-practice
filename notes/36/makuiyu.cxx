// Source
// https://leetcode.com/problems/valid-sudoku/discuss/15464/My-short-solution-by-C%2B%2B.-O(n2)
// Added my own comments

#include <vector>
using std::vector;

class Solution {
    public:
        bool isValidSudoku(vector<vector<char>> &board) {
            // check 9 rows, cols, and blks
            // each time check 9 numbers
            // hence 9 by 9 matrices
            int checklist_row[9][9] = {0}, checklist_col[9][9] = {0},
                checklist_blk[9][9] = {0};

            for (int r = 0; r < board.size(); ++r)
                for (int c = 0; c < board[r].size(); ++c)
                    if (board[r][c] != '.') {
                        int num = board[r][c] - '0' - 1, b = r / 3 * 3 + c / 3;
                        /*
                         * `b` calculates index of block
                         * visualization:
                         *     COL 2 3 4 5 6 7 8
                         * ROW 0 0 0 1 1 1 2 2 2
                         *  1  0 0 0 1 1 1 2 2 2
                         *  2  0 0 0 1 1 1 2 2 2
                         *  3  3 3 3 4 4 4 5 5 5
                         *  4  3 3 3 4 4 4 5 5 5
                         *  5  3 3 3 4 4 4 5 5 5
                         *  6  6 6 6 7 7 7 8 8 8
                         *  7  6 6 6 7 7 7 8 8 8
                         *  8  6 6 6 7 7 7 8 8 8
                         */
                        if (checklist_row[r][num] || checklist_col[c][num] ||
                            checklist_blk[b][num]) {
                            return false;
                        }
                        checklist_row[r][num] = checklist_col[c][num] =
                            checklist_blk[b][num] = 1;
                    }

            return true;
        }
};
