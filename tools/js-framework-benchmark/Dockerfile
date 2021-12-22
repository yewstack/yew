
FROM ubuntu:19.10
COPY install_rust.sh /root/
RUN echo "unsafe-perm = true" > /root/.npmrc
RUN echo "export NG_CLI_ANALYTICS=ci" >> /root/.npmrc
RUN echo "{ \"allow_root\": true }" >  /root/.bowerrc

# replace shell with bash so we can source files
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

RUN apt-get update
RUN apt-get install -y m4 libtinfo5 libghc-zlib-dev rsync ghc haskell-stack curl g++ make git openjdk-8-jdk dos2unix

ENV NVM_DIR /usr/local/nvm
RUN mkdir -p $NVM_DIR
ENV NODE_VERSION 10.16.3
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.0/install.sh | bash

# install node and npm
RUN source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

RUN curl https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb > /root/google-chrome-stable_current_amd64.deb
RUN apt install -y /root/google-chrome-stable_current_amd64.deb

RUN mkdir /server
RUN mkdir /build
RUN mkdir /src

COPY package.json /server
WORKDIR /server
RUN npm install

# Volume before chown changes owwner
VOLUME /src
VOLUME /build
WORKDIR /build

# Install rust
RUN dos2unix /root/install_rust.sh
RUN bash /root/install_rust.sh

# USER user

RUN npm install
EXPOSE 8080
CMD ["/server/node_modules/.bin/http-server","-c-1","/build"]

