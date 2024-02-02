#[doc = "Register `tzc_rom0_r1` reader"]
pub type R = crate::R<TzcRom0R1Spec>;
#[doc = "Field `tzc_rom0_r1_end` reader - "]
pub type TzcRom0R1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom0_r1_start` reader - "]
pub type TzcRom0R1StartR = crate::FieldReader<u16>;
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
#[doc = "tzc_rom0_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom0_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRom0R1Spec;
impl crate::RegisterSpec for TzcRom0R1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom0_r1::R`](R) reader structure"]
impl crate::Readable for TzcRom0R1Spec {}
#[doc = "`reset()` method sets tzc_rom0_r1 to value 0xffff"]
impl crate::Resettable for TzcRom0R1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
