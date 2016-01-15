pub struct IORegister8(pub u32);
pub struct IORegister16(pub u32);
pub struct IORegister32(pub u32);

pub const POSTFLG: IORegister8 = IORegister8(0x4000300);
pub const HALTCNT: IORegister8 = IORegister8(0x4000301);

pub const DISPCNT: IORegister16 = IORegister16(0x4000000);
pub const DISPSTAT: IORegister16 = IORegister16(0x4000004);
pub const VCOUNT: IORegister16 = IORegister16(0x4000006);
pub const BG0CNT: IORegister16 = IORegister16(0x4000008);
pub const BG1CNT: IORegister16 = IORegister16(0x400000a);
pub const BG2CNT: IORegister16 = IORegister16(0x400000c);
pub const BG3CNT: IORegister16 = IORegister16(0x400000e);
pub const BG0HOFS: IORegister16 = IORegister16(0x4000010);
pub const BG0VOFS: IORegister16 = IORegister16(0x4000012);
pub const BG1HOFS: IORegister16 = IORegister16(0x4000014);
pub const BG1VOFS: IORegister16 = IORegister16(0x4000016);
pub const BG2HOFS: IORegister16 = IORegister16(0x4000018);
pub const BG2VOFS: IORegister16 = IORegister16(0x400001a);
pub const BG3HOFS: IORegister16 = IORegister16(0x400001c);
pub const BG3VOFS: IORegister16 = IORegister16(0x400001e);
pub const BG2PA: IORegister16 = IORegister16(0x4000020);
pub const BG2PB: IORegister16 = IORegister16(0x4000022);
pub const BG2PC: IORegister16 = IORegister16(0x4000024);
pub const BG2PD: IORegister16 = IORegister16(0x4000026);
pub const BG3PA: IORegister16 = IORegister16(0x4000030);
pub const BG3PB: IORegister16 = IORegister16(0x4000032);
pub const BG3PC: IORegister16 = IORegister16(0x4000034);
pub const BG3PD: IORegister16 = IORegister16(0x4000036);
pub const WIN0H: IORegister16 = IORegister16(0x4000040);
pub const WIN1H: IORegister16 = IORegister16(0x4000042);
pub const WIN0V: IORegister16 = IORegister16(0x4000044);
pub const WIN1V: IORegister16 = IORegister16(0x4000046);
pub const WININ: IORegister16 = IORegister16(0x4000048);
pub const WINOUT: IORegister16 = IORegister16(0x400004a);
pub const MOSAIC: IORegister16 = IORegister16(0x400004c);
pub const BLDCNT: IORegister16 = IORegister16(0x4000050);
pub const BLDALPHA: IORegister16 = IORegister16(0x4000052);
pub const BLDY: IORegister16 = IORegister16(0x4000054);
pub const SOUND1CNT_L: IORegister16 = IORegister16(0x4000060);
pub const SOUND1CNT_H: IORegister16 = IORegister16(0x4000062);
pub const SOUND1CNT_X: IORegister16 = IORegister16(0x4000064);
pub const SOUND2CNT_L: IORegister16 = IORegister16(0x4000068);
pub const SOUND2CNT_H: IORegister16 = IORegister16(0x400006c);
pub const SOUND3CNT_L: IORegister16 = IORegister16(0x4000070);
pub const SOUND3CNT_H: IORegister16 = IORegister16(0x4000072);
pub const SOUND3CNT_X: IORegister16 = IORegister16(0x4000074);
pub const SOUND4CNT_L: IORegister16 = IORegister16(0x4000078);
pub const SOUND4CNT_H: IORegister16 = IORegister16(0x400007c);
pub const SOUNDCNT_L: IORegister16 = IORegister16(0x4000080);
pub const SOUNDCNT_H: IORegister16 = IORegister16(0x4000082);
pub const SOUNDCNT_X: IORegister16 = IORegister16(0x4000084);
pub const SOUNDBIAS: IORegister16 = IORegister16(0x4000088);
pub const WAVE_RAM0_L: IORegister16 = IORegister16(0x4000090);
pub const WAVE_RAM0_H: IORegister16 = IORegister16(0x4000092);
pub const WAVE_RAM1_L: IORegister16 = IORegister16(0x4000094);
pub const WAVE_RAM1_H: IORegister16 = IORegister16(0x4000096);
pub const WAVE_RAM2_L: IORegister16 = IORegister16(0x4000098);
pub const WAVE_RAM2_H: IORegister16 = IORegister16(0x400009a);
pub const WAVE_RAM3_L: IORegister16 = IORegister16(0x400009c);
pub const WAVE_RAM3_H: IORegister16 = IORegister16(0x400009e);
pub const FIF0_A_L: IORegister16 = IORegister16(0x40000a0);
pub const FIFO_A_H: IORegister16 = IORegister16(0x40000a2);
pub const FIFO_B_L: IORegister16 = IORegister16(0x40000a4);
pub const FIFO_B_H: IORegister16 = IORegister16(0x40000a6);
pub const DMA0CNT_L: IORegister16 = IORegister16(0x40000b8);
pub const DMA0CNT_H: IORegister16 = IORegister16(0x40000ba);
pub const DMA1CNT_L: IORegister16 = IORegister16(0x40000c4);
pub const DMA1CNT_H: IORegister16 = IORegister16(0x40000c6);
pub const DMA2CNT_L: IORegister16 = IORegister16(0x40000d0);
pub const DMA2CNT_H: IORegister16 = IORegister16(0x40000d2);
pub const DMA3CNT_L: IORegister16 = IORegister16(0x40000dc);
pub const DMA3CNT_H: IORegister16 = IORegister16(0x40000de);
pub const TM0CNT_L: IORegister16 = IORegister16(0x4000100);
pub const TM0CNT_H: IORegister16 = IORegister16(0x4000102);
pub const TM1CNT_L: IORegister16 = IORegister16(0x4000104);
pub const TM1CNT_H: IORegister16 = IORegister16(0x4000106);
pub const TM2CNT_L: IORegister16 = IORegister16(0x4000108);
pub const TM2CNT_H: IORegister16 = IORegister16(0x400010a);
pub const TM3CNT_L: IORegister16 = IORegister16(0x400010c);
pub const TM3CNT_H: IORegister16 = IORegister16(0x400010e);
pub const SIOMULTI0: IORegister16 = IORegister16(0x4000120);
pub const SIOMULTI1: IORegister16 = IORegister16(0x4000122);
pub const SIOMULTI2: IORegister16 = IORegister16(0x4000124);
pub const SIOMULTI3: IORegister16 = IORegister16(0x4000126);
pub const SIOCNT: IORegister16 = IORegister16(0x4000128);
pub const SIOMLT_SEND: IORegister16 = IORegister16(0x400012a);
pub const KEYINPUT: IORegister16 = IORegister16(0x4000130);
pub const KEYCNT: IORegister16 = IORegister16(0x4000132);
pub const RCNT: IORegister16 = IORegister16(0x4000134);
pub const IR: IORegister16 = IORegister16(0x4000136);
pub const JOYCNT: IORegister16 = IORegister16(0x4000140);
pub const JOY_STAT: IORegister16 = IORegister16(0x4000158);
pub const IE: IORegister16 = IORegister16(0x4000200);
pub const IF: IORegister16 = IORegister16(0x4000202);
pub const WAITCNT: IORegister16 = IORegister16(0x4000204);
pub const IME: IORegister16 = IORegister16(0x4000208);

