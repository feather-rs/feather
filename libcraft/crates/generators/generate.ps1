$generators = Get-ChildItem "python" -Filter *.py

Write-Host "Running python generators"
foreach ($generator in $generators) {
    python python/$generator
}

Write-Host "Running rust generators"
cargo run --package libcraft-generators --bin libcraft-generators

cargo fmt