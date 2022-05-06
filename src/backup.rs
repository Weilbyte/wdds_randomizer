use std::path::Path;
use std::fs;
use crate::data::EPSIODE_COUNT;

fn run_backup_op(archive_root: &Path, reverse: bool) {
    for i in 1..5 {
        for j in 1..EPSIODE_COUNT[i - 1]+1 {
            let data_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_data.ttarch2", i, j));
            let mesh_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_txmesh.ttarch2", i, j));
            let orig_data_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_data.ttarch2.ORIG", i, j));
            let orig_mesh_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_txmesh.ttarch2.ORIG", i, j));

            if !reverse {
                if !orig_data_archive.exists() {
                    println!("Backing up {}", data_archive.display());
                    fs::copy(&data_archive, &orig_data_archive).unwrap();
                } 
                if !orig_mesh_archive.exists() {
                    println!("Backing up {}", mesh_archive.display());
                    fs::copy(&mesh_archive, &orig_mesh_archive).unwrap();
                }
            } else {
                if orig_data_archive.exists() {
                    println!("Restoring {}", orig_data_archive.display());
                    fs::remove_file(&data_archive).unwrap();
                    fs::rename(&orig_data_archive, &data_archive).unwrap();
                }
                if orig_mesh_archive.exists() {
                    println!("Restoring {}", orig_mesh_archive.display());
                    fs::remove_file(&mesh_archive).unwrap();
                    fs::rename(&orig_mesh_archive, &mesh_archive).unwrap();
                }
            }
        }
    }
}

pub fn restore(archive_root: &Path) {
    run_backup_op(&archive_root, true);
}

pub fn backup(archive_root: &Path) {
    run_backup_op(&archive_root, false);
}