#[doc = "Register `ef_crc_ctrl_0` reader"]
pub type R = crate::R<EfCrcCtrl0Spec>;
#[doc = "Register `ef_crc_ctrl_0` writer"]
pub type W = crate::W<EfCrcCtrl0Spec>;
#[doc = "Field `ef_crc_busy` reader - "]
pub type EfCrcBusyR = crate::BitReader;
#[doc = "Field `ef_crc_trig` reader - "]
pub type EfCrcTrigR = crate::BitReader;
#[doc = "Field `ef_crc_trig` writer - "]
pub type EfCrcTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_en` reader - "]
pub type EfCrcEnR = crate::BitReader;
#[doc = "Field `ef_crc_en` writer - "]
pub type EfCrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_mode` reader - "]
pub type EfCrcModeR = crate::BitReader;
#[doc = "Field `ef_crc_mode` writer - "]
pub type EfCrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_error` reader - "]
pub type EfCrcErrorR = crate::BitReader;
#[doc = "Field `ef_crc_dout_inv_en` reader - "]
pub type EfCrcDoutInvEnR = crate::BitReader;
#[doc = "Field `ef_crc_dout_inv_en` writer - "]
pub type EfCrcDoutInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_dout_endian` reader - "]
pub type EfCrcDoutEndianR = crate::BitReader;
#[doc = "Field `ef_crc_dout_endian` writer - "]
pub type EfCrcDoutEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_din_endian` reader - "]
pub type EfCrcDinEndianR = crate::BitReader;
#[doc = "Field `ef_crc_din_endian` writer - "]
pub type EfCrcDinEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_int` reader - "]
pub type EfCrcIntR = crate::BitReader;
#[doc = "Field `ef_crc_int_clr` reader - "]
pub type EfCrcIntClrR = crate::BitReader;
#[doc = "Field `ef_crc_int_clr` writer - "]
pub type EfCrcIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_int_set` reader - "]
pub type EfCrcIntSetR = crate::BitReader;
#[doc = "Field `ef_crc_int_set` writer - "]
pub type EfCrcIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_lock` reader - "]
pub type EfCrcLockR = crate::BitReader;
#[doc = "Field `ef_crc_lock` writer - "]
pub type EfCrcLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_crc_slp_n` reader - "]
pub type EfCrcSlpNR = crate::FieldReader<u16>;
#[doc = "Field `ef_crc_slp_n` writer - "]
pub type EfCrcSlpNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_crc_busy(&self) -> EfCrcBusyR {
        EfCrcBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&self) -> EfCrcTrigR {
        EfCrcTrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&self) -> EfCrcEnR {
        EfCrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&self) -> EfCrcModeR {
        EfCrcModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_crc_error(&self) -> EfCrcErrorR {
        EfCrcErrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&self) -> EfCrcDoutInvEnR {
        EfCrcDoutInvEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&self) -> EfCrcDoutEndianR {
        EfCrcDoutEndianR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&self) -> EfCrcDinEndianR {
        EfCrcDinEndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ef_crc_int(&self) -> EfCrcIntR {
        EfCrcIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&self) -> EfCrcIntClrR {
        EfCrcIntClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&self) -> EfCrcIntSetR {
        EfCrcIntSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&self) -> EfCrcLockR {
        EfCrcLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&self) -> EfCrcSlpNR {
        EfCrcSlpNR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&mut self) -> EfCrcTrigW<'_, EfCrcCtrl0Spec> {
        EfCrcTrigW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&mut self) -> EfCrcEnW<'_, EfCrcCtrl0Spec> {
        EfCrcEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&mut self) -> EfCrcModeW<'_, EfCrcCtrl0Spec> {
        EfCrcModeW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&mut self) -> EfCrcDoutInvEnW<'_, EfCrcCtrl0Spec> {
        EfCrcDoutInvEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&mut self) -> EfCrcDoutEndianW<'_, EfCrcCtrl0Spec> {
        EfCrcDoutEndianW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&mut self) -> EfCrcDinEndianW<'_, EfCrcCtrl0Spec> {
        EfCrcDinEndianW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&mut self) -> EfCrcIntClrW<'_, EfCrcCtrl0Spec> {
        EfCrcIntClrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&mut self) -> EfCrcIntSetW<'_, EfCrcCtrl0Spec> {
        EfCrcIntSetW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&mut self) -> EfCrcLockW<'_, EfCrcCtrl0Spec> {
        EfCrcLockW::new(self, 11)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&mut self) -> EfCrcSlpNW<'_, EfCrcCtrl0Spec> {
        EfCrcSlpNW::new(self, 16)
    }
}
#[doc = "ef_crc_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl0Spec;
impl crate::RegisterSpec for EfCrcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_0::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_0::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_crc_ctrl_0 to value 0x00ff_0224"]
impl crate::Resettable for EfCrcCtrl0Spec {
    const RESET_VALUE: u32 = 0x00ff_0224;
}
