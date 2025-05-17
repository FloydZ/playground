from MeasureSuiteCommandLine import MS
from python_nano_bench.asm import Asm
from python_builder.compile_commands import CompileCommands

def test_nanobench_simple():
    t = "vpandq  ymm0, ymm0, qword ptr [rip + .LCPI0_1]{1to4}"
    s, i = Asm.parse([t])
    assert s[0] == "vpandq  ymm0, ymm0, qword ptr [rax]{1to4}"
    assert i

def test_compile_commands_runner():
    """ if this fails something fishy is going on """
    c = CompileCommands("compile_commands.json")
    assert c.available()
    assert c.targets()
    assert len(c.targets()) == 1

def test_ms():
    data = """mov rax, [rsi]
    add rax, [rdx]
    mov [rdi], rax
    ret"""
    
    files = [data]
    w = MS(files)
    w.run()


test_nanobench_simple()
test_ms()
