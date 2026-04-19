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

#
# Create a container with flatc installed
#
FROM ubuntu:latest
RUN apt-get update && apt-get upgrade -y
RUN apt-get update --fix-missing
RUN apt-get install -y build-essential git sudo file rustup cmake
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
