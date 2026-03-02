$password = "testpass"

# Create vault
$password | ./target/release/nullcrypt.exe create test.vault

# Pack files
$password | ./target/release/nullcrypt.exe pack test.vault test1.txt test2.txt test3.txt

# List contents
Write-Host "`n=== Listing vault contents ===" -ForegroundColor Cyan
$password | ./target/release/nullcrypt.exe list test.vault

# Extract specific files
Write-Host "`n=== Extracting test1.txt and test3.txt ===" -ForegroundColor Cyan
New-Item -ItemType Directory -Path extracted -Force | Out-Null
$password | ./target/release/nullcrypt.exe extract test.vault test1.txt test3.txt --output extracted

# Verify extracted files
Write-Host "`n=== Extracted files ===" -ForegroundColor Cyan
Get-ChildItem extracted

# Test extracting non-existent file
Write-Host "`n=== Extracting with non-existent file ===" -ForegroundColor Cyan
$password | ./target/release/nullcrypt.exe extract test.vault test1.txt nonexistent.txt --output extracted

# Cleanup
Remove-Item test.vault, test1.txt, test2.txt, test3.txt -Force
Remove-Item extracted -Recurse -Force
