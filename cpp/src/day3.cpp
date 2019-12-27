// Advent of code
// Day 3 : https://adventofcode.com/2019/day/3

#include <fmt/format.h>
#include <string>
#include <utility>
#include <sstream>

std::vector<std::string> split(std::string input, char delimiter) {
  /* https://www.fluentcpp.com/2017/04/21/how-to-split-a-string-in-c/ */
  std::vector<std::string> tokens;
  std::string token;
  std::istringstream tokenStream(input);
  while (std::getline(tokenStream, token, delimiter))
    {
      tokens.push_back(token);
    }
  return tokens;
}

struct v2 {
  double x{0};
  double y{0};
};

std::vector<v2> getPointsList(std::vector<std::string> const& toParse) {
  std::vector<v2> result;
  result.emplace_back(v2{});
  v2 position;
  for (auto el : toParse) {
    std::stringstream ss;
    ss << el;
    int num;
    char letter;
    ss >> letter >> num;
    switch (letter) {
    case 'R':{
      position.x += num;
    } break;
    case 'D':{
      position.y -= num;
    } break;
    case 'U':{
      position.y += num;
    } break;
    case 'L':{
      position.x -= num;
    } break;
    }

    result.emplace_back(position);
  }

  return result;
}


constexpr double manhattanDistance(v2 pos1, v2 pos2) noexcept {
  return pos1.x + pos2.x + pos1.y + pos2.y;
}

std::vector<v2> intersections(std::vector<v2> const& line1, std::vector<v2> const& line2) {
  /* https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect */
  /* https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection */
  std::vector<v2> result;

  for (std::size_t i = 0; i < line1.size() - 1; i++) {
    for (std::size_t j = 0; j < line2.size() - 1; j++) {
      const double& x1 = line1[i].x;
      const double& y1 = line1[i].y;
      const double& x2 = line1[i+1].x;
      const double& y2 = line1[i+1].y;

      const double& x3 = line2[j].x;
      const double& y3 = line2[j].y;
      const double& x4 = line2[j+1].x;
      const double& y4 = line2[j+1].y;

      double numT = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4));
      double numU = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3));
      double den  = ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

      if (den != 0.0) {
        double t = numT / den;
        double u = numU / den;

        if ((t >= 0.0 && t <= 1.0) && (u >= 0.0 && u <= 1.0)) {
          v2 temp;
          temp.x = x1 + t * (x2 - x1);
          temp.y = y1 + t * (y2 - y1);

          result.push_back(temp);
        }
      }
    }
  }

  return result;
}

double findClosestManhattanIntersection(std::string const& input) {
  std::vector<std::string> lines{split(input, '\n')};
  std::vector<std::string> line1Str{split(lines[0], ',')};
  std::vector<std::string> line2Str{split(lines[1], ',')};
  std::vector<v2> line1{getPointsList(line1Str)};
  std::vector<v2> line2{getPointsList(line2Str)};

  std::vector<v2> res = intersections(line1, line2);

  double closestDist = 100'000;
  for (auto el : res) {
    double test = manhattanDistance(v2{}, el);
    if (test < closestDist && test > 0.0) {
      closestDist = test;
    }
  }
  return closestDist;
}

