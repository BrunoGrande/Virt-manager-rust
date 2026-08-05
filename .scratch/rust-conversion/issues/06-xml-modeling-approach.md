---
id: 06
title: XML modeling approach for virtinst-core
type: grilling
status: resolved
blocked_by: [02, 03]
claimed_by: null
---

## Question

Should `virtinst-core`'s domain/storage/network/nodedev XML layer be built
as hand-written structs + `quick-xml`'s serde support (one struct per XML
element, direct mapping), or should it port `xmlbuilder.py`'s dynamic
XPath-descriptor engine more literally (a generic property-to-XPath binding
system)? Resolve once ticket 02 (real libvirt API surface) and ticket 03
(real test-suite coverage) show the actual shape and size of the surface
this needs to model — a guess now risks under- or over-building relative to
what full driver parity actually requires.

## Resolution

**Typed structs bound to XPaths via a derive macro, reading/writing through
a mutable, order-preserving DOM** — neither of the two original options as
literally framed. Ticket 03's inventory showed the acceptance bar isn't just
"build correct XML from scratch" (where plain structs + serde would be
sufficient) — it includes `testDomainRoundtrip` (parse unmodified XML →
re-serialize → byte match) and `virt-xml --edit` scenarios that must touch
only the targeted field while preserving everything else in the document
verbatim, including foreign `xmlns:qemu` passthrough elements
(`tests/test_xmlparse.py`, per ticket 03's inventory). A plain
deserialize-to-struct/reserialize model (hand-written structs + `quick-xml`
serde) can't naturally satisfy that without bolting on ad hoc catch-all/raw-
fragment handling until it converges on this approach anyway. Porting
`xmlbuilder.py`'s dynamic XPath-descriptor engine literally would satisfy
the preservation requirement (it's upstream's actual mechanism) but carries
unnecessary runtime dispatch overhead and gives up Rust's compile-time field
safety for no benefit — Rust's macro system can do the same property→XPath
binding at compile time instead of runtime.

Landing approach: struct-per-XML-element definitions (as in the plain-struct
option) with fields annotated for XPath binding, generated via a derive
macro, operating over a mutable order-preserving DOM built on the
[`edit-xml`](https://lib.rs/crates/edit-xml) crate (quick-xml-based
read/modify/write tree) rather than a hand-rolled DOM. This reuses an
existing crate instead of building bespoke tree-mutation machinery, while
giving both compile-time field safety (structs) and upstream's
preserve-the-rest-of-the-document behavior (DOM-level targeted mutation).

Fallback if this proves insufficient: plain `quick-xml` serde with bolted-on
raw-fragment preservation for the specific fields the golden fixtures show
need it — only add that complexity if `edit-xml`-backed XPath binding
actually fails a round-trip/preservation test, not upfront.
