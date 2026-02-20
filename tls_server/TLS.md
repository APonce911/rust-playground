## How to generate TLS Keys / Certificates

Linux commands to generate new keys and certificates for TLS server/client

### Server
Generate CA private key

    openssl genrsa -out ca.key 4096

Self-sign the CA certificate (10 year validity)

    openssl req -new -x509 -days 3650 -key ca.key -out ca.crt \
      -subj "/C=US/ST=Local/L=Local/O=MyCA/CN=MyLocalCA"

Generate server private key

    openssl genrsa -out server.key 2048

Generate server CSR

    openssl req -new -key server.key -out server.csr \
      -subj "/C=US/ST=Local/L=Local/O=MyServer/CN=localhost"

Create a SAN extension file (required for modern TLS)

    cat > server_ext.cnf <<EOF
    [req]
    distinguished_name = req_distinguished_name
    [req_distinguished_name]
    [v3_req]
    subjectAltName = @alt_names
    [alt_names]
    DNS.1 = localhost
    IP.1  = 127.0.0.1
    EOF

Sign the server certificate with the CA

    openssl x509 -req -days 825 -in server.csr \
      -CA ca.crt -CAkey ca.key -CAcreateserial \
      -out server.crt \
      -extensions v3_req -extfile server_ext.cnf

Bundle into a PFX (PKCS#12) — set your own export password or leave empty

    openssl pkcs12 -export \
      -out server.pfx \
      -inkey server.key \
      -in server.crt \
      -certfile ca.crt \
      -passout pass:yourpassword

### Client (one-way TLS)

Convert the CA certificate to DER format for the client

    openssl x509 -in ca.crt -outform DER -out ca_cert.der


### Client (mutual TLS)

Generate client private key

    openssl genrsa -out client.key 2048

Generate client CSR

    openssl req -new -key client.key -out client.csr \
      -subj "/C=US/ST=Local/L=Local/O=MyClient/CN=client"

Sign the client certificate with the CA

    openssl x509 -req -days 825 -in client.csr \
      -CA ca.crt -CAkey ca.key -CAcreateserial \
      -out client.crt

Convert to DER format

    openssl x509 -in client.crt -outform DER -out client.der

### Sanity Check

Confirm the server cert was signed by your CA

    openssl verify -CAfile ca.crt server.crt

Confirm the SAN includes 127.0.0.1 (critical — you're connecting by IP)

    openssl x509 -in server.crt -noout -ext subjectAltName

Confirm the PFX bundles the full chain

    openssl pkcs12 -info -in server.pfx -passin pass:yourpassword -nokeys
