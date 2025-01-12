use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

pub fn migrate(microkv: &MicroKV) -> color_eyre::Result<()> {
    let n_microkv = microkv.namespace("task-darwinia-ethereum");
    migrate_tracker_ethereum(&n_microkv)?;
    migrate_tracker_darwinia(&n_microkv)?;
    migrate_affirm(&n_microkv)?;
    Ok(())
}

fn migrate_tracker_ethereum(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    if let Some(value) = microkv.get("scan.ethereum.finish")? {
        if value.is_number() {
            let last_block = value.as_u64().unwrap();
            microkv.put("scan.ethereum.redeem.current", &last_block)?;
            microkv.put("scan.ethereum.check.current", &last_block)?;
            microkv.put("scan.ethereum.affirm.current", &last_block)?;
        }
    }
    if let Some(value) = microkv.get("scan.ethereum.running")? {
        let mut is_running = false;
        if value.is_boolean() {
            is_running = value.as_bool().unwrap_or(false);
        }
        if value.is_string() {
            is_running = value.as_str().map_or(false, |v| v == "true");
        }
        if is_running {
            microkv.put("scan.ethereum.redeem.running", &true)?;
            microkv.put("scan.ethereum.check.running", &true)?;
            microkv.put("scan.ethereum.affirm.running", &true)?;
        }
    }
    for key in &[
        "scan.ethereum.running",
        "scan.ethereum.finish",
        "scan.ethereum.current",
        "scan.ethereum.next",
        "scan.ethereum.skipped",
        "scan.ethereum.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_tracker_darwinia(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    if let Some(value) = microkv.get("scan.darwinia.finish")? {
        if value.is_number() {
            microkv.put("scan.darwinia.current", &value.as_u64().unwrap_or(0))?;
        }
    }
    for key in &[
        "scan.darwinia.finish",
        "scan.darwinia.next",
        "scan.darwinia.skipped",
        "scan.darwinia.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_affirm(microkv: &NamespaceMicroKV) -> color_eyre::Result<()> {
    if let Some(value) = microkv.get("target")? {
        if value.is_number() {
            microkv.put("affirm.target", &value.as_u64().unwrap_or(0))?;
        }
        microkv.delete("target")?;
    }
    if let Some(value) = microkv.get("relayed")? {
        if value.is_number() {
            microkv.put("affirm.relayed", &value.as_u64().unwrap_or(0))?;
        }
        microkv.delete("relayed")?;
    }
    Ok(())
}
