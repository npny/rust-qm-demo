# Install script designed for OSX/homebrew :

PURPLE='\033[1;35m'
NC='\033[0m'

echo "${PURPLE}Installing Rust and SFML C bindings${NC}"
brew install rust sfml csfml

echo "${PURPLE}Building rust-sfml${NC}"
git clone git@github.com:jeremyletang/rust-sfml.git && cd rust-sfml
cargo build
cp target/debug/deps/*.rlib ../deps
cp target/debug/libsfml.rlib ../deps
cd ..

echo "${PURPLE}Building rust-num${NC}"
git clone git@github.com:rust-num/num.git && cd num
cargo build
cp target/debug/deps/*.rlib ../deps
cp target/debug/libnum.rlib ../deps
cd ..

echo "${PURPLE}Cleaning up${NC}"
#rm -rf rust-sfml num

echo "${PURPLE}Compiling main.rs${NC}"
rustc main.rs -L ./deps

echo "${PURPLE}Running main.rs${NC}"
./main