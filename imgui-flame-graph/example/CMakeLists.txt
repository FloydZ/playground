#add_executable(imgui-flame-graph-example main.cpp ${PUBLIC_CXX_FILES})
add_executable(imgui-flame-graph-example main.cpp )

target_link_libraries(imgui-flame-graph-example PRIVATE ${SDL2_LIBRARIES} ${gl3w_LIBRARIES} ImGui::imgui libflamegraph GL)
target_compile_definitions(imgui-flame-graph-example PRIVATE IMGUI_IMPL_OPENGL_LOADER_GL3W)
target_compile_features(imgui-flame-graph-example PRIVATE cxx_constexpr)
