// Dear ImGui: standalone example application for SDL2 + OpenGL
// (SDL is a cross-platform general purpose library for handling windows, inputs, OpenGL/Vulkan/Metal graphics context
// creation, etc.) If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

#include "imgui.h"
#include "imgui_impl_opengl3.h"
#include "imgui_impl_sdl2.h"
#include <stdio.h>
#define SDL_MAIN_HANDLED
#include <SDL.h>
#if defined(IMGUI_IMPL_OPENGL_ES2)
#    include <SDL_opengles2.h>
#else
#    include <SDL_opengl.h>
#endif

#include "../imgui_widget_flamegraph.h"
#include <random>

class TimerSingleton
{
public:
    TimerSingleton(const TimerSingleton&) = delete;
    TimerSingleton(TimerSingleton&&)      = delete;

    static TimerSingleton& Get()
    {
        static TimerSingleton instance;
        return instance;
    }

    void Push()
    {
        m_curr_depth++;
        if (m_curr_depth > m_max_depth)
            m_max_depth = m_curr_depth;
    }

    void Pop() { m_curr_depth--; }

    int GetCurrDepth() const { return m_curr_depth; }

    int GetMaxDepth() const { return m_max_depth; }

    std::chrono::microseconds GetGlobalStart() { return m_global_start; };

    void Upload(ScopeTimeData scope_time)
    {
        if (m_scope_times.size() == 0)
            m_global_start = scope_time.start;
        else
            m_global_start = m_global_start.count() > scope_time.start.count() ? scope_time.start : m_global_start;

        m_scope_times.push_back(scope_time);
    }

    const std::vector<ScopeTimeData>& GetScopeTimes() { return m_scope_times; }

    void Clear()
    {
        m_curr_depth = -1;
        m_max_depth  = -1;
        m_scope_times.clear();
    }

private:
    TimerSingleton() {}

    int m_curr_depth = -1;
    int m_max_depth  = -1;

    std::chrono::microseconds m_global_start;

    std::vector<ScopeTimeData> m_scope_times;
};

class Timer
{
public:
    Timer(const std::string& name, const std::string& filename)
        : m_name(name)
        , m_filename(filename)
        , m_stopped(false)
    {
        m_stopped = false;

        size_t pos = m_name.find(cdel_str);
        if (pos != std::string::npos)
        {
            m_name.erase(pos, cdel_str.length());
        }
        m_start_timepoint = std::chrono::steady_clock::now();

        TimerSingleton::Get().Push();
    }

    ~Timer()
    {
        if (!m_stopped)
            Stop();
    }

    void Stop()
    {
        auto end_timepoint = std::chrono::steady_clock::now();
        auto high_res_start =
            std::chrono::time_point_cast<std::chrono::microseconds>(m_start_timepoint).time_since_epoch();
        auto elapsed_time =
            std::chrono::time_point_cast<std::chrono::microseconds>(end_timepoint).time_since_epoch() -
            std::chrono::time_point_cast<std::chrono::microseconds>(m_start_timepoint).time_since_epoch();

        TimerSingleton::Get().Upload({m_name,
                                      m_filename,
                                      high_res_start,
                                      elapsed_time,
                                      TimerSingleton::Get().GetCurrDepth(),
                                      std::this_thread::get_id()});

        m_stopped = true;

        TimerSingleton::Get().Pop();
    }

private:
    std::string                                        m_name;
    std::string                                        m_filename;
    std::chrono::time_point<std::chrono::steady_clock> m_start_timepoint;
    bool                                               m_stopped;

    inline static const std::string cdel_str = "__cdecl ";
};

#if defined(__GNUC__) || (defined(__MWERKS__) && (__MWERKS__ >= 0x3000)) || (defined(__ICC) && (__ICC >= 600)) || \
    defined(__ghs__)
#    define FUNC_SIG __PRETTY_FUNCTION__
#elif defined(__DMC__) && (__DMC__ >= 0x810)
#    define FUNC_SIG __PRETTY_FUNCTION__
#elif (defined(__FUNCSIG__) || (_MSC_VER))
#    define FUNC_SIG __FUNCSIG__
#elif (defined(__INTEL_COMPILER) && (__INTEL_COMPILER >= 600)) || (defined(__IBMCPP__) && (__IBMCPP__ >= 500))
#    define FUNC_SIG __FUNCTION__
#elif defined(__BORLANDC__) && (__BORLANDC__ >= 0x550)
#    define FUNC_SIG __FUNC__
#elif defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 199901)
#    define FUNC_SIG __func__
#elif defined(__cplusplus) && (__cplusplus >= 201103)
#    define FUNC_SIG __func__
#else
#    define FUNC_SIG "FUNC_SIG unknown!"
#endif

#define SCOPE_TIMER_LINE(name, filename, line)      Timer timer##line(name, filename)
#define SCOPE_TIMER_PASTELINE(name, filename, line) SCOPE_TIMER_LINE(name, filename, line)
#define SCOPE_TIMER(name)                           SCOPE_TIMER_PASTELINE(name, __FILE__, __LINE__)
#define FUNCTION_TIMER()                            SCOPE_TIMER(FUNC_SIG)

