# rustberry

rustberry is a bare-metal kernel bootable on the [Raspberry Pi](https://www.raspberrypi.org/).

## How to build and run it
We first need a ARM compatible rust compiler so let's compile our own.
This was conducted on an Debian Linux machine.

1. Make sure you have installed the dependencies:

   * `g++` 4.7 or `clang++` 3.x
   * `python` 2.6 or later (but not 3.x)
   * GNU `make` 3.81 or later
   * `curl`
   * `git`
   * ARM toolchains `gcc-4.4-arm-linux-gnueabihf` and `gcc-arm-none-eabi`

2. Clone the rust sources and compile

   ```
   $ git clone https://github.com/rust-lang/rust.git
   $ cd rust
   $ ./configure --target=arm-unknown-linux-gnueabihf,x86_64-unknown-linux-gnu --release-channel=nightly
   $ make
   $ sudo make install
   ```

3. Clone the rustberry sources and compile

   ```
   $ git clone https://github.com/CTerasa/rustberry.git
   $ cd rustberry
   $ make
   ```
   Now you should have a bootable `kernel.img` in the `images` subdirectory.

4. Install on SD-Card

   To run your kernel we need some extra files on the SD-Card. 
   These are found under https://github.com/raspberrypi/firmware/tree/master/boot. 
   We only need the files not the whole repo. 
   Download the following files:
   * `start.elf`
   * `bootcode.bin`
   Format your micro-SD-Card with a fat32 filesystem and copy these files onto it.
   Also copy the `kernel.img` to the card.

5. Have fun

## Notes

   As mentioned earlier this was conducted on a x64 Debian machine.
   For refernce the ruste version was c85f30736913cf42549d8e0fd40049b346b4cec4.
   
## Credits

   * Bare-Metal code sample https://github.com/neykov/armboot
   * Bootcode and Raspberry Pi samples https://github.com/dwelch67/raspberrypi





