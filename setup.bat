@echo off
echo Setting up LLMTerminal virtual environment...

REM Check if Python is installed
python --version >nul 2>&1
if errorlevel 1 (
    echo Python not found! Please install Python 3.8+ first.
    pause
    exit /b 1
)

echo Creating virtual environment...
python -m venv venv

echo Activating virtual environment...
call venv\Scripts\activate.bat

echo Upgrading pip...
python -m pip install --upgrade pip

echo Installing dependencies...
pip install -r requirements.txt

REM Check if .env file exists
if not exist ".env" (
    echo Creating .env file from template...
    copy ".env.example" ".env"
    echo Please edit .env file and add your API keys!
)

echo Setup complete!
echo To activate the virtual environment, run: venv\Scripts\activate.bat
echo To run the application, use: python main.py
pause
