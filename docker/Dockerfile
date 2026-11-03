# MIT License
#
# Copyright (c) 2026 Andrew Ellis Page
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#
# SPDX short identifier: MIT

## Container to build&test
## obtain key from from github.com => Settings => Credentials => Personal Access Token
## enable write:packages
##
## ```bash
## docker build -t containername .
## export GHCR_KEY=xxxxx
## echo $GHCR_KEY | docker login ghcr.io -u USERNAME --password-stdin
## docker tag containername  ghcr.io/username(lowercase)/containername
## docker push ghcr.io/username(lowercase)/avxbuild-arm64
## ```
##
##  Then enable repo or make container public:
##     github.com => Packages



#
# Create a container with flatc installed
#
FROM ubuntu:latest
RUN apt update --fix-missing
RUN apt install -y build-essential git sudo file rustup cmake
RUN rustup default stable
RUN mkdir /opt/build
RUN sudo -u ubuntu rustup default stable
RUN cd /opt/build && \
    git clone https://github.com/google/flatbuffers.git && \
    cd flatbuffers && \
    git checkout v25.12.19 && \
    mkdir release && \
    cd release && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/opt/flatbuffers .. && \
    make -j `nproc` install && \
    cd && \
    rm -rf /opt/build
