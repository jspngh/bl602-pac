#[doc = "Register `l1c_config` reader"]
pub type R = crate::R<L1cConfigSpec>;
#[doc = "Register `l1c_config` writer"]
pub type W = crate::W<L1cConfigSpec>;
#[doc = "Field `l1c_cacheable` reader - "]
pub type L1cCacheableR = crate::BitReader;
#[doc = "Field `l1c_cacheable` writer - "]
pub type L1cCacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_cnt_en` reader - "]
pub type L1cCntEnR = crate::BitReader;
#[doc = "Field `l1c_cnt_en` writer - "]
pub type L1cCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_invalid_en` reader - "]
pub type L1cInvalidEnR = crate::BitReader;
#[doc = "Field `l1c_invalid_en` writer - "]
pub type L1cInvalidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_invalid_done` reader - "]
pub type L1cInvalidDoneR = crate::BitReader;
#[doc = "Field `l1c_way_dis` reader - "]
pub type L1cWayDisR = crate::FieldReader;
#[doc = "Field `l1c_way_dis` writer - "]
pub type L1cWayDisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `irom_2t_access` reader - "]
pub type Irom2tAccessR = crate::BitReader;
#[doc = "Field `irom_2t_access` writer - "]
pub type Irom2tAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_bypass` reader - "]
pub type L1cBypassR = crate::BitReader;
#[doc = "Field `l1c_bypass` writer - "]
pub type L1cBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_bmx_err_en` reader - "]
pub type L1cBmxErrEnR = crate::BitReader;
#[doc = "Field `l1c_bmx_err_en` writer - "]
pub type L1cBmxErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_bmx_arb_mode` reader - "]
pub type L1cBmxArbModeR = crate::FieldReader;
#[doc = "Field `l1c_bmx_arb_mode` writer - "]
pub type L1cBmxArbModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `l1c_bmx_timeout_en` reader - "]
pub type L1cBmxTimeoutEnR = crate::FieldReader;
#[doc = "Field `l1c_bmx_timeout_en` writer - "]
pub type L1cBmxTimeoutEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `l1c_bmx_busy_option_dis` reader - "]
pub type L1cBmxBusyOptionDisR = crate::BitReader;
#[doc = "Field `l1c_bmx_busy_option_dis` writer - "]
pub type L1cBmxBusyOptionDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `early_resp_dis` reader - "]
pub type EarlyRespDisR = crate::BitReader;
#[doc = "Field `early_resp_dis` writer - "]
pub type EarlyRespDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_dis` reader - "]
pub type WrapDisR = crate::BitReader;
#[doc = "Field `wrap_dis` writer - "]
pub type WrapDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&self) -> L1cCacheableR {
        L1cCacheableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&self) -> L1cCntEnR {
        L1cCntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&self) -> L1cInvalidEnR {
        L1cInvalidEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&self) -> L1cInvalidDoneR {
        L1cInvalidDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&self) -> L1cWayDisR {
        L1cWayDisR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&self) -> Irom2tAccessR {
        Irom2tAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&self) -> L1cBypassR {
        L1cBypassR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&self) -> L1cBmxErrEnR {
        L1cBmxErrEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&self) -> L1cBmxArbModeR {
        L1cBmxArbModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&self) -> L1cBmxTimeoutEnR {
        L1cBmxTimeoutEnR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&self) -> L1cBmxBusyOptionDisR {
        L1cBmxBusyOptionDisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&self) -> EarlyRespDisR {
        EarlyRespDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&self) -> WrapDisR {
        WrapDisR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&mut self) -> L1cCacheableW<'_, L1cConfigSpec> {
        L1cCacheableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&mut self) -> L1cCntEnW<'_, L1cConfigSpec> {
        L1cCntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&mut self) -> L1cInvalidEnW<'_, L1cConfigSpec> {
        L1cInvalidEnW::new(self, 2)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&mut self) -> L1cWayDisW<'_, L1cConfigSpec> {
        L1cWayDisW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&mut self) -> Irom2tAccessW<'_, L1cConfigSpec> {
        Irom2tAccessW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&mut self) -> L1cBypassW<'_, L1cConfigSpec> {
        L1cBypassW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&mut self) -> L1cBmxErrEnW<'_, L1cConfigSpec> {
        L1cBmxErrEnW::new(self, 15)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&mut self) -> L1cBmxArbModeW<'_, L1cConfigSpec> {
        L1cBmxArbModeW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&mut self) -> L1cBmxTimeoutEnW<'_, L1cConfigSpec> {
        L1cBmxTimeoutEnW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&mut self) -> L1cBmxBusyOptionDisW<'_, L1cConfigSpec> {
        L1cBmxBusyOptionDisW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&mut self) -> EarlyRespDisW<'_, L1cConfigSpec> {
        EarlyRespDisW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&mut self) -> WrapDisW<'_, L1cConfigSpec> {
        WrapDisW::new(self, 26)
    }
}
#[doc = "l1c_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cConfigSpec;
impl crate::RegisterSpec for L1cConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1c_config::R`](R) reader structure"]
impl crate::Readable for L1cConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`l1c_config::W`](W) writer structure"]
impl crate::Writable for L1cConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets l1c_config to value 0x0600_0f00"]
impl crate::Resettable for L1cConfigSpec {
    const RESET_VALUE: u32 = 0x0600_0f00;
}
