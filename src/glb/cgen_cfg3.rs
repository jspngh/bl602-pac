#[doc = "Register `cgen_cfg3` reader"]
pub type R = crate::R<CgenCfg3Spec>;
#[doc = "Register `cgen_cfg3` writer"]
pub type W = crate::W<CgenCfg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "cgen_cfg3.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenCfg3Spec;
impl crate::RegisterSpec for CgenCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg3::R`](R) reader structure"]
impl crate::Readable for CgenCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg3::W`](W) writer structure"]
impl crate::Writable for CgenCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cgen_cfg3 to value 0"]
impl crate::Resettable for CgenCfg3Spec {}
