cd c_wrapper
make 
cd ..

cargo b

cd target/debug
valgrind ./ogdf-rs