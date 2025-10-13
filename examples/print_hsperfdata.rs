use std::collections::HashSet;

use hsperf::JvmMonitor;
use nix::unistd::sleep;

pub fn main() {
    let properies_of_interest = vec![
        "java.threads.live",
        "java.threads.livePeak",
        "sun.rt.safepointSyncTime",
        "sun.rt.safepoints",
        "sun.rt.safepointTime",
        "sun.rt.applicationTime",
        "sun.gc.cause",
        "sun.gc.lastCause",
        "sun.gc.collector.0.name",
        "sun.gc.collector.1.name",
        "sun.gc.collector.2.name",
        "sun.gc.generation.0.name",
        "sun.gc.generation.0.spaces",
        "sun.gc.generation.1.name",
        "sun.gc.generation.1.spaces",
        "sun.gc.collector.0.invocations",
        "sun.gc.collector.0.time",
        "sun.gc.collector.0.lastEntryTime",
        "sun.gc.collector.0.lastExitTime",
        "sun.gc.collector.1.invocations",
        "sun.gc.collector.1.time",
        "sun.gc.collector.1.lastEntryTime",
        "sun.gc.collector.1.lastExitTime",
        "sun.gc.collector.2.invocations",
        "sun.gc.collector.2.time",
        "sun.gc.collector.2.lastEntryTime",
        "sun.gc.collector.2.lastExitTime",
        "sun.gc.generation.1.capacity",
        "sun.gc.generation.1.space.0.capacity",
        "sun.gc.generation.1.space.0.used",
        "sun.gc.generation.0.capacity",
        "sun.gc.generation.0.space.0.capacity",
        "sun.gc.generation.0.space.0.used",
        "sun.gc.generation.0.space.1.capacity",
        "sun.gc.generation.0.space.1.used",
        "sun.gc.generation.0.space.2.capacity",
        "sun.gc.generation.0.space.2.used",
        "sun.gc.generation.0.agetable.bytes.00",
        "sun.gc.tlab.allocThreads",
        "sun.gc.tlab.fills",
        "sun.gc.tlab.maxFills",
        "sun.gc.tlab.alloc",
        "sun.gc.tlab.gcWaste",
        "sun.gc.tlab.maxGcWaste",
        "sun.gc.tlab.refillWaste",
        "sun.gc.tlab.maxRefillWaste",
        "sun.gc.tlab.slowAlloc",
        "sun.gc.tlab.maxSlowAlloc",
        "sun.gc.metaspace.capacity",
        "sun.gc.metaspace.maxCapacity",
        "sun.gc.metaspace.used",
    ]
    .into_iter()
    .collect::<HashSet<&str>>();
    let mut monitors: Vec<JvmMonitor> = hsperf::JavaVirtualMachine::list_jvms()
        .into_iter()
        .flat_map(|jvm| jvm.monitor())
        .map(|monitor| monitor.only(|s| properies_of_interest.contains(s)))
        .collect();
    if monitors.is_empty() {
        println!("no jvms!");
    } else {
        for monitor in monitors.iter_mut() {
            println!("For pid: {}", monitor.pid());
            for entry in monitor.constants() {
                println!("\t{}: {:?}", entry.name(), entry.value());
            }
            for entry in monitor.refresh().unwrap() {
                println!("\t{}: {:?}", entry.name(), entry.value());
            }
        }
        for i in 0..1000 {
            for monitor in monitors.iter_mut() {
                println!("For pid: {}, update {i}", monitor.pid());
                let entries = &mut monitor.refresh().unwrap();
                for entry in entries.iter() {
                    println!("\t{}: {:?}", entry.name(), entry.value());
                }
            }
            sleep(2);
        }
    }
}
