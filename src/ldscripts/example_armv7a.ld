MEMORY {
    /* 16MB SPI ROM */
    rom : ORIGIN = 0, LENGTH = 16M

    /* 512MB DDR RAM */
    ram : org = 0x00100000, l = 512M
}
SECTIONS {
    . = 0;
    .vector_table ORIGIN(rom) : {
        KEEP(*(.vector_table));
    } > rom
    . = 0x20;
    .text : {
        *(.text)
    } > rom

    /DISCARD/ : {
        *(.ARM.exidx)
    } 
}