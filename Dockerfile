# Install Tailwind CLI
FROM node:latest AS node_builder
RUN npm i -g --prefix /usr/node_builder tailwindcss


# Install Rust and its WASM toolchain
FROM rust:latest AS rust_builder
WORKDIR /usr/local/bin
RUN rustup target add wasm32-unknown-unknown
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# Copy Tailwind CLI executable from node_builder stage
WORKDIR /myapp
COPY --from=node_builder /usr/node_builder/bin/tailwindcss /usr/local/bin/tailwindcss

# Build the application using Trunk
COPY ./Cargo.toml .
COPY ./build.rs .
COPY ./index.html .
COPY ./src src
COPY ./public public
COPY ./style style
RUN trunk build --release

# Copy compiled files from rust_builder stage and serve them using Nginx
FROM nginx:latest
COPY --from=rust_builder /myapp/dist /dist
RUN mkdir -p /etc/nginx/html
RUN cp /dist/index.html /etc/nginx/html/
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
