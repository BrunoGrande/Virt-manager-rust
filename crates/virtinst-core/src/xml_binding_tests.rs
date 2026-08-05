//! The one runnable check ticket 06's whole approach exists to satisfy:
//! parse a document, mutate exactly one bound field through the derive
//! macro, and confirm everything else — including a foreign namespaced
//! element the struct never modeled at all — survives byte-for-byte.
//! This is ticket 03's acceptance bar (`virt-xml --edit` preserving
//! foreign `xmlns:qemu` elements), exercised directly rather than
//! assumed to work because the macro compiles.

use crate::devices::{DeviceDisk, DeviceList};
use crate::domain::Clock;
use virtinst_xml::{parse_libvirt_xml, XmlBound};

const FIXTURE: &str = r#"<domain type="qemu">
  <name>test-vm</name>
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
