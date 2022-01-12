@echo off

if not exist WORKSPACE (
    echo Run from project root
    exit /b 1
)

if exist bazel-dist (
    rd /s /q bazel-dist
)
set BUILDARGS=-k -c opt dist --color=yes
call bazel build %BUILDARGS% || exit /b 1
tar xvf bazel-bin\dist.tar || exit /b 1
