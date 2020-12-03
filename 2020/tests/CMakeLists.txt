add_executable(day1_tests test_day1.cpp)
target_link_libraries(day1_tests day_01)
add_test(NAME day1 COMMAND day1_tests)

add_executable(day2_tests test_day2.cpp)
target_link_libraries(day2_tests day_02)
add_test(NAME day2 COMMAND day2_tests)