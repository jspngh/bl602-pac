#[doc = "Register `utx_config` reader"]
pub type R = crate::R<UtxConfigSpec>;
#[doc = "Register `utx_config` writer"]
pub type W = crate::W<UtxConfigSpec>;
#[doc = "Field `cr_utx_en` reader - "]
pub type CrUtxEnR = crate::BitReader;
#[doc = "Field `cr_utx_en` writer - "]
pub type CrUtxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_cts_en` reader - "]
pub type CrUtxCtsEnR = crate::BitReader;
#[doc = "Field `cr_utx_cts_en` writer - "]
pub type CrUtxCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_frm_en` reader - "]
pub type CrUtxFrmEnR = crate::BitReader;
#[doc = "Field `cr_utx_frm_en` writer - "]
pub type CrUtxFrmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_prt_en` reader - "]
pub type CrUtxPrtEnR = crate::BitReader;
#[doc = "Field `cr_utx_prt_en` writer - "]
pub type CrUtxPrtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_prt_sel` reader - "]
pub type CrUtxPrtSelR = crate::BitReader;
#[doc = "Field `cr_utx_prt_sel` writer - "]
pub type CrUtxPrtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_ir_en` reader - "]
pub type CrUtxIrEnR = crate::BitReader;
#[doc = "Field `cr_utx_ir_en` writer - "]
pub type CrUtxIrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_ir_inv` reader - "]
pub type CrUtxIrInvR = crate::BitReader;
#[doc = "Field `cr_utx_ir_inv` writer - "]
pub type CrUtxIrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_bit_cnt_d` reader - "]
pub type CrUtxBitCntDR = crate::FieldReader;
#[doc = "Field `cr_utx_bit_cnt_d` writer - "]
pub type CrUtxBitCntDW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_utx_bit_cnt_p` reader - "]
pub type CrUtxBitCntPR = crate::FieldReader;
#[doc = "Field `cr_utx_bit_cnt_p` writer - "]
pub type CrUtxBitCntPW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_utx_len` reader - "]
pub type CrUtxLenR = crate::FieldReader<u16>;
#[doc = "Field `cr_utx_len` writer - "]
pub type CrUtxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&self) -> CrUtxEnR {
        CrUtxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&self) -> CrUtxCtsEnR {
        CrUtxCtsEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&self) -> CrUtxFrmEnR {
        CrUtxFrmEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&self) -> CrUtxPrtEnR {
        CrUtxPrtEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&self) -> CrUtxPrtSelR {
        CrUtxPrtSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&self) -> CrUtxIrEnR {
        CrUtxIrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&self) -> CrUtxIrInvR {
        CrUtxIrInvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&self) -> CrUtxBitCntDR {
        CrUtxBitCntDR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&self) -> CrUtxBitCntPR {
        CrUtxBitCntPR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&self) -> CrUtxLenR {
        CrUtxLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&mut self) -> CrUtxEnW<'_, UtxConfigSpec> {
        CrUtxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&mut self) -> CrUtxCtsEnW<'_, UtxConfigSpec> {
        CrUtxCtsEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&mut self) -> CrUtxFrmEnW<'_, UtxConfigSpec> {
        CrUtxFrmEnW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&mut self) -> CrUtxPrtEnW<'_, UtxConfigSpec> {
        CrUtxPrtEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&mut self) -> CrUtxPrtSelW<'_, UtxConfigSpec> {
        CrUtxPrtSelW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&mut self) -> CrUtxIrEnW<'_, UtxConfigSpec> {
        CrUtxIrEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&mut self) -> CrUtxIrInvW<'_, UtxConfigSpec> {
        CrUtxIrInvW::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&mut self) -> CrUtxBitCntDW<'_, UtxConfigSpec> {
        CrUtxBitCntDW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&mut self) -> CrUtxBitCntPW<'_, UtxConfigSpec> {
        CrUtxBitCntPW::new(self, 12)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&mut self) -> CrUtxLenW<'_, UtxConfigSpec> {
        CrUtxLenW::new(self, 16)
    }
}
#[doc = "utx_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtxConfigSpec;
impl crate::RegisterSpec for UtxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utx_config::R`](R) reader structure"]
impl crate::Readable for UtxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`utx_config::W`](W) writer structure"]
impl crate::Writable for UtxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets utx_config to value 0x1700"]
impl crate::Resettable for UtxConfigSpec {
    const RESET_VALUE: u32 = 0x1700;
}
