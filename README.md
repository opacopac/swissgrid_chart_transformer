# Swissgrid Chart Transformer

Chart Transformer (Reprojection) from CH1903 (LV03) to WSG84.


## Command Line Usage

### Syntax
 * `swissgrid_chart_transformer [OPTIONS] --chart \<CHART_FILE\> --output \<OUTPUT_FILE\>`
 * Options:
   * `-r` for two reference points & rotation _(Format: \<pixel1_x\> \<pixel1_y\> \<lon1\> \<lat1\> \<pixel2_x\> \<pixel2_y\> \<lon2\> \<lat2\>)_
   * `-s` for two reference points & stretch _(Format: same as above)_
   * `-t` for one reference point & scale & DPI _(Format: \<pixel1_x\> \<pixel1_y\> \<lon1\> \<lat1\> \<scale\> \<dpi\>)_

#### Example: Two Reference Points & Rotation
```bash
swissgrid_chart_transformer -r 41 191 7.490458 46.919443 194 358 7.494211 46.916595 \
--chart chart_in.png \
--output chart_out.png
```

#### Example: Two Reference Points & Stretch
```bash
swissgrid_chart_transformer -s 41 191 7.490458 46.919443 194 358 7.494211 46.916595 \
--chart chart_in.png \
--outputchart_out.png
```

#### Example: One Reference Points & Scale (1:10000) & DPI (200)
```bash
swissgrid_chart_transformer -t 41 191 7.490458 46.919443 10000 200 \
--chart chart_in.png \
--output chart_out.png
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
