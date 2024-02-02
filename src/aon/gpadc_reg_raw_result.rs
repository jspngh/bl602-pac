#[doc = "Register `gpadc_reg_raw_result` reader"]
pub type R = crate::R<GpadcRegRawResultSpec>;
#[doc = "Register `gpadc_reg_raw_result` writer"]
pub type W = crate::W<GpadcRegRawResultSpec>;
#[doc = "Field `gpadc_raw_data` reader - "]
pub type GpadcRawDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GpadcRawDataR {
        GpadcRawDataR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "gpadc_reg_raw_result.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_raw_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_raw_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegRawResultSpec;
impl crate::RegisterSpec for GpadcRegRawResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_raw_result::R`](R) reader structure"]
impl crate::Readable for GpadcRegRawResultSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_raw_result::W`](W) writer structure"]
impl crate::Writable for GpadcRegRawResultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpadc_reg_raw_result to value 0"]
impl crate::Resettable for GpadcRegRawResultSpec {}
