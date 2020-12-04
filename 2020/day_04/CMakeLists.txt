add_library(day_04 INTERFACE)
target_include_directories(day_04 INTERFACE ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries(day_04 INTERFACE fmt::fmt)