# Random Images Rust #

## 📔 Prepare
> Folder: `{project folder}/image`
```shell
git clone https://github.com/Sakura1943/random-images-rs.git
cd random-images-rs
mkdir ./images
```



## 🏃 Running
### 1) 📔 Method One
```shell
wget https://github.com/Sakura1943/random-images-rs/releases/download/Packages/random_images.tar.gz
mkdir ./random_images
tar -zxvf ./random_images.tar.gz -C ./random_images
cd ./random_images
chmod +x ./main
./main
```

### 2) 📔 Method Two
#### ⚙ Building
```shell
cargo build --release
./target/release/random_images
```

#### 🏃 Running

```shell
./target/release/random_images
```
