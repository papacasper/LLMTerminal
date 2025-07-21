# Fetch latest OpenAI and Claude models, update a local app config
param(
    [string]$OpenAIKey = $env:OPENAI_API_KEY,
    [string]$ClaudeKey = $env:ANTHROPIC_API_KEY
)

Write-Host "=== Fetching Latest Models ===" -ForegroundColor Green

# Create models directory if it doesn't exist
if (!(Test-Path "./models")) {
    New-Item -ItemType Directory -Path "./models"
}

# Fetch OpenAI Models
if ($OpenAIKey) {
    Write-Host "Fetching OpenAI models..." -ForegroundColor Yellow
    try {
        $openaiResponse = Invoke-RestMethod -Uri "https://api.openai.com/v1/models" -Headers @{
            "Authorization" = "Bearer $OpenAIKey"
            "Content-Type" = "application/json"
        }
        $openaiModels = $openaiResponse.data | ForEach-Object { $_.id } | Sort-Object
        
        Write-Host "=== Latest OpenAI Models ===" -ForegroundColor Cyan
        $openaiModels | ForEach-Object { Write-Host "  $_" }
        
        # Save to file
        $openaiModels -join "`n" | Out-File -FilePath "./models/openai_models.txt" -Encoding UTF8
        
        # Also save as JSON for programmatic use
        $openaiModels | ConvertTo-Json | Out-File -FilePath "./models/openai_models.json" -Encoding UTF8
        
        Write-Host "OpenAI models saved to ./models/openai_models.txt" -ForegroundColor Green
    }
    catch {
        Write-Host "Error fetching OpenAI models: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "OPENAI_API_KEY not provided. Skipping OpenAI models." -ForegroundColor Yellow
}

# Fetch Claude Models  
if ($ClaudeKey) {
    Write-Host "`nFetching Claude models..." -ForegroundColor Yellow
    try {
        $claudeResponse = Invoke-RestMethod -Uri "https://api.anthropic.com/v1/models" -Headers @{
            "x-api-key" = $ClaudeKey
            "anthropic-version" = "2023-06-01"
            "Content-Type" = "application/json"
        }
        $claudeModels = $claudeResponse.data | ForEach-Object { $_.id } | Sort-Object
        
        Write-Host "=== Latest Claude Models ===" -ForegroundColor Cyan
        $claudeModels | ForEach-Object { Write-Host "  $_" }
        
        # Save to file
        $claudeModels -join "`n" | Out-File -FilePath "./models/claude_models.txt" -Encoding UTF8
        
        # Also save as JSON for programmatic use
        $claudeModels | ConvertTo-Json | Out-File -FilePath "./models/claude_models.json" -Encoding UTF8
        
        Write-Host "Claude models saved to ./models/claude_models.txt" -ForegroundColor Green
    }
    catch {
        Write-Host "Error fetching Claude models: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "ANTHROPIC_API_KEY not provided. Skipping Claude models." -ForegroundColor Yellow
}

# Update app models if script exists
if (Test-Path "./update_models.ps1") {
    Write-Host "`nRunning model update script..." -ForegroundColor Yellow
    & "./update_models.ps1"
} else {
    Write-Host "`nNo update_models.ps1 script found. You can run this manually to update your app config." -ForegroundColor Yellow
}

Write-Host "`n=== Model Fetch Complete ===" -ForegroundColor Green
