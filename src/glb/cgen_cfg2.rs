#[doc = "Register `cgen_cfg2` reader"]
pub type R = crate::R<CgenCfg2Spec>;
#[doc = "Register `cgen_cfg2` writer"]
pub type W = crate::W<CgenCfg2Spec>;
#[doc = "Field `cgen_s2` reader - "]
pub type CgenS2R = crate::BitReader;
#[doc = "Field `cgen_s2` writer - "]
pub type CgenS2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s3` reader - "]
pub type CgenS3R = crate::BitReader;
#[doc = "Field `cgen_s3` writer - "]
pub type CgenS3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&self) -> CgenS2R {
        CgenS2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&self) -> CgenS3R {
        CgenS3R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&mut self) -> CgenS2W<'_, CgenCfg2Spec> {
        CgenS2W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&mut self) -> CgenS3W<'_, CgenCfg2Spec> {
        CgenS3W::new(self, 4)
    }
}
#[doc = "cgen_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenCfg2Spec;
impl crate::RegisterSpec for CgenCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg2::R`](R) reader structure"]
impl crate::Readable for CgenCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg2::W`](W) writer structure"]
impl crate::Writable for CgenCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cgen_cfg2 to value 0x11"]
impl crate::Resettable for CgenCfg2Spec {
    const RESET_VALUE: u32 = 0x11;
}
