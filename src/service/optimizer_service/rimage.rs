use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use crate::domain::{
    error::{DomainErr, DomainResult},
    param::optimizer_service_param::*,
    service::OptimizerService,
};

use async_trait::async_trait;
use rimage::{
    codecs::{
        avif::AvifEncoder, mozjpeg::MozJpegEncoder, oxipng::OxiPngEncoder, webp::WebPEncoder,
    },
    operations::icc::ApplySRGB,
};

use zune_core::{bit_depth::BitDepth, bytestream::ZByteWriterTrait, colorspace::ColorSpace};
use zune_image::{
    codecs::ImageFormat,
    core_filters::{colorspace::ColorspaceConv, depth::Depth},
    errors::ImageErrors,
    image::Image,
    pipelines::Pipeline,
    traits::{EncoderTrait, OperationsTrait},
};
use zune_imageprocs::auto_orient::AutoOrient;

pub struct OptimizerServiceRImageImpl {}

#[derive(Default)]
struct OptResult {
    image: Vec<u8>,

}

impl OptimizerServiceRImageImpl {
    fn optimize(
        &self,
        input: &Path,
        quality: f32,
        weight: Option<usize>,
        height: Option<usize>,
    ) -> DomainResult<OptResult> {
        let mut pipeline = Pipeline::<Image>::new();

        let img = self.decode(&input)?;

        let ext = match input.extension() {
            Some(e) => e,
            None => {
                return Err(DomainErr::new(
                    "cannot find extention".to_string(),
                    crate::domain::error::ErrKind::UnprocessableErr,
                ));
            }
        };

        let mut available_encoder = match ext.to_str().unwrap() {
            "jpeg" => AvailableEncoders::MozJpeg(Box::new(self.get_jpg_encoder(quality))),
            "png" => AvailableEncoders::OxiPng(Box::new(self.get_png_encoder())),
            "avif" => AvailableEncoders::Avif(Box::new(self.get_avif_encoder(quality))),
            "webp" => AvailableEncoders::Webp(Box::new(self.get_webp_encoder(quality))),
            name => panic!("Encoder \"{name}\" not found"),
        };

        pipeline.chain_operations(Box::new(Depth::new(BitDepth::Eight)));
        pipeline.chain_operations(Box::new(ColorspaceConv::new(ColorSpace::RGBA)));

        pipeline.chain_operations(Box::new(AutoOrient));
        pipeline.chain_operations(Box::new(ApplySRGB));

        if weight.is_some() || height.is_some() {
            let resize_op = self.resize_operation(&img, (weight, height));

            pipeline.chain_operations(resize_op);
        }

        pipeline.chain_decoder(img);

        pipeline.advance_to_end()?;

        let mut image = Vec::new();

        available_encoder.encode(&pipeline.images()[0], &mut image)?;

        Ok(OptResult {
            image,
        })
    }

    pub fn decode<P: AsRef<Path>>(&self, f: P) -> Result<Image, ImageErrors> {
        Image::open(f.as_ref()).or_else(|e| {
            if matches!(e, ImageErrors::ImageDecoderNotImplemented(_)) {
                let mut file = File::open(f.as_ref())?;

                {
                    let mut file_content = vec![];

                    file.read_to_end(&mut file_content)?;
                    file.seek(SeekFrom::Start(0))?;

                    if libavif::is_avif(&file_content) {
                        use rimage::codecs::avif::AvifDecoder;

                        let decoder = AvifDecoder::try_new(file)?;

                        return Image::from_decoder(decoder);
                    };
                    file.seek(SeekFrom::Start(0))?;
                }

                {
                    if f.as_ref()
                        .extension()
                        .is_some_and(|f| f.eq_ignore_ascii_case("webp"))
                    {
                        use rimage::codecs::webp::WebPDecoder;

                        let decoder = WebPDecoder::try_new(file)?;

                        return Image::from_decoder(decoder);
                    }

                    file.seek(SeekFrom::Start(0))?;
                }

                {
                    if f.as_ref().extension().is_some_and(|f| {
                        f.eq_ignore_ascii_case("tiff") | f.eq_ignore_ascii_case("tif")
                    }) {
                        use rimage::codecs::tiff::TiffDecoder;

                        let decoder = TiffDecoder::try_new(file)?;

                        return Image::from_decoder(decoder);
                    }

                    file.seek(SeekFrom::Start(0))?;
                }

                Err(ImageErrors::ImageDecoderNotImplemented(
                    ImageFormat::Unknown,
                ))
            } else {
                Err(e)
            }
        })
    }

    pub fn resize_operation(
        &self,
        img: &Image,
        resize: (Option<usize>, Option<usize>),
    ) -> Box<dyn OperationsTrait> {
        use fast_image_resize::ResizeAlg;
        use rimage::operations::resize::Resize;

        let (mut o_w, mut o_h) = img.dimensions();

        if let Some(w) = resize.0 {
            o_w = w
        }

        if let Some(h) = resize.1 {
            o_h = h
        }

        Box::new(Resize::new(o_w, o_h, ResizeAlg::Nearest))
    }

    fn get_png_encoder(&self) -> OxiPngEncoder {
        use rimage::codecs::oxipng::OxiPngOptions;

        let preset_level = 2;
        let opts = OxiPngOptions::from_preset(preset_level);

        OxiPngEncoder::new_with_options(opts)
    }

    fn get_webp_encoder(&self, quality: f32) -> WebPEncoder {
        use rimage::codecs::webp::WebPOptions;

        let mut options = WebPOptions::new().unwrap();

        options.quality = quality;
        WebPEncoder::new_with_options(options)
    }

    fn get_jpg_encoder(&self, quality: f32) -> MozJpegEncoder {
        use rimage::codecs::mozjpeg::MozJpegOptions;

        let mut opts = MozJpegOptions::default();

        opts.quality = quality;

        MozJpegEncoder::new_with_options(opts)
    }

    fn get_avif_encoder(&self, quality: f32) -> AvifEncoder {
        use rimage::codecs::avif::AvifOptions;

        let mut opts = AvifOptions::default();
        opts.quality = quality;

        AvifEncoder::new_with_options(opts)
    }
}

pub enum AvailableEncoders {
    MozJpeg(Box<MozJpegEncoder>),
    OxiPng(Box<OxiPngEncoder>),
    Avif(Box<AvifEncoder>),
    Webp(Box<WebPEncoder>),
}

impl AvailableEncoders {
    pub fn to_extension(&self) -> &'static str {
        match self {
            AvailableEncoders::MozJpeg(_) => "jpg",
            AvailableEncoders::OxiPng(_) => "png",
            AvailableEncoders::Avif(_) => "avif",
            AvailableEncoders::Webp(_) => "webp",
        }
    }

    pub fn encode<T: ZByteWriterTrait>(
        &mut self,
        img: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        match self {
            AvailableEncoders::MozJpeg(enc) => enc.encode(img, sink),
            AvailableEncoders::OxiPng(enc) => enc.encode(img, sink),
            AvailableEncoders::Avif(enc) => enc.encode(img, sink),
            AvailableEncoders::Webp(enc) => enc.encode(img, sink),
        }
    }
}

#[async_trait]
impl OptimizerService for OptimizerServiceRImageImpl {
    async fn process(&self, param: ProcessParam) -> DomainResult<ProcessResult> {
        let image_path = format!("./tmp/{}", param.image.full_name);

        let res = self.optimize(
            Path::new(&image_path), 
            param.specification.quality, 
            param.specification.width, 
            param.specification.height
        )?;

        Ok(ProcessResult{
            data: res.image
        })

    }
}
