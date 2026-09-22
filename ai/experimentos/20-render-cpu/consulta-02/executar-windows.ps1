param([string]$Model = "renderer-analyst:latest")
$ErrorActionPreference = "Stop"
$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot "../../../..")).Path
Push-Location $projectRoot
try {
    # Compile apenas os executaveis portaveis; capture_execution e Unix.
    & cargo build --locked --bin validate_evidence --bin explain_evidence
    if ($LASTEXITCODE -ne 0) { throw "Falha na compilacao." }
    $config = Get-Content -Raw (Join-Path $PSScriptRoot "config-windows.json") | ConvertFrom-Json
    $config.project_root = $projectRoot
    $config.model = $Model
    $config.request_id = "RENDER_CPU_V2_" + [guid]::NewGuid().ToString("N")
    $runRoot = Join-Path $PSScriptRoot $config.request_id
    New-Item -ItemType Directory -Path $runRoot | Out-Null
    $configPath = Join-Path $runRoot "config.json"
    $utf8 = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($configPath, ($config | ConvertTo-Json -Depth 10), $utf8)
    $targetRoot = "target"
    if ($env:CARGO_TARGET_DIR) { $targetRoot = $env:CARGO_TARGET_DIR }
    $coordinator = Join-Path $targetRoot "debug/explain_evidence.exe"
    & $coordinator $configPath (Join-Path $runRoot "attempt")
    if ($LASTEXITCODE -ne 0) { throw "O ciclo falhou; examine os registros em $runRoot" }
    $zipPath = "$runRoot.zip"
    Compress-Archive -LiteralPath $runRoot -DestinationPath $zipPath
    Write-Host "Envie este arquivo: $zipPath"
    Write-Host "Registros: $runRoot"
    Write-Host "Resposta recebida ainda exige avaliacao pela rubrica."
} finally {
    Pop-Location
}
