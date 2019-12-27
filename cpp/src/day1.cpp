// Advent of code 
// Day 1 : https://adventofcode.com/2019/day/1

#include <vector>
#include <fmt/format.h>

constexpr int calculateFuelForModule(int moduleMass) {
    int fuel = moduleMass / 3 - 2;
    if (fuel < 0) {
        return 0;
    } else {
        return fuel;
    }
}

std::vector<int> modules{
                         129880,
                         115705,
                         118585,
                         124631,
                         81050,
                         138183,
                         61173,
                         95354,
                         130788,
                         89082,
                         75554,
                         110104,
                         140528,
                         71783,
                         125889,
                         126602,
                         73089,
                         76822,
                         51774,
                         85940,
                         81004,
                         149584,
                         145921,
                         105570,
                         142370,
                         80823,
                         147779,
                         115651,
                         70250,
                         67763,
                         128192,
                         51298,
                         134963,
                         73510,
                         90976,
                         141216,
                         65134,
                         140468,
                         143998,
                         101711,
                         88477,
                         53335,
                         138328,
                         141186,
                         149804,
                         64950,
                         53107,
                         54648,
                         97557,
                         85927,
                         125038,
                         80514,
                         64912,
                         140591,
                         114229,
                         57089,
                         123464,
                         127572,
                         137169,
                         146550,
                         51138,
                         115504,
                         128034,
                         147244,
                         108107,
                         101205,
                         51498,
                         136829,
                         140171,
                         59441,
                         144489,
                         139384,
                         145841,
                         96771,
                         116821,
                         88599,
                         126780,
                         65012,
                         67621,
                         129699,
                         149639,
                         97590,
                         147527,
                         117462,
                         146709,
                         60527,
                         107643,
                         92956,
                         72177,
                         92285,
                         62475,
                         63099,
                         66904,
                         77268,
                         62945,
                         134364,
                         106924,
                         117842,
                         130016,
                         123712
};


int main() {

    // Tests from examples
    static_assert(calculateFuelForModule(12) == 2);
    static_assert(calculateFuelForModule(14) == 2);
    static_assert(calculateFuelForModule(1969) == 654);
    static_assert(calculateFuelForModule(100756) == 33583);

    // Answer for part 1
    int totalFuel = 0;
    for (auto module : modules) {
        totalFuel += calculateFuelForModule(module);
    }

    fmt::print("Answer for part 1: {}\n", totalFuel);

    // Tests from examples
    static_assert(calculateFuelForModule(14) + calculateFuelForModule(2) == 2);
    static_assert(calculateFuelForModule(1969) + calculateFuelForModule(654) + calculateFuelForModule(216) + calculateFuelForModule(70) + calculateFuelForModule(21) + calculateFuelForModule(5)  == 966);
    static_assert(calculateFuelForModule(100756) + calculateFuelForModule(33583) + calculateFuelForModule(11192) + calculateFuelForModule(3728) + calculateFuelForModule(1240) + calculateFuelForModule(411) + calculateFuelForModule(135) + calculateFuelForModule(43) + calculateFuelForModule(12) + calculateFuelForModule(2) == 50346);

    // Answer for part 2
    totalFuel = 0;
    for (auto module : modules) {
        int moduleFuel = calculateFuelForModule(module);
        while (moduleFuel > 0) {
            totalFuel += moduleFuel;
            moduleFuel = calculateFuelForModule(moduleFuel);
        }

    }

    fmt::print("Answer for part 2: {}\n", totalFuel);
}
