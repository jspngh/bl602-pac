#[doc = "Register `tzc_rom_ctrl` reader"]
pub type R = crate::R<TzcRomCtrlSpec>;
#[doc = "Field `tzc_rom0_r0_id0_en` reader - "]
pub type TzcRom0R0Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_id0_en` reader - "]
pub type TzcRom0R1Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_id0_en` reader - "]
pub type TzcRom1R0Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_id0_en` reader - "]
pub type TzcRom1R1Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_id1_en` reader - "]
pub type TzcRom0R0Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_id1_en` reader - "]
pub type TzcRom0R1Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_id1_en` reader - "]
pub type TzcRom1R0Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_id1_en` reader - "]
pub type TzcRom1R1Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_en` reader - "]
pub type TzcRom0R0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_en` reader - "]
pub type TzcRom0R1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_en` reader - "]
pub type TzcRom1R0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_en` reader - "]
pub type TzcRom1R1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_lock` reader - "]
pub type TzcRom0R0LockR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_lock` reader - "]
pub type TzcRom0R1LockR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_lock` reader - "]
pub type TzcRom1R0LockR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_lock` reader - "]
pub type TzcRom1R1LockR = crate::BitReader;
#[doc = "Field `tzc_sboot_done` reader - "]
pub type TzcSbootDoneR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&self) -> TzcRom0R0Id0EnR {
        TzcRom0R0Id0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&self) -> TzcRom0R1Id0EnR {
        TzcRom0R1Id0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&self) -> TzcRom1R0Id0EnR {
        TzcRom1R0Id0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&self) -> TzcRom1R1Id0EnR {
        TzcRom1R1Id0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&self) -> TzcRom0R0Id1EnR {
        TzcRom0R0Id1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&self) -> TzcRom0R1Id1EnR {
        TzcRom0R1Id1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&self) -> TzcRom1R0Id1EnR {
        TzcRom1R0Id1EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&self) -> TzcRom1R1Id1EnR {
        TzcRom1R1Id1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&self) -> TzcRom0R0EnR {
        TzcRom0R0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&self) -> TzcRom0R1EnR {
        TzcRom0R1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&self) -> TzcRom1R0EnR {
        TzcRom1R0EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&self) -> TzcRom1R1EnR {
        TzcRom1R1EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&self) -> TzcRom0R0LockR {
        TzcRom0R0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&self) -> TzcRom0R1LockR {
        TzcRom0R1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&self) -> TzcRom1R0LockR {
        TzcRom1R0LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&self) -> TzcRom1R1LockR {
        TzcRom1R1LockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TzcSbootDoneR {
        TzcSbootDoneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "tzc_rom_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomCtrlSpec;
impl crate::RegisterSpec for TzcRomCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcRomCtrlSpec {}
#[doc = "`reset()` method sets tzc_rom_ctrl to value 0x0f0f"]
impl crate::Resettable for TzcRomCtrlSpec {
    const RESET_VALUE: u32 = 0x0f0f;
}
