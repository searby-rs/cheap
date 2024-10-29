PWD=$(shell pwd)
SRC=${PWD}/src
OUT=${PWD}/.out
INC=${OUT}/include/cheap
LIB=${OUT}/lib
DEB=${PWD}/target/debug
REL=${PWD}/target/release

debug:
	mkdir -p ${OUT}
	mkdir -p ${INC}
	mkdir -p ${LIB}
	cargo build
	cp -rf ${DEB}/libcheap.a ${LIB}
	cp -rf ${DEB}/libcheap.so ${LIB}
	cbindgen --profile debug --package-version --lang c ${SRC}/lib.rs --output ${INC}/cheap.h
	cbindgen --profile debug --package-version --lang c++ ${SRC}/lib.rs --output ${INC}/cheap.hpp

release:
	mkdir -p ${OUT}
	mkdir -p ${INC}
	mkdir -p ${LIB}
	cargo build --release
	cp -rf ${REL}/libcheap.a ${LIB}
	cp -rf ${REL}/libcheap.so ${LIB}
	cbindgen --profile release --package-version --lang c ${SRC}/lib.rs --output ${INC}/cheap.h
	cbindgen --profile release --package-version --lang c++ ${SRC}/lib.rs --output ${INC}/cheap.hpp

install:
	cp -rf ${INC} /usr/local/include
	cp -rf ${LIB}/libcheap.a /usr/local/lib
	cp -rf ${LIB}/libcheap.so /usr/local/lib

install_termux:
	cp -rf ${INC} /data/data/com.termux/files/usr/include
	cp -rf ${LIB}/libcheap.a /data/data/com.termux/files/usr/lib
	cp -rf ${LIB}/libcheap.so /data/data/com.termux/files/usr/lib

uninstall:
	rm -rf /usr/local/include/cheap
	rm -rf /usr/local/lib/libcheap.a
	rm -rf /usr/local/lib/libcheap.so

uninstall_termux:
	rm -rf /data/data/com.termux/files/usr/include/cheap
	rm -rf /data/data/com.termux/files/usr/lib/libcheap.a
	rm -rf /data/data/com.termux/files/usr/lib/libcheap.so

clean:
	cargo clean
	rm -rf ${OUT}
