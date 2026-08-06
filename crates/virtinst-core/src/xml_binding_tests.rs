//! The one runnable check ticket 06's whole approach exists to satisfy:
//! parse a document, mutate exactly one bound field through the derive
//! macro, and confirm everything else — including a foreign namespaced
//! element the struct never modeled at all — survives byte-for-byte.
//! This is ticket 03's acceptance bar (`virt-xml --edit` preserving
//! foreign `xmlns:qemu` elements), exercised directly rather than
//! assumed to work because the macro compiles.

use crate::devices::{
    DeviceController, DeviceDisk, DeviceFilesystem, DeviceGraphics, DeviceHostdev, DeviceInput,
    DeviceList, DeviceNetwork, DeviceSerial, DeviceSound,
};
use crate::domain::{Clock, CurrentMemory};
use crate::guest::Guest;
use virtinst_xml::{parse_libvirt_xml, XmlBound};

const FIXTURE: &str = r#"<domain type="qemu">
  <name>test-vm</name>
  <title>My Test VM</title>
  <description>A VM used for testing.</description>
  <currentMemory unit="KiB">1048576</currentMemory>
  <clock offset="utc"/>
  <devices>
    <disk type="file" device="disk">
      <driver name="qemu" type="qcow2"/>
      <source file="/old/path.qcow2"/>
      <target dev="vda" bus="virtio"/>
      <shareable/>
    </disk>
    <disk type="file" device="cdrom">
      <target dev="hda" bus="ide"/>
    </disk>
    <interface type="network">
      <mac address="52:54:00:12:34:56"/>
      <source network="default"/>
      <model type="virtio"/>
      <target dev="vnet0"/>
      <link state="up"/>
    </interface>
    <graphics type="vnc" port="-1" autoport="yes" listen="127.0.0.1"/>
    <controller type="scsi" index="0" model="virtio-scsi">
      <driver queues="4"/>
    </controller>
    <input type="tablet" bus="usb"/>
    <sound model="ich9" multichannel="yes">
      <audio id="1"/>
    </sound>
    <filesystem type="mount" accessmode="mapped">
      <source dir="/host/share"/>
      <target dir="/mnt/share"/>
      <readonly/>
    </filesystem>
    <hostdev mode="subsystem" type="pci" managed="yes">
      <source>
        <address domain="0x0000" bus="0x00" slot="0x02" function="0x0"/>
      </source>
    </hostdev>
    <serial type="pty">
      <target type="isa-serial" port="0">
        <model name="isa-serial"/>
      </target>
    </serial>
  </devices>
  <metadata>
    <app:foo xmlns:app="http://example.com/app" note="untouched-foreign-namespace"/>
  </metadata>
</domain>"#;

