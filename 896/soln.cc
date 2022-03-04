class Solution {
public:
    bool isMonotonic(vector<int>& nums) {
        if(nums.size() <= 1){
            return true;
        }
        bool is_monotone_dec = true;
        bool is_monotone_inc = true;
        for(size_t s = 1; s < nums.size(); ++s){
            if(nums.at(s) > nums.at(s-1)){
                is_monotone_dec = false;
                break;
            }
        }
        for(size_t s = 1; s < nums.size(); ++s){
            if(nums.at(s) < nums.at(s-1)){
                is_monotone_inc = false;
                break;
            }
        }
        return is_monotone_inc || is_monotone_dec;
    }
};
