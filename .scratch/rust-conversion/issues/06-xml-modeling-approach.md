---
id: 06
title: XML modeling approach for virtinst-core
type: grilling
status: open
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

HITL — resolve via `/grilling` with the user once tickets 02 and 03 land.
