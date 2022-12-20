@echo off
echo --------------------------------------------------
echo Building Debug
echo --------------------------------------------------
echo[
call "tools\Setup.bat"
cargo build 
del %OUT_DIR%"\CheatMenuRe3.asi" /Q
del %OUT_DIR%"\CheatMenuRe3.pdb" /Q
echo F|xcopy /s "target\debug\cheat_menu_re3.dll" %OUT_DIR%"\CheatMenuRe3.asi" /K /D /H /Y 
echo F|xcopy /s "target\debug\cheat_menu_re3.pdb" %OUT_DIR%"\CheatMenuRe3.pdb" /K /D /H /Y 

