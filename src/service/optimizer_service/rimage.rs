use std::{io::Cursor, path::Path, slice, str::FromStr, sync::Arc};
use rimage::{config::{Codec, EncoderConfig, QuantizationConfig, ResizeConfig, ResizeType}, image::ImageResult, Decoder, Encoder};
use zune_image::image::Image;
use crate::domain::{error::{DomainErr, DomainResult, ErrKind}, param::{file_storage_service_param::StoreParam, image_service_param::OptImgParam, optimizer_service_param::ProcessResult}, service::{FileStorageService, OptimizerService}};

pub struct OptimizerServiceRImageImpl {
    pub file_storage_service: Arc<dyn FileStorageService>
}

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

    fn optimize(&self, in_path: &Path, conf: EncoderConfig) -> ImageResult<Vec<u8>> {
        let decoder = Decoder::from_path(in_path)?;
        let image = decoder.decode()?;

        let buf: Box<[u8]> = vec![0u8].into_boxed_slice(); // Create a Box<[u8]>
        let ptr = buf.as_ptr();
        let len = buf.len();
        
        // Prevent the Box from deallocating the memory when it goes out of scope.
        std::mem::forget(&buf);

      
        let cursor = Cursor::new(buf); // Create the Cursor
            
        let encoder = Encoder::new(cursor, image).with_config(conf);
        
        encoder.encode()?;

        
        unsafe {
            let data = slice::from_raw_parts(ptr, len);

            let _ = Box::from_raw(data.as_ptr() as *mut u8); // Deallocates when _ goes out of scope

            Ok(data.to_vec())
        }   
    }
    
}

impl OptimizerService for OptimizerServiceRImageImpl {
    fn process(&self, param: OptImgParam) -> DomainResult<ProcessResult> {
        let image_path = format!("./tmp/{}", param.image.full_name);
        
        let conf = self.prepair_conf(
            param.specification.quality, 
            &param.image.ext(),
            &param.specification.filter,
            param.specification.quantization,
            param.specification.dithering,
            param.specification.width, 
            param.specification.height
        )?;

        let res = self.optimize(
            Path::new(&image_path), 
            conf
        )?;

        let store_param = StoreParam {data: res};
        self.file_storage_service.store(store_param);

        Ok(ProcessResult{})
    }
}