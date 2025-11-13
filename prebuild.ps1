$target = $env:TARGET
$alloc = $env:ALLOC

Remove-Item -Path dist -Recurse -ErrorAction SilentlyContinue
New-Item -Path dist -ItemType Directory -Force

##
## Lib AHQ C
##

# Build libahqc
Set-Location .\ahqrt\libahqc

cargo build --release --target $target

Set-Location ..\..

# Copy Data
Copy-Item -Path .\ahqrt\libahqc\target\$target\release\*ahqc.* -Destination .\dist -Recurse

##
## Get llama cpp tool
##
Set-Location .\getllamacpp

cargo build --release --target $target
Copy-Item -Path .\target\$target\release\*getllamacpp* -Destination .\server -Recurse

Set-Location ..

##
## Server
##
Copy-Item -Path .\ahqrt\libahqc\target\$target\release\*ahqc.* -Destination .\server -Recurse

Set-Location .\server

cargo build --release --target $target --no-default-features --features $alloc

Set-Location ..

## Copy
Copy-Item -Path .\server\target\$target\release\*ahqai-server* -Destination .\dist -Recurse

##
##
## FINAL COPY & COMPRESSION
##
##
Copy-Item -Path .\parts\* -Destination .\dist\ -Recurse

"$env:VERSION" > .\dist\.version

Compress-Archive -Path .\dist\* -DestinationPath .\ahqai-server-$target-$alloc.zip -Update
