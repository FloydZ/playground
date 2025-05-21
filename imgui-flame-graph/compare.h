#include "imgui.h"


#include <vector>
#include <algorithm>
#include <string>

// Mapping from source lines to multiple assembly lines
struct CodeMapEntry {
    int sourceLine;
    std::vector<int> assemblyLines;
};

// Call this inside your ImGui frame
static
void DrawSourceToAssemblyVisualizer(
    const std::vector<std::string>& sourceCode,
    const std::vector<std::string>& assemblyCode,
    const std::vector<CodeMapEntry>& mappings)
{
    ImGui::Begin("Code Visualizer");

    const float gap = 150;
    const float contentWidth = ImGui::GetContentRegionAvail().x;
    const float columnWidth = (contentWidth - gap) / 2.0f;
    float lineHeight = ImGui::GetTextLineHeightWithSpacing();

    ImDrawList* drawList = ImGui::GetWindowDrawList();
    const ImVec2 startCursor = ImGui::GetCursorScreenPos();

    std::vector<ImVec2> sourceLinePositions(sourceCode.size());
    std::vector<ImVec2> asmLinePositions(assemblyCode.size());

    //
    const ImVec2 sourceColumnPos = startCursor;
    ImGui::SetCursorScreenPos(sourceColumnPos);
    ImGui::BeginGroup();
    for (int i = 0; i < sourceCode.size(); ++i) {
        const ImVec2 textPos = ImGui::GetCursorScreenPos();
        sourceLinePositions[i] = ImVec2(textPos.x, textPos.y + lineHeight * 0.5f);
        ImGui::Text("%s", sourceCode[i].c_str());
    }
    ImGui::EndGroup();


    // Right side: Assembly code
    const ImVec2 asmColumnPos = ImVec2(sourceColumnPos.x + columnWidth + gap, sourceColumnPos.y);
    ImGui::SetCursorScreenPos(asmColumnPos);
    ImGui::BeginGroup();
    for (int i = 0; i < assemblyCode.size(); ++i) {
        ImVec2 textPos = ImGui::GetCursorScreenPos();
        asmLinePositions[i] = ImVec2(textPos.x, textPos.y + lineHeight * 0.5f);
        ImGui::Text("%s", assemblyCode[i].c_str());
    }
    ImGui::EndGroup();

    // Draw background blocks for source lines
    for (const auto& map : mappings) {
        if (map.sourceLine >= sourceLinePositions.size() || map.assemblyLines.empty())
            continue;

        ImVec2 srcPos = sourceLinePositions[map.sourceLine];
        ImVec2 topLeft = ImVec2(sourceColumnPos.x-3 , srcPos.y - lineHeight * 0.5f);
        ImVec2 bottomRight = ImVec2(sourceColumnPos.x + columnWidth - 10, srcPos.y + lineHeight * 0.5f);

        ImU32 color = IM_COL32(255, 200 - (map.sourceLine * 25) % 155, 100, 40);
        drawList->AddRectFilled(topLeft, bottomRight, color, 4.0f);
        drawList->AddRect(topLeft, bottomRight, IM_COL32(255, 180, 100, 100), 4.0f);
    }

    // Draw background blocks for assembly groups
    for (const auto& map : mappings) {
        if (map.assemblyLines.empty()) continue;

        const std::vector<int> lines = map.assemblyLines;
        const int firstLine = lines.front();
        const int lastLine = lines.back();

        if (firstLine >= asmLinePositions.size() || lastLine >= asmLinePositions.size())
            continue;

        ImVec2 topLeft = ImVec2(asmColumnPos.x - 5, asmLinePositions[firstLine].y - lineHeight * 0.5f);
        ImVec2 bottomRight = ImVec2(asmColumnPos.x + columnWidth - 10, asmLinePositions[lastLine].y + lineHeight * 0.5f);

        ImU32 color = IM_COL32(100, 100 + (map.sourceLine * 40) % 155, 255, 40);
        drawList->AddRectFilled(topLeft, bottomRight, color, 4.0f);
        drawList->AddRect(topLeft, bottomRight, IM_COL32(100, 100, 255, 100), 4.0f);
    }

    // Draw bounding-box Bezier curves from source to assembly blocks
    for (const auto& map : mappings) {
        if (map.sourceLine >= sourceLinePositions.size() || map.assemblyLines.empty())
            continue;

        // Sort and get bounds of assembly lines
        std::vector<int> asmLines = map.assemblyLines;
        std::sort(asmLines.begin(), asmLines.end());

        int asmFirst = asmLines.front();
        int asmLast  = asmLines.back();

        if (asmFirst >= asmLinePositions.size() || asmLast >= asmLinePositions.size())
            continue;

        // Get top/bottom of source line block
        const float srcY = sourceLinePositions[map.sourceLine].y;
        const float srcX = sourceLinePositions[map.sourceLine].x + columnWidth ;
        ImVec2 srcTopRight = ImVec2(srcX + 5, srcY - lineHeight * 0.5f);
        ImVec2 srcBotRight = ImVec2(srcX + 5, srcY + lineHeight * 0.5f);

        // Get top/bottom of assembly block
        ImVec2 asmTopLeft = ImVec2(asmLinePositions[asmFirst].x - 5, asmLinePositions[asmFirst].y - lineHeight * 0.5f);
        ImVec2 asmBotLeft = ImVec2(asmLinePositions[asmLast].x - 5, asmLinePositions[asmLast].y + lineHeight * 0.5f);

        ImU32 arrowColor = IM_COL32(255, 100, 0, 255);

        // Top corner Bezier
        drawList->AddBezierQuadratic(
            srcTopRight,
            ImVec2((srcTopRight.x + asmTopLeft.x) * 0.5f, srcTopRight.y),
            asmTopLeft,
            arrowColor,
            2.0f
        );

        // Bottom corner Bezier
        drawList->AddBezierQuadratic(
            srcBotRight,
            ImVec2((srcBotRight.x + asmBotLeft.x) * 0.5f, srcBotRight.y),
            asmBotLeft,
            arrowColor,
            2.0f
        );
    }

    ImGui::End(); // End "Code Visualizer"
}
