//! The one runnable check ticket 06's whole approach exists to satisfy:
//! parse a document, mutate exactly one bound field through the derive
//! macro, and confirm everything else — including a foreign namespaced
//! element the struct never modeled at all — survives byte-for-byte.
//! This is ticket 03's acceptance bar (`virt-xml --edit` preserving
//! foreign `xmlns:qemu` elements), exercised directly rather than
//! assumed to work because the macro compiles.

use crate::devices::{DeviceDisk, DeviceGraphics, DeviceList, DeviceNetwork};
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
    // nested-element attribute groups (driver/source/target).
    let mut disk = DeviceDisk::from_element(&doc, disk_el);
    assert_eq!(disk, DeviceDisk {
        disk_type: Some("file".into()),
        device: Some("disk".into()),
        driver_name: Some("qemu".into()),
        driver_type: Some("qcow2".into()),
        source_file: Some("/old/path.qcow2".into()),
        target_dev: Some("vda".into()),
        target_bus: Some("virtio".into()),
    });

    // Edit exactly one field.
    disk.source_file = Some("/new/path.qcow2".into());
    disk.write_to(&mut doc, disk_el);

    let out = doc.write_str().expect("serializes");

    // The targeted change landed.
    assert!(out.contains(r#"file="/new/path.qcow2""#));
    assert!(!out.contains("/old/path.qcow2"));

    // Every other disk field the struct also binds is untouched.
    assert!(out.contains(r#"name="qemu""#));
    assert!(out.contains(r#"type="qcow2""#));
    assert!(out.contains(r#"dev="vda""#));
    assert!(out.contains(r#"bus="virtio""#));

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
