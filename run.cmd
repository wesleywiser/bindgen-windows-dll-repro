cd cpp

cl /c example.c /Fo

link /dll example.obj /def:example.def

cd ..

cargo build
if %errorlevel% neq 0 exit /b %errorlevel%

copy .\cpp\example.dll .\target\debug\example.dll

cargo run
