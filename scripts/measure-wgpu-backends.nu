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
        | parse --regex '^(?<key>Rss|Pss|Pss_Anon|Pss_File|Pss_Shmem|Private_Clean|Private_Dirty|Private_Hugetlb|Swap):\s+(?<value>\d+)\s+kB$'
        | update value {|row| $row.value | into int }
    )

    let kib = {|key|
        let matches = $memory | where key == $key

        if ($matches | is-empty) {
            0
        } else {
            $matches | get 0.value
        }
    }

    {
        rss_mib: ((do $kib Rss) / 1024.0)
        pss_mib: ((do $kib Pss) / 1024.0)
        pss_anon_mib: ((do $kib Pss_Anon) / 1024.0)
        pss_file_mib: ((do $kib Pss_File) / 1024.0)
        pss_shmem_mib: ((do $kib Pss_Shmem) / 1024.0)
        private_clean_mib: ((do $kib Private_Clean) / 1024.0)
        private_dirty_mib: ((do $kib Private_Dirty) / 1024.0)
        private_mib: (
            ((do $kib Private_Clean)
                + (do $kib Private_Dirty)
                + (do $kib Private_Hugetlb)) / 1024.0
        )
        swap_mib: ((do $kib Swap) / 1024.0)
    }
}

def summarize-results [results: table] {
    let metrics = [
        {name: rss, column: rss_mib}
        {name: pss, column: pss_mib}
        {name: pss_anon, column: pss_anon_mib}
        {name: pss_file, column: pss_file_mib}
        {name: pss_shmem, column: pss_shmem_mib}
        {name: private, column: private_mib}
        {name: private_clean, column: private_clean_mib}
        {name: private_dirty, column: private_dirty_mib}
        {name: swap, column: swap_mib}
    ]

    $results
    | group-by backend
    | transpose backend samples
    | each {|group|
        $metrics | each {|metric|
            let values = $group.samples | get $metric.column

            {
                backend: $group.backend
                metric: $metric.name
                runs: ($values | length)
                mean_mib: ($values | math avg | math round --precision 2)
                median_mib: ($values | math median | math round --precision 2)
                min_mib: ($values | math min | math round --precision 2)
                max_mib: ($values | math max | math round --precision 2)
                stddev_mib: ($values | math stddev | math round --precision 2)
            }
        }
    }
    | flatten
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
        {name: default, environment: "", runs: $runs}
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
                ...$sample
            }
        }
    }

    summarize-results $results | table --index false
}
