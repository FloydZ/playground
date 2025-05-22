#include "helper.h"
#include "../compare.h"


int main() {
    auto window = init();
    ImGuiIO& io = ImGui::GetIO();
    ImVec4 clear_color = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);

    std::vector<std::string> source = {
        "int sum = 0;",
        "for (int i = 0; i < 10; i++)",
        "    sum += i;"
    };

    std::vector<std::string> assembly = {
        "mov eax, 0",
        "mov ecx, 0",
        "cmp ecx, 10",
        "jl .loop",
        "add eax, ecx",
        "inc ecx",
        "jmp .loop"
    };

    std::vector<CodeMapEntry> map = {
        {0, {0}},
        {1, {1, 2, 3}},
        {2, {5}}
    };


    // high dpi stuff
    const float imgui_additional_scale = 4;
    ImGui::GetStyle().ScaleAllSizes(imgui_additional_scale);

    bool done = false;
    while (!done) {
        SDL_Event event;
        while (SDL_PollEvent(&event))
        {
            ImGui_ImplSDL2_ProcessEvent(&event);
            if (event.type == SDL_QUIT)
                done = true;
            if (event.type == SDL_WINDOWEVENT && event.window.event == SDL_WINDOWEVENT_CLOSE &&
                event.window.windowID == SDL_GetWindowID(window))
                done = true;
        }

        // Start the Dear ImGui frame
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplSDL2_NewFrame();
        ImGui::NewFrame();

        DrawSourceToAssemblyVisualizer(source, assembly, map);

        // Rendering
        ImGui::Render();
        glViewport(0, 0, (int)io.DisplaySize.x, (int)io.DisplaySize.y);
        glClearColor(
            clear_color.x * clear_color.w, clear_color.y * clear_color.w, clear_color.z * clear_color.w, clear_color.w);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        SDL_GL_SwapWindow(window);
    }

    deinit(window);
    return 0;
}
