use {
    futures::{executor::block_on, future, prelude::*},
    heim::process::{Process, ProcessResult},
    metrics::counter,
};

fn process_cpu_time(process: &Process, pid_s: String) -> impl Future<Output = ProcessResult<()>> {
    process.cpu_time().and_then(move |time| {
        counter!("process_cpu_seconds_total", time.user().get() as u64,
                 "mode" => "user", "pid" => pid_s.clone());
        counter!("process_cpu_seconds_total", time.system().get() as u64,
                 "mode" => "system", "pid" => pid_s);
        future::ok(())
    })
}

fn process_gather() {
    let _ = block_on(
        Process::current()
            .map_ok(|process| {
                let pid_s = process.pid().to_string();

                let mut vec = vec![];
                vec.push(process_cpu_time(&process, pid_s.clone()));

                future::join_all(vec)
            })
            .then(|_| future::ready(())),
    );
}

pub fn register_proxy(sink: &mut metrics_runtime::Sink) {
    sink.proxy("proxy", || {
        process_gather();
        vec![]
    });
}
