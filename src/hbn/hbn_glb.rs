#[doc = "Register `HBN_GLB` reader"]
pub type R = crate::R<HbnGlbSpec>;
#[doc = "Register `HBN_GLB` writer"]
pub type W = crate::W<HbnGlbSpec>;
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HbnRootClkSelR = crate::FieldReader;
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub type HbnRootClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HbnUartClkSelR = crate::BitReader;
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub type HbnUartClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_f32k_sel` reader - "]
pub type HbnF32kSelR = crate::FieldReader;
#[doc = "Field `hbn_f32k_sel` writer - "]
pub type HbnF32kSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_pu_rc32k` reader - "]
pub type HbnPuRc32kR = crate::BitReader;
#[doc = "Field `hbn_pu_rc32k` writer - "]
pub type HbnPuRc32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` reader - "]
pub type SwLdo11socVoutSelAonR = crate::FieldReader;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` writer - "]
pub type SwLdo11socVoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sw_ldo11_rt_vout_sel` reader - "]
pub type SwLdo11RtVoutSelR = crate::FieldReader;
#[doc = "Field `sw_ldo11_rt_vout_sel` writer - "]
pub type SwLdo11RtVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sw_ldo11_aon_vout_sel` reader - "]
pub type SwLdo11AonVoutSelR = crate::FieldReader;
#[doc = "Field `sw_ldo11_aon_vout_sel` writer - "]
pub type SwLdo11AonVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HbnRootClkSelR {
        HbnRootClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HbnUartClkSelR {
        HbnUartClkSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HbnF32kSelR {
        HbnF32kSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&self) -> HbnPuRc32kR {
        HbnPuRc32kR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SwLdo11socVoutSelAonR {
        SwLdo11socVoutSelAonR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SwLdo11RtVoutSelR {
        SwLdo11RtVoutSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SwLdo11AonVoutSelR {
        SwLdo11AonVoutSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&mut self) -> HbnRootClkSelW<'_, HbnGlbSpec> {
        HbnRootClkSelW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&mut self) -> HbnUartClkSelW<'_, HbnGlbSpec> {
        HbnUartClkSelW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&mut self) -> HbnF32kSelW<'_, HbnGlbSpec> {
        HbnF32kSelW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&mut self) -> HbnPuRc32kW<'_, HbnGlbSpec> {
        HbnPuRc32kW::new(self, 5)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SwLdo11socVoutSelAonW<'_, HbnGlbSpec> {
        SwLdo11socVoutSelAonW::new(self, 16)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SwLdo11RtVoutSelW<'_, HbnGlbSpec> {
        SwLdo11RtVoutSelW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SwLdo11AonVoutSelW<'_, HbnGlbSpec> {
        SwLdo11AonVoutSelW::new(self, 28)
    }
}
#[doc = "HBN_GLB.\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_glb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_glb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnGlbSpec;
impl crate::RegisterSpec for HbnGlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_glb::R`](R) reader structure"]
impl crate::Readable for HbnGlbSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_glb::W`](W) writer structure"]
impl crate::Writable for HbnGlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_GLB to value 0xaa0a_0020"]
impl crate::Resettable for HbnGlbSpec {
    const RESET_VALUE: u32 = 0xaa0a_0020;
}
