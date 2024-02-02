#[doc = "Register `ten_ac` reader"]
pub type R = crate::R<TenAcSpec>;
#[doc = "Register `ten_ac` writer"]
pub type W = crate::W<TenAcSpec>;
#[doc = "Field `atest_op_cc` reader - "]
pub type AtestOpCcR = crate::FieldReader;
#[doc = "Field `atest_op_cc` writer - "]
pub type AtestOpCcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `atest_dac_en` reader - "]
pub type AtestDacEnR = crate::BitReader;
#[doc = "Field `atest_dac_en` writer - "]
pub type AtestDacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_in_trx_sw` reader - "]
pub type AtestInTrxSwR = crate::BitReader;
#[doc = "Field `atest_in_trx_sw` writer - "]
pub type AtestInTrxSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_in_en` reader - "]
pub type AtestInEnR = crate::BitReader;
#[doc = "Field `atest_in_en` writer - "]
pub type AtestInEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_gain_r9` reader - "]
pub type AtestGainR9R = crate::FieldReader;
#[doc = "Field `atest_gain_r9` writer - "]
pub type AtestGainR9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `atest_gain_r8` reader - "]
pub type AtestGainR8R = crate::FieldReader;
#[doc = "Field `atest_gain_r8` writer - "]
pub type AtestGainR8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `atest_gain_r7` reader - "]
pub type AtestGainR7R = crate::FieldReader;
#[doc = "Field `atest_gain_r7` writer - "]
pub type AtestGainR7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `atest_gain_r6` reader - "]
pub type AtestGainR6R = crate::FieldReader;
#[doc = "Field `atest_gain_r6` writer - "]
pub type AtestGainR6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `atest_gain_r5` reader - "]
pub type AtestGainR5R = crate::FieldReader;
#[doc = "Field `atest_gain_r5` writer - "]
pub type AtestGainR5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `atest_out_en_q` reader - "]
pub type AtestOutEnQR = crate::BitReader;
#[doc = "Field `atest_out_en_q` writer - "]
pub type AtestOutEnQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_out_en_i` reader - "]
pub type AtestOutEnIR = crate::BitReader;
#[doc = "Field `atest_out_en_i` writer - "]
pub type AtestOutEnIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_in_en_q` reader - "]
pub type AtestInEnQR = crate::BitReader;
#[doc = "Field `atest_in_en_q` writer - "]
pub type AtestInEnQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atest_in_en_i` reader - "]
pub type AtestInEnIR = crate::BitReader;
#[doc = "Field `atest_in_en_i` writer - "]
pub type AtestInEnIW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn atest_op_cc(&self) -> AtestOpCcR {
        AtestOpCcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atest_dac_en(&self) -> AtestDacEnR {
        AtestDacEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atest_in_trx_sw(&self) -> AtestInTrxSwR {
        AtestInTrxSwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atest_in_en(&self) -> AtestInEnR {
        AtestInEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn atest_gain_r9(&self) -> AtestGainR9R {
        AtestGainR9R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn atest_gain_r8(&self) -> AtestGainR8R {
        AtestGainR8R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn atest_gain_r7(&self) -> AtestGainR7R {
        AtestGainR7R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn atest_gain_r6(&self) -> AtestGainR6R {
        AtestGainR6R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn atest_gain_r5(&self) -> AtestGainR5R {
        AtestGainR5R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn atest_out_en_q(&self) -> AtestOutEnQR {
        AtestOutEnQR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn atest_out_en_i(&self) -> AtestOutEnIR {
        AtestOutEnIR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn atest_in_en_q(&self) -> AtestInEnQR {
        AtestInEnQR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn atest_in_en_i(&self) -> AtestInEnIR {
        AtestInEnIR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn atest_op_cc(&mut self) -> AtestOpCcW<'_, TenAcSpec> {
        AtestOpCcW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atest_dac_en(&mut self) -> AtestDacEnW<'_, TenAcSpec> {
        AtestDacEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atest_in_trx_sw(&mut self) -> AtestInTrxSwW<'_, TenAcSpec> {
        AtestInTrxSwW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atest_in_en(&mut self) -> AtestInEnW<'_, TenAcSpec> {
        AtestInEnW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn atest_gain_r9(&mut self) -> AtestGainR9W<'_, TenAcSpec> {
        AtestGainR9W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn atest_gain_r8(&mut self) -> AtestGainR8W<'_, TenAcSpec> {
        AtestGainR8W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn atest_gain_r7(&mut self) -> AtestGainR7W<'_, TenAcSpec> {
        AtestGainR7W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn atest_gain_r6(&mut self) -> AtestGainR6W<'_, TenAcSpec> {
        AtestGainR6W::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn atest_gain_r5(&mut self) -> AtestGainR5W<'_, TenAcSpec> {
        AtestGainR5W::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn atest_out_en_q(&mut self) -> AtestOutEnQW<'_, TenAcSpec> {
        AtestOutEnQW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn atest_out_en_i(&mut self) -> AtestOutEnIW<'_, TenAcSpec> {
        AtestOutEnIW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn atest_in_en_q(&mut self) -> AtestInEnQW<'_, TenAcSpec> {
        AtestInEnQW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn atest_in_en_i(&mut self) -> AtestInEnIW<'_, TenAcSpec> {
        AtestInEnIW::new(self, 23)
    }
}
#[doc = "ac test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TenAcSpec;
impl crate::RegisterSpec for TenAcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ten_ac::R`](R) reader structure"]
impl crate::Readable for TenAcSpec {}
#[doc = "`write(|w| ..)` method takes [`ten_ac::W`](W) writer structure"]
impl crate::Writable for TenAcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ten_ac to value 0"]
impl crate::Resettable for TenAcSpec {}
