add_executable(day1_tests test_day1.cpp)
target_link_libraries(day1_tests day_01)
add_test(NAME day1 COMMAND day1_tests)

add_executable(day2_tests test_day2.cpp)
target_link_libraries(day2_tests day_02)
add_test(NAME day2 COMMAND day2_tests)

add_executable(day3_tests test_day3.cpp)
target_link_libraries(day3_tests day_03)
add_test(NAME day3 COMMAND day3_tests)

add_executable(day4_tests test_day4.cpp)
target_link_libraries(day4_tests day_04)
add_test(NAME day4 COMMAND day4_tests)

add_executable(day5_tests test_day5.cpp)
target_link_libraries(day5_tests day_05)
add_test(NAME day5 COMMAND day5_tests)

add_executable(day6_tests test_day6.cpp)
target_link_libraries(day6_tests day_06)
add_test(NAME day6 COMMAND day6_tests)

add_executable(day7_tests test_day7.cpp)
target_link_libraries(day7_tests day_07)
add_test(NAME day7 COMMAND day7_tests)

add_executable(day8_tests test_day8.cpp)
target_link_libraries(day8_tests day_08)
add_test(NAME day8 COMMAND day8_tests)

add_executable(gconsole_tests test_gconsole.cpp)
target_link_libraries(gconsole_tests gconsole)
add_test(NAME gconsole COMMAND gconsole_tests)