pub const BG2X: IORegister32 = IORegister32(0x4000028);
pub const BG2Y: IORegister32 = IORegister32(0x400002c);
pub const BG3X: IORegister32 = IORegister32(0x4000038);
pub const BG3Y: IORegister32 = IORegister32(0x400003c);
pub const FIFO_A: IORegister32 = IORegister32(0x40000a0);
pub const FIFO_B: IORegister32 = IORegister32(0x40000a4);
pub const DMA0SAD: IORegister32 = IORegister32(0x40000b0);
pub const DMA0DAD: IORegister32 = IORegister32(0x40000b4);
pub const DMA1SAD: IORegister32 = IORegister32(0x40000bc);
pub const DMA1DAD: IORegister32 = IORegister32(0x40000c0);
pub const DMA2SAD: IORegister32 = IORegister32(0x40000c8);
pub const DMA2DAD: IORegister32 = IORegister32(0x40000cc);
pub const DMA3SAD: IORegister32 = IORegister32(0x40000d4);
pub const DMA3DAD: IORegister32 = IORegister32(0x40000d8);
pub const SIODATA32: IORegister32 = IORegister32(0x4000120);
pub const JOY_RECV: IORegister32 = IORegister32(0x4000150);
pub const JOY_TRANS: IORegister32 = IORegister32(0x4000154);
