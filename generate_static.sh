#!/bin/bash

sed -i '/<body>/,$d' index.html
printf "<body>\n" >> index.html
cargo run --bin hydrated --target x86_64-unknown-linux-gnu -F ssr 2>/dev/null >> index.html
printf "</body>\n\n</html>\n" >> index.html
