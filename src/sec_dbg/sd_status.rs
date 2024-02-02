#[doc = "Register `sd_status` reader"]
pub type R = crate::R<SdStatusSpec>;
#[doc = "Register `sd_status` writer"]
pub type W = crate::W<SdStatusSpec>;
#[doc = "Field `sd_dbg_pwd_busy` reader - "]
pub type SdDbgPwdBusyR = crate::BitReader;
#[doc = "Field `sd_dbg_pwd_trig` reader - "]
pub type SdDbgPwdTrigR = crate::BitReader;
#[doc = "Field `sd_dbg_pwd_trig` writer - "]
pub type SdDbgPwdTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_cci_read_en` reader - "]
pub type SdDbgCciReadEnR = crate::BitReader;
#[doc = "Field `sd_dbg_cci_read_en` writer - "]
pub type SdDbgCciReadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_cci_clk_sel` reader - "]
pub type SdDbgCciClkSelR = crate::BitReader;
#[doc = "Field `sd_dbg_cci_clk_sel` writer - "]
pub type SdDbgCciClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_pwd_cnt` reader - "]
pub type SdDbgPwdCntR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_mode` reader - "]
pub type SdDbgModeR = crate::FieldReader;
#[doc = "Field `sd_dbg_ena` reader - "]
pub type SdDbgEnaR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&self) -> SdDbgPwdBusyR {
        SdDbgPwdBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&self) -> SdDbgPwdTrigR {
        SdDbgPwdTrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&self) -> SdDbgCciReadEnR {
        SdDbgCciReadEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&self) -> SdDbgCciClkSelR {
        SdDbgCciClkSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&self) -> SdDbgPwdCntR {
        SdDbgPwdCntR::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&self) -> SdDbgModeR {
        SdDbgModeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&self) -> SdDbgEnaR {
        SdDbgEnaR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&mut self) -> SdDbgPwdTrigW<'_, SdStatusSpec> {
        SdDbgPwdTrigW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&mut self) -> SdDbgCciReadEnW<'_, SdStatusSpec> {
        SdDbgCciReadEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&mut self) -> SdDbgCciClkSelW<'_, SdStatusSpec> {
        SdDbgCciClkSelW::new(self, 3)
    }
}
#[doc = "sd_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdStatusSpec;
impl crate::RegisterSpec for SdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_status::R`](R) reader structure"]
impl crate::Readable for SdStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_status::W`](W) writer structure"]
impl crate::Writable for SdStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sd_status to value 0"]
impl crate::Resettable for SdStatusSpec {}
