#[doc = "Register `gpdac_config` reader"]
pub type R = crate::R<GpdacConfigSpec>;
#[doc = "Register `gpdac_config` writer"]
pub type W = crate::W<GpdacConfigSpec>;
#[doc = "Field `gpdac_en` reader - "]
pub type GpdacEnR = crate::BitReader;
#[doc = "Field `gpdac_en` writer - "]
pub type GpdacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_en2` reader - "]
pub type GpdacEn2R = crate::BitReader;
#[doc = "Field `gpdac_en2` writer - "]
pub type GpdacEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsm_mode` reader - "]
pub type DsmModeR = crate::FieldReader;
#[doc = "Field `dsm_mode` writer - "]
pub type DsmModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_mode` reader - "]
pub type GpdacModeR = crate::FieldReader;
#[doc = "Field `gpdac_mode` writer - "]
pub type GpdacModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpdac_ch_a_sel` reader - "]
pub type GpdacChASelR = crate::FieldReader;
#[doc = "Field `gpdac_ch_a_sel` writer - "]
pub type GpdacChASelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpdac_ch_b_sel` reader - "]
pub type GpdacChBSelR = crate::FieldReader;
#[doc = "Field `gpdac_ch_b_sel` writer - "]
pub type GpdacChBSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rsvd_31_24` reader - "]
pub type Rsvd31_24R = crate::FieldReader;
#[doc = "Field `rsvd_31_24` writer - "]
pub type Rsvd31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&self) -> GpdacEnR {
        GpdacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&self) -> GpdacEn2R {
        GpdacEn2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&self) -> DsmModeR {
        DsmModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&self) -> GpdacModeR {
        GpdacModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&self) -> GpdacChASelR {
        GpdacChASelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&self) -> GpdacChBSelR {
        GpdacChBSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> Rsvd31_24R {
        Rsvd31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&mut self) -> GpdacEnW<'_, GpdacConfigSpec> {
        GpdacEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&mut self) -> GpdacEn2W<'_, GpdacConfigSpec> {
        GpdacEn2W::new(self, 1)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&mut self) -> DsmModeW<'_, GpdacConfigSpec> {
        DsmModeW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&mut self) -> GpdacModeW<'_, GpdacConfigSpec> {
        GpdacModeW::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&mut self) -> GpdacChASelW<'_, GpdacConfigSpec> {
        GpdacChASelW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&mut self) -> GpdacChBSelW<'_, GpdacConfigSpec> {
        GpdacChBSelW::new(self, 20)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> Rsvd31_24W<'_, GpdacConfigSpec> {
        Rsvd31_24W::new(self, 24)
    }
}
#[doc = "gpdac_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacConfigSpec;
impl crate::RegisterSpec for GpdacConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config::R`](R) reader structure"]
impl crate::Readable for GpdacConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config::W`](W) writer structure"]
impl crate::Writable for GpdacConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_config to value 0"]
impl crate::Resettable for GpdacConfigSpec {}
