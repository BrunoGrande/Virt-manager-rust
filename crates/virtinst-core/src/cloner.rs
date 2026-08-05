//! VM cloning — upstream's `cloner.py`, wrapped by `virt-clone` (ticket
//! 16). Parse source VM, override MAC/disk paths, validate, define new
//! domain. Same "fresh document, no DOM-preservation needed" shape as
//! ticket 08's Create VM wizard — added to this module list per ticket
//! 16 (a minor omission in ticket 15's original table).

// TODO: pub struct Cloner { ... }
