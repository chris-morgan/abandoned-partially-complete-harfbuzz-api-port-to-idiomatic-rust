These files are an incomplete port of the HarfBuzz API to idiomatic Rust, based on direct translation of `hb-*.h`; it expressly doesn’t use harfbuzz-sys. Translation was mostly manual, with plenty of regular expressions and Vim macros helping out.

I did this as part of an aborted attempt to make a text layout engine in Rust in late 2016. They are abandonware, as I hope that when I return to the project [skribo](https://github.com/linebender/skribo) will have it all covered.

My own part of this I dual license MIT/Apache-2.0, but given that the work is a direct derivative of the HarfBuzz header files it *may* be practically constrained to HarfBuzz’s license, MIT, despite the fact that it’s only the public API that actually ended up there in the end. Dunno. If you want to use it Apache-2.0, *cave quo vadis*.
