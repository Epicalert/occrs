echo "Test: file 1"
target/debug/occrs tests/test-1 | cmp -s - tests/test-1-out || exit 1
echo "Test: pipe 1"
cat tests/test-1 | target/debug/occrs | cmp -s - tests/test-1-out || exit 1
echo "Test: file 2"
target/debug/occrs tests/test-2 | cmp -s - tests/test-2-out || exit 1
echo "Test: pipe 2"
cat tests/test-2 | target/debug/occrs | cmp -s - tests/test-2-out || exit 1
