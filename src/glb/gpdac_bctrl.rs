#[doc = "Register `gpdac_bctrl` reader"]
pub type R = crate::R<GpdacBctrlSpec>;
#[doc = "Register `gpdac_bctrl` writer"]
pub type W = crate::W<GpdacBctrlSpec>;
#[doc = "Field `gpdac_b_en` reader - "]
pub type GpdacBEnR = crate::BitReader;
#[doc = "Field `gpdac_b_en` writer - "]
pub type GpdacBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_iob_en` reader - "]
pub type GpdacIobEnR = crate::BitReader;
#[doc = "Field `gpdac_iob_en` writer - "]
pub type GpdacIobEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_b_rng` reader - "]
pub type GpdacBRngR = crate::FieldReader;
#[doc = "Field `gpdac_b_rng` writer - "]
pub type GpdacBRngW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_b_outmux` reader - "]
pub type GpdacBOutmuxR = crate::FieldReader;
#[doc = "Field `gpdac_b_outmux` writer - "]
pub type GpdacBOutmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&self) -> GpdacBEnR {
        GpdacBEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&self) -> GpdacIobEnR {
        GpdacIobEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&self) -> GpdacBRngR {
        GpdacBRngR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&self) -> GpdacBOutmuxR {
        GpdacBOutmuxR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&mut self) -> GpdacBEnW<'_, GpdacBctrlSpec> {
        GpdacBEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&mut self) -> GpdacIobEnW<'_, GpdacBctrlSpec> {
        GpdacIobEnW::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&mut self) -> GpdacBRngW<'_, GpdacBctrlSpec> {
        GpdacBRngW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&mut self) -> GpdacBOutmuxW<'_, GpdacBctrlSpec> {
        GpdacBOutmuxW::new(self, 20)
    }
}
#[doc = "gpdac_bctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_bctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_bctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacBctrlSpec;
impl crate::RegisterSpec for GpdacBctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_bctrl::R`](R) reader structure"]
impl crate::Readable for GpdacBctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_bctrl::W`](W) writer structure"]
impl crate::Writable for GpdacBctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_bctrl to value 0x000c_0000"]
impl crate::Resettable for GpdacBctrlSpec {
    const RESET_VALUE: u32 = 0x000c_0000;
}
