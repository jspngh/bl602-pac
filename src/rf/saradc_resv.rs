#[doc = "Register `saradc_resv` reader"]
pub type R = crate::R<SaradcResvSpec>;
#[doc = "Register `saradc_resv` writer"]
pub type W = crate::W<SaradcResvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SARADC Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_resv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_resv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaradcResvSpec;
impl crate::RegisterSpec for SaradcResvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_resv::R`](R) reader structure"]
impl crate::Readable for SaradcResvSpec {}
#[doc = "`write(|w| ..)` method takes [`saradc_resv::W`](W) writer structure"]
impl crate::Writable for SaradcResvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets saradc_resv to value 0"]
impl crate::Resettable for SaradcResvSpec {}
