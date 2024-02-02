#[doc = "Register `reg_data_1_lock` reader"]
pub type R = crate::R<RegData1LockSpec>;
#[doc = "Register `reg_data_1_lock` writer"]
pub type W = crate::W<RegData1LockSpec>;
#[doc = "Field `RESERVED_9_0` reader - "]
pub type Reserved9_0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_9_0` writer - "]
pub type Reserved9_0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `wr_lock_key_slot_6` reader - "]
pub type WrLockKeySlot6R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_6` writer - "]
pub type WrLockKeySlot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_7` reader - "]
pub type WrLockKeySlot7R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_7` writer - "]
pub type WrLockKeySlot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_8` reader - "]
pub type WrLockKeySlot8R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_8` writer - "]
pub type WrLockKeySlot8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_9` reader - "]
pub type WrLockKeySlot9R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_9` writer - "]
pub type WrLockKeySlot9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_25_16` reader - "]
pub type Reserved25_16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_25_16` writer - "]
pub type Reserved25_16W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rd_lock_key_slot_6` reader - "]
pub type RdLockKeySlot6R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_6` writer - "]
pub type RdLockKeySlot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_7` reader - "]
pub type RdLockKeySlot7R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_7` writer - "]
pub type RdLockKeySlot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_8` reader - "]
pub type RdLockKeySlot8R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_8` writer - "]
pub type RdLockKeySlot8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_9` reader - "]
pub type RdLockKeySlot9R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_9` writer - "]
pub type RdLockKeySlot9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&self) -> Reserved9_0R {
        Reserved9_0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WrLockKeySlot6R {
        WrLockKeySlot6R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WrLockKeySlot7R {
        WrLockKeySlot7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WrLockKeySlot8R {
        WrLockKeySlot8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WrLockKeySlot9R {
        WrLockKeySlot9R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&self) -> Reserved25_16R {
        Reserved25_16R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RdLockKeySlot6R {
        RdLockKeySlot6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RdLockKeySlot7R {
        RdLockKeySlot7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RdLockKeySlot8R {
        RdLockKeySlot8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RdLockKeySlot9R {
        RdLockKeySlot9R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&mut self) -> Reserved9_0W<'_, RegData1LockSpec> {
        Reserved9_0W::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&mut self) -> WrLockKeySlot6W<'_, RegData1LockSpec> {
        WrLockKeySlot6W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&mut self) -> WrLockKeySlot7W<'_, RegData1LockSpec> {
        WrLockKeySlot7W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&mut self) -> WrLockKeySlot8W<'_, RegData1LockSpec> {
        WrLockKeySlot8W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&mut self) -> WrLockKeySlot9W<'_, RegData1LockSpec> {
        WrLockKeySlot9W::new(self, 13)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&mut self) -> Reserved25_16W<'_, RegData1LockSpec> {
        Reserved25_16W::new(self, 16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&mut self) -> RdLockKeySlot6W<'_, RegData1LockSpec> {
        RdLockKeySlot6W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&mut self) -> RdLockKeySlot7W<'_, RegData1LockSpec> {
        RdLockKeySlot7W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&mut self) -> RdLockKeySlot8W<'_, RegData1LockSpec> {
        RdLockKeySlot8W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&mut self) -> RdLockKeySlot9W<'_, RegData1LockSpec> {
        RdLockKeySlot9W::new(self, 29)
    }
}
#[doc = "reg_data_1_lock.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_1_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_1_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegData1LockSpec;
impl crate::RegisterSpec for RegData1LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_data_1_lock::R`](R) reader structure"]
impl crate::Readable for RegData1LockSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_data_1_lock::W`](W) writer structure"]
impl crate::Writable for RegData1LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_data_1_lock to value 0"]
impl crate::Resettable for RegData1LockSpec {}
