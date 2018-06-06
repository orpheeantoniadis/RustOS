void kmain() {
    char hello[30] = "Hello World!";
    char color_byte = 0x1f;
    unsigned short *buffer_ptr = (unsigned short*)(0x000B8000 + 1988);
    
    for (int i = 0; i < 12; i++) {
        *buffer_ptr = (color_byte << 8) | hello[i];
        buffer_ptr += 1;
    }
    while(1);
}