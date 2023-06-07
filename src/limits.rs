use crate::image::ImageError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[allow(missing_copy_implementations)]
pub struct Limits {
    pub max_image_width: Option<u32>,
    pub max_image_height: Option<u32>,
    pub max_alloc: Option<u64>,
    _non_exhaustive: (),
}

impl Default for Limits {
    fn default() -> Limits {
        Limits {
            max_image_width: None,
            max_image_height: None,
            max_alloc: Some(512 * 1024 * 1024),
            _non_exhaustive: (),
        }
    }
}

impl Limits {
    pub fn reserve(&mut self, amount: u64) -> Result<(), ImageError> {
        if let Some(max_alloc) = self.max_alloc.as_mut() {
            if *max_alloc < amount {
                return Err(ImageError::Limits);
            }

            *max_alloc -= amount;
        }

        Ok(())
    }
}
