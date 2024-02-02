#[doc = "Register `tzc_rom0_r1` reader"]
pub type R = crate::R<TzcRom0R1Spec>;
#[doc = "Register `tzc_rom0_r1` writer"]
pub type W = crate::W<TzcRom0R1Spec>;
#[doc = "Field `tzc_rom0_r1_end` reader - "]
pub type TzcRom0R1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom0_r1_end` writer - "]
pub type TzcRom0R1EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_rom0_r1_start` reader - "]
pub type TzcRom0R1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom0_r1_start` writer - "]
pub type TzcRom0R1StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r1_end(&self) -> TzcRom0R1EndR {
        TzcRom0R1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r1_start(&self) -> TzcRom0R1StartR {
        TzcRom0R1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r1_end(&mut self) -> TzcRom0R1EndW<'_, TzcRom0R1Spec> {
        TzcRom0R1EndW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r1_start(&mut self) -> TzcRom0R1StartW<'_, TzcRom0R1Spec> {
        TzcRom0R1StartW::new(self, 16)
    }
}
#[doc = "tzc_rom0_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom0_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom0_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRom0R1Spec;
impl crate::RegisterSpec for TzcRom0R1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom0_r1::R`](R) reader structure"]
impl crate::Readable for TzcRom0R1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom0_r1::W`](W) writer structure"]
impl crate::Writable for TzcRom0R1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tzc_rom0_r1 to value 0xffff"]
impl crate::Resettable for TzcRom0R1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
