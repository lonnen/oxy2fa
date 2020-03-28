# oxy2fa

[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/tmd5xwpm9cg5vr4k/branch/master?svg=true)](https://ci.appveyor.com/project/lonnen/oxy2fa/branch/master)
[![TravisCI Build Status](https://www.travis-ci.com/lonnen/oxy2fa.svg?branch=master)](https://www.travis-ci.com/lonnen/oxy2fa)


A two-factor authentication agent.

Unlike QR based software, this relies on text-based codes. When prompted with a QR code, look for an option to use a text code with you two-factor secret instead, which will be a short string of numbers and letters.

In order to effectively generate time-based (TOTP) codes, which are the default, the system clock must be accurate within 60s. The keychain is stored unencrypted in the text file `$HOME/.oxy2fa`.

Usage:

    $ oxy2fa tokens
    970914 name1

    $ oxy2fa add name2 ksjfu82hrasdf434
    Added 2fa key for name2

    $ oxy2fa tokens name2
    449903 name2

    $ oxy2fa tokens
    970914 name1
    943215 name2

* [Documentation](/)
* [Release Notes](/)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.