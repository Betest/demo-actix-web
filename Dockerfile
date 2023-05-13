# FROM rust:latest

# WORKDIR /usr/src/prueba

# COPY . .

# RUN cargo build --release

# CMD [ "cargo", "run" ]

###########################

# FROM rust:alpine3.16

# WORKDIR /usr/src/prueba
# COPY . .

# RUN cargo install --path .

# CMD ["prueba"]

###########################

FROM rust:1.69-buster AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Production stage

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/demo-actix-web .

# RUN cargo install --path .

CMD ["demo-actix-web"]

#############################
# Establece la imagen base de Rust
# FROM rust:latest

# # Establece el directorio de trabajo en el contenedor
# WORKDIR /app

# # Copia todo el código fuente del proyecto
# COPY . .

# # Ejecuta el comando de compilación del proyecto
# RUN cargo build --release

# # Configura las variables de entorno necesarias para ejecutar el análisis con SonarQube
# ENV SONAR_HOST_URL=${SONAR_HOST_URL}
# ENV SONAR_LOGIN=sqa_c2c7dd9d4a31f1f679b080cfc8d50cb506ed633e

# # Agrega SonarScanner a la imagen
# ADD https://binaries.sonarsource.com/Distribution/sonar-scanner-cli/sonar-scanner-cli-4.6.2.2472-linux.zip /usr/local/
# RUN unzip /usr/local/sonar-scanner-cli-4.6.2.2472-linux.zip -d /usr/local/
# ENV PATH $PATH:/usr/local/sonar-scanner-4.6.2.2472-linux/bin

# # Agrega el directorio de instalación de Rust al PATH del contenedor
# ENV PATH="/usr/local/cargo/bin:${PATH}"

# # Establece el comando por defecto que se ejecutará al iniciar el contenedor
# CMD ["cargo", "test"]