//
// Created by Derek Witt on 12/2/20.
//

#include "day_01.hpp"
#include <array>

#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MAIN  // in only one cpp file
#include <boost/test/unit_test.hpp>

BOOST_AUTO_TEST_CASE( day1_1 )
{
    std::array<int, 6> in = {1721,
            979,
            366,
            299,
            675,
            1456,};

    BOOST_TEST( day_01_1(in) == 514579);
}
BOOST_AUTO_TEST_CASE( day1_2 )
{
    std::array<int, 6> in = {1721,
                             979,
                             366,
                             299,
                             675,
                             1456,};

    BOOST_TEST( day_01_2(in) == 241861950);
}

