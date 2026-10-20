# Requires Windows PowerShell 5.1.
#
# O renderer e consumidor: fornece somente o perfil (playme.json) com fontes,
# lexico, Modelfile, modelo, instrucoes e criterios. A execucao pertence ao
# PLAYME generico do Librarian (tools/playme/PLAYME.ps1).

[CmdletBinding()]
param(
    [string]$Question,
    [switch]$PrepareOnly,
    [string]$OutputDirectory,
    [string]$LibrarianPath
)

$ErrorActionPreference = 'Stop'

if (-not $LibrarianPath) {
    $LibrarianPath = Join-Path $PSScriptRoot '..\librarian'
}
$Script = Join-Path (Resolve-Path -LiteralPath $LibrarianPath).Path 'tools\playme\PLAYME.ps1'

$Forward = @{ ProfilePath = (Join-Path $PSScriptRoot 'playme.json') }
if ($Question) { $Forward.Question = $Question }
if ($PrepareOnly) { $Forward.PrepareOnly = $true }
if ($OutputDirectory) { $Forward.OutputDirectory = $OutputDirectory }

& $Script @Forward
exit $LASTEXITCODE
