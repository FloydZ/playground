#pragma once

#include <climits>

#include <imgui.h>
#include <vector>
#include <chrono>
#include <string>
#include <thread>

struct ScopeTimeData
{
    std::string               name;
    std::string               filename;
    std::chrono::microseconds start;
    std::chrono::microseconds duration;
    int                       depth;
    std::thread::id           thread_id;
};

class FlameGraphDrawer
{
public:
    static void Draw(const std::vector<ScopeTimeData>& scope_times,
                     int                               max_depth,
                     std::chrono::microseconds         global_start,
                     ImVec2                            graph_size = ImVec2(0, 0));
};
