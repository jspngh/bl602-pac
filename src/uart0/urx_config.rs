#[doc = "Register `urx_config` reader"]
pub type R = crate::R<UrxConfigSpec>;
#[doc = "Register `urx_config` writer"]
pub type W = crate::W<UrxConfigSpec>;
#[doc = "Field `cr_urx_en` reader - "]
pub type CrUrxEnR = crate::BitReader;
#[doc = "Field `cr_urx_en` writer - "]
pub type CrUrxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rts_sw_mode` reader - "]
pub type CrUrxRtsSwModeR = crate::BitReader;
#[doc = "Field `cr_urx_rts_sw_mode` writer - "]
pub type CrUrxRtsSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rts_sw_val` reader - "]
pub type CrUrxRtsSwValR = crate::BitReader;
#[doc = "Field `cr_urx_rts_sw_val` writer - "]
pub type CrUrxRtsSwValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_abr_en` reader - "]
pub type CrUrxAbrEnR = crate::BitReader;
#[doc = "Field `cr_urx_abr_en` writer - "]
pub type CrUrxAbrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_prt_en` reader - "]
pub type CrUrxPrtEnR = crate::BitReader;
#[doc = "Field `cr_urx_prt_en` writer - "]
pub type CrUrxPrtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_prt_sel` reader - "]
pub type CrUrxPrtSelR = crate::BitReader;
#[doc = "Field `cr_urx_prt_sel` writer - "]
pub type CrUrxPrtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ir_en` reader - "]
pub type CrUrxIrEnR = crate::BitReader;
#[doc = "Field `cr_urx_ir_en` writer - "]
pub type CrUrxIrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ir_inv` reader - "]
pub type CrUrxIrInvR = crate::BitReader;
#[doc = "Field `cr_urx_ir_inv` writer - "]
pub type CrUrxIrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_bit_cnt_d` reader - "]
pub type CrUrxBitCntDR = crate::FieldReader;
#[doc = "Field `cr_urx_bit_cnt_d` writer - "]
pub type CrUrxBitCntDW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_urx_deg_en` reader - "]
pub type CrUrxDegEnR = crate::BitReader;
#[doc = "Field `cr_urx_deg_en` writer - "]
pub type CrUrxDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_deg_cnt` reader - "]
pub type CrUrxDegCntR = crate::FieldReader;
#[doc = "Field `cr_urx_deg_cnt` writer - "]
pub type CrUrxDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_urx_len` reader - "]
pub type CrUrxLenR = crate::FieldReader<u16>;
#[doc = "Field `cr_urx_len` writer - "]
pub type CrUrxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&self) -> CrUrxEnR {
        CrUrxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CrUrxRtsSwModeR {
        CrUrxRtsSwModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CrUrxRtsSwValR {
        CrUrxRtsSwValR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&self) -> CrUrxAbrEnR {
        CrUrxAbrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&self) -> CrUrxPrtEnR {
        CrUrxPrtEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&self) -> CrUrxPrtSelR {
        CrUrxPrtSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&self) -> CrUrxIrEnR {
        CrUrxIrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&self) -> CrUrxIrInvR {
        CrUrxIrInvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&self) -> CrUrxBitCntDR {
        CrUrxBitCntDR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&self) -> CrUrxDegEnR {
        CrUrxDegEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&self) -> CrUrxDegCntR {
        CrUrxDegCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&self) -> CrUrxLenR {
        CrUrxLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&mut self) -> CrUrxEnW<'_, UrxConfigSpec> {
        CrUrxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CrUrxRtsSwModeW<'_, UrxConfigSpec> {
        CrUrxRtsSwModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&mut self) -> CrUrxRtsSwValW<'_, UrxConfigSpec> {
        CrUrxRtsSwValW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&mut self) -> CrUrxAbrEnW<'_, UrxConfigSpec> {
        CrUrxAbrEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&mut self) -> CrUrxPrtEnW<'_, UrxConfigSpec> {
        CrUrxPrtEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&mut self) -> CrUrxPrtSelW<'_, UrxConfigSpec> {
        CrUrxPrtSelW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&mut self) -> CrUrxIrEnW<'_, UrxConfigSpec> {
        CrUrxIrEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&mut self) -> CrUrxIrInvW<'_, UrxConfigSpec> {
        CrUrxIrInvW::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&mut self) -> CrUrxBitCntDW<'_, UrxConfigSpec> {
        CrUrxBitCntDW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&mut self) -> CrUrxDegEnW<'_, UrxConfigSpec> {
        CrUrxDegEnW::new(self, 11)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&mut self) -> CrUrxDegCntW<'_, UrxConfigSpec> {
        CrUrxDegCntW::new(self, 12)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&mut self) -> CrUrxLenW<'_, UrxConfigSpec> {
        CrUrxLenW::new(self, 16)
    }
}
#[doc = "urx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxConfigSpec;
impl crate::RegisterSpec for UrxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_config::R`](R) reader structure"]
impl crate::Readable for UrxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_config::W`](W) writer structure"]
impl crate::Writable for UrxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets urx_config to value 0x0700"]
impl crate::Resettable for UrxConfigSpec {
    const RESET_VALUE: u32 = 0x0700;
}
