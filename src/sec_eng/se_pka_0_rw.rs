#[doc = "Register `se_pka_0_rw` reader"]
pub type R = crate::R<SePka0RwSpec>;
#[doc = "Register `se_pka_0_rw` writer"]
pub type W = crate::W<SePka0RwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "se_pka_0_rw.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0RwSpec;
impl crate::RegisterSpec for SePka0RwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_rw::R`](R) reader structure"]
impl crate::Readable for SePka0RwSpec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_rw::W`](W) writer structure"]
impl crate::Writable for SePka0RwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_pka_0_rw to value 0"]
impl crate::Resettable for SePka0RwSpec {}
