#!/usr/bin/env nu

# Measure app-iced's proportional and private memory on each Linux wgpu backend.

def read-memory [pid: int] {
    let smaps = $"/proc/($pid)/smaps_rollup"

    if not ($smaps | path exists) {
        error make {msg: $"app-iced PID ($pid) exited before it could be measured"}
    }

    let memory = (
        open --raw $smaps
        | decode utf-8
        | lines
        | parse --regex '^(?<key>Rss|Pss|Private_Clean|Private_Dirty|Private_Hugetlb):\s+(?<value>\d+)\s+kB$'
        | update value {|row| $row.value | into int }
    )

    let kib = {|key|
        $memory
        | where key == $key
        | get 0.value
    }

    {
        rss_mib: ((do $kib Rss) / 1024.0)
        pss_mib: ((do $kib Pss) / 1024.0)
        private_mib: (
            ((do $kib Private_Clean)
                + (do $kib Private_Dirty)
                + (do $kib Private_Hugetlb)) / 1024.0
        )
    }
}

def stop-process [pid: int] {
    do { ^kill -TERM $pid } | complete | ignore

    for _ in 1..10 {
        if not ($"/proc/($pid)" | path exists) {
            return
        }
        sleep 100ms
    }

    if ($"/proc/($pid)" | path exists) {
        do { ^kill -KILL $pid } | complete | ignore
    }
}

def main [
    --runs: int = 1              # Number of measurements per backend.
    --duration: duration = 10sec # How long each app instance runs before sampling.
    --binary: path               # Use an existing app-iced binary instead of building one.
] {
    if $nu.os-info.name != "linux" {
        error make {msg: "this script requires Linux /proc memory accounting"}
    }
    if $runs < 1 {
        error make {msg: "--runs must be at least 1"}
    }
    if $duration <= 0sec {
        error make {msg: "--duration must be greater than zero"}
    }

    let project_root = $env.FILE_PWD | path dirname
    let executable = if $binary == null {
        print "Building app-iced in release mode..."
        cd $project_root
        ^cargo build --release --package app-iced
        $project_root | path join target release app-iced
    } else {
        $binary | path expand --no-symlink
    }

    if not ($executable | path exists) {
        error make {msg: $"app-iced binary not found at ($executable)"}
    }

    let backends = [
        {name: default, environment: "", runs: 1}
        {name: vulkan, environment: vulkan, runs: $runs}
        {name: gl, environment: gl, runs: $runs}
    ]
    mut results = []

    for backend in $backends {
        for run in 1..$backend.runs {
            print --stderr $"Measuring ($backend.name), run ($run)/($backend.runs)..."

            # Redirect all inherited descriptors so the launcher can exit immediately.
            let pid = (
                ^sh -c 'if [ -n "$1" ]; then export WGPU_BACKEND="$1"; else unset WGPU_BACKEND; fi; "$2" >/dev/null 2>&1 & echo $!' sh $backend.environment $executable
                | str trim
                | into int
            )
            sleep $duration

            let sample = try {
                read-memory $pid
            } catch {|error|
                stop-process $pid
                error make $error
            }

            stop-process $pid

            $results = $results | append {
                backend: $backend.name
                run: $run
                rss_mib: ($sample.rss_mib | math round --precision 2)
                pss_mib: ($sample.pss_mib | math round --precision 2)
                private_mib: ($sample.private_mib | math round --precision 2)
            }
        }
    }

    $results | table --index false
}
