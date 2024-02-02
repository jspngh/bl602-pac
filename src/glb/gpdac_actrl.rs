#[doc = "Register `gpdac_actrl` reader"]
pub type R = crate::R<GpdacActrlSpec>;
#[doc = "Register `gpdac_actrl` writer"]
pub type W = crate::W<GpdacActrlSpec>;
#[doc = "Field `gpdac_a_en` reader - "]
pub type GpdacAEnR = crate::BitReader;
#[doc = "Field `gpdac_a_en` writer - "]
pub type GpdacAEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ioa_en` reader - "]
pub type GpdacIoaEnR = crate::BitReader;
#[doc = "Field `gpdac_ioa_en` writer - "]
pub type GpdacIoaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_a_rng` reader - "]
pub type GpdacARngR = crate::FieldReader;
#[doc = "Field `gpdac_a_rng` writer - "]
pub type GpdacARngW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_a_outmux` reader - "]
pub type GpdacAOutmuxR = crate::FieldReader;
#[doc = "Field `gpdac_a_outmux` writer - "]
pub type GpdacAOutmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GpdacAEnR {
        GpdacAEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GpdacIoaEnR {
        GpdacIoaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GpdacARngR {
        GpdacARngR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GpdacAOutmuxR {
        GpdacAOutmuxR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&mut self) -> GpdacAEnW<'_, GpdacActrlSpec> {
        GpdacAEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&mut self) -> GpdacIoaEnW<'_, GpdacActrlSpec> {
        GpdacIoaEnW::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&mut self) -> GpdacARngW<'_, GpdacActrlSpec> {
        GpdacARngW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&mut self) -> GpdacAOutmuxW<'_, GpdacActrlSpec> {
        GpdacAOutmuxW::new(self, 20)
    }
}
#[doc = "gpdac_actrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_actrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_actrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacActrlSpec;
impl crate::RegisterSpec for GpdacActrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_actrl::R`](R) reader structure"]
impl crate::Readable for GpdacActrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_actrl::W`](W) writer structure"]
impl crate::Writable for GpdacActrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_actrl to value 0x000c_0000"]
impl crate::Resettable for GpdacActrlSpec {
    const RESET_VALUE: u32 = 0x000c_0000;
}