#[test]
fn disk_edit_preserves_everything_else() {
    let mut doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");

    let devices = root.find(&doc, "devices").expect("<devices>");
    let disk_el = devices.find(&doc, "disk").expect("<disk>");

    // Read: every field lands correctly, including the three separate
    // nested-element attribute groups (driver/source/target) and the
    // presence-based readonly/shareable/transient markers.
    let mut disk = DeviceDisk::from_element(&doc, disk_el);
    assert_eq!(disk, DeviceDisk {
        disk_type: Some("file".into()),
        device: Some("disk".into()),
        driver_name: Some("qemu".into()),
        driver_type: Some("qcow2".into()),
        source_file: Some("/old/path.qcow2".into()),
        target_dev: Some("vda".into()),
        target_bus: Some("virtio".into()),
        read_only: false,
        shareable: true,
        transient: false,
    });

    // Edit exactly one field.
    disk.source_file = Some("/new/path.qcow2".into());
    disk.write_to(&mut doc, disk_el);

    let out = doc.write_str().expect("serializes");

    // The targeted change landed.
    assert!(out.contains(r#"file="/new/path.qcow2""#));
    assert!(!out.contains("/old/path.qcow2"));

    // Every other disk field the struct also binds is untouched,
    // including <shareable/> — write_to for `present` fields must
    // preserve true just as faithfully as it creates/removes on change.
    assert!(out.contains(r#"name="qemu""#));
    assert!(out.contains(r#"type="qcow2""#));
    assert!(out.contains(r#"dev="vda""#));
    assert!(out.contains(r#"bus="virtio""#));

    // `present` fields specifically: re-read the same element rather
    // than whole-document substring search — the fixture now has an
    // unrelated <filesystem><readonly/></filesystem> elsewhere, so a
    // bare `!out.contains("<readonly")` would (correctly) fail without
    // this actually being a bug in this disk's own write_to.
    let after = DeviceDisk::from_element(&doc, disk_el);
    assert!(after.shareable, "write_to must preserve an untouched `present` field, not just create/remove on change");
    assert!(!after.read_only);
    assert!(!after.transient);

    // Content this struct never modeled at all survives verbatim —
    // the actual preservation guarantee, not just "the fields I know
    // about round-trip".
    assert!(out.contains("<name>test-vm</name>"));
    assert!(out.contains(r#"xmlns:app="http://example.com/app""#));
    assert!(out.contains(r#"note="untouched-foreign-namespace""#));
}

#[test]
fn clock_binds_a_flat_attribute() {
    let doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let clock_el = root.find(&doc, "clock").expect("<clock>");

    let clock = Clock::from_element(&doc, clock_el);
    assert_eq!(clock.offset, Some("utc".to_string()));
}

#[test]
fn missing_optional_field_reads_as_none_not_a_panic() {
    let doc = parse_libvirt_xml(r#"<disk type="file" device="disk"><target dev="vda"/></disk>"#)
        .expect("fixture parses");
    let disk_el = doc.root_element().expect("root element");

    let disk = DeviceDisk::from_element(&doc, disk_el);
    // <driver> and <source> are absent entirely, and target@bus is
    // absent within an element that *does* exist — both should read
    // as None, not panic on a missing find().
    assert_eq!(disk.driver_name, None);
    assert_eq!(disk.source_file, None);
    assert_eq!(disk.target_bus, None);
    assert_eq!(disk.target_dev, Some("vda".to_string()));
}

#[test]
fn writing_an_absent_field_creates_only_the_needed_path() {
    let mut doc = parse_libvirt_xml(r#"<disk type="file" device="disk"/>"#)
        .expect("fixture parses");
    let disk_el = doc.root_element().expect("root element");

    let disk = DeviceDisk {
        disk_type: Some("file".into()),
        device: Some("disk".into()),
        source_file: Some("/fresh.qcow2".into()),
        ..Default::default()
    };
    disk.write_to(&mut doc, disk_el);

    let out = doc.write_str().expect("serializes");
    assert!(out.contains("<source"));
    assert!(out.contains(r#"file="/fresh.qcow2""#));
    // Fields that were never set stay entirely absent — no empty
    // <driver/> or <target/> elements created as a side effect.
    assert!(!out.contains("<driver"));
    assert!(!out.contains("<target"));
}

#[test]
fn list_read_collects_every_repeated_element() {
    let doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let devices_el = root.find(&doc, "devices").expect("<devices>");

    let devices = DeviceList::from_element(&doc, devices_el);
    assert_eq!(devices.disks.len(), 2);
    assert_eq!(devices.disks[0].target_dev, Some("vda".into()));
    assert_eq!(devices.disks[1].target_dev, Some("hda".into()));
    assert_eq!(devices.disks[1].device, Some("cdrom".into()));

    // Two different `list` fields on the same struct, sharing one
    // container element — each must find only its own tag, not the
    // other's (`disks` still reads exactly 2, not 3, with the
    // <interface> mixed in among the <disk> siblings).
    assert_eq!(devices.interfaces.len(), 1);
    assert_eq!(devices.interfaces[0].macaddr, Some("52:54:00:12:34:56".into()));
    assert_eq!(devices.interfaces[0].source_network, Some("default".into()));
    assert_eq!(devices.interfaces[0].model, Some("virtio".into()));

    assert_eq!(devices.graphics.len(), 1);
    assert_eq!(devices.graphics[0].graphics_type, Some("vnc".into()));
    assert_eq!(devices.graphics[0].port, Some("-1".into()));
    assert_eq!(devices.graphics[0].autoport, Some(true));
    assert_eq!(devices.graphics[0].listen, Some("127.0.0.1".into()));

    assert_eq!(devices.controllers.len(), 1);
    assert_eq!(devices.controllers[0].controller_type, Some("scsi".into()));
    assert_eq!(devices.controllers[0].index, Some(0));
    assert_eq!(devices.controllers[0].driver_queues, Some(4));

    assert_eq!(devices.inputs.len(), 1);
    assert_eq!(devices.inputs[0].input_type, Some("tablet".into()));
    assert_eq!(devices.inputs[0].bus, Some("usb".into()));

    assert_eq!(devices.sounds.len(), 1);
    assert_eq!(devices.sounds[0].model, Some("ich9".into()));
    assert_eq!(devices.sounds[0].multichannel, Some(true));
    assert_eq!(devices.sounds[0].audio_id, Some("1".into()));

    assert_eq!(devices.filesystems.len(), 1);
    assert_eq!(devices.filesystems[0].source_dir, Some("/host/share".into()));
    assert_eq!(devices.filesystems[0].target_dir, Some("/mnt/share".into()));
    assert!(devices.filesystems[0].readonly);

    assert_eq!(devices.hostdevs.len(), 1);
    assert_eq!(devices.hostdevs[0].hostdev_type, Some("pci".into()));
    assert_eq!(devices.hostdevs[0].managed, Some(true));
    assert_eq!(devices.hostdevs[0].pci_domain, Some("0x0000".into()));
    assert_eq!(devices.hostdevs[0].pci_slot, Some("0x02".into()));
    // USB fields genuinely absent from this PCI-addressed hostdev.
    assert_eq!(devices.hostdevs[0].vendor, None);

    assert_eq!(devices.serials.len(), 1);
    assert_eq!(devices.serials[0].char_type, Some("pty".into()));
    assert_eq!(devices.serials[0].target_type, Some("isa-serial".into()));
    assert_eq!(devices.serials[0].target_port, Some(0));
    assert_eq!(devices.serials[0].target_model_name, Some("isa-serial".into()));
}

#[test]
fn serial_reads_source_child_and_its_own_multi_segment_target_model() {
    let doc = parse_libvirt_xml(
        r#"<serial type="tcp">
             <source mode="bind" host="127.0.0.1" service="2445" tls="yes"/>
             <target type="isa-serial" port="1">
               <model name="usb-serial"/>
             </target>
           </serial>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let serial = DeviceSerial::from_element(&doc, el);
    assert_eq!(serial.source_mode, Some("bind".into()));
    assert_eq!(serial.source_host, Some("127.0.0.1".into()));
    assert_eq!(serial.source_service, Some(2445));
    assert_eq!(serial.source_tls, Some(true));
    assert_eq!(serial.target_model_name, Some("usb-serial".into()));
    // No <source path="..."/> in this fixture.
    assert_eq!(serial.source_path, None);
}

#[test]
fn hostdev_reads_multi_segment_path_for_usb_vendor_product() {
    // source/vendor/@id and source/product/@id - two levels deep, the
    // first struct to actually need more than one path segment.
    let doc = parse_libvirt_xml(
        r#"<hostdev mode="subsystem" type="usb">
             <source>
               <vendor id="0x1234"/>
               <product id="0x5678"/>
             </source>
           </hostdev>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let hostdev = DeviceHostdev::from_element(&doc, el);
    assert_eq!(hostdev.vendor, Some("0x1234".into()));
    assert_eq!(hostdev.product, Some("0x5678".into()));
    // PCI-only fields genuinely absent from this USB-addressed hostdev.
    assert_eq!(hostdev.pci_domain, None);
}

#[test]
fn hostdev_reads_text_content_under_a_multi_segment_path() {
    // source/interface has no @attr in upstream's XPath - it's the
    // element's own text content, nested two levels deep.
    let doc = parse_libvirt_xml(
        r#"<hostdev mode="capabilities" type="net">
             <source><interface>eth0</interface></source>
           </hostdev>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let hostdev = DeviceHostdev::from_element(&doc, el);
    assert_eq!(hostdev.net_interface, Some("eth0".into()));
}

#[test]
fn present_field_toggles_element_existence_both_ways() {
    // Starts with no <readonly/> at all.
    let mut doc = parse_libvirt_xml(
        r#"<filesystem type="mount"><source dir="/a"/><target dir="/b"/></filesystem>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let mut fs = DeviceFilesystem::from_element(&doc, el);
    assert!(!fs.readonly);

    // false -> true creates exactly the marker element.
    fs.readonly = true;
    fs.write_to(&mut doc, el);
    let out = doc.write_str().expect("serializes");
    assert!(out.contains("<readonly"));
    assert!(out.contains(r#"dir="/a""#)); // untouched sibling content

    // true -> false removes only that element, nothing else.
    let mut fs = DeviceFilesystem::from_element(&doc, el);
    assert!(fs.readonly);
    fs.readonly = false;
    fs.write_to(&mut doc, el);
    let out = doc.write_str().expect("serializes");
    assert!(!out.contains("<readonly"));
    assert!(out.contains(r#"dir="/a""#));
    assert!(out.contains(r#"dir="/b""#));

    // Writing `false` when it was already absent is a harmless no-op,
    // not an error.
    let fs = DeviceFilesystem::from_element(&doc, el);
    assert!(!fs.readonly);
    fs.write_to(&mut doc, el);
    assert!(!doc.write_str().expect("serializes").contains("<readonly"));
}

#[test]
fn sound_optional_bool_absent_when_multichannel_not_set() {
    let doc = parse_libvirt_xml(r#"<sound model="ich6"/>"#).expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let sound = DeviceSound::from_element(&doc, el);
    assert_eq!(sound.model, Some("ich6".into()));
    assert_eq!(sound.multichannel, None);
    assert_eq!(sound.audio_id, None);
}

#[test]
fn input_reads_nested_source_fields() {
    let doc = parse_libvirt_xml(
        r#"<input type="evdev" bus="virtio"><source dev="/dev/input/event1" grab="all"/></input>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let input = DeviceInput::from_element(&doc, el);
    assert_eq!(input.source_dev, Some("/dev/input/event1".into()));
    assert_eq!(input.source_grab, Some("all".into()));
    assert_eq!(input.source_evdev, None);
}

#[test]
fn integer_attributes_read_and_edit_correctly() {
    let mut doc = parse_libvirt_xml(
        r#"<controller type="scsi" index="0" model="virtio-scsi"/>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let mut controller = DeviceController::from_element(&doc, el);
    assert_eq!(controller.index, Some(0));
    assert_eq!(controller.driver_queues, None); // <driver> absent entirely

    controller.index = Some(1);
    controller.write_to(&mut doc, el);

    let out = doc.write_str().expect("serializes");
    assert!(out.contains(r#"index="1""#));
    assert!(!out.contains(r#"index="0""#));
    // driver_queues was never set — write_to must not invent <driver>.
    assert!(!out.contains("<driver"));
}

#[test]
fn graphics_reads_self_attributes_and_one_nested_spice_attribute() {
    let doc = parse_libvirt_xml(
        r#"<graphics type="spice" autoport="yes"><image compression="auto_glz"/></graphics>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let gfx = DeviceGraphics::from_element(&doc, el);
    assert_eq!(gfx.graphics_type, Some("spice".into()));
    assert_eq!(gfx.autoport, Some(true));
    assert_eq!(gfx.image_compression, Some("auto_glz".into()));
    // Not present in this fixture at all.
    assert_eq!(gfx.port, None);
}

#[test]
fn optional_bool_stays_absent_instead_of_defaulting_to_false() {
    // No autoport attribute at all — must read as None, not Some(false),
    // and writing the struct straight back must NOT invent one.
    let mut doc = parse_libvirt_xml(r#"<graphics type="vnc" port="5900"/>"#)
        .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let gfx = DeviceGraphics::from_element(&doc, el);
    assert_eq!(gfx.autoport, None);

    gfx.write_to(&mut doc, el);
    let out = doc.write_str().expect("serializes");
    assert!(!out.contains("autoport"));
}

#[test]
fn network_reads_distinct_source_fields_for_bridge_vs_network_type() {
    // source_network and source_bridge both bind under the same
    // <source> path but different attributes — only the one actually
    // present in the XML should come back Some.
    let doc = parse_libvirt_xml(
        r#"<interface type="bridge"><source bridge="virbr0"/></interface>"#,
    )
    .expect("fixture parses");
    let el = doc.root_element().expect("root element");

    let iface = DeviceNetwork::from_element(&doc, el);
    assert_eq!(iface.source_bridge, Some("virbr0".into()));
    assert_eq!(iface.source_network, None);
}

#[test]
fn list_add_appends_without_touching_existing_items() {
    let mut doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let devices_el = root.find(&doc, "devices").expect("<devices>");

    let new_disk = DeviceDisk {
        disk_type: Some("file".into()),
        device: Some("disk".into()),
        target_dev: Some("vdc".into()),
        target_bus: Some("virtio".into()),
        ..Default::default()
    };
    virtinst_xml::list_add(&mut doc, devices_el, &[], &new_disk);

    let devices = DeviceList::from_element(&doc, devices_el);
    assert_eq!(devices.disks.len(), 3);
    assert_eq!(devices.disks[2].target_dev, Some("vdc".into()));

    // The two pre-existing disks are still exactly as they were —
    // add_list only ever appends, it doesn't rebuild the container.
    assert_eq!(devices.disks[0].target_dev, Some("vda".into()));
    assert_eq!(devices.disks[1].target_dev, Some("hda".into()));

    let out = doc.write_str().expect("serializes");
    assert!(out.contains(r#"file="/old/path.qcow2""#)); // untouched
    assert!(out.contains(r#"note="untouched-foreign-namespace""#)); // untouched
}

#[test]
fn list_remove_removes_only_the_targeted_element() {
    let mut doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let devices_el = root.find(&doc, "devices").expect("<devices>");

    let disk_els = devices_el.find_all(&doc, "disk");
    assert_eq!(disk_els.len(), 2);
    let cdrom_el = disk_els[1]; // the <disk device="cdrom"> one

    virtinst_xml::list_remove(&mut doc, cdrom_el).expect("detach succeeds");

    let devices = DeviceList::from_element(&doc, devices_el);
    assert_eq!(devices.disks.len(), 1);
    assert_eq!(devices.disks[0].target_dev, Some("vda".into()));

    let out = doc.write_str().expect("serializes");
    assert!(!out.contains(r#"dev="hda""#));
    // Everything else in the document is untouched.
    assert!(out.contains(r#"file="/old/path.qcow2""#));
    assert!(out.contains("<name>test-vm</name>"));
    assert!(out.contains(r#"note="untouched-foreign-namespace""#));
}

#[test]
fn attribute_and_text_bind_to_the_same_element() {
    let doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let mem_el = root.find(&doc, "currentMemory").expect("<currentMemory>");

    let mem = CurrentMemory::from_element(&doc, mem_el);
    assert_eq!(mem.unit, Some("KiB".to_string()));
    assert_eq!(mem.value, Some("1048576".to_string()));
}

#[test]
fn editing_text_preserves_the_sibling_attribute() {
    let mut doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");
    let mem_el = root.find(&doc, "currentMemory").expect("<currentMemory>");

    let mut mem = CurrentMemory::from_element(&doc, mem_el);
    mem.value = Some("2097152".to_string());
    mem.write_to(&mut doc, mem_el);

    let out = doc.write_str().expect("serializes");
    assert!(out.contains(">2097152<"));
    assert!(!out.contains(">1048576<"));
    // Editing the text field didn't disturb the attribute field.
    assert!(out.contains(r#"unit="KiB""#));
}

#[test]
fn guest_reads_description_and_title_via_path_plus_text() {
    let doc = parse_libvirt_xml(FIXTURE).expect("fixture parses");
    let root = doc.root_element().expect("root element");

    let guest = Guest::from_element(&doc, root);
    assert_eq!(guest.title, Some("My Test VM".to_string()));
    assert_eq!(
        guest.description,
        Some("A VM used for testing.".to_string())
    );
}

#[test]
fn guest_missing_description_reads_as_none() {
    let doc = parse_libvirt_xml(r#"<domain type="qemu"><name>bare</name></domain>"#)
        .expect("fixture parses");
    let root = doc.root_element().expect("root element");

    let guest = Guest::from_element(&doc, root);
    assert_eq!(guest.description, None);
    assert_eq!(guest.title, None);
}

#[test]
fn writing_a_new_description_creates_only_that_element() {
    let mut doc =
        parse_libvirt_xml(r#"<domain type="qemu"><name>bare</name><clock offset="utc"/></domain>"#)
            .expect("fixture parses");
    let root = doc.root_element().expect("root element");

    let guest = Guest {
        description: Some("Added after the fact.".to_string()),
        ..Default::default()
    };
    guest.write_to(&mut doc, root);

    let out = doc.write_str().expect("serializes");
    assert!(out.contains("<description>Added after the fact.</description>"));
    // No <title> was created — only the field that was actually set.
    assert!(!out.contains("<title"));
    // Pre-existing, unrelated content is untouched.
    assert!(out.contains("<name>bare</name>"));
    assert!(out.contains(r#"offset="utc""#));
}
