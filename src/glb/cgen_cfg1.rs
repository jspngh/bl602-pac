#[doc = "Register `cgen_cfg1` reader"]
pub type R = crate::R<CgenCfg1Spec>;
#[doc = "Register `cgen_cfg1` writer"]
pub type W = crate::W<CgenCfg1Spec>;
#[doc = "Field `cgen_s1` reader - "]
pub type CgenS1R = crate::FieldReader<u16>;
#[doc = "Field `cgen_s1` writer - "]
pub type CgenS1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cgen_s1a` reader - "]
pub type CgenS1aR = crate::FieldReader;
#[doc = "Field `cgen_s1a` writer - "]
pub type CgenS1aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&self) -> CgenS1R {
        CgenS1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&self) -> CgenS1aR {
        CgenS1aR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&mut self) -> CgenS1W<'_, CgenCfg1Spec> {
        CgenS1W::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&mut self) -> CgenS1aW<'_, CgenCfg1Spec> {
        CgenS1aW::new(self, 16)
    }
}
#[doc = "cgen_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenCfg1Spec;
impl crate::RegisterSpec for CgenCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg1::R`](R) reader structure"]
impl crate::Readable for CgenCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg1::W`](W) writer structure"]
impl crate::Writable for CgenCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cgen_cfg1 to value 0x00ff_ffff"]
impl crate::Resettable for CgenCfg1Spec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
