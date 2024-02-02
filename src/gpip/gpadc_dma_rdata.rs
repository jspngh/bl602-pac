#[doc = "Register `gpadc_dma_rdata` reader"]
pub type R = crate::R<GpadcDmaRdataSpec>;
#[doc = "Field `gpadc_dma_rdata` reader - "]
pub type GpadcDmaRdataR = crate::FieldReader<u32>;
#[doc = "Field `rsvd_31_26` reader - "]
pub type Rsvd31_26R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GpadcDmaRdataR {
        GpadcDmaRdataR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> Rsvd31_26R {
        Rsvd31_26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[doc = "gpadc_dma_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_dma_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcDmaRdataSpec;
impl crate::RegisterSpec for GpadcDmaRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_dma_rdata::R`](R) reader structure"]
impl crate::Readable for GpadcDmaRdataSpec {}
#[doc = "`reset()` method sets gpadc_dma_rdata to value 0"]
impl crate::Resettable for GpadcDmaRdataSpec {}
