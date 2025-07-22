# PowerShell run script for LLMTerminal

Write-Host "Starting LLMTerminal..." -ForegroundColor Green

# Check if virtual environment exists
if (-not (Test-Path "venv")) {
    Write-Host "Virtual environment not found! Please run setup.ps1 first." -ForegroundColor Red
    exit 1
}

# Check if .env file exists
if (-not (Test-Path ".env")) {
    Write-Host ".env file not found! Please create it with your API keys." -ForegroundColor Red
    exit 1
}

# Activate virtual environment
Write-Host "Activating virtual environment..." -ForegroundColor Yellow
& .\venv\Scripts\Activate.ps1

# Run the application
Write-Host "Launching LLMTerminal GUI..." -ForegroundColor Green
python main.py
