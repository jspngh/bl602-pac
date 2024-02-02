#[doc = "Register `irtx_int_sts` reader"]
pub type R = crate::R<IrtxIntStsSpec>;
#[doc = "Register `irtx_int_sts` writer"]
pub type W = crate::W<IrtxIntStsSpec>;
#[doc = "Field `irtx_end_int` reader - "]
pub type IrtxEndIntR = crate::BitReader;
#[doc = "Field `cr_irtx_end_mask` reader - "]
pub type CrIrtxEndMaskR = crate::BitReader;
#[doc = "Field `cr_irtx_end_mask` writer - "]
pub type CrIrtxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_end_clr` reader - "]
pub type CrIrtxEndClrR = crate::BitReader;
#[doc = "Field `cr_irtx_end_clr` writer - "]
pub type CrIrtxEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_end_en` reader - "]
pub type CrIrtxEndEnR = crate::BitReader;
#[doc = "Field `cr_irtx_end_en` writer - "]
pub type CrIrtxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irtx_end_int(&self) -> IrtxEndIntR {
        IrtxEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&self) -> CrIrtxEndMaskR {
        CrIrtxEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&self) -> CrIrtxEndClrR {
        CrIrtxEndClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&self) -> CrIrtxEndEnR {
        CrIrtxEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&mut self) -> CrIrtxEndMaskW<'_, IrtxIntStsSpec> {
        CrIrtxEndMaskW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&mut self) -> CrIrtxEndClrW<'_, IrtxIntStsSpec> {
        CrIrtxEndClrW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&mut self) -> CrIrtxEndEnW<'_, IrtxIntStsSpec> {
        CrIrtxEndEnW::new(self, 24)
    }
}
#[doc = "irtx_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxIntStsSpec;
impl crate::RegisterSpec for IrtxIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_int_sts::R`](R) reader structure"]
impl crate::Readable for IrtxIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_int_sts::W`](W) writer structure"]
impl crate::Writable for IrtxIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irtx_int_sts to value 0x0100_0100"]
impl crate::Resettable for IrtxIntStsSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
