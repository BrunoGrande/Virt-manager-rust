# GUI screen inventory

Source: `/usr/share/virt-manager` (virt-manager 5.1.0 installed package),
`virtManager/` package, 31 `.ui` files under `ui/`. Every `.ui` file paired
with its controller `.py` by name; LOC is the controller only (`wc -l`).

## Top-level screens (map's original list, now located)

| Screen | `.ui` | Controller(s) | LOC |
|---|---|---|---|
| VM Manager main window (list of connections + VMs — this **is** "connection manager", there's no separate screen) | `manager.ui` | `manager.py` | 1026 |
| New Connection dialog | `createconn.ui` (+ `connectauth.ui` for credential prompts) | `createconn.py` | 287 |
| VM details window | `details.ui` | `details/details.py` | 2522 |
| Add Hardware wizard | `addhardware.ui` | `addhardware.py` | 1622 |
| Create VM wizard | `createvm.ui` | `createvm.py` | 2106 |
| Clone VM dialog | `clone.ui` | `clone.py` | 626 |
| Migrate VM dialog | `migrate.ui` | `migrate.py` | 434 |
| Host details window (wraps storage/network tabs) | `host.ui` | `host.py` | 219 |
| Host storage tab | `hoststorage.ui` | `hoststorage.py` | 663 |
| Host network tab | `hostnets.ui` | `hostnets.py` | 375 |
| Snapshot management (tab in VM details) | `snapshots.ui`, `snapshotsnew.ui` | `details/snapshots.py` | 853 |
| Delete VM/storage dialog | `delete.ui` | `delete.py` | 636 |
| Preferences dialog | `preferences.ui` | `preferences.py` | 447 |
| XML editor tab (embedded in details/host/network/pool) | `xmleditor.ui` | `xmleditor.py` | 234 |
| Systray + VM right-click menu | *(no .ui, built in code)* | `systray.py`, `vmmenu.py` | 514 + 324 |
| About dialog | `about.ui` | `about.py` | 45 |
| Console viewer (VNC/SPICE/serial) — **own ticket per map, not detailed here** | `console.ui` | `details/console.py`, `details/viewers.py`, `details/serialcon.py`, `details/sshtunnels.py` | 949+772+378+328 |

## Shared sub-widget layer (not in the map's original list — found during this inventory)

These are reusable panels embedded inside the wizards/details window above,
not standalone screens. Worth their own thin tickets only if a specific one
turns out to be non-trivial to port; otherwise they fall out of whichever
top-level screen ticket uses them first.

| Widget | `.ui` | Controller | LOC | Used by |
|---|---|---|---|---|
| Storage config sub-panel | `addstorage.ui` | `device/addstorage.py` | 344 | Add Hardware, Create VM |
| Network device sub-panel | `netlist.ui` | `device/netlist.py` | 411 | Add Hardware, Create VM |
| Graphics/display sub-panel | `gfxdetails.ui` | `device/gfxdetails.py` | 328 | Add Hardware, details |
| Filesystem device sub-panel | `fsdetails.ui` | `device/fsdetails.py` | 275 | Add Hardware, details |
| TPM device sub-panel | `tpmdetails.ui` | `device/tpmdetails.py` | 173 | Add Hardware, details |
| vsock device sub-panel | `vsockdetails.ui` | `device/vsockdetails.py` | 72 | Add Hardware, details |
| Media (CD/disk) chooser combo | *(inline)* | `device/mediacombo.py` | 188 | Add Hardware, details |
| OS selection list (feeds ticket 04) | `oslist.ui` | `oslist.py` | 239 | Create VM |
| Storage volume browser | `storagebrowse.ui` | `storagebrowse.py` | 203 | Add Hardware, Create VM, pools |
| Async job / progress modal | `asyncjob.ui` | `asyncjob.py` | 334 | any long-running op |
| Create Network wizard | `createnet.ui` | `createnet.py` | 461 | Host network tab |
| Create Storage Pool wizard | `createpool.ui` | `createpool.py` | 397 | Host storage tab |
| Create Storage Volume wizard | `createvol.ui` | `createvol.py` | 336 | Host storage tab |

## Not a screen (infrastructure, no port-a-screen ticket needed)

`connmanager.py` (67, multi-connection bookkeeping), `engine.py` (506, app
lifecycle), `config.py` (639, GSettings wrapper), `error.py` (369, error
dialog helper), `baseclass.py` (408, GtkBuilder base class),
`virtmanager.py` (318, GApplication/CLI entrypoint) — these are plumbing
ticket 07's crate-boundaries follow-up (`virtinst-core`/app-shell split)
will need to account for, not GUI screens in their own right.

## Total

31 `.ui` files, ~23.6k LOC of controller code across 17 top-level screens
and 13 shared sub-widgets.
