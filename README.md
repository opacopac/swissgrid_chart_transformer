# Swissgrid Chart Transformer

Chart Transformer from CH1903 (LV03) to WSG84

## Command Line Usage
TBD


## Instructions for Developers

### Compiling the project locally
```bash
cargo build --release
```

### Using Docker
1. Build the image
```bash
docker build -t swissgrid_chart_transformer .
```

2. Run the container
```bash
docker run -it --rm --name swissgrid_chart_transformer swissgrid_chart_transformer
```
