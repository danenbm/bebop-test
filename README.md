# Bebop Test
* TransactionInfo struct is in schemas/transaction_info.bop.
* The build.rs script will compile the schema file and generate the Rust structs and trait implementations.
* Generated code will end up in src/generated.
* Basic test of serialization/deserialization in src/main.rs.

# Installation
```
bash -c "$(curl https://bebop.sh)"
echo 'export PATH=$PATH:/usr/local/bin' >> /home/your_username/.profile
export PATH=$PATH:/usr/local/bin
```
