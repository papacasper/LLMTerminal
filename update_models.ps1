# Update app models from fetched model lists

Write-Host "=== Updating App Models ===" -ForegroundColor Green

# Function to get the latest model from a file
function Get-LatestModel {
    param([string]$FilePath, [string]$FilterPattern = "")
    
    if (Test-Path $FilePath) {
        $models = Get-Content $FilePath | Where-Object { $_ -ne "" }
        
        if ($FilterPattern) {
            $filteredModels = $models | Where-Object { $_ -match $FilterPattern }
            if ($filteredModels) {
                return $filteredModels | Select-Object -Last 1
            }
        }
        
        return $models | Select-Object -Last 1
    }
    return $null
}

# Get latest models
$latestClaude = Get-LatestModel -FilePath "./models/claude_models.txt" -FilterPattern "claude-3"
$latestOpenAI = Get-LatestModel -FilePath "./models/openai_models.txt" -FilterPattern "gpt-4"

# Fallback to any model if GPT-4 not found
if (!$latestOpenAI) {
    $latestOpenAI = Get-LatestModel -FilePath "./models/openai_models.txt" -FilterPattern "gpt"
}

Write-Host "Latest Claude model found: $latestClaude" -ForegroundColor Cyan
Write-Host "Latest OpenAI model found: $latestOpenAI" -ForegroundColor Cyan

# Update config.json if it exists
if (Test-Path "./config.json") {
    try {
        $config = Get-Content "./config.json" | ConvertFrom-Json
        
        $updated = $false
        
        if ($latestClaude -and $config.claude_model) {
            $oldClaudeModel = $config.claude_model
            $config.claude_model = $latestClaude
            Write-Host "Updated Claude model: $oldClaudeModel -> $latestClaude" -ForegroundColor Green
            $updated = $true
        }
        
        if ($latestOpenAI -and $config.openai_model) {
            $oldOpenAIModel = $config.openai_model
            $config.openai_model = $latestOpenAI
            Write-Host "Updated OpenAI model: $oldOpenAIModel -> $latestOpenAI" -ForegroundColor Green
            $updated = $true
        }
        
        if ($updated) {
            $config | ConvertTo-Json -Depth 10 | Out-File -FilePath "./config.json" -Encoding UTF8
            Write-Host "Configuration updated successfully!" -ForegroundColor Green
        } else {
            Write-Host "No updates needed or models not found." -ForegroundColor Yellow
        }
    }
    catch {
        Write-Host "Error updating config.json: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "config.json not found. Skipping config update." -ForegroundColor Yellow
}

Write-Host "=== App Update Complete ===" -ForegroundColor Green
