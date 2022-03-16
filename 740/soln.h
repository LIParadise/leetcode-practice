#include <utility>
#include <vector>

class Solution {
    public:
        int deleteAndEarn(std::vector<int> &nums);

    private:
        /* modified from https://stackoverflow.com/a/3279550
         * Objective:
         * Implement a triangular array using 1D array
         * Conceptual indexing: (row,col)
         * (For `mSize == 6`, hosting 21 entries)
         *                    0,5
         *                 1,5   0,4
         *              2,5   1,4   0,3
         *           3,5   2,4   1,3   0,2
         *        4,5   3,4   2,3   1,2   0,1
         *     5,5   4,4   3,3   2,2   1,1   0,0
         * The underlying 1D array is s.t.
         * (0,5), (0,4), (0,3), (0,2), (0,1), (0,0), (1,5), ...
         * [0]    [1]    [2]    [3]    [4]    [5]    [6]    ...
         */
        class TriangularMatrix {
            public:
                // (default) constructor
                TriangularMatrix(std::size_t size = 0)
                    : mSize(size),
                      privateRaw1DArr(
                          mSize ? new int[((mSize % 2)
                                               ? ((mSize + 1) / 2) * mSize
                                               : (mSize / 2) * (mSize + 1))]()
                                : nullptr),
                      triangMatrix(mSize ? new int *[mSize]() : nullptr) {
                    if (mSize) {
                        triangMatrix[0] = privateRaw1DArr;
                        for (std::size_t s = 1; s < mSize; ++s) {
                            triangMatrix[s] =
                                triangMatrix[s - 1] + ((mSize + 1) - s);
                        }
                    }
                };

                // copy-constructor
                TriangularMatrix(const TriangularMatrix &other)
                    : TriangularMatrix(other.mSize) {
                    // Note that this is non-throwing, because of the data types
                    // being used; more attention to detail with regards to
                    // exceptions must be given in a more general case, however
                    if (mSize) {
                        std::copy(other.privateRaw1DArr,
                                  other.privateRaw1DArr + mSize,
                                  privateRaw1DArr);
                    }
                }

                // copy-and-swap idiom
                friend void swap(TriangularMatrix &first,
                                 TriangularMatrix &second) {
                    // enable ADL (not necessary in our case, but good practice)
                    using std::swap;

                    // by swapping the members of two objects,
                    // the two objects are effectively swapped
                    swap(first.mSize, second.mSize);
                    swap(first.privateRaw1DArr, second.privateRaw1DArr);
                    swap(first.triangMatrix, second.triangMatrix);
                }

                // copy-assignment operator
                TriangularMatrix &operator=(TriangularMatrix other) {
                    swap(*this, other);
                    return *this;
                }

                // move constructor
                // initialize via default constructor, C++11 only
                TriangularMatrix(TriangularMatrix &&other) noexcept
                    : TriangularMatrix() {
                    swap(*this, other);
                }

                // dtor
                ~TriangularMatrix() {
                    delete[] privateRaw1DArr;
                    delete[] triangMatrix;
                }

                int at(const std::size_t row, const std::size_t col) const {
                    if (row <= col && col < mSize) {
                        const auto coordinate = translateRowCol(row, col);
                        return triangMatrix[coordinate.first]
                                           [coordinate.second];
                    } else {
                        return 0;
                    }
                }

                void set(const std::size_t row, const std::size_t col,
                         const int v) {
                    if (row <= col && col < mSize) {
                        const auto coordinate = translateRowCol(row, col);
                        triangMatrix[coordinate.first][coordinate.second] = v;
                    }
                }

            private:
                std::size_t mSize;
                int *privateRaw1DArr;
                int **triangMatrix;
                inline std::pair<std::size_t, std::size_t>
                translateRowCol(const std::size_t row,
                                const std::size_t col) const {
                    return std::make_pair(row, mSize - 1 - col);
                }
        };
};
