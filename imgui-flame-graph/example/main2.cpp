#include "helper.h"

#include <string>
#include <vector>
#include <sstream>
#include <unordered_map>

#include "../TextEditor.h"


struct AssemblyLine {
    std::string assembly;
    uint64_t addr;
};


// Split a string into lines
std::vector<std::string> SplitLines(const std::string& text) {
    std::istringstream stream(text);
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(stream, line)) {
        lines.push_back(line);
    }
    return lines;
}

std::vector<AssemblyLine> GetMockDisassembly(int lineNumber) {
    // Simulate a few different instructions
    switch (lineNumber % 3) {
        case 0:
            return {};
        case 1:
            return {
                    { "push ebp", 0x00000008 },
                    { "mov ebp, esp", 0x0000000C }
            };
        default:
            return {
                    { "call 0x80483f0 <add>", 0x00000010 },
                    { "pop ebp", 0x00000015 }
            };
    }
}

// Renders a C code editor with optional inline assembly (line-by-line)
void ShowCCodeEditorLineByLine() {
    static TextEditor editor;
    static bool initialized = false;
    if (!initialized) {
        editor.SetLanguageDefinition(TextEditor::LanguageDefinition::C());
        editor.SetText(R"(int add(int a, int b) {
    return a + b;
}

int main() {
    int result = add(5, 3);
    return result;
})");
        initialized = true;
    }

    static std::string cCode = R"(int add(int a, int b) {
    return a + b;
}

int main() {
    int result = add(5, 3);
    return result;
})";

    static bool showDisasm = true;
    ImGui::Begin("C Code Editor with Inline Assembly");
    editor.Render("##CCodeEditor", ImVec2(600, 300));  // adjust size as needed

    editor.SetLineOverlayCallback([](int lineNumber, const ImVec2& pos) {
        auto asmLines = GetMockDisassembly(lineNumber);
        if (asmLines.empty()) return;

        static std::unordered_map<int, bool> expanded;
        if (expanded.find(lineNumber) == expanded.end())
            expanded[lineNumber] = true;

        ImGui::SetCursorScreenPos(pos);
        ImGui::Indent(40.0f); // match indent

        ImGui::PushID(lineNumber);
        if (ImGui::SmallButton(expanded[lineNumber] ? "▾" : "▸")) {
            expanded[lineNumber] = !expanded[lineNumber];
        }

        if (expanded[lineNumber]) {
            int i = 0;
            for (const auto& asmLine : asmLines) {
                ImGui::PushID(i);
                ImGui::TextColored(ImVec4(1.0f, 0.85f, 0.6f, 1.0f), "0x%08llx:", asmLine.addr);
                //if (ImGui::IsItemHovered()) {
                //    ImGui::BeginTooltip();
                //    ImGui::Text("Instruction address: 0x%08llx", asmLine.addr);
                //    ImGui::EndTooltip();
                //}

                ImGui::SameLine();

                // Instruction
                ImGui::TextColored(ImVec4(0.6f, 1.0f, 0.6f, 1.0f), "%s", asmLine.assembly.c_str());
                //if (ImGui::IsItemHovered()) {
                //    ImGui::BeginTooltip();
                //    ImGui::Text("Assembly: %s", asmLine.assembly.c_str());
                //    ImGui::EndTooltip();
                //}
                ImGui::PopID();
                i+=1;
            }
        }
        ImGui::Dummy(ImVec2(0, asmLines.size() * ImGui::GetTextLineHeightWithSpacing()));

        ImGui::PopID();
        ImGui::Unindent(40.0f);
    });

    //auto lines = SplitLines(cCode);
    //static std::unordered_map<int, bool> lineExpanded;  // Toggle state per line

    //for (int i = 0; i < lines.size(); ++i) {
    //    ImGui::PushID(i);

    //    auto asmLines = GetMockDisassembly(i);
    //    bool hasAsm = !asmLines.empty();

    //    // Initialize toggle state to true (expanded) if it hasn't been set yet
    //    if (lineExpanded.find(i) == lineExpanded.end())
    //        lineExpanded[i] = true;

    //    // Only show toggle if there's assembly for the line
    //    if (hasAsm) {
    //        if (ImGui::SmallButton(lineExpanded[i] ? "▾" : "▸")) {
    //            lineExpanded[i] = !lineExpanded[i];
    //        }
    //        ImGui::SameLine();
    //    } else {
    //        ImGui::Dummy(ImVec2(ImGui::CalcTextSize("▸").x + ImGui::GetStyle().ItemSpacing.x, 0));
    //        ImGui::SameLine();
    //    }

    //    ImGui::TextColored(ImVec4(0.8f, 0.9f, 1.0f, 1.0f), "%4d ", i + 1);
    //    ImGui::SameLine();
    //    ImGui::TextUnformatted(lines[i].c_str());

    //    // Show assembly if enabled and exists
    //    if (lineExpanded[i] && hasAsm) {
    //        for (const auto& asmLine : asmLines) {
    //            ImGui::Indent(40.0f);

    //            // Create a new ImGui group for the address + instruction line
    //            ImGui::BeginGroup();

    //            // Address
    //            ImGui::TextColored(ImVec4(1.0f, 0.85f, 0.6f, 1.0f), "0x%08llx:", asmLine.addr);
    //            if (ImGui::IsItemHovered()) {
    //                ImGui::BeginTooltip();
    //                ImGui::Text("Instruction address: 0x%08llx", asmLine.addr);
    //                ImGui::EndTooltip();
    //            }

    //            ImGui::SameLine();

    //            // Instruction
    //            ImGui::TextColored(ImVec4(0.6f, 1.0f, 0.6f, 1.0f), "%s", asmLine.assembly.c_str());
    //            if (ImGui::IsItemHovered()) {
    //                ImGui::BeginTooltip();
    //                ImGui::Text("Assembly: %s", asmLine.assembly.c_str());
    //                ImGui::EndTooltip();
    //            }

    //            ImGui::EndGroup();
    //            ImGui::Unindent(40.0f);
    //        }
    //    }

    //    ImGui::PopID();
    //}

    ImGui::End();
}


int main() {
    auto window = init();
    ImGuiIO& io = ImGui::GetIO();

    // Main loop
    bool done = false;
    while (!done) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
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
            ShowCCodeEditorLineByLine();
        ImGui::Render();
        glViewport(0, 0, (int)io.DisplaySize.x, (int)io.DisplaySize.y);
        //glClearColor(clear_color.x * clear_color.w, clear_color.y * clear_color.w, clear_color.z * clear_color.w, clear_color.w);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        SDL_GL_SwapWindow(window);
    }

    deinit(window);
    return 0;
}