void TestFun()
{
    FUNCTION_TIMER();

    std::random_device              rd;        // Will be used to obtain a seed for the random number engine
    std::mt19937                    gen(rd()); // Standard mersenne_twister_engine seeded with rd()
    std::uniform_int_distribution<> dis(1, 3000);

    {
        SCOPE_TIMER("Test Scope 1");

        int count = dis(gen);
        for (int i = 0; i < count; i++)
        {
            int x = dis(gen);
        }

        {
            SCOPE_TIMER("Test Scope 2");

            int count2 = dis(gen);
            for (int i = 0; i < count2; i++)
            {
                int x = dis(gen);
            }
        }

        {
            SCOPE_TIMER("Test Scope 3");

            int count3 = dis(gen);
            for (int i = 0; i < count3; i++)
            {
                int x = dis(gen);
            }
        }
    }

    {
        SCOPE_TIMER("Test Scope 4");

        int count = dis(gen);
        for (int i = 0; i < count; i++)
        {
            int x = dis(gen);
        }
    }
}

// Main code
int main(int, char**)
{
    // Setup SDL
    // (Some versions of SDL before <2.0.10 appears to have performance/stalling issues on a minority of Windows
    // systems, depending on whether SDL_INIT_GAMECONTROLLER is enabled or disabled.. updating to latest version of SDL
    // is recommended!)
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER | SDL_INIT_GAMECONTROLLER) != 0)
    {
        printf("Error: %s\n", SDL_GetError());
        return -1;
    }

    // Decide GL+GLSL versions
#if defined(IMGUI_IMPL_OPENGL_ES2)
    // GL ES 2.0 + GLSL 100
    const char* glsl_version = "#version 100";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
#elif defined(__APPLE__)
    // GL 3.2 Core + GLSL 150
    const char* glsl_version = "#version 150";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG); // Always required on Mac
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);
#else
    // GL 3.0 + GLSL 130
    const char* glsl_version = "#version 130";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
#endif

    // Create window with graphics context
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
    SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);
    SDL_WindowFlags window_flags =
        (SDL_WindowFlags)(SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE | SDL_WINDOW_ALLOW_HIGHDPI);
    SDL_Window* window = SDL_CreateWindow(
        "Dear ImGui SDL2+OpenGL3 example", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 1280, 720, window_flags);
    SDL_GLContext gl_context = SDL_GL_CreateContext(window);
    SDL_GL_MakeCurrent(window, gl_context);
    SDL_GL_SetSwapInterval(1); // Enable vsync

    // Setup Dear ImGui context
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    (void)io;
    // io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls
    // io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls

    // Setup Dear ImGui style
    ImGui::StyleColorsDark();
    // ImGui::StyleColorsClassic();

    // Setup Platform/Renderer backends
    ImGui_ImplSDL2_InitForOpenGL(window, gl_context);
    ImGui_ImplOpenGL3_Init(glsl_version);

    // Load Fonts
    // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use
    // ImGui::PushFont()/PopFont() to select them.
    // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
    // - If the file cannot be loaded, the function will return NULL. Please handle those errors in your application
    // (e.g. use an assertion, or display an error and quit).
    // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling
    // ImFontAtlas::Build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
    // - Read 'docs/FONTS.md' for more instructions and details.
    // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double
    // backslash \\ !
    // io.Fonts->AddFontDefault();
    // io.Fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0f);
    // io.Fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0f);
    // io.Fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0f);
    // io.Fonts->AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0f);
    // ImFont* font = io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\ArialUni.ttf", 18.0f, NULL,
    // io.Fonts->GetGlyphRangesJapanese()); IM_ASSERT(font != NULL);

    // Our state
    bool   show_demo_window     = true;
    bool   show_another_window  = false;
    bool   show_profiler_window = true;
    ImVec4 clear_color          = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);


    std::vector<ScopeTimeData> scopeTime = TimerSingleton::Get().GetScopeTimes();
    int maxDepth = TimerSingleton::Get().GetMaxDepth();
    std::chrono::microseconds globalStart = TimerSingleton::Get().GetGlobalStart();

    // Main loop
    bool done = false;
    bool update = true;
    while (!done)
    {
        // Poll and handle events (inputs, window resize, etc.)
        // You can read the io.WantCaptureMouse, io.WantCaptureKeyboard flags to tell if dear imgui wants to use your
        // inputs.
        // - When io.WantCaptureMouse is true, do not dispatch mouse input data to your main application.
        // - When io.WantCaptureKeyboard is true, do not dispatch keyboard input data to your main application.
        // Generally you may always pass all inputs to dear imgui, and hide them from your application based on those
        // two flags.
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

        TestFun();

         {
            ImGui::Begin("Hello, world!");
            ImGui::Checkbox("Updae", &update);
            ImGui::End();
        }
        //FlameGraphDrawer::Draw(TimerSingleton::Get().GetScopeTimes(),
        //                       TimerSingleton::Get().GetMaxDepth(),
        //                       TimerSingleton::Get().GetGlobalStart());

        if (update) {
            scopeTime = TimerSingleton::Get().GetScopeTimes();
            maxDepth = TimerSingleton::Get().GetMaxDepth();
            globalStart = TimerSingleton::Get().GetGlobalStart();
        }

        ImGui::Begin("Profiler");
        FlameGraphDrawer::Draw(scopeTime, maxDepth, globalStart);
        ImGui::End();

        // Rendering
        ImGui::Render();
        glViewport(0, 0, (int)io.DisplaySize.x, (int)io.DisplaySize.y);
        glClearColor(
            clear_color.x * clear_color.w, clear_color.y * clear_color.w, clear_color.z * clear_color.w, clear_color.w);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        SDL_GL_SwapWindow(window);

        TimerSingleton::Get().Clear();
    }

    // Cleanup
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplSDL2_Shutdown();
    ImGui::DestroyContext();

    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
