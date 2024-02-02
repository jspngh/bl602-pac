#[doc = "Register `pds_ram1` reader"]
pub type R = crate::R<PdsRam1Spec>;
#[doc = "Register `pds_ram1` writer"]
pub type W = crate::W<PdsRam1Spec>;
#[doc = "Field `cr_np_sram_pwr` reader - "]
pub type CrNpSramPwrR = crate::FieldReader;
#[doc = "Field `cr_np_sram_pwr` writer - "]
pub type CrNpSramPwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&self) -> CrNpSramPwrR {
        CrNpSramPwrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&mut self) -> CrNpSramPwrW<'_, PdsRam1Spec> {
        CrNpSramPwrW::new(self, 0)
    }
}
#[doc = "pds_ram1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsRam1Spec;
impl crate::RegisterSpec for PdsRam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ram1::R`](R) reader structure"]
impl crate::Readable for PdsRam1Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ram1::W`](W) writer structure"]
impl crate::Writable for PdsRam1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pds_ram1 to value 0"]
impl crate::Resettable for PdsRam1Spec {}
