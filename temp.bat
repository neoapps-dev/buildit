@echo off
echo [DEBUG] Arguments: %*
echo [BuildIt Builder] You have reached this compiler!
echo [BuildIt Builder] Building BuildIt...
cargo build --release
copy target\release\buildit.exe buildit.exe
echo [Buildit Builder] Done, try `.\buildit.exe`
exit /b

