# Download an image with build tools (cargo)
FROM rust:latest 

# Copy the project into our build image, into a new directory at /app
COPY . /app 
# Move to /app
WORKDIR /app 

# Install our app at /app/logger
RUN cargo install --path . --root /app/logger 

# Run this command when the image is run
CMD ["/app/logger/bin/logger"] 