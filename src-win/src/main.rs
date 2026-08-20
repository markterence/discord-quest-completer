// Ce fichier est vide car toute la logique est dans main.cpp
// Le point d'entrée réel est défini par /ENTRY:mainEntryPoint dans build.rs
#![no_main]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}