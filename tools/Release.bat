@echo off
echo --------------------------------------------------
echo Building Release
echo --------------------------------------------------
echo[
call "tools\Setup.bat"
cargo build --release
del %OUT_DIR%"\CheatMenuRe3.asi" /Q
del %OUT_DIR%"\CheatMenuRe3.pdb" /Q
echo F|xcopy /s "target\release\cheat_menu_re3.dll" %OUT_DIR%"\CheatMenuRe3.asi" /F /K /D /H /Y 