# bebop-test

Transaction Info struct is in schemas/transaction_info.bop.

The build.rs script will compile the schema file and generate the Rust structs and trait implementations needed for serialization/deserialization.

# Installation
```
bash -c "$(curl https://bebop.sh)"
echo 'export PATH=$PATH:/usr/local/bin' >> /home/your_username/.profile
export PATH=$PATH:/usr/local/bin
```