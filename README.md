# Swissgrid Chart Transformer

Chart Transformer (Reprojection) from CH1903 (LV03) to WSG84.


## Command Line Usage

### Example: Two Reference Points & Rotation
```bash
swissgrid_chart_transformer -r 41 191 7.490458 46.919443 194 358 7.494211 46.916595 \
--chart /path/to/input/chart_in.png \
--output /path/to/output/chart_out.png
```


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
