# YetAnotherMinecraftClone

## Commands

### Dependencies

#### alsa
- Linux (if no sudo) :
```shell
mkdir -p ~/local/alsa
cd ~/local/alsa
wget https://www.alsa-project.org/files/pub/lib/alsa-lib-1.2.10.tar.bz2
tar xvjf alsa-lib-1.2.10.tar.bz2
cd alsa-lib-1.2.10
./configure --prefix=$HOME/local/alsa
make
make install
export PKG_CONFIG_PATH=$HOME/local/alsa/lib/pkgconfig:$PKG_CONFIG_PATH
```

- Linux (if sudo) :
```shell
sudo apt-get install libasound2-dev
```

### Build and run

- build projects :
```shell
cargo build --release
```

- start server :
```shell
cargo run -p server
```

- start client :
```shell
cargo run -p client
```

- only build shared :
```shell
cargo build -p shared
```
