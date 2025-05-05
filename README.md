# Swissgrid Chart Transformer

Chart Transformer (Reprojection) from CH1903 (LV03) to WSG84.


## Command Line Usage

### Syntax
 * `swissgrid_chart_transformer [OPTIONS] --chart \<CHART_FILE\> --output \<OUTPUT_FILE\>`
 * Options:
   * `-r` for two reference points & rotation _(Format: \<pixel1_x\> \<pixel1_y\> \<coord1_e\> \<coord_n\> \<pixel2_x\> \<pixel2_y\> \<coord2_e\> \<coord2_n\>)_
   * `-s` for two reference points & stretch _(Format: same as above)_
   * `-t` for one reference point & scale & DPI _(Format: \<pixel_x\> \<pixel_y\> \<coord_e\> \<coord_n\> \<scale\> \<dpi\>)_

#### Example: Two Reference Points & Rotation
```bash
swissgrid_chart_transformer -r 41 191 600000 200000 194 358 605000 204000 \
--chart chart_in.png \
--output chart_out.png
```

#### Example: Two Reference Points & Stretch
```bash
swissgrid_chart_transformer -s 41 191 600000 200000 194 358 605000 204000 \
--chart chart_in.png \
--outputchart_out.png
```

#### Example: One Reference Points & Scale (1:10000) & DPI (200)
```bash
swissgrid_chart_transformer -t 41 191 600000 200000 10000 200 \
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
