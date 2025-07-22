@echo off
echo Starting LLMTerminal...

REM Check if virtual environment exists
if not exist "venv" (
    echo Virtual environment not found! Please run setup.bat first.
    pause
    exit /b 1
)

REM Check if .env file exists
if not exist ".env" (
    echo .env file not found! Please create it with your API keys.
    pause
    exit /b 1
)

echo Activating virtual environment...
call venv\Scripts\activate.bat

echo Launching LLMTerminal GUI...
python main.py

pause
