@echo off
setlocal
set "name=%~1"
set "gitUrl=%~2"
cd %~dp0
git remote add  %name% %gitUrl%
git fetch %name%
git read-tree --prefix=%name%/ %name%/main
git checkout -- .
git add .
git commit -m "add %name%"
git merge -s subtree %name%/main --allow-unrelated-histories
endlocal