int main() {
  std::pair example1{std::make_pair("R8,U5,L5,D3\nU7,R6,D4,L4", 6)};
  std::pair example2{std::make_pair("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 159)};
  std::pair example3{std::make_pair("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135)};


  double closestDist1 = findClosestManhattanIntersection(example1.first);
  //fmt::print("{} - expected {} got {}\n\n\n", closestDist1 == example1.second, example1.second, closestDist1);
  double closestDist2 = findClosestManhattanIntersection(example2.first);
  //fmt::print("{} - expected {} got {}\n\n\n", closestDist2 == example2.second, example2.second, closestDist2);
  double closestDist3 = findClosestManhattanIntersection(example3.first);
  //fmt::print("{} - expected {} got {}\n\n\n", closestDist3 == example3.second, example3.second, closestDist3);

  std::string input{
            "R1001,D915,R511,D336,L647,D844,R97,D579,L336,U536,L645,D448,R915,D473,L742,D470,R230,D558,R214,D463,L374,D450,R68,U625,L937,D135,L860,U406,L526,U555,R842,D988,R819,U995,R585,U218,L516,D756,L438,U921,R144,D62,R238,U144,R286,U934,L682,U13,L287,D588,L880,U630,L882,D892,R559,D696,L329,D872,L946,U219,R593,U536,R402,U946,L866,U690,L341,U729,R84,U997,L579,D609,R407,D846,R225,U953,R590,U79,R590,U725,L890,D384,L442,D364,R600,D114,R39,D962,R413,U698,R762,U520,L180,D557,R35,U902,L476,U95,R830,U858,L312,U879,L85,U620,R505,U248,L341,U81,L323,U296,L53,U532,R963,D30,L380,D60,L590,U699,R967,U88,L725,D730,R706,D337,L248,D46,R131,U541,L313,U508,R120,D719,R28,U342,R555,U780,R397,D523,L619,D820,R865,D4,L790,D544,L873,D249,L220,U343,R818,U803,R309,D576,R811,D717,L800,D171,R523,U630,L854,U265,R207,U147,R518,U237,R822,D672,L140,U580,R408,D739,L519,U759,R664,D61,R258,D313,R472,U437,R975,U828,L54,D892,L370,U509,L80,U593,L268,U856,L177,U950,L266,U29,R493,D228,L110,U390,L92,U8,L288,U732,R459,D422,R287,D359,R915,U295,R959,U215,R82,D357,L970,D782,L653,U399,L50,D720,R788,D396,L562,D560,R798,D196,R79,D732,R332,D957,L106,D199,R756,U379,R716,U282,R812,U346,R592,D416,L454,U612,L160,U884,R373,U306,R55,D492,R175,D233,L249,D616,L342,D650,L181,U868,L761,D170,L976,U711,R377,D113,L548,U39,R62,D99,R853,U249,L951,U617,R257,U457,R430,D355,L541,U595,L176,D987,R365,D77,L181,D192,L688,D942,R617,U484,R247,U180,R771,D392,R184,U597,L682,U454,R856,U616,R174,U629,L607,U41,L970,D602,R402,D208,R826\nL994,U238,R605,U233,L509,U81,R907,U880,R666,D86,R6,U249,R345,D492,L912,U770,L827,D107,R988,D525,L471,U706,R31,U485,R835,D778,R419,D461,L937,D740,R559,U309,L379,U385,R828,D698,R276,U914,L911,U969,R282,D365,L43,D911,R256,D592,L451,U162,L829,D564,R349,U279,R19,D110,R259,D551,L172,D899,L924,D819,R532,U737,L794,U995,R168,D359,R847,U426,R224,U984,L929,D531,L797,U292,L332,D280,R317,D648,R776,D52,R916,U363,R919,U890,R583,U961,L89,D680,L894,D226,L83,U68,R551,U413,R259,D468,L702,U453,L128,U986,R238,U805,R431,U546,R944,D142,R677,D783,R336,D220,R40,U391,R5,D760,L963,D764,R653,U932,R473,U311,L189,D883,R216,U391,L634,U275,L691,U975,R130,D543,L163,U736,R964,U729,R752,D531,R90,D471,R687,D341,R441,U562,R570,U278,R570,U177,L232,U781,L874,U258,R180,D28,R916,D395,R96,U954,L222,U578,L394,U775,L851,D18,L681,D912,L761,U945,L866,D12,R420,D168,R490,U679,R521,D91,L782,U583,L823,U656,L365,D517,R319,U725,L824,D531,L747,U822,R893,D162,L11,D913,L295,D65,L393,D351,L432,U828,L131,D384,R311,U381,L26,D635,L180,D395,L576,D836,R548,D820,L219,U749,L64,D2,L992,U104,L501,U247,R693,D862,R16,U346,R332,U618,R387,U4,L206,U943,R734,D164,R771,U17,L511,D475,L75,U965,R116,D627,R243,D77,R765,D831,L51,U879,R207,D500,R289,U749,R206,D850,R832,U407,L985,U514,R290,U617,L697,U812,L633,U936,R214,D447,R509,D585,R787,D500,R305,D598,R866,U781,L771,D350,R558,U669,R284,D686,L231,U574,L539,D337,L135,D751,R315,D344,L694,D947,R118,U377,R50,U181,L96,U904,L776,D268,L283,U233,L757,U536,L161,D881,R724,D572,R322"
  };

  double closestDist = findClosestManhattanIntersection(input);
  fmt::print("Part 1 answer : {}\n", closestDist);

  std::pair exampleSteps1{std::make_pair("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 610)};
  std::pair exampleSteps2{std::make_pair("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 410)};

  std::vector<std::string> lines{split(exampleSteps1.first, '\n')};
  std::vector<std::string> line1Str{split(lines[0], ',')};
  std::vector<std::string> line2Str{split(lines[1], ',')};
  std::vector<v2> line1{getPointsList(line1Str)};
  std::vector<v2> line2{getPointsList(line2Str)};

  std::vector<v2> res = intersections(line1, line2);

  int numSteps = 0;

  fmt::print("line 1 size {} line 2 size {}\n", line1.size(), line2.size());

  for (std::size_t i = 0; i < line1.size() - 1; i++) {
    bool foundIntersection = false;
    
    for (auto const& intersection : res) {
      if ((intersection.x >= line1[i].x && intersection.x <= line1[i+1].x)
        || (intersection.x >= line1[i+1].x && intersection.x <= line1[i].x)) {
        fmt::print("intersection {}\n", i);
        numSteps += intersection.x;
        foundIntersection = true;
        break;
      } else if ((intersection.y >= line1[i].y && intersection.y <= line1[i+1].y)
        || (intersection.y >= line1[i+1].y && intersection.y <= line1[i].y)) {
        fmt::print("intersection {}\n", i);
        numSteps += intersection.y;
        foundIntersection = true;
        break;
      }
    }
    
    numSteps += (line1[i].x == line1[i+1].x) ? line1[i].y : line1[i].x;
    fmt::print("i {} numSteps {}\n", i, numSteps);
    if (foundIntersection) {
      break;
    }
  }
  fmt::print("Example 1 steps : expected {} got {}\n", exampleSteps1.second, numSteps);

  int steps = 0;


  fmt::print("Part 2 answer : {}\n", steps);

  return 0;
}
