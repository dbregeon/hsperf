pub fn main() {
    let monitor = hsperf::JavaVirtualMachine::list_jvms()
        .into_iter()
        .map(|jvm| jvm.monitor())
        .next()
        .unwrap()
        .unwrap();

    for (entry_name, entry) in monitor.entries().iter() {
        println!(
            "{{ \"name\":\"{}\", \"unit\":\"{:?}\" }}",
            entry_name,
            entry.unit()
        );
    }
}
