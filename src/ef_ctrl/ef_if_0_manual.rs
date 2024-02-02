#[doc = "Register `ef_if_0_manual` reader"]
pub type R = crate::R<EfIf0ManualSpec>;
#[doc = "Register `ef_if_0_manual` writer"]
pub type W = crate::W<EfIf0ManualSpec>;
#[doc = "Field `ef_if_a` reader - "]
pub type EfIfAR = crate::FieldReader<u16>;
#[doc = "Field `ef_if_a` writer - "]
pub type EfIfAW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ef_if_pd` reader - "]
pub type EfIfPdR = crate::BitReader;
#[doc = "Field `ef_if_pd` writer - "]
pub type EfIfPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_ps` reader - "]
pub type EfIfPsR = crate::BitReader;
#[doc = "Field `ef_if_ps` writer - "]
pub type EfIfPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_strobe` reader - "]
pub type EfIfStrobeR = crate::BitReader;
#[doc = "Field `ef_if_strobe` writer - "]
pub type EfIfStrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_pgenb` reader - "]
pub type EfIfPgenbR = crate::BitReader;
#[doc = "Field `ef_if_pgenb` writer - "]
pub type EfIfPgenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_load` reader - "]
pub type EfIfLoadR = crate::BitReader;
#[doc = "Field `ef_if_load` writer - "]
pub type EfIfLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_csb` reader - "]
pub type EfIfCsbR = crate::BitReader;
#[doc = "Field `ef_if_csb` writer - "]
pub type EfIfCsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_q` reader - "]
pub type EfIf0QR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_manual` reader - "]
pub type EfIfProtCodeManualR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_manual` writer - "]
pub type EfIfProtCodeManualW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&self) -> EfIfAR {
        EfIfAR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&self) -> EfIfPdR {
        EfIfPdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&self) -> EfIfPsR {
        EfIfPsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&self) -> EfIfStrobeR {
        EfIfStrobeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&self) -> EfIfPgenbR {
        EfIfPgenbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&self) -> EfIfLoadR {
        EfIfLoadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&self) -> EfIfCsbR {
        EfIfCsbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ef_if_0_q(&self) -> EfIf0QR {
        EfIf0QR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&self) -> EfIfProtCodeManualR {
        EfIfProtCodeManualR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&mut self) -> EfIfAW<'_, EfIf0ManualSpec> {
        EfIfAW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&mut self) -> EfIfPdW<'_, EfIf0ManualSpec> {
        EfIfPdW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&mut self) -> EfIfPsW<'_, EfIf0ManualSpec> {
        EfIfPsW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&mut self) -> EfIfStrobeW<'_, EfIf0ManualSpec> {
        EfIfStrobeW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&mut self) -> EfIfPgenbW<'_, EfIf0ManualSpec> {
        EfIfPgenbW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&mut self) -> EfIfLoadW<'_, EfIf0ManualSpec> {
        EfIfLoadW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&mut self) -> EfIfCsbW<'_, EfIf0ManualSpec> {
        EfIfCsbW::new(self, 15)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&mut self) -> EfIfProtCodeManualW<'_, EfIf0ManualSpec> {
        EfIfProtCodeManualW::new(self, 24)
    }
}
#[doc = "ef_if_0_manual.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_0_manual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_0_manual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIf0ManualSpec;
impl crate::RegisterSpec for EfIf0ManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_0_manual::R`](R) reader structure"]
impl crate::Readable for EfIf0ManualSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_0_manual::W`](W) writer structure"]
impl crate::Writable for EfIf0ManualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_0_manual to value 0xe400"]
impl crate::Resettable for EfIf0ManualSpec {
    const RESET_VALUE: u32 = 0xe400;
}
