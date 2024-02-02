#[doc = "Register `CH_CONTROL` reader"]
pub type R = crate::R<ChControlSpec>;
#[doc = "Register `CH_CONTROL` writer"]
pub type W = crate::W<ChControlSpec>;
#[doc = "Field `TransferSize` reader - "]
pub type TransferSizeR = crate::FieldReader<u16>;
#[doc = "Field `TransferSize` writer - "]
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SBSize` reader - "]
pub type SbsizeR = crate::FieldReader;
#[doc = "Field `SBSize` writer - "]
pub type SbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBSize` reader - "]
pub type DbsizeR = crate::FieldReader;
#[doc = "Field `DBSize` writer - "]
pub type DbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWidth` reader - "]
pub type SwidthR = crate::FieldReader;
#[doc = "Field `SWidth` writer - "]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWidth` reader - "]
pub type DwidthR = crate::FieldReader;
#[doc = "Field `DWidth` writer - "]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLargerD` reader - "]
pub type SlargerDR = crate::BitReader;
#[doc = "Field `SLargerD` writer - "]
pub type SlargerDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SI` reader - "]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - "]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI` reader - "]
pub type DiR = crate::BitReader;
#[doc = "Field `DI` writer - "]
pub type DiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Prot` reader - "]
pub type ProtR = crate::FieldReader;
#[doc = "Field `Prot` writer - "]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I` reader - "]
pub type IR = crate::BitReader;
#[doc = "Field `I` writer - "]
pub type IW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&self) -> SbsizeR {
        SbsizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&self) -> DbsizeR {
        DbsizeR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slarger_d(&self) -> SlargerDR {
        SlargerDR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TransferSizeW<'_, ChControlSpec> {
        TransferSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SbsizeW<'_, ChControlSpec> {
        SbsizeW::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DbsizeW<'_, ChControlSpec> {
        DbsizeW::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SwidthW<'_, ChControlSpec> {
        SwidthW::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<'_, ChControlSpec> {
        DwidthW::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slarger_d(&mut self) -> SlargerDW<'_, ChControlSpec> {
        SlargerDW::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, ChControlSpec> {
        SiW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, ChControlSpec> {
        DiW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, ChControlSpec> {
        ProtW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, ChControlSpec> {
        IW::new(self, 31)
    }
}
#[doc = "DMA channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChControlSpec;
impl crate::RegisterSpec for ChControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_control::R`](R) reader structure"]
impl crate::Readable for ChControlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_control::W`](W) writer structure"]
impl crate::Writable for ChControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CONTROL to value 0x0c48_9000"]
impl crate::Resettable for ChControlSpec {
    const RESET_VALUE: u32 = 0x0c48_9000;
}
