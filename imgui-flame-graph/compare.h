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

        // Generate unique label for InputText using ## to hide label text
        //std::string inputLabel = "##source_line_" + std::to_string(i);
        //ImGui::PushItemWidth(columnWidth - 20);
        //ImGui::InputText(inputLabel.c_str(), &sourceCode[i]);
        //ImGui::PopItemWidth();
    }
    ImGui::EndGroup();

    for (const auto& map : mappings) {
        if (map.sourceLine >= sourceLinePositions.size() || map.assemblyLines.empty())
            continue;

        const int firstLine = map.assemblyLines.front();
        const int lastLine = map.assemblyLines.back();

        const ImU32 color = IM_COL32(255, 200 - (map.sourceLine * 25) % 155, 100, 40);
        const ImU32 edgeColor = color;

        // Draw background blocks for source lines
        {
            const ImVec2 srcPos = sourceLinePositions[map.sourceLine];
            const ImVec2 topLeft = ImVec2(sourceColumnPos.x-3 , srcPos.y - lineHeight * 0.5f);
            const ImVec2 bottomRight = ImVec2(sourceColumnPos.x + columnWidth - 10, srcPos.y + lineHeight * 0.5f);

            drawList->AddRectFilled(topLeft, bottomRight, color, 4.0f);
            drawList->AddRect(topLeft, bottomRight, edgeColor, 4.0f);
        }

        // Draw background blocks for assembly groups
        {
            const ImVec2 topLeft = ImVec2(asmColumnPos.x - 5, asmLinePositions[firstLine].y - lineHeight * 0.5f);
            const ImVec2 bottomRight = ImVec2(asmColumnPos.x + columnWidth - 10, asmLinePositions[lastLine].y + lineHeight * 0.5f);

            drawList->AddRectFilled(topLeft, bottomRight, color, 4.0f);
            drawList->AddRect(topLeft, bottomRight, edgeColor, 4.0f);
        }
        {
            const float srcY = sourceLinePositions[map.sourceLine].y;
            const float srcX = sourceLinePositions[map.sourceLine].x + columnWidth ;
            const ImVec2 srcTopRight = ImVec2(srcX + 5, srcY - lineHeight * 0.5f);
            const ImVec2 srcBotRight = ImVec2(srcX + 5, srcY + lineHeight * 0.5f);

            // Get top/bottom of assembly block
            const ImVec2 asmTopLeft = ImVec2(asmLinePositions[firstLine].x - 5, asmLinePositions[firstLine].y - lineHeight * 0.5f);
            const ImVec2 asmBotLeft = ImVec2(asmLinePositions[lastLine].x - 5, asmLinePositions[lastLine].y + lineHeight * 0.5f);

            //const auto mid1 = ImVec2((srcTopRight.x + asmTopLeft.x) * 0.5f, srcTopRight.y);

            drawList->PathLineTo(srcBotRight);
            drawList->PathBezierQuadraticCurveTo( srcBotRight, asmBotLeft, 2.0);
            //drawList->PathLineTo(q1);
            //drawList->PathBezierCurveTo(c2, c2, q0);
            //drawList->PathFillConvex(col);

            // Top corner Bezier
            //drawList->AddBezierQuadratic(
            //    srcTopRight,
            //    ImVec2((srcTopRight.x + asmTopLeft.x) * 0.5f, srcTopRight.y),
            //    asmTopLeft,
            //    color,
            //    2.0f
            //);

            //// Bottom corner Bezier
            //drawList->AddBezierQuadratic(
            //    srcBotRight,
            //    ImVec2((srcBotRight.x + asmBotLeft.x) * 0.5f, srcBotRight.y),
            //    asmBotLeft,
            //    color,
            //    2.0f
            //);
        }
    }

    ImGui::End(); // End "Code Visualizer"
}
