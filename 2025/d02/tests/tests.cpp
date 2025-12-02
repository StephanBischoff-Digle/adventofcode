#include <string>
#include <string_view>
#include <vector>

#include <catch2/catch_test_macros.hpp>

#include "interval.hpp"
#include "parsing.hpp"

SCENARIO("An input line is parsed correctly", "[parsing]") {
    GIVEN("a valid input line") {
        std::string input{ "10-20" };

        WHEN("the line is parsed") {
            auto maybe_interval = advent::parse_line(
                std::string_view{ input });

            THEN("the parse result yields a value") {
                REQUIRE(maybe_interval != std::nullopt);
                advent::Interval ival = *maybe_interval;

                AND_THEN("the resulting interval is correct") {
                    REQUIRE(ival.d_start == 10);
                    REQUIRE(ival.d_end == 20);
                }
            }
        }
    }


    GIVEN("an invalid input line") {
        std::string input{ "10" };

        WHEN("the line is parsed") {
            auto maybe_interval = advent::parse_line(
                std::string_view{ input });

            THEN("the parse result yields a nullopt") {
                REQUIRE_FALSE(maybe_interval != std::nullopt);
            }
        }
    }
}

SCENARIO("Interval can decide on in and out", "[interval]") {
    GIVEN("an Interval of non-zero size") {
        advent::Interval ival{ 10, 20 };

        WHEN("a value outside the value is checked") {
            std::vector<size_t> vals{ 5, 6, 7, 8, 9, 21, 22, 23, 24, 25 };
            for (auto&& v : vals) {
                CAPTURE(v);
                auto result = ival.is_inside(v);

                THEN("the interval marks it outside") {
                    REQUIRE_FALSE(result);
                }
            }
        }

        WHEN("a value inside the value is checked") {
            std::vector<size_t> vals{ 10, 11, 12, 13, 14, 15,
                                      16, 17, 18, 19, 20 };

            for (auto&& v : vals) {
                CAPTURE(v);
                auto result = ival.is_inside(v);

                THEN("the interval marks it outside") {
                    REQUIRE(result);
                }
            }
        }
    }
}
