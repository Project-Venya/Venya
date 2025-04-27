@echo off

set PYTHON_VERSION=3.11.7
set OS_NAME=windows
set DEST_DIR=resources\%OS_NAME%

if exist %DEST_DIR% (
    echo Python already downloaded for %OS_NAME%
    exit /b 0
)

mkdir %DEST_DIR%

echo Downloading Python for %OS_NAME%...
curl -L -o %DEST_DIR%\python.zip https://github.com/indygreg/python-build-standalone/releases/download/20240107/cpython-%PYTHON_VERSION%+20240107-%OS_NAME%-amd64-install_only.zip

powershell -Command "Expand-Archive -Path '%DEST_DIR%\python.zip' -DestinationPath '%DEST_DIR%'"
del %DEST_DIR%\python.zip

:: Install packages from requirements.txt
echo Installing packages from requirements.txt via pip...
%DEST_DIR%\python3 -m pip install -r ./requirements.txt

:: Download hifigan checkpoints
echo Downloading hifigan checkpoints...
mkdir %DEST_DIR%\hifigan
mkdir %DEST_DIR%\hifigan\checkpoints
curl -L -o %DEST_DIR%\hifigan\checkpoints\generator_v1.pt https://github.com/jik876/hifi-gan/releases/download/v0.1/generator_v1
curl -L -o %DEST_DIR%\hifigan\checkpoints\config.json https://raw.githubusercontent.com/jik876/hifi-gan/master/config_v1.json

echo Python %PYTHON_VERSION% and packages from requirements.txt setup complete in %DEST_DIR%
