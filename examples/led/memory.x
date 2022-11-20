MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* see https://github.com/0hotpotman0/BLE_52840_Core/blob/main/cores/nRF5/linker/nrf52840_s140_v7.ld */
  MBR                               : ORIGIN = 0x00000000, LENGTH = 4K
  SOFTDEVICE                        : ORIGIN = 0x00001000, LENGTH = 114688
  FLASH                             : ORIGIN = 0x00027000, LENGTH = 0xED000 - 0x27000
  RAM                               : ORIGIN = 0x20020000, LENGTH = 128K
}
