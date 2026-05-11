@echo off
setlocal EnableExtensions

cd /d "%~dp0"

set CARGO_NAME=lyna
set OUTPUT_DIR=artifacts
set ADDINS_DIR=..\os\addins
set TEMPLATES_BASE=..\bsl\src\ExternalDataProcessors\Lyna\Templates
set VCPKG_ROOT=R:\Repos\vcpkg

call :BuildPackage Lua54
if errorlevel 1 goto :error

call :BuildPackage LuaJIT luajit
if errorlevel 1 goto :error

@echo Build and packaging completed successfully.
exit /b 0

:error
@echo An error occurred during the build or packaging process.
exit /b 1

:BuildPackage
set "VARIANT_NAME=%~1"
set "BACKEND_FEATURE=%~2"
if defined BACKEND_FEATURE (
  set "CARGO_EXTRA=--no-default-features --features %BACKEND_FEATURE%"
  :: LuaJIT + zig/lld: unwind-символы для линковки luajit (см. zig/issues/14089)
  set "WSL_LUAJIT_LDFLAGS=TARGET_LDFLAGS=-lunwind"
) else (
  set "CARGO_EXTRA="
  set "WSL_LUAJIT_LDFLAGS="
)

if exist "%OUTPUT_DIR%" rmdir /S /Q "%OUTPUT_DIR%"
mkdir "%OUTPUT_DIR%"

:: --- x86_64-pc-windows-msvc ---
set OPENSSL_DIR=%VCPKG_ROOT%\installed\x64-windows
set OPENSSL_LIB_DIR=%VCPKG_ROOT%\installed\x64-windows\lib
set OPENSSL_INCLUDE_DIR=%VCPKG_ROOT%\installed\x64-windows\include

cargo build --release --target x86_64-pc-windows-msvc %CARGO_EXTRA%
if errorlevel 1 exit /b 1

:: --- x86_64-unknown-linux-gnu ---
wsl -d OracleLinux_9_1 env LIBRARY_PATH=/usr/lib64 OPENSSL_DIR=/usr %WSL_LUAJIT_LDFLAGS% cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17 %CARGO_EXTRA%
if errorlevel 1 exit /b 1

:: --- i686-pc-windows-msvc ---
set OPENSSL_DIR=%VCPKG_ROOT%\installed\x86-windows
set OPENSSL_LIB_DIR=%VCPKG_ROOT%\installed\x86-windows\lib
set OPENSSL_INCLUDE_DIR=%VCPKG_ROOT%\installed\x86-windows\include

cargo build --release --target i686-pc-windows-msvc %CARGO_EXTRA%
if errorlevel 1 exit /b 1

:: --- i686-unknown-linux-gnu ---
wsl -d OracleLinux_9_1 env LIBRARY_PATH=/usr/lib OPENSSL_DIR=/usr OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include %WSL_LUAJIT_LDFLAGS% cargo zigbuild --release --target i686-unknown-linux-gnu.2.17 %CARGO_EXTRA%
if errorlevel 1 exit /b 1

copy /y target\x86_64-pc-windows-msvc\release\%CARGO_NAME%.dll "%OUTPUT_DIR%\AddIn_x64_windows.dll"
if errorlevel 1 exit /b 1

copy /y target\i686-pc-windows-msvc\release\%CARGO_NAME%.dll "%OUTPUT_DIR%\AddIn_x86_windows.dll"
if errorlevel 1 exit /b 1

copy /y target\x86_64-unknown-linux-gnu\release\lib%CARGO_NAME%.so "%OUTPUT_DIR%\AddIn_x64_linux.so"
if errorlevel 1 exit /b 1

copy /y target\i686-unknown-linux-gnu\release\lib%CARGO_NAME%.so "%OUTPUT_DIR%\AddIn_x86_linux.so"
if errorlevel 1 exit /b 1

copy /y MANIFEST.XML "%OUTPUT_DIR%\MANIFEST.XML"
if errorlevel 1 exit /b 1

powershell -NoProfile -Command "Compress-Archive -Path '%OUTPUT_DIR%\*' -Force -DestinationPath '%VARIANT_NAME%.zip'"
if errorlevel 1 exit /b 1

if not exist "%ADDINS_DIR%" mkdir "%ADDINS_DIR%"
copy /y "%VARIANT_NAME%.zip" "%ADDINS_DIR%\%VARIANT_NAME%.zip"
if errorlevel 1 exit /b 1

set "TMPL=%TEMPLATES_BASE%\%VARIANT_NAME%"
if not exist "%TMPL%" mkdir "%TMPL%"
copy /y "%VARIANT_NAME%.zip" "%TMPL%\Template.addin"
if errorlevel 1 exit /b 1

del "%VARIANT_NAME%.zip"
if exist "%OUTPUT_DIR%" rmdir /S /Q "%OUTPUT_DIR%"

exit /b 0
