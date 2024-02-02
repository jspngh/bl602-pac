#[doc = "Register `tzc_rom1_r1` reader"]
pub type R = crate::R<TzcRom1R1Spec>;
#[doc = "Register `tzc_rom1_r1` writer"]
pub type W = crate::W<TzcRom1R1Spec>;
#[doc = "Field `tzc_rom1_r1_end` reader - "]
pub type TzcRom1R1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom1_r1_end` writer - "]
pub type TzcRom1R1EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_rom1_r1_start` reader - "]
pub type TzcRom1R1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom1_r1_start` writer - "]
pub type TzcRom1R1StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom1_r1_end(&self) -> TzcRom1R1EndR {
        TzcRom1R1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom1_r1_start(&self) -> TzcRom1R1StartR {
        TzcRom1R1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom1_r1_end(&mut self) -> TzcRom1R1EndW<'_, TzcRom1R1Spec> {
        TzcRom1R1EndW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom1_r1_start(&mut self) -> TzcRom1R1StartW<'_, TzcRom1R1Spec> {
        TzcRom1R1StartW::new(self, 16)
    }
}
#[doc = "tzc_rom1_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom1_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom1_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRom1R1Spec;
impl crate::RegisterSpec for TzcRom1R1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom1_r1::R`](R) reader structure"]
impl crate::Readable for TzcRom1R1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom1_r1::W`](W) writer structure"]
impl crate::Writable for TzcRom1R1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tzc_rom1_r1 to value 0xffff"]
impl crate::Resettable for TzcRom1R1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
