#[doc = "Register `fbdv` reader"]
pub type R = crate::R<FbdvSpec>;
#[doc = "Register `fbdv` writer"]
pub type W = crate::W<FbdvSpec>;
#[doc = "Field `lo_fbdv_halfstep_en_hw` reader - "]
pub type LoFbdvHalfstepEnHwR = crate::BitReader;
#[doc = "Field `lo_fbdv_halfstep_en_hw` writer - "]
pub type LoFbdvHalfstepEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_fbdv_halfstep_en` reader - "]
pub type LoFbdvHalfstepEnR = crate::BitReader;
#[doc = "Field `lo_fbdv_halfstep_en` writer - "]
pub type LoFbdvHalfstepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_fbdv_sel_sample_clk` reader - "]
pub type LoFbdvSelSampleClkR = crate::FieldReader;
#[doc = "Field `lo_fbdv_sel_sample_clk` writer - "]
pub type LoFbdvSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_fbdv_sel_fb_clk` reader - "]
pub type LoFbdvSelFbClkR = crate::FieldReader;
#[doc = "Field `lo_fbdv_sel_fb_clk` writer - "]
pub type LoFbdvSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_fbdv_rst` reader - "]
pub type LoFbdvRstR = crate::BitReader;
#[doc = "Field `lo_fbdv_rst` writer - "]
pub type LoFbdvRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_fbdv_rst_hw` reader - "]
pub type LoFbdvRstHwR = crate::BitReader;
#[doc = "Field `lo_fbdv_rst_hw` writer - "]
pub type LoFbdvRstHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&self) -> LoFbdvHalfstepEnHwR {
        LoFbdvHalfstepEnHwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&self) -> LoFbdvHalfstepEnR {
        LoFbdvHalfstepEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&self) -> LoFbdvSelSampleClkR {
        LoFbdvSelSampleClkR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&self) -> LoFbdvSelFbClkR {
        LoFbdvSelFbClkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&self) -> LoFbdvRstR {
        LoFbdvRstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&self) -> LoFbdvRstHwR {
        LoFbdvRstHwR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&mut self) -> LoFbdvHalfstepEnHwW<'_, FbdvSpec> {
        LoFbdvHalfstepEnHwW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&mut self) -> LoFbdvHalfstepEnW<'_, FbdvSpec> {
        LoFbdvHalfstepEnW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&mut self) -> LoFbdvSelSampleClkW<'_, FbdvSpec> {
        LoFbdvSelSampleClkW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&mut self) -> LoFbdvSelFbClkW<'_, FbdvSpec> {
        LoFbdvSelFbClkW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&mut self) -> LoFbdvRstW<'_, FbdvSpec> {
        LoFbdvRstW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&mut self) -> LoFbdvRstHwW<'_, FbdvSpec> {
        LoFbdvRstHwW::new(self, 20)
    }
}
#[doc = "fbdv.\n\nYou can [`read`](crate::Reg::read) this register and get [`fbdv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbdv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbdvSpec;
impl crate::RegisterSpec for FbdvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbdv::R`](R) reader structure"]
impl crate::Readable for FbdvSpec {}
#[doc = "`write(|w| ..)` method takes [`fbdv::W`](W) writer structure"]
impl crate::Writable for FbdvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets fbdv to value 0"]
impl crate::Resettable for FbdvSpec {}
