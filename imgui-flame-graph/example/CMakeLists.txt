add_executable(imgui-flame-graph-example main.cpp )
target_link_libraries(imgui-flame-graph-example PRIVATE ${SDL2_LIBRARIES} ${gl3w_LIBRARIES} ImGui::imgui libflamegraph GL)
target_compile_definitions(imgui-flame-graph-example PRIVATE IMGUI_IMPL_OPENGL_LOADER_GL3W)
target_compile_features(imgui-flame-graph-example PRIVATE cxx_constexpr)

add_executable(demo demo.cpp ../imgui/imgui_demo.cpp)
target_link_libraries(demo PRIVATE ${SDL2_LIBRARIES} ${gl3w_LIBRARIES} ImGui::imgui GL)


add_executable(node node.cpp 
    ../imgui-node-editor/crude_json.cpp
    ../imgui-node-editor/imgui_canvas.cpp
    ../imgui-node-editor/imgui_node_editor.cpp
    ../imgui-node-editor/imgui_node_editor_api.cpp
)
target_link_libraries(node PRIVATE ${SDL2_LIBRARIES} ${gl3w_LIBRARIES} ImGui::imgui GL)


add_executable(compare compare.cpp )
target_link_libraries(compare PRIVATE ${SDL2_LIBRARIES} ${gl3w_LIBRARIES} ImGui::imgui GL)
