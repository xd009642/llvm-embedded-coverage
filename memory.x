MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 2M
  RAM (rwx) : ORIGIN = 0x200001B4, LENGTH = 320k - 0x1B4
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    __llvm_prf_names : 
    {
    
    } > RAM
}

