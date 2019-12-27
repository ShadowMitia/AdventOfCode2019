// Advent of code
// Day 2 : https://adventofcode.com/2019/day/2

#include <fmt/format.h>
#include <array>

template <std::size_t N>
constexpr std::array<std::size_t, N> runIntcodeProgram(
                                             std::array<std::size_t, N> const& program) {
  std::array<std::size_t, N> result{program};
  for (std::size_t i = 0; i < N-4; i += 4) {
    std::size_t pc{i};
    std::size_t opcode{pc};
    std::size_t op1{result[pc + 1]};
    std::size_t op2{result[pc + 2]};
    std::size_t resPosition{result[pc + 3]};
    switch (result[opcode]) {
    case 99:
      return result;
      break;
    case 1:
      result[resPosition] = result[op1] + result[op2];
      break;
    case 2:
      result[resPosition] = result[op1] * result[op2];
      break;
    default:
      /* Unrecognised op */
      return result;
      break;
    }
  }
  return result;
}

template <std::size_t N>
constexpr bool compare(std::array<std::size_t, N> const left, std::array<std::size_t, N> const right) {
  for (std::size_t i = 0; i < N; i++) {
    if (left[i] != right[i]) {
      return false;
    }
  }
  return true;
}

constexpr std::array<std::size_t, 125> intcodeProgram{
  1,/*0*/12,/*0*/2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0
    };

constexpr std::array<std::size_t, 125> initIntcodeProgram(long unsigned int input1, long unsigned int input2) {
  std::array<std::size_t, 125> result{1, input1,input2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0
    };
  return result;
}

int main() {
  constexpr std::array<std::size_t, 5> input1{runIntcodeProgram(std::array<std::size_t, 5>{1, 0, 0, 0, 99})};
  constexpr std::array<std::size_t, 5> output1{2, 0, 0, 0, 99};

  constexpr std::array<std::size_t, 5> input2{runIntcodeProgram(std::array<std::size_t, 5>{2, 3, 0, 3, 99})};
  constexpr std::array<std::size_t, 5> output2{2, 3, 0, 6, 99};

  constexpr std::array<std::size_t, 6> input3{runIntcodeProgram(std::array<std::size_t, 6>{2, 4, 4, 5, 99, 0})};
  constexpr std::array<std::size_t, 6> output3{2, 4, 4, 5, 99, 9801};

  constexpr std::array<std::size_t, 9> input4{runIntcodeProgram(std::array<std::size_t, 9>{1, 1, 1, 4, 99, 5, 6, 0, 99})};
  constexpr std::array<std::size_t, 9> output4{30, 1, 1, 4, 2, 5, 6, 0, 99};

  static_assert(compare(input1, output1) == true);
  static_assert(compare(input2, output2) == true);
  static_assert(compare(input3, output3) == true);
  static_assert(compare(input4, output4) == true);

  constexpr auto answerPart1 = runIntcodeProgram(intcodeProgram);

  fmt::print("Answer for part 1 : {}\n", answerPart1[0]);

  long unsigned int noun = 0;
  long unsigned int verb = 0;

  constexpr int expectedOutput = 19690720;

  bool finished = false;

  for (long unsigned int i = 0; i < 100; i++) {
    for (long unsigned int j = 0; j < 100; j++) {
      if (runIntcodeProgram(initIntcodeProgram(i, j))[0] == expectedOutput) {
        noun = i;
        verb = j;
        finished = true;
        break;
      }
    }
    if (finished) {
      break;
    }
  }
  

  fmt::print("Answer for part 2 : {}\n", 100 * noun + verb);
  
}
