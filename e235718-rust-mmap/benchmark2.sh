#!/bin/zsh

rm test
echo "cp start1" >> data/iostat2.log ; cp 3GB_file test
sleep 5
rm test
echo "cp start2" >> data/iostat2.log ; cp 3GB_file test
sleep 5

rm test
echo "mmap_copy start1" >> data/iostat2.log ; cargo run -- mmap_copy 3GB_file test
sleep 5
rm test
echo "mmap_copy start2" >> data/iostat2.log ; cargo run -- mmap_copy 3GB_file test
sleep 5

rm test
echo "mmap_write start1" >> data/iostat2.log ; cargo run -- mmap_write 3GB_file test
sleep 5
rm test
echo "mmap_write start2" >> data/iostat2.log ; cargo run -- mmap_write 3GB_file test
sleep 5

rm test
echo "read_write start1" >> data/iostat2.log ; cargo run -- read_write 3GB_file test
sleep 5
rm test
echo "read_write start2" >> data/iostat2.log ; cargo run -- read_write 3GB_file test
sleep 5