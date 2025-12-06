$ErrorActionPreference = "Stop"

Write-Host "🚀 STARTING BUILD PROCESS v0.0.1..." -ForegroundColor Cyan

# 1. Build Rust Backend (CLI)
Write-Host "`n📦 Building Rust CLI (macrocli)..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Rust build failed" }

# 2. Build C# Frontend (GUI)
Write-Host "`n🖥️  Building WPF GUI..." -ForegroundColor Yellow
cd gui
dotnet restore
dotnet publish -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -p:EnableCompressionInSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true -o ../release_temp
if ($LASTEXITCODE -ne 0) { throw "C# build failed" }
cd ..

# 3. Assemble Release Folder
Write-Host "`n📂 Assembling Release Package..." -ForegroundColor Yellow
$releaseDir = "release/Macropad_v0.0.1_Win64"
if (Test-Path $releaseDir) { Remove-Item -Recurse -Force $releaseDir }
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

# Copy files
Copy-Item "target/release/macrocli.exe" -Destination "$releaseDir/macrocli.exe"
Copy-Item "release_temp/MacropadGUI.exe" -Destination "$releaseDir/MacropadGUI.exe"

# Cleanup temp
Remove-Item -Recurse -Force "release_temp"

# 4. Create ZIP
Write-Host "`n🤐 Zipping..." -ForegroundColor Yellow
$zipFile = "release/Macropad_v0.0.1_Win64.zip"
if (Test-Path $zipFile) { Remove-Item -Force $zipFile }
Compress-Archive -Path "$releaseDir/*" -DestinationPath $zipFile

Write-Host "`n✅ BUILD SUCCESS!" -ForegroundColor Green
Write-Host "   Folder: $releaseDir"
Write-Host "   Zip:    $zipFile"
