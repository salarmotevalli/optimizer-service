use crate::{
    api::http::{Config, UploadForm},
    domain::error::{DomainErr, DomainResult, ErrKind},
};

pub struct UploadImageValidation<'a> {
    form: &'a UploadForm,
    config: Config,
}

impl<'a> UploadImageValidation<'a> {
    pub fn new(form: &'a UploadForm, config: Config) -> Self {
        Self { form, config }
    }
}

impl<'a> UploadImageValidation<'a> {
    pub fn validate(&self) -> DomainResult<()> {
        // TODO: also sign url service need to check file format.
        let mime = match &self.form.file.content_type {
            Some(mime) => mime,
            None => {
                return Err(DomainErr::new(
                    "missing content type".to_string(),
                    ErrKind::UnprocessableErr,
                ));
            }
        };

        if mime.type_() != "image" {
            return Err(DomainErr::new(
                "invalid file type".to_string(),
                ErrKind::UnprocessableErr,
            ));
        }

        // Check against allowed image formats
        let format = mime.subtype().as_str();
        if !&self.config.image_white_list.contains(&format.to_string()) {
            return Err(DomainErr::new(
                format!("unsupported image format: {}", format),
                ErrKind::UnprocessableErr,
            ));
        }

        Ok(())
    }
}
