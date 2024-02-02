#[doc = "Register `tzc_rom_ctrl` reader"]
pub type R = crate::R<TzcRomCtrlSpec>;
#[doc = "Register `tzc_rom_ctrl` writer"]
pub type W = crate::W<TzcRomCtrlSpec>;
#[doc = "Field `tzc_rom0_r0_id0_en` reader - "]
pub type TzcRom0R0Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_id0_en` writer - "]
pub type TzcRom0R0Id0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r1_id0_en` reader - "]
pub type TzcRom0R1Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_id0_en` writer - "]
pub type TzcRom0R1Id0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r0_id0_en` reader - "]
pub type TzcRom1R0Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_id0_en` writer - "]
pub type TzcRom1R0Id0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r1_id0_en` reader - "]
pub type TzcRom1R1Id0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_id0_en` writer - "]
pub type TzcRom1R1Id0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r0_id1_en` reader - "]
pub type TzcRom0R0Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_id1_en` writer - "]
pub type TzcRom0R0Id1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r1_id1_en` reader - "]
pub type TzcRom0R1Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_id1_en` writer - "]
pub type TzcRom0R1Id1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r0_id1_en` reader - "]
pub type TzcRom1R0Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_id1_en` writer - "]
pub type TzcRom1R0Id1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r1_id1_en` reader - "]
pub type TzcRom1R1Id1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_id1_en` writer - "]
pub type TzcRom1R1Id1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r0_en` reader - "]
pub type TzcRom0R0EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_en` writer - "]
pub type TzcRom0R0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r1_en` reader - "]
pub type TzcRom0R1EnR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_en` writer - "]
pub type TzcRom0R1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r0_en` reader - "]
pub type TzcRom1R0EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_en` writer - "]
pub type TzcRom1R0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r1_en` reader - "]
pub type TzcRom1R1EnR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_en` writer - "]
pub type TzcRom1R1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r0_lock` reader - "]
pub type TzcRom0R0LockR = crate::BitReader;
#[doc = "Field `tzc_rom0_r0_lock` writer - "]
pub type TzcRom0R0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom0_r1_lock` reader - "]
pub type TzcRom0R1LockR = crate::BitReader;
#[doc = "Field `tzc_rom0_r1_lock` writer - "]
pub type TzcRom0R1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r0_lock` reader - "]
pub type TzcRom1R0LockR = crate::BitReader;
#[doc = "Field `tzc_rom1_r0_lock` writer - "]
pub type TzcRom1R0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom1_r1_lock` reader - "]
pub type TzcRom1R1LockR = crate::BitReader;
#[doc = "Field `tzc_rom1_r1_lock` writer - "]
pub type TzcRom1R1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sboot_done` reader - "]
pub type TzcSbootDoneR = crate::FieldReader;
#[doc = "Field `tzc_sboot_done` writer - "]
pub type TzcSbootDoneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&mut self) -> TzcRom0R0Id0EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R0Id0EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&mut self) -> TzcRom0R1Id0EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R1Id0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&mut self) -> TzcRom1R0Id0EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R0Id0EnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&mut self) -> TzcRom1R1Id0EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R1Id0EnW::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&mut self) -> TzcRom0R0Id1EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R0Id1EnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&mut self) -> TzcRom0R1Id1EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R1Id1EnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&mut self) -> TzcRom1R0Id1EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R0Id1EnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&mut self) -> TzcRom1R1Id1EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R1Id1EnW::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&mut self) -> TzcRom0R0EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R0EnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&mut self) -> TzcRom0R1EnW<'_, TzcRomCtrlSpec> {
        TzcRom0R1EnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&mut self) -> TzcRom1R0EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R0EnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&mut self) -> TzcRom1R1EnW<'_, TzcRomCtrlSpec> {
        TzcRom1R1EnW::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&mut self) -> TzcRom0R0LockW<'_, TzcRomCtrlSpec> {
        TzcRom0R0LockW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&mut self) -> TzcRom0R1LockW<'_, TzcRomCtrlSpec> {
        TzcRom0R1LockW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&mut self) -> TzcRom1R0LockW<'_, TzcRomCtrlSpec> {
        TzcRom1R0LockW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&mut self) -> TzcRom1R1LockW<'_, TzcRomCtrlSpec> {
        TzcRom1R1LockW::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&mut self) -> TzcSbootDoneW<'_, TzcRomCtrlSpec> {
        TzcSbootDoneW::new(self, 28)
    }
}
#[doc = "tzc_rom_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomCtrlSpec;
impl crate::RegisterSpec for TzcRomCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcRomCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcRomCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tzc_rom_ctrl to value 0x0f0f"]
impl crate::Resettable for TzcRomCtrlSpec {
    const RESET_VALUE: u32 = 0x0f0f;
}
