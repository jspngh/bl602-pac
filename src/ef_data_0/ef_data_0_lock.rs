#[doc = "Register `ef_data_0_lock` reader"]
pub type R = crate::R<EfData0LockSpec>;
#[doc = "Register `ef_data_0_lock` writer"]
pub type W = crate::W<EfData0LockSpec>;
#[doc = "Field `ef_ana_trim_1` reader - "]
pub type EfAnaTrim1R = crate::FieldReader<u16>;
#[doc = "Field `ef_ana_trim_1` writer - "]
pub type EfAnaTrim1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `wr_lock_key_slot_4_l` reader - "]
pub type WrLockKeySlot4LR = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_4_l` writer - "]
pub type WrLockKeySlot4LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_5_l` reader - "]
pub type WrLockKeySlot5LR = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_5_l` writer - "]
pub type WrLockKeySlot5LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_boot_mode` reader - "]
pub type WrLockBootModeR = crate::BitReader;
#[doc = "Field `wr_lock_boot_mode` writer - "]
pub type WrLockBootModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_dbg_pwd` reader - "]
pub type WrLockDbgPwdR = crate::BitReader;
#[doc = "Field `wr_lock_dbg_pwd` writer - "]
pub type WrLockDbgPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_sw_usage_0` reader - "]
pub type WrLockSwUsage0R = crate::BitReader;
#[doc = "Field `wr_lock_sw_usage_0` writer - "]
pub type WrLockSwUsage0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_wifi_mac` reader - "]
pub type WrLockWifiMacR = crate::BitReader;
#[doc = "Field `wr_lock_wifi_mac` writer - "]
pub type WrLockWifiMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_0` reader - "]
pub type WrLockKeySlot0R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_0` writer - "]
pub type WrLockKeySlot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_1` reader - "]
pub type WrLockKeySlot1R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_1` writer - "]
pub type WrLockKeySlot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_2` reader - "]
pub type WrLockKeySlot2R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_2` writer - "]
pub type WrLockKeySlot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_3` reader - "]
pub type WrLockKeySlot3R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_3` writer - "]
pub type WrLockKeySlot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_4_h` reader - "]
pub type WrLockKeySlot4HR = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_4_h` writer - "]
pub type WrLockKeySlot4HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_5_h` reader - "]
pub type WrLockKeySlot5HR = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_5_h` writer - "]
pub type WrLockKeySlot5HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_dbg_pwd` reader - "]
pub type RdLockDbgPwdR = crate::BitReader;
#[doc = "Field `rd_lock_dbg_pwd` writer - "]
pub type RdLockDbgPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_0` reader - "]
pub type RdLockKeySlot0R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_0` writer - "]
pub type RdLockKeySlot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_1` reader - "]
pub type RdLockKeySlot1R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_1` writer - "]
pub type RdLockKeySlot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_2` reader - "]
pub type RdLockKeySlot2R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_2` writer - "]
pub type RdLockKeySlot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_3` reader - "]
pub type RdLockKeySlot3R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_3` writer - "]
pub type RdLockKeySlot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_4` reader - "]
pub type RdLockKeySlot4R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_4` writer - "]
pub type RdLockKeySlot4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_5` reader - "]
pub type RdLockKeySlot5R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_5` writer - "]
pub type RdLockKeySlot5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&self) -> EfAnaTrim1R {
        EfAnaTrim1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&self) -> WrLockKeySlot4LR {
        WrLockKeySlot4LR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&self) -> WrLockKeySlot5LR {
        WrLockKeySlot5LR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WrLockBootModeR {
        WrLockBootModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WrLockDbgPwdR {
        WrLockDbgPwdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WrLockSwUsage0R {
        WrLockSwUsage0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WrLockWifiMacR {
        WrLockWifiMacR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WrLockKeySlot0R {
        WrLockKeySlot0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WrLockKeySlot1R {
        WrLockKeySlot1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WrLockKeySlot2R {
        WrLockKeySlot2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WrLockKeySlot3R {
        WrLockKeySlot3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&self) -> WrLockKeySlot4HR {
        WrLockKeySlot4HR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&self) -> WrLockKeySlot5HR {
        WrLockKeySlot5HR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RdLockDbgPwdR {
        RdLockDbgPwdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RdLockKeySlot0R {
        RdLockKeySlot0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RdLockKeySlot1R {
        RdLockKeySlot1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RdLockKeySlot2R {
        RdLockKeySlot2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RdLockKeySlot3R {
        RdLockKeySlot3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RdLockKeySlot4R {
        RdLockKeySlot4R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RdLockKeySlot5R {
        RdLockKeySlot5R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&mut self) -> EfAnaTrim1W<'_, EfData0LockSpec> {
        EfAnaTrim1W::new(self, 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&mut self) -> WrLockKeySlot4LW<'_, EfData0LockSpec> {
        WrLockKeySlot4LW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&mut self) -> WrLockKeySlot5LW<'_, EfData0LockSpec> {
        WrLockKeySlot5LW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&mut self) -> WrLockBootModeW<'_, EfData0LockSpec> {
        WrLockBootModeW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&mut self) -> WrLockDbgPwdW<'_, EfData0LockSpec> {
        WrLockDbgPwdW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&mut self) -> WrLockSwUsage0W<'_, EfData0LockSpec> {
        WrLockSwUsage0W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&mut self) -> WrLockWifiMacW<'_, EfData0LockSpec> {
        WrLockWifiMacW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&mut self) -> WrLockKeySlot0W<'_, EfData0LockSpec> {
        WrLockKeySlot0W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&mut self) -> WrLockKeySlot1W<'_, EfData0LockSpec> {
        WrLockKeySlot1W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&mut self) -> WrLockKeySlot2W<'_, EfData0LockSpec> {
        WrLockKeySlot2W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&mut self) -> WrLockKeySlot3W<'_, EfData0LockSpec> {
        WrLockKeySlot3W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&mut self) -> WrLockKeySlot4HW<'_, EfData0LockSpec> {
        WrLockKeySlot4HW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&mut self) -> WrLockKeySlot5HW<'_, EfData0LockSpec> {
        WrLockKeySlot5HW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&mut self) -> RdLockDbgPwdW<'_, EfData0LockSpec> {
        RdLockDbgPwdW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&mut self) -> RdLockKeySlot0W<'_, EfData0LockSpec> {
        RdLockKeySlot0W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&mut self) -> RdLockKeySlot1W<'_, EfData0LockSpec> {
        RdLockKeySlot1W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&mut self) -> RdLockKeySlot2W<'_, EfData0LockSpec> {
        RdLockKeySlot2W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&mut self) -> RdLockKeySlot3W<'_, EfData0LockSpec> {
        RdLockKeySlot3W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&mut self) -> RdLockKeySlot4W<'_, EfData0LockSpec> {
        RdLockKeySlot4W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&mut self) -> RdLockKeySlot5W<'_, EfData0LockSpec> {
        RdLockKeySlot5W::new(self, 31)
    }
}
#[doc = "ef_data_0_lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_0_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_0_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfData0LockSpec;
impl crate::RegisterSpec for EfData0LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_data_0_lock::R`](R) reader structure"]
impl crate::Readable for EfData0LockSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_data_0_lock::W`](W) writer structure"]
impl crate::Writable for EfData0LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_data_0_lock to value 0"]
impl crate::Resettable for EfData0LockSpec {}
