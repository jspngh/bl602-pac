#[doc = "Register `temp_comp` reader"]
pub type R = crate::R<TempCompSpec>;
#[doc = "Register `temp_comp` writer"]
pub type W = crate::W<TempCompSpec>;
#[doc = "Field `const_acal` reader - "]
pub type ConstAcalR = crate::FieldReader;
#[doc = "Field `const_acal` writer - "]
pub type ConstAcalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `const_fcal` reader - "]
pub type ConstFcalR = crate::FieldReader;
#[doc = "Field `const_fcal` writer - "]
pub type ConstFcalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `temp_comp_en` reader - "]
pub type TempCompEnR = crate::BitReader;
#[doc = "Field `temp_comp_en` writer - "]
pub type TempCompEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&self) -> ConstAcalR {
        ConstAcalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&self) -> ConstFcalR {
        ConstFcalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&self) -> TempCompEnR {
        TempCompEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&mut self) -> ConstAcalW<'_, TempCompSpec> {
        ConstAcalW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&mut self) -> ConstFcalW<'_, TempCompSpec> {
        ConstFcalW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&mut self) -> TempCompEnW<'_, TempCompSpec> {
        TempCompEnW::new(self, 16)
    }
}
#[doc = "temp_comp.\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempCompSpec;
impl crate::RegisterSpec for TempCompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp_comp::R`](R) reader structure"]
impl crate::Readable for TempCompSpec {}
#[doc = "`write(|w| ..)` method takes [`temp_comp::W`](W) writer structure"]
impl crate::Writable for TempCompSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets temp_comp to value 0"]
impl crate::Resettable for TempCompSpec {}
