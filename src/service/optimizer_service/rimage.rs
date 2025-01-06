use std::{fs::{self, File}, path::{Path}, str::FromStr};

use rimage::{config::{Codec, EncoderConfig, QuantizationConfig, ResizeConfig, ResizeType}, image::ImageResult, Decoder, Encoder};

use crate::domain::{error::{DomainErr, DomainResult, ErrKind}, param::image_service_param::OptImgParam, service::OptimizerService};

pub struct OptimizerServiceRImageImpl {}

impl OptimizerServiceRImageImpl {
    pub fn prepair_conf(
        &self,
        quality: f32,
        codec: &str,
        filter: &str,
        quantization: Option<u8>,
        dithering: Option<f32>,
        width: Option<usize>,
        height: Option<usize>,
    ) -> DomainResult<EncoderConfig> {
        let c = Codec::from_str(codec).map_err(|e| DomainErr::new(e, ErrKind::UnExpectedErr))?;

        let mut quantization_config = QuantizationConfig::new();

        if let Some(quality) = quantization {
            quantization_config = quantization_config.with_quality(quality)?
        }

        if let Some(dithering) = dithering {
            quantization_config = quantization_config.with_dithering(dithering / 100.0)?
        }

        let resize_filter = ResizeType::from_str(filter).map_err(|e| DomainErr::new(e, ErrKind::UnExpectedErr))?;

        let mut resize_config = ResizeConfig::new(resize_filter);

        if let Some(w) = width {
            resize_config = resize_config.with_width(w);
        }

        if let Some(h) = height {
            resize_config = resize_config.with_height(h);
        }

        let mut conf = EncoderConfig::new(c).with_quality(quality)?;

        if quantization.is_some() || dithering.is_some() {
            conf = conf.with_quantization(quantization_config);
        }

        if width.is_some() || height.is_some() {
            conf = conf.with_resize(resize_config);
        }

        Ok(conf)
    }

    fn optimize(&self, in_path: &Path, out_path: &Path, conf: EncoderConfig) -> ImageResult<()> {
        let decoder = Decoder::from_path(in_path)?;

        let image = decoder.decode()?;

        fs::create_dir_all(out_path.parent().unwrap())?;
        let out_file = File::create(out_path)?;

        let encoder = Encoder::new(out_file, image).with_config(conf);
        encoder.encode()?;

        Ok(())
    }
}

impl OptimizerService for OptimizerServiceRImageImpl {
    fn process(&self, param: OptImgParam) -> DomainResult<()> {
        let image_path = format!("./tmp/{}", param.image.full_name);
        let output_path = format!("./tmp/opted/{}", param.image.full_name);
        
        let conf = self.prepair_conf(
            param.specification.quality, 
            &param.image.ext(),
            &param.specification.filter,
            param.specification.quantization,
            param.specification.dithering,
            param.specification.width, 
            param.specification.height
        )?;

        self.optimize(
            Path::new(&image_path), 
            Path::new(&output_path),
            conf
        )?;

        todo!();
    }
}