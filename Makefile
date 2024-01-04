SOURCES_DIR=.
BUILD_DIR=dist


all: init build test


.SILENT:
init:
	cmake -S ${SOURCES_DIR} -B ${BUILD_DIR} -G "Unix Makefiles"


.SILENT:
build:
	cd ${BUILD_DIR} && \
	cmake --build .


.SILENT:
test:
	cd ${BUILD_DIR} && \
	ctest