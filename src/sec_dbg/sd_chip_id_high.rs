#[doc = "Register `sd_chip_id_high` reader"]
pub type R = crate::R<SdChipIdHighSpec>;
#[doc = "Field `sd_chip_id_high` reader - "]
pub type SdChipIdHighR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_high(&self) -> SdChipIdHighR {
        SdChipIdHighR::new(self.bits)
    }
}
#[doc = "sd_chip_id_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_high::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdChipIdHighSpec;
impl crate::RegisterSpec for SdChipIdHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_chip_id_high::R`](R) reader structure"]
impl crate::Readable for SdChipIdHighSpec {}
#[doc = "`reset()` method sets sd_chip_id_high to value 0"]
impl crate::Resettable for SdChipIdHighSpec {}
