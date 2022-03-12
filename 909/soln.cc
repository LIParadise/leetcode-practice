#include "soln.h"
#include <algorithm>
#include <cstddef>
#include <queue>
#include <tuple>

namespace SolutionHelper {
std::pair<std::size_t, std::size_t>
boardIndexTranslator(const std::size_t n, const std::size_t idx) {
    std::size_t row_idx = n - ((idx - 1) / n) - 1;
    std::size_t col_idx;
    if ((row_idx + n) % 2 == 1) {
        col_idx = (idx - 1) % n;
    } else {
        col_idx = (n - 1) - ((idx - 1) % n);
    }
    return std::make_pair(row_idx, col_idx);
}
} // namespace SolutionHelper

int Solution::snakesAndLadders(std::vector<std::vector<int>> &board) {
    constexpr auto idxTrans = SolutionHelper::boardIndexTranslator;
    std::vector<std::vector<int>> bfsTable(board.size(),
                                           std::vector<int>(board.size(), -1));
    std::size_t row_idx, col_idx;
    std::queue<std::pair<std::size_t, int>> bfsQ;
    for (std::size_t s = 1; s <= 6 && ((s - 1) / board.size()) < board.size();
         ++s) {
        bfsQ.push(std::make_pair(s, 1));
    }
    while (!bfsQ.empty()) {
        std::size_t boardIdx;
        int steps;

        // BFS candidate
        std::tie(boardIdx, steps) = bfsQ.front();
        bfsQ.pop();
        std::tie(row_idx, col_idx) = idxTrans(board.size(), boardIdx);
        // Take the snake/ladder
        if (board.at(row_idx).at(col_idx) >= 1) {
            boardIdx = board.at(row_idx).at(col_idx);
            std::tie(row_idx, col_idx) =
                idxTrans(board.size(), board.at(row_idx).at(col_idx));
        }

        // BFS markings
        if (bfsTable.at(row_idx).at(col_idx) == -1) {
            bfsTable.at(row_idx).at(col_idx) = steps;
            for (std::size_t s = boardIdx + 1;
                 s <= (boardIdx + 6) && ((s - 1) / board.size()) < board.size();
                 ++s) {
                bfsQ.push(std::make_pair(s, steps + 1));
            }
        }
    }

    row_idx = 0;
    col_idx = (board.size() % 2) ? (board.size() - 1) : 0;
    return bfsTable.at(row_idx).at(col_idx);
}
