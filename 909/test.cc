#include "soln.h"
#include <iostream>

int main() {
    Solution s;
    int ret;

    // [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
    std::vector<std::vector<int>> board;
    board.emplace_back(std::vector<int>({-1, -1, -1, -1, -1, -1}));
    board.emplace_back(std::vector<int>({-1, -1, -1, -1, -1, -1}));
    board.emplace_back(std::vector<int>({-1, -1, -1, -1, -1, -1}));
    board.emplace_back(std::vector<int>({-1, 35, -1, -1, 13, -1}));
    board.emplace_back(std::vector<int>({-1, -1, -1, -1, -1, -1}));
    board.emplace_back(std::vector<int>({-1, 15, -1, -1, -1, -1}));
    ret = s.snakesAndLadders(board);
    std::cout << "My: " << ret << "; Expected: " << 4 << std::endl;

    // [[-1,-1],[-1,3]]
    board.clear();
    board.emplace_back(std::vector<int>({-1, -1}));
    board.emplace_back(std::vector<int>({-1, 3}));
    ret = s.snakesAndLadders(board);
    std::cout << "My: " << ret << "; Expected: " << 1 << std::endl;

    // [[-1,-1,-1],[-1,9,8],[-1,8,9]]
    board.clear();
    board.emplace_back(std::vector<int>({-1, -1, -1}));
    board.emplace_back(std::vector<int>({-1, 9, 8}));
    board.emplace_back(std::vector<int>({-1, 8, 9}));
    ret = s.snakesAndLadders(board);
    std::cout << "My: " << ret << "; Expected: " << 1 << std::endl;

    // [[-1,1,2,-1],[2,13,15,-1],[-1,10,-1,-1],[-1,6,2,8]]
    board.clear();
    board.emplace_back(std::vector<int>({-1, 1, 2, -1}));
    board.emplace_back(std::vector<int>({2, 13, 15, -1}));
    board.emplace_back(std::vector<int>({-1, 10, -1, -1}));
    board.emplace_back(std::vector<int>({-1, 6, 2, 8}));
    ret = s.snakesAndLadders(board);
    std::cout << "My: " << ret << "; Expected: " << 2 << std::endl;

    return 0;
}
