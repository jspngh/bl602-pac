#[doc = "Register `pds_stat` reader"]
pub type R = crate::R<PdsStatSpec>;
#[doc = "Field `ro_pds_state` reader - "]
pub type RoPdsStateR = crate::FieldReader;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RoPdsRfStateR = crate::FieldReader;
#[doc = "Field `ro_pds_pll_state` reader - "]
pub type RoPdsPllStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RoPdsStateR {
        RoPdsStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RoPdsRfStateR {
        RoPdsRfStateR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&self) -> RoPdsPllStateR {
        RoPdsPllStateR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "pds_stat.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsStatSpec;
impl crate::RegisterSpec for PdsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_stat::R`](R) reader structure"]
impl crate::Readable for PdsStatSpec {}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PdsStatSpec {}
