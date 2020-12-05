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