MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* NRF52832 with Softdevice S113 7.x */
  FLASH : ORIGIN = 0x00000000 + 0x1C000, LENGTH = 512K - 0x1C000
  RAM : ORIGIN = 0x20000000 + 0x6e28, LENGTH = 64K - 0x6e28
}
