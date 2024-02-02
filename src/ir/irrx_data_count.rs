#[doc = "Register `irrx_data_count` reader"]
pub type R = crate::R<IrrxDataCountSpec>;
#[doc = "Register `irrx_data_count` writer"]
pub type W = crate::W<IrrxDataCountSpec>;
#[doc = "Field `sts_irrx_data_cnt` reader - "]
pub type StsIrrxDataCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&self) -> StsIrrxDataCntR {
        StsIrrxDataCntR::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "irrx_data_count.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataCountSpec;
impl crate::RegisterSpec for IrrxDataCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_count::R`](R) reader structure"]
impl crate::Readable for IrrxDataCountSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_data_count::W`](W) writer structure"]
impl crate::Writable for IrrxDataCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_data_count to value 0"]
impl crate::Resettable for IrrxDataCountSpec {}
