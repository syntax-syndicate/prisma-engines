(function() {var implementors = {
"ahash":[["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;<a class=\"struct\" href=\"ahash/struct.AHashSet.html\" title=\"struct ahash::AHashSet\">AHashSet</a>&lt;T, S&gt;&gt; for &amp;<a class=\"struct\" href=\"ahash/struct.AHashSet.html\" title=\"struct ahash::AHashSet\">AHashSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"]],
"bitvec":[["impl&lt;T, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/boxed/struct.BitBox.html\" title=\"struct bitvec::boxed::BitBox\">BitBox</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXorAssign.html\" title=\"trait core::ops::bit::BitXorAssign\">BitXorAssign</a>&lt;Rhs&gt;,</span>"],["impl&lt;T, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/vec/struct.BitVec.html\" title=\"struct bitvec::vec::BitVec\">BitVec</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXorAssign.html\" title=\"trait core::ops::bit::BitXorAssign\">BitXorAssign</a>&lt;Rhs&gt;,</span>"],["impl&lt;A, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/array/struct.BitArray.html\" title=\"struct bitvec::array::BitArray\">BitArray</a>&lt;A, O&gt;<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"bitvec/view/trait.BitViewSized.html\" title=\"trait bitvec::view::BitViewSized\">BitViewSized</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;A::<a class=\"associatedtype\" href=\"bitvec/view/trait.BitView.html#associatedtype.Store\" title=\"type bitvec::view::BitView::Store\">Store</a>, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXorAssign.html\" title=\"trait core::ops::bit::BitXorAssign\">BitXorAssign</a>&lt;Rhs&gt;,</span>"]],
"enumflags2":[["impl&lt;T, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;B&gt; for <a class=\"struct\" href=\"enumflags2/struct.BitFlags.html\" title=\"struct enumflags2::BitFlags\">BitFlags</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"enumflags2/trait.BitFlag.html\" title=\"trait enumflags2::BitFlag\">BitFlag</a>,\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"enumflags2/struct.BitFlags.html\" title=\"struct enumflags2::BitFlags\">BitFlags</a>&lt;T&gt;&gt;,</span>"]],
"hashbrown":[["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S&gt;&gt; for &amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"]],
"hashlink":[["impl&lt;'a, 'b, T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'b <a class=\"struct\" href=\"hashlink/linked_hash_set/struct.LinkedHashSet.html\" title=\"struct hashlink::linked_hash_set::LinkedHashSet\">LinkedHashSet</a>&lt;T, S&gt;&gt; for &amp;'a <a class=\"struct\" href=\"hashlink/linked_hash_set/struct.LinkedHashSet.html\" title=\"struct hashlink::linked_hash_set::LinkedHashSet\">LinkedHashSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"]],
"indexmap":[["impl&lt;T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S2&gt;&gt; for &amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S1&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"]],
"lsp_types":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"lsp_types/struct.WatchKind.html\" title=\"struct lsp_types::WatchKind\">WatchKind</a>"]],
"mysql_common":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/packets/struct.BinlogDumpFlags.html\" title=\"struct mysql_common::packets::BinlogDumpFlags\">BinlogDumpFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/binlog/consts/struct.SemiSyncFlags.html\" title=\"struct mysql_common::binlog::consts::SemiSyncFlags\">SemiSyncFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.SqlMode.html\" title=\"struct mysql_common::constants::SqlMode\">SqlMode</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/binlog/consts/struct.UserVarFlags.html\" title=\"struct mysql_common::binlog::consts::UserVarFlags\">UserVarFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.StatusFlags.html\" title=\"struct mysql_common::constants::StatusFlags\">StatusFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.CapabilityFlags.html\" title=\"struct mysql_common::constants::CapabilityFlags\">CapabilityFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.StmtExecuteParamFlags.html\" title=\"struct mysql_common::constants::StmtExecuteParamFlags\">StmtExecuteParamFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.StmtExecuteParamsFlags.html\" title=\"struct mysql_common::constants::StmtExecuteParamsFlags\">StmtExecuteParamsFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/binlog/consts/struct.EventFlags.html\" title=\"struct mysql_common::binlog::consts::EventFlags\">EventFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/binlog/consts/struct.RowsEventFlags.html\" title=\"struct mysql_common::binlog::consts::RowsEventFlags\">RowsEventFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.CursorType.html\" title=\"struct mysql_common::constants::CursorType\">CursorType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/binlog/consts/struct.GtidFlags.html\" title=\"struct mysql_common::binlog::consts::GtidFlags\">GtidFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.Flags2.html\" title=\"struct mysql_common::constants::Flags2\">Flags2</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"mysql_common/constants/struct.ColumnFlags.html\" title=\"struct mysql_common::constants::ColumnFlags\">ColumnFlags</a>"]],
"napi":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"napi/struct.PropertyAttributes.html\" title=\"struct napi::PropertyAttributes\">PropertyAttributes</a>"]],
"num_bigint":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"],["impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'b <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"],["impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'b <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"]],
"openssl":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.ExtensionContext.html\" title=\"struct openssl::ssl::ExtensionContext\">ExtensionContext</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/x509/verify/struct.X509VerifyFlags.html\" title=\"struct openssl::x509::verify::X509VerifyFlags\">X509VerifyFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ocsp/struct.OcspFlag.html\" title=\"struct openssl::ocsp::OcspFlag\">OcspFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/cipher_ctx/struct.CipherCtxFlags.html\" title=\"struct openssl::cipher_ctx::CipherCtxFlags\">CipherCtxFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/x509/verify/struct.X509CheckFlags.html\" title=\"struct openssl::x509::verify::X509CheckFlags\">X509CheckFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslVerifyMode.html\" title=\"struct openssl::ssl::SslVerifyMode\">SslVerifyMode</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/cms/struct.CMSOptions.html\" title=\"struct openssl::cms::CMSOptions\">CMSOptions</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/pkcs7/struct.Pkcs7Flags.html\" title=\"struct openssl::pkcs7::Pkcs7Flags\">Pkcs7Flags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslOptions.html\" title=\"struct openssl::ssl::SslOptions\">SslOptions</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslSessionCacheMode.html\" title=\"struct openssl::ssl::SslSessionCacheMode\">SslSessionCacheMode</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.ShutdownState.html\" title=\"struct openssl::ssl::ShutdownState\">ShutdownState</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslMode.html\" title=\"struct openssl::ssl::SslMode\">SslMode</a>"]],
"ordermap":[["impl&lt;'a, 'b, T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;'b <a class=\"struct\" href=\"ordermap/set/struct.OrderSet.html\" title=\"struct ordermap::set::OrderSet\">OrderSet</a>&lt;T, S2&gt;&gt; for &amp;'a <a class=\"struct\" href=\"ordermap/set/struct.OrderSet.html\" title=\"struct ordermap::set::OrderSet\">OrderSet</a>&lt;T, S1&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"]],
"parser_database":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"parser_database/enum.ReferentialAction.html\" title=\"enum parser_database::ReferentialAction\">ReferentialAction</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"parser_database/enum.IndexAlgorithm.html\" title=\"enum parser_database::IndexAlgorithm\">IndexAlgorithm</a>"]],
"psl_core":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"psl_core/datamodel_connector/capabilities/enum.ConnectorCapability.html\" title=\"enum psl_core::datamodel_connector::capabilities::ConnectorCapability\">ConnectorCapability</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"psl_core/enum.PreviewFeature.html\" title=\"enum psl_core::PreviewFeature\">PreviewFeature</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"psl_core/datamodel_connector/enum.StringFilter.html\" title=\"enum psl_core::datamodel_connector::StringFilter\">StringFilter</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"psl_core/datamodel_connector/enum.RelationMode.html\" title=\"enum psl_core::datamodel_connector::RelationMode\">RelationMode</a>"]],
"quaint_test_setup":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"quaint_test_setup/struct.Tags.html\" title=\"struct quaint_test_setup::Tags\">Tags</a>"]],
"query_engine":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"query_engine/features/enum.Feature.html\" title=\"enum query_engine::features::Feature\">Feature</a>"]],
"rusqlite":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rusqlite/struct.OpenFlags.html\" title=\"struct rusqlite::OpenFlags\">OpenFlags</a>"]],
"rustix":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/termios/struct.OutputModes.html\" title=\"struct rustix::termios::OutputModes\">OutputModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/termios/struct.ControlModes.html\" title=\"struct rustix::termios::ControlModes\">ControlModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.XattrFlags.html\" title=\"struct rustix::fs::XattrFlags\">XattrFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/io/struct.DupFlags.html\" title=\"struct rustix::io::DupFlags\">DupFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.StatVfsMountFlags.html\" title=\"struct rustix::fs::StatVfsMountFlags\">StatVfsMountFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.StatxFlags.html\" title=\"struct rustix::fs::StatxFlags\">StatxFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/inotify/struct.CreateFlags.html\" title=\"struct rustix::fs::inotify::CreateFlags\">CreateFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/io/struct.ReadWriteFlags.html\" title=\"struct rustix::io::ReadWriteFlags\">ReadWriteFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/termios/struct.LocalModes.html\" title=\"struct rustix::termios::LocalModes\">LocalModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/termios/struct.InputModes.html\" title=\"struct rustix::termios::InputModes\">InputModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.AtFlags.html\" title=\"struct rustix::fs::AtFlags\">AtFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.MemfdFlags.html\" title=\"struct rustix::fs::MemfdFlags\">MemfdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.ResolveFlags.html\" title=\"struct rustix::fs::ResolveFlags\">ResolveFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.FallocateFlags.html\" title=\"struct rustix::fs::FallocateFlags\">FallocateFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.OFlags.html\" title=\"struct rustix::fs::OFlags\">OFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.Mode.html\" title=\"struct rustix::fs::Mode\">Mode</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.RenameFlags.html\" title=\"struct rustix::fs::RenameFlags\">RenameFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.Access.html\" title=\"struct rustix::fs::Access\">Access</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/struct.SealFlags.html\" title=\"struct rustix::fs::SealFlags\">SealFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/io/struct.FdFlags.html\" title=\"struct rustix::io::FdFlags\">FdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"rustix/fs/inotify/struct.WatchFlags.html\" title=\"struct rustix::fs::inotify::WatchFlags\">WatchFlags</a>"]],
"sql_schema_describer":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"sql_schema_describer/mssql/enum.IndexBits.html\" title=\"enum sql_schema_describer::mssql::IndexBits\">IndexBits</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"sql_schema_describer/enum.TableProperties.html\" title=\"enum sql_schema_describer::TableProperties\">TableProperties</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"sql_schema_describer/postgres/enum.Circumstances.html\" title=\"enum sql_schema_describer::postgres::Circumstances\">Circumstances</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"sql_schema_describer/postgres/enum.ConstraintOption.html\" title=\"enum sql_schema_describer::postgres::ConstraintOption\">ConstraintOption</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"sql_schema_describer/mysql/enum.Circumstances.html\" title=\"enum sql_schema_describer::mysql::Circumstances\">Circumstances</a>"]],
"subtle":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"subtle/struct.Choice.html\" title=\"struct subtle::Choice\">Choice</a>"]],
"test_setup":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"test_setup/enum.Capabilities.html\" title=\"enum test_setup::Capabilities\">Capabilities</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"test_setup/enum.Tags.html\" title=\"enum test_setup::Tags\">Tags</a>"]],
"tiberius":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"enum\" href=\"tiberius/enum.ColumnFlag.html\" title=\"enum tiberius::ColumnFlag\">ColumnFlag</a>"]],
"tracing_subscriber":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"tracing_subscriber/fmt/format/struct.FmtSpan.html\" title=\"struct tracing_subscriber::fmt::format::FmtSpan\">FmtSpan</a>"]],
"typenum":[["impl&lt;Ul: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>, Bl: <a class=\"trait\" href=\"typenum/marker_traits/trait.Bit.html\" title=\"trait typenum::marker_traits::Bit\">Bit</a>, Ur: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;: PrivateXor&lt;Ur&gt;,\n    PrivateXorOut&lt;<a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;, Ur&gt;: Trim,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>"],["impl&lt;Ur: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UTerm.html\" title=\"struct typenum::uint::UTerm\">UTerm</a>"]],
"wasm_bindgen":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;<a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>&gt; for &amp;'a <a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a>&lt;&amp;<a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>&gt; for <a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for <a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/bit/trait.BitXor.html\" title=\"trait core::ops::bit::BitXor\">BitXor</a> for &amp;<a class=\"struct\" href=\"wasm_bindgen/struct.JsValue.html\" title=\"struct wasm_bindgen::JsValue\">JsValue</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()