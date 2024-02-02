#[doc = "Register `sd_chip_id_low` reader"]
pub type R = crate::R<SdChipIdLowSpec>;
#[doc = "Field `sd_chip_id_low` reader - "]
pub type SdChipIdLowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&self) -> SdChipIdLowR {
        SdChipIdLowR::new(self.bits)
    }
}
#[doc = "sd_chip_id_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdChipIdLowSpec;
impl crate::RegisterSpec for SdChipIdLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_chip_id_low::R`](R) reader structure"]
impl crate::Readable for SdChipIdLowSpec {}
#[doc = "`reset()` method sets sd_chip_id_low to value 0"]
impl crate::Resettable for SdChipIdLowSpec {}
