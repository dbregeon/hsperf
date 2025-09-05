pub fn main() {
    let mut monitor = hsperf::JavaVirtualMachine::list_jvms()
        .into_iter()
        .map(|jvm| jvm.monitor())
        .next()
        .unwrap()
        .unwrap();

    for entry in monitor.constants().iter() {
        println!(
            "{{ \"name\":\"{}\", \"unit\":\"{:?}\" }}",
            entry.name(),
            entry.unit()
        );
    }
    for entry in monitor.refresh().unwrap().iter() {
        println!(
            "{{ \"name\":\"{}\", \"variability\":\"{:?}\", \"unit\":\"{:?}\" }}",
            entry.name(),
            entry.variability(),
            entry.unit()
        );
    }
}
