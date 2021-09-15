echo "cat /sys/class/net/lo/statistics/tx_packets" | RUST_LOG=debug ../../target/debug/pipe --id tx_packets
