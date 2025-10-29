Register-PSRepository -Default
Install-Module -Name PSToml -Force -SkipPublisherCheck -AcceptLicense -Scope CurrentUser -ErrorAction Continue

$obj = ConvertFrom-Toml (Get-Content "./server/Cargo.toml")

$version = $obj.package.version

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
