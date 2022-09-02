cd c_wrapper
make 
cd ..

cargo b
cd target/debug/
valgrind --leak-check=full ./ogdf-rs