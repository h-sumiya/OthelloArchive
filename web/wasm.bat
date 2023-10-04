@echo off
setlocal

cd webai

wasm-pack build --target web --release

xcopy /s /e /i /h "pkg" "..\ai-othello\src\lib\pkg"

cd ..

endlocal