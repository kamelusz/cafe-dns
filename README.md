## cafe-dns
Незательевая песочница для знакомства с Rust. Предназначена для реализации библиотеки умеющей в [rfc2782](https://tools.ietf.org/html/rfc2782) с последующей интеграцией в самописный [XMPP](https://tools.ietf.org/html/rfc6120)-клиент на C++.

На данный момент имеет базовый функционал для QTYPE A и SRV:
```
# ./target/debug/cafe-resolver --host _xmpp-client._tcp.jabber.ru --qtype SRV
_xmpp-client._tcp.jabber.ru: priority 10, weight 0, allports.jabber.ru:443
_xmpp-client._tcp.jabber.ru: priority 0, weight 0, jabber.ru:5222

# ./target/debug/cafe-resolver --host mail.ru --qtype A
mail.ru: 94.100.180.200
mail.ru: 217.69.139.202
mail.ru: 217.69.139.200
mail.ru: 94.100.180.201
```

### Ссылки на тему interop
* [How I Wrote a Modern C++ Library in Rust](https://hsivonen.fi/modern-cpp-in-rust/)
* [Rust/C++ interop in Firefox](https://firefox-source-docs.mozilla.org/writing-rust-code/ffi.html)
