#pragma once
#include <vector>

class Solution {
    public:
        double new21Game(int n, int k, int maxPts);

    private:
        std::vector<double> dp_arr;
#ifdef LIPARADISE_DBG
        void dump();
#endif // LIPARADISE_DBG
};
