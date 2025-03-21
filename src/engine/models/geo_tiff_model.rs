use std::io::BufReader;

use gdal::Dataset;

pub struct GeoTiffModel {
    ds: gdal::Dataset,
}

impl GeoTiffModel {
    pub fn new(geo_tiff_file_path: &str) -> Self {
        Self {
            ds: gdal::Dataset::open(geo_tiff_file_path).unwrap(),
        }
    }

    pub fn get_data(&self) -> Vec<Vec<f32>> {
        let raster_band = self.ds.rasterband(1).unwrap();
        let (width, height) = raster_band.size();
        let mut buf = vec![0.0; width * height];

        raster_band
            .read_into_slice::<f32>(
                (0, 0),
                raster_band.size(),
                (width, height),
                buf.as_mut_slice(),
                None,
            )
            .unwrap();

        let mut out: Vec<Vec<f32>> = vec![vec![0.0; height]; width];

        for i in 0..width {
            for j in 0..height {
                if buf[(j) * width + i] > 0.0 {
                    out[i][j] = buf[(j) * width + i];
                } else {
                    out[i][j] = 0.0;
                }
            }
        }

        out
    }
}
