$versionLine = Get-Content "./server/Cargo.toml" | Select-String -Pattern '^version\s*='

$version = $versionLine -split '=' | Select-Object -Last 1
$version = $version.Trim().Replace('"', '')

$suffix = Get-Date -Format "yyyy.MM.dd.HH.mm.ss"

$out = $version

if ($env:BRANCH_NAME -eq "main") {
  $out = "$version-prerelease.$suffix"
}

if ($env:GITHUB_OUTPUT -ne "") {
  "tag=server-v$out" >> "$env:GITHUB_OUTPUT"
  "out=v$out" >> "$env:GITHUB_OUTPUT"
  "abs=v$version" >> "$env:GITHUB_OUTPUT"
}
