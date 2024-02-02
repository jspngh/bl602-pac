#[doc = "Register `singen_ctrl4` reader"]
pub type R = crate::R<SingenCtrl4Spec>;
#[doc = "Register `singen_ctrl4` writer"]
pub type W = crate::W<SingenCtrl4Spec>;
#[doc = "Field `singen_fix_q` reader - "]
pub type SingenFixQR = crate::FieldReader<u16>;
#[doc = "Field `singen_fix_q` writer - "]
pub type SingenFixQW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `singen_fix_en_q` reader - "]
pub type SingenFixEnQR = crate::BitReader;
#[doc = "Field `singen_fix_en_q` writer - "]
pub type SingenFixEnQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `singen_fix_i` reader - "]
pub type SingenFixIR = crate::FieldReader<u16>;
#[doc = "Field `singen_fix_i` writer - "]
pub type SingenFixIW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `singen_fix_en_i` reader - "]
pub type SingenFixEnIR = crate::BitReader;
#[doc = "Field `singen_fix_en_i` writer - "]
pub type SingenFixEnIW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&self) -> SingenFixQR {
        SingenFixQR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&self) -> SingenFixEnQR {
        SingenFixEnQR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&self) -> SingenFixIR {
        SingenFixIR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&self) -> SingenFixEnIR {
        SingenFixEnIR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&mut self) -> SingenFixQW<'_, SingenCtrl4Spec> {
        SingenFixQW::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&mut self) -> SingenFixEnQW<'_, SingenCtrl4Spec> {
        SingenFixEnQW::new(self, 12)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&mut self) -> SingenFixIW<'_, SingenCtrl4Spec> {
        SingenFixIW::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&mut self) -> SingenFixEnIW<'_, SingenCtrl4Spec> {
        SingenFixEnIW::new(self, 28)
    }
}
#[doc = "singen_ctrl4.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenCtrl4Spec;
impl crate::RegisterSpec for SingenCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ctrl4::R`](R) reader structure"]
impl crate::Readable for SingenCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`singen_ctrl4::W`](W) writer structure"]
impl crate::Writable for SingenCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets singen_ctrl4 to value 0"]
impl crate::Resettable for SingenCtrl4Spec {}
