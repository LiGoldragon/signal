use signal::NameDigest;
fn main() {
    let digest = NameDigest::of_bytes(b"signal identifier proof");
    println!("local={} ({} bits)", digest.local(), signal::LOCAL_BITS);
    println!(
        "cluster={} ({} bits)",
        digest.cluster(),
        signal::CLUSTER_BITS
    );
    println!("public={} ({} bits)", digest.public(), signal::PUBLIC_BITS);
    println!("digest={:02x?} (full, non-authenticating digest)", digest.0);
}
