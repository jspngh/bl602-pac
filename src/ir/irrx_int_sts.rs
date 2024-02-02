#[doc = "Register `irrx_int_sts` reader"]
pub type R = crate::R<IrrxIntStsSpec>;
#[doc = "Register `irrx_int_sts` writer"]
pub type W = crate::W<IrrxIntStsSpec>;
#[doc = "Field `irrx_end_int` reader - "]
pub type IrrxEndIntR = crate::BitReader;
#[doc = "Field `cr_irrx_end_mask` reader - "]
pub type CrIrrxEndMaskR = crate::BitReader;
#[doc = "Field `cr_irrx_end_mask` writer - "]
pub type CrIrrxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_end_clr` reader - "]
pub type CrIrrxEndClrR = crate::BitReader;
#[doc = "Field `cr_irrx_end_clr` writer - "]
pub type CrIrrxEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_end_en` reader - "]
pub type CrIrrxEndEnR = crate::BitReader;
#[doc = "Field `cr_irrx_end_en` writer - "]
pub type CrIrrxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irrx_end_int(&self) -> IrrxEndIntR {
        IrrxEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irrx_end_mask(&self) -> CrIrrxEndMaskR {
        CrIrrxEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irrx_end_clr(&self) -> CrIrrxEndClrR {
        CrIrrxEndClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irrx_end_en(&self) -> CrIrrxEndEnR {
        CrIrrxEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irrx_end_mask(&mut self) -> CrIrrxEndMaskW<'_, IrrxIntStsSpec> {
        CrIrrxEndMaskW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irrx_end_clr(&mut self) -> CrIrrxEndClrW<'_, IrrxIntStsSpec> {
        CrIrrxEndClrW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irrx_end_en(&mut self) -> CrIrrxEndEnW<'_, IrrxIntStsSpec> {
        CrIrrxEndEnW::new(self, 24)
    }
}
#[doc = "irrx_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxIntStsSpec;
impl crate::RegisterSpec for IrrxIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_int_sts::R`](R) reader structure"]
impl crate::Readable for IrrxIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_int_sts::W`](W) writer structure"]
impl crate::Writable for IrrxIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_int_sts to value 0x0100_0100"]
impl crate::Resettable for IrrxIntStsSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
