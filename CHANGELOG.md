# 1.0.0 (2026-03-25)


### Bug Fixes

* handle real API response shapes ([d8440c2](https://github.com/orangerabbit-io/forwardemail/commit/d8440c2afb3323ae6c5213df2defc769ade85984))
* update flake package version to 1.0.0 ([de20a43](https://github.com/orangerabbit-io/forwardemail/commit/de20a43046c40f689f30c50d676eba1eed457473))


### Features

* add account model with row type and serde tests ([32ca949](https://github.com/orangerabbit-io/forwardemail/commit/32ca9495ff2e43a1328eb35cbff31c79a31ef4f2))
* add alias model with row type and serde tests ([dfb9421](https://github.com/orangerabbit-io/forwardemail/commit/dfb942121c6d5292a883ff56616eaed5aba9f146))
* add CLI structure with clap subcommands and lazy auth dispatch ([2b05fb0](https://github.com/orangerabbit-io/forwardemail/commit/2b05fb04332c8a918bd60d9538fd74f1e139c303))
* add config module with 3-tier API key resolution ([65b1f73](https://github.com/orangerabbit-io/forwardemail/commit/65b1f73accc022ec78287f18e47c179522005b98))
* add domain model with row type and serde tests ([56e3487](https://github.com/orangerabbit-io/forwardemail/commit/56e348776a37e9a689ae49f5565ddb87893c361c))
* add email model with EmailLimit and serde tests ([2c0a336](https://github.com/orangerabbit-io/forwardemail/commit/2c0a336b7b4abebc00c68ce2ebec83066671505b))
* add HTTP client with Basic Auth and unauthenticated mode ([b3df86a](https://github.com/orangerabbit-io/forwardemail/commit/b3df86a25b579d4c4098668fae7fdc0128b9d802))
* add invite, member, catch-all password, encrypt, and log models ([e7c5811](https://github.com/orangerabbit-io/forwardemail/commit/e7c5811d75a837d0203ee7693370fdce7ec57476))
* add output module with markdown table, JSON, KV formatting ([133f96d](https://github.com/orangerabbit-io/forwardemail/commit/133f96d11e341364c5b387a900fde31b62c7d296))
* add TUI app state machine with navigation and event loop ([d6df788](https://github.com/orangerabbit-io/forwardemail/commit/d6df7880c81d1e14d9c5666489c0a3afb0195cc0))
* add TUI views with dashboard, domain/alias/email browsing ([b44ce0c](https://github.com/orangerabbit-io/forwardemail/commit/b44ce0cf3023105e76162dfbc8fff69d6e91f747))
* implement account commands (create, get, update) ([8893d75](https://github.com/orangerabbit-io/forwardemail/commit/8893d756fc1110efb9cf48a0e5ebdfe1055c2446))
* implement alias commands (list, CRUD, generate-password) ([35eb287](https://github.com/orangerabbit-io/forwardemail/commit/35eb287c6b8c165f708c8ef3ac68055677b9d54f))
* implement catch-all password commands (list, create, delete) ([b8c8948](https://github.com/orangerabbit-io/forwardemail/commit/b8c89486fbfb4257955efc484677d93d4d0360a3))
* implement domain commands (list, CRUD, verify) ([cdec331](https://github.com/orangerabbit-io/forwardemail/commit/cdec3315bc73f3ce943d24831efadcd63dce06ce))
* implement email commands (list, send, get, delete, limit) ([a9f2189](https://github.com/orangerabbit-io/forwardemail/commit/a9f2189c7c7946fe8e1edf9a0d08aebb86e05b40))
* implement encrypt command with unauthenticated client ([67d9f58](https://github.com/orangerabbit-io/forwardemail/commit/67d9f58644f75bdabca5b8ca05c4518fa96e83a0))
* implement invite commands (create, accept, remove) ([5840b58](https://github.com/orangerabbit-io/forwardemail/commit/5840b58da4c169fd28768db951ff734a7cd52e02))
* implement log download with gzip decompression ([e58ae68](https://github.com/orangerabbit-io/forwardemail/commit/e58ae68ddc1d59e428d535d92d989a234dbfea46))
* implement member commands (update, remove) ([99d8a67](https://github.com/orangerabbit-io/forwardemail/commit/99d8a67bfd899810c4db0a0fe67531d7cdffea99))
* scaffold workspace with forwardemail-lib and forwardemail crates ([8122756](https://github.com/orangerabbit-io/forwardemail/commit/8122756d8919db7c23166454a703b61a73575ce6))

# 1.0.0 (2026-03-25)


### Bug Fixes

* handle real API response shapes ([d8440c2](https://github.com/orangerabbit-io/forwardemail/commit/d8440c2afb3323ae6c5213df2defc769ade85984))


### Features

* add account model with row type and serde tests ([32ca949](https://github.com/orangerabbit-io/forwardemail/commit/32ca9495ff2e43a1328eb35cbff31c79a31ef4f2))
* add alias model with row type and serde tests ([dfb9421](https://github.com/orangerabbit-io/forwardemail/commit/dfb942121c6d5292a883ff56616eaed5aba9f146))
* add CLI structure with clap subcommands and lazy auth dispatch ([2b05fb0](https://github.com/orangerabbit-io/forwardemail/commit/2b05fb04332c8a918bd60d9538fd74f1e139c303))
* add config module with 3-tier API key resolution ([65b1f73](https://github.com/orangerabbit-io/forwardemail/commit/65b1f73accc022ec78287f18e47c179522005b98))
* add domain model with row type and serde tests ([56e3487](https://github.com/orangerabbit-io/forwardemail/commit/56e348776a37e9a689ae49f5565ddb87893c361c))
* add email model with EmailLimit and serde tests ([2c0a336](https://github.com/orangerabbit-io/forwardemail/commit/2c0a336b7b4abebc00c68ce2ebec83066671505b))
* add HTTP client with Basic Auth and unauthenticated mode ([b3df86a](https://github.com/orangerabbit-io/forwardemail/commit/b3df86a25b579d4c4098668fae7fdc0128b9d802))
* add invite, member, catch-all password, encrypt, and log models ([e7c5811](https://github.com/orangerabbit-io/forwardemail/commit/e7c5811d75a837d0203ee7693370fdce7ec57476))
* add output module with markdown table, JSON, KV formatting ([133f96d](https://github.com/orangerabbit-io/forwardemail/commit/133f96d11e341364c5b387a900fde31b62c7d296))
* add TUI app state machine with navigation and event loop ([d6df788](https://github.com/orangerabbit-io/forwardemail/commit/d6df7880c81d1e14d9c5666489c0a3afb0195cc0))
* add TUI views with dashboard, domain/alias/email browsing ([b44ce0c](https://github.com/orangerabbit-io/forwardemail/commit/b44ce0cf3023105e76162dfbc8fff69d6e91f747))
* implement account commands (create, get, update) ([8893d75](https://github.com/orangerabbit-io/forwardemail/commit/8893d756fc1110efb9cf48a0e5ebdfe1055c2446))
* implement alias commands (list, CRUD, generate-password) ([35eb287](https://github.com/orangerabbit-io/forwardemail/commit/35eb287c6b8c165f708c8ef3ac68055677b9d54f))
* implement catch-all password commands (list, create, delete) ([b8c8948](https://github.com/orangerabbit-io/forwardemail/commit/b8c89486fbfb4257955efc484677d93d4d0360a3))
* implement domain commands (list, CRUD, verify) ([cdec331](https://github.com/orangerabbit-io/forwardemail/commit/cdec3315bc73f3ce943d24831efadcd63dce06ce))
* implement email commands (list, send, get, delete, limit) ([a9f2189](https://github.com/orangerabbit-io/forwardemail/commit/a9f2189c7c7946fe8e1edf9a0d08aebb86e05b40))
* implement encrypt command with unauthenticated client ([67d9f58](https://github.com/orangerabbit-io/forwardemail/commit/67d9f58644f75bdabca5b8ca05c4518fa96e83a0))
* implement invite commands (create, accept, remove) ([5840b58](https://github.com/orangerabbit-io/forwardemail/commit/5840b58da4c169fd28768db951ff734a7cd52e02))
* implement log download with gzip decompression ([e58ae68](https://github.com/orangerabbit-io/forwardemail/commit/e58ae68ddc1d59e428d535d92d989a234dbfea46))
* implement member commands (update, remove) ([99d8a67](https://github.com/orangerabbit-io/forwardemail/commit/99d8a67bfd899810c4db0a0fe67531d7cdffea99))
* scaffold workspace with forwardemail-lib and forwardemail crates ([8122756](https://github.com/orangerabbit-io/forwardemail/commit/8122756d8919db7c23166454a703b61a73575ce6))

# 1.0.0 (2026-03-25)


### Bug Fixes

* handle real API response shapes ([d8440c2](https://github.com/orangerabbit-io/forwardemail/commit/d8440c2afb3323ae6c5213df2defc769ade85984))


### Features

* add account model with row type and serde tests ([32ca949](https://github.com/orangerabbit-io/forwardemail/commit/32ca9495ff2e43a1328eb35cbff31c79a31ef4f2))
* add alias model with row type and serde tests ([dfb9421](https://github.com/orangerabbit-io/forwardemail/commit/dfb942121c6d5292a883ff56616eaed5aba9f146))
* add CLI structure with clap subcommands and lazy auth dispatch ([2b05fb0](https://github.com/orangerabbit-io/forwardemail/commit/2b05fb04332c8a918bd60d9538fd74f1e139c303))
* add config module with 3-tier API key resolution ([65b1f73](https://github.com/orangerabbit-io/forwardemail/commit/65b1f73accc022ec78287f18e47c179522005b98))
* add domain model with row type and serde tests ([56e3487](https://github.com/orangerabbit-io/forwardemail/commit/56e348776a37e9a689ae49f5565ddb87893c361c))
* add email model with EmailLimit and serde tests ([2c0a336](https://github.com/orangerabbit-io/forwardemail/commit/2c0a336b7b4abebc00c68ce2ebec83066671505b))
* add HTTP client with Basic Auth and unauthenticated mode ([b3df86a](https://github.com/orangerabbit-io/forwardemail/commit/b3df86a25b579d4c4098668fae7fdc0128b9d802))
* add invite, member, catch-all password, encrypt, and log models ([e7c5811](https://github.com/orangerabbit-io/forwardemail/commit/e7c5811d75a837d0203ee7693370fdce7ec57476))
* add output module with markdown table, JSON, KV formatting ([133f96d](https://github.com/orangerabbit-io/forwardemail/commit/133f96d11e341364c5b387a900fde31b62c7d296))
* add TUI app state machine with navigation and event loop ([d6df788](https://github.com/orangerabbit-io/forwardemail/commit/d6df7880c81d1e14d9c5666489c0a3afb0195cc0))
* add TUI views with dashboard, domain/alias/email browsing ([b44ce0c](https://github.com/orangerabbit-io/forwardemail/commit/b44ce0cf3023105e76162dfbc8fff69d6e91f747))
* implement account commands (create, get, update) ([8893d75](https://github.com/orangerabbit-io/forwardemail/commit/8893d756fc1110efb9cf48a0e5ebdfe1055c2446))
* implement alias commands (list, CRUD, generate-password) ([35eb287](https://github.com/orangerabbit-io/forwardemail/commit/35eb287c6b8c165f708c8ef3ac68055677b9d54f))
* implement catch-all password commands (list, create, delete) ([b8c8948](https://github.com/orangerabbit-io/forwardemail/commit/b8c89486fbfb4257955efc484677d93d4d0360a3))
* implement domain commands (list, CRUD, verify) ([cdec331](https://github.com/orangerabbit-io/forwardemail/commit/cdec3315bc73f3ce943d24831efadcd63dce06ce))
* implement email commands (list, send, get, delete, limit) ([a9f2189](https://github.com/orangerabbit-io/forwardemail/commit/a9f2189c7c7946fe8e1edf9a0d08aebb86e05b40))
* implement encrypt command with unauthenticated client ([67d9f58](https://github.com/orangerabbit-io/forwardemail/commit/67d9f58644f75bdabca5b8ca05c4518fa96e83a0))
* implement invite commands (create, accept, remove) ([5840b58](https://github.com/orangerabbit-io/forwardemail/commit/5840b58da4c169fd28768db951ff734a7cd52e02))
* implement log download with gzip decompression ([e58ae68](https://github.com/orangerabbit-io/forwardemail/commit/e58ae68ddc1d59e428d535d92d989a234dbfea46))
* implement member commands (update, remove) ([99d8a67](https://github.com/orangerabbit-io/forwardemail/commit/99d8a67bfd899810c4db0a0fe67531d7cdffea99))
* scaffold workspace with forwardemail-lib and forwardemail crates ([8122756](https://github.com/orangerabbit-io/forwardemail/commit/8122756d8919db7c23166454a703b61a73575ce6))

# 1.0.0 (2026-03-25)


### Bug Fixes

* handle real API response shapes ([d8440c2](https://github.com/orangerabbit-io/forwardemail/commit/d8440c2afb3323ae6c5213df2defc769ade85984))


### Features

* add account model with row type and serde tests ([32ca949](https://github.com/orangerabbit-io/forwardemail/commit/32ca9495ff2e43a1328eb35cbff31c79a31ef4f2))
* add alias model with row type and serde tests ([dfb9421](https://github.com/orangerabbit-io/forwardemail/commit/dfb942121c6d5292a883ff56616eaed5aba9f146))
* add CLI structure with clap subcommands and lazy auth dispatch ([2b05fb0](https://github.com/orangerabbit-io/forwardemail/commit/2b05fb04332c8a918bd60d9538fd74f1e139c303))
* add config module with 3-tier API key resolution ([65b1f73](https://github.com/orangerabbit-io/forwardemail/commit/65b1f73accc022ec78287f18e47c179522005b98))
* add domain model with row type and serde tests ([56e3487](https://github.com/orangerabbit-io/forwardemail/commit/56e348776a37e9a689ae49f5565ddb87893c361c))
* add email model with EmailLimit and serde tests ([2c0a336](https://github.com/orangerabbit-io/forwardemail/commit/2c0a336b7b4abebc00c68ce2ebec83066671505b))
* add HTTP client with Basic Auth and unauthenticated mode ([b3df86a](https://github.com/orangerabbit-io/forwardemail/commit/b3df86a25b579d4c4098668fae7fdc0128b9d802))
* add invite, member, catch-all password, encrypt, and log models ([e7c5811](https://github.com/orangerabbit-io/forwardemail/commit/e7c5811d75a837d0203ee7693370fdce7ec57476))
* add output module with markdown table, JSON, KV formatting ([133f96d](https://github.com/orangerabbit-io/forwardemail/commit/133f96d11e341364c5b387a900fde31b62c7d296))
* add TUI app state machine with navigation and event loop ([d6df788](https://github.com/orangerabbit-io/forwardemail/commit/d6df7880c81d1e14d9c5666489c0a3afb0195cc0))
* add TUI views with dashboard, domain/alias/email browsing ([b44ce0c](https://github.com/orangerabbit-io/forwardemail/commit/b44ce0cf3023105e76162dfbc8fff69d6e91f747))
* implement account commands (create, get, update) ([8893d75](https://github.com/orangerabbit-io/forwardemail/commit/8893d756fc1110efb9cf48a0e5ebdfe1055c2446))
* implement alias commands (list, CRUD, generate-password) ([35eb287](https://github.com/orangerabbit-io/forwardemail/commit/35eb287c6b8c165f708c8ef3ac68055677b9d54f))
* implement catch-all password commands (list, create, delete) ([b8c8948](https://github.com/orangerabbit-io/forwardemail/commit/b8c89486fbfb4257955efc484677d93d4d0360a3))
* implement domain commands (list, CRUD, verify) ([cdec331](https://github.com/orangerabbit-io/forwardemail/commit/cdec3315bc73f3ce943d24831efadcd63dce06ce))
* implement email commands (list, send, get, delete, limit) ([a9f2189](https://github.com/orangerabbit-io/forwardemail/commit/a9f2189c7c7946fe8e1edf9a0d08aebb86e05b40))
* implement encrypt command with unauthenticated client ([67d9f58](https://github.com/orangerabbit-io/forwardemail/commit/67d9f58644f75bdabca5b8ca05c4518fa96e83a0))
* implement invite commands (create, accept, remove) ([5840b58](https://github.com/orangerabbit-io/forwardemail/commit/5840b58da4c169fd28768db951ff734a7cd52e02))
* implement log download with gzip decompression ([e58ae68](https://github.com/orangerabbit-io/forwardemail/commit/e58ae68ddc1d59e428d535d92d989a234dbfea46))
* implement member commands (update, remove) ([99d8a67](https://github.com/orangerabbit-io/forwardemail/commit/99d8a67bfd899810c4db0a0fe67531d7cdffea99))
* scaffold workspace with forwardemail-lib and forwardemail crates ([8122756](https://github.com/orangerabbit-io/forwardemail/commit/8122756d8919db7c23166454a703b61a73575ce6))

# 1.0.0 (2026-03-25)


### Features

* add account model with row type and serde tests ([32ca949](https://github.com/orangerabbit-io/forwardemail/commit/32ca9495ff2e43a1328eb35cbff31c79a31ef4f2))
* add alias model with row type and serde tests ([dfb9421](https://github.com/orangerabbit-io/forwardemail/commit/dfb942121c6d5292a883ff56616eaed5aba9f146))
* add CLI structure with clap subcommands and lazy auth dispatch ([2b05fb0](https://github.com/orangerabbit-io/forwardemail/commit/2b05fb04332c8a918bd60d9538fd74f1e139c303))
* add config module with 3-tier API key resolution ([65b1f73](https://github.com/orangerabbit-io/forwardemail/commit/65b1f73accc022ec78287f18e47c179522005b98))
* add domain model with row type and serde tests ([56e3487](https://github.com/orangerabbit-io/forwardemail/commit/56e348776a37e9a689ae49f5565ddb87893c361c))
* add email model with EmailLimit and serde tests ([2c0a336](https://github.com/orangerabbit-io/forwardemail/commit/2c0a336b7b4abebc00c68ce2ebec83066671505b))
* add HTTP client with Basic Auth and unauthenticated mode ([b3df86a](https://github.com/orangerabbit-io/forwardemail/commit/b3df86a25b579d4c4098668fae7fdc0128b9d802))
* add invite, member, catch-all password, encrypt, and log models ([e7c5811](https://github.com/orangerabbit-io/forwardemail/commit/e7c5811d75a837d0203ee7693370fdce7ec57476))
* add output module with markdown table, JSON, KV formatting ([133f96d](https://github.com/orangerabbit-io/forwardemail/commit/133f96d11e341364c5b387a900fde31b62c7d296))
* add TUI app state machine with navigation and event loop ([d6df788](https://github.com/orangerabbit-io/forwardemail/commit/d6df7880c81d1e14d9c5666489c0a3afb0195cc0))
* add TUI views with dashboard, domain/alias/email browsing ([b44ce0c](https://github.com/orangerabbit-io/forwardemail/commit/b44ce0cf3023105e76162dfbc8fff69d6e91f747))
* implement account commands (create, get, update) ([8893d75](https://github.com/orangerabbit-io/forwardemail/commit/8893d756fc1110efb9cf48a0e5ebdfe1055c2446))
* implement alias commands (list, CRUD, generate-password) ([35eb287](https://github.com/orangerabbit-io/forwardemail/commit/35eb287c6b8c165f708c8ef3ac68055677b9d54f))
* implement catch-all password commands (list, create, delete) ([b8c8948](https://github.com/orangerabbit-io/forwardemail/commit/b8c89486fbfb4257955efc484677d93d4d0360a3))
* implement domain commands (list, CRUD, verify) ([cdec331](https://github.com/orangerabbit-io/forwardemail/commit/cdec3315bc73f3ce943d24831efadcd63dce06ce))
* implement email commands (list, send, get, delete, limit) ([a9f2189](https://github.com/orangerabbit-io/forwardemail/commit/a9f2189c7c7946fe8e1edf9a0d08aebb86e05b40))
* implement encrypt command with unauthenticated client ([67d9f58](https://github.com/orangerabbit-io/forwardemail/commit/67d9f58644f75bdabca5b8ca05c4518fa96e83a0))
* implement invite commands (create, accept, remove) ([5840b58](https://github.com/orangerabbit-io/forwardemail/commit/5840b58da4c169fd28768db951ff734a7cd52e02))
* implement log download with gzip decompression ([e58ae68](https://github.com/orangerabbit-io/forwardemail/commit/e58ae68ddc1d59e428d535d92d989a234dbfea46))
* implement member commands (update, remove) ([99d8a67](https://github.com/orangerabbit-io/forwardemail/commit/99d8a67bfd899810c4db0a0fe67531d7cdffea99))
* scaffold workspace with forwardemail-lib and forwardemail crates ([8122756](https://github.com/orangerabbit-io/forwardemail/commit/8122756d8919db7c23166454a703b61a73575ce6))

# 1.0.0 (2026-03-16)


### Features

* add account model with row type and serde tests ([7824030](https://github.com/orangerabbit-io/forwardemail/commit/7824030db795576dbaafc18e68b6abaeb2436968))
* add alias model with row type and serde tests ([424a5e8](https://github.com/orangerabbit-io/forwardemail/commit/424a5e8a9b6681829abd291c007fdbb8bf01d6d6))
* add CLI structure with clap subcommands and lazy auth dispatch ([ad8b2ef](https://github.com/orangerabbit-io/forwardemail/commit/ad8b2efbc99304d7cd3d0552190a978f3afdea87))
* add config module with 3-tier API key resolution ([1675004](https://github.com/orangerabbit-io/forwardemail/commit/1675004e32f80327b93662c39899dfd9ab1f3522))
* add domain model with row type and serde tests ([7ecbb93](https://github.com/orangerabbit-io/forwardemail/commit/7ecbb93eee2bb27e3872456a7f3c1ce83f15bee9))
* add email model with EmailLimit and serde tests ([400ed87](https://github.com/orangerabbit-io/forwardemail/commit/400ed875f525c649b373e067abd57c0ff96713b0))
* add HTTP client with Basic Auth and unauthenticated mode ([8c85105](https://github.com/orangerabbit-io/forwardemail/commit/8c851052962d0f571b0e3d2179a64736bf44b897))
* add invite, member, catch-all password, encrypt, and log models ([f828114](https://github.com/orangerabbit-io/forwardemail/commit/f82811427f637cdbb9874d9ddc034b34cb1c980a))
* add output module with markdown table, JSON, KV formatting ([8b3398a](https://github.com/orangerabbit-io/forwardemail/commit/8b3398a0bd551ac3023890a7ecc28c29398f7eeb))
* add TUI app state machine with navigation and event loop ([eaea389](https://github.com/orangerabbit-io/forwardemail/commit/eaea3896c261ba39c2a14f82fb6ca89b14ba7db0))
* add TUI views with dashboard, domain/alias/email browsing ([247bd92](https://github.com/orangerabbit-io/forwardemail/commit/247bd929f59aad9d02cee08af8ed6acc9d309616))
* implement account commands (create, get, update) ([3388666](https://github.com/orangerabbit-io/forwardemail/commit/33886666b95dfdaa35d84f47dbd2916a49967604))
* implement alias commands (list, CRUD, generate-password) ([74bae37](https://github.com/orangerabbit-io/forwardemail/commit/74bae37d3e5a30ff3b3a9cdc04ee94bfbf5d5de4))
* implement catch-all password commands (list, create, delete) ([c55bb12](https://github.com/orangerabbit-io/forwardemail/commit/c55bb126a741ab1523577204938173d03dc68537))
* implement domain commands (list, CRUD, verify) ([c06f43d](https://github.com/orangerabbit-io/forwardemail/commit/c06f43d710e8a342e3833cea26c421412cf262ee))
* implement email commands (list, send, get, delete, limit) ([bc4af8a](https://github.com/orangerabbit-io/forwardemail/commit/bc4af8a4ee5c0b3414898ec0268e8feee9a618c2))
* implement encrypt command with unauthenticated client ([523d872](https://github.com/orangerabbit-io/forwardemail/commit/523d872bdc875d92cd8a850aa52f51f11fdb3afe))
* implement invite commands (create, accept, remove) ([6473f73](https://github.com/orangerabbit-io/forwardemail/commit/6473f730f990331c31b4f91728ecff4d7c143973))
* implement log download with gzip decompression ([a842f5b](https://github.com/orangerabbit-io/forwardemail/commit/a842f5b67a78b5d7283ef8d1d1ce1ae43862d563))
* implement member commands (update, remove) ([9c760c2](https://github.com/orangerabbit-io/forwardemail/commit/9c760c222cc49c8dfc68ee679557b5b4517264ba))
* scaffold workspace with forwardemail-lib and forwardemail crates ([050c753](https://github.com/orangerabbit-io/forwardemail/commit/050c7537b1296b30afec290a76b5044a7ab648c2))
