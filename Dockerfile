FROM ubuntu:bionic

# install required package to install LLVM and Rust
RUN apt-get update
RUN apt-get install -y wget gnupg software-properties-common git curl gcc zlib1g-dev

# install LLVM
RUN wget -O gpg.key https://apt.llvm.org/llvm-snapshot.gpg.key
RUN apt-key add gpg.key
RUN add-apt-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-10 main"
RUN apt-get update
RUN apt-get install -y clang-10 lldb-10 lld-10 clangd-10

# install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH "/root/.cargo/bin:$PATH"
RUN cargo -vV

# configure ssh for vscode Remote container plugin
RUN apt-get install -y openssh-server
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config
RUN sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config
RUN service ssh start
COPY id_rsa.pub /root/authorized_keys
RUN mkdir ~/.ssh && \
    mv ~/authorized_keys ~/.ssh/authorized_keys && \
    chmod 0600 ~/.ssh/authorized_keys
CMD ["/usr/sbin/sshd", "-D"]
