use std::path::Path;
use std::fs;
use rand::{self, Rng};
use crate::data::EPSIODE_COUNT;
use crate::ttarchext::Ttarchext;

fn unpack_data(i: usize, j: usize, archive_root: &Path, ttarchext: &Ttarchext, output_to: &Path) {
    let data_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_data.ttarch2", i, j));
    ttarchext.archive_to_folder(&data_archive, &output_to);
}

fn unpack_mesh(i: usize, j: usize, archive_root: &Path, ttarchext: &Ttarchext, output_to: &Path) {
    let mesh_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_txmesh.ttarch2", i, j));
    ttarchext.archive_to_folder(&mesh_archive, &output_to);
}

fn pack_data(i: usize, j: usize, archive_root: &Path, ttarchext: &Ttarchext, folder: &Path) {
    let data_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_data.ttarch2", i, j));
    ttarchext.folder_to_archive(&folder, &data_archive)
}

fn pack_mesh(i: usize, j: usize, archive_root: &Path, ttarchext: &Ttarchext, folder: &Path) {
    let mesh_archive = archive_root.join(&format!("WDC_pc_WalkingDead{}0{}_txmesh.ttarch2", i, j));
    ttarchext.folder_to_archive(&folder, &mesh_archive);
}

fn prepare_temp_folder(temp_folder: &Path) {
    if temp_folder.exists() {
        fs::remove_dir_all(&temp_folder).unwrap();
    }
    fs::create_dir(&temp_folder).unwrap();
}

fn get_swap_index(swap_from: &str, swap_list: &[&str]) -> usize {
    let index = rand::thread_rng().gen_range(0..swap_list.len());
    let candidate = swap_list[index];
    if candidate == swap_from {
        return get_swap_index(swap_from, swap_list);
    } else {
        index
    }
}

fn swap(i: usize, j: usize, data_folder: &Path, mesh_folder: &Path) {
    let list = crate::data::REPLACABLE_LIST[i-1][j-1].clone();
    let tmp_file = data_folder.join("temp.file");
    
    for file in fs::read_dir(mesh_folder).unwrap() {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();
        let file_skl = data_folder.join(format!("{}.skl", file_name));

        if file_path.extension().unwrap() != "d3dmesh" {
            continue;
        }

        // if file_name is in REPLACABLE_LIST[i-1][j-1]
        if list.contains(&file_name) {
            // get random file in list
            let random_name = list[get_swap_index(&file_name, &list)];
            let random_mesh = mesh_folder.join(format!("{}.d3dmesh", random_name));
            let random_skl = data_folder.join(format!("{}.skl", random_name));

            //println!("{} swapped with {}", file_name, random_name);
            
            // swap mesh
            fs::rename(&file_path, &tmp_file).unwrap();
            fs::rename(&random_mesh, &file_path).unwrap();
            fs::rename(&tmp_file, &random_mesh).unwrap();

            // swap skeleton (if exists)
            if random_skl.exists() && file_skl.exists() {
                fs::rename(&file_skl, &tmp_file).unwrap();
                fs::rename(&random_skl, &file_skl).unwrap();
                fs::rename(&tmp_file, &random_skl).unwrap();
            }
        }
    }

}

pub fn randomize(archive_root: &Path) {
    let ttarchext = Ttarchext::new();
    let temp_folder = archive_root.join("randomizer_temp");
    prepare_temp_folder(&temp_folder);

    // 1..{SUPPORTED SEASONS + 1}
    // 1..5 - ALL SEASONS
    // 1..2 - JUST S1
    for i in 1..2 {
        for j in 1..EPSIODE_COUNT[i - 1]+1 {
            let temp_data = archive_root.join("tmp_data");
            let temp_mesh = archive_root.join("tmp_mesh");
            prepare_temp_folder(&temp_data);
            prepare_temp_folder(&temp_mesh);

            println!("Unpacking S{}E{}", i, j);
            unpack_data(i, j, archive_root, &ttarchext, &temp_data);
            unpack_mesh(i, j, archive_root, &ttarchext, &temp_mesh);

            println!("Randomizing S{}E{}", i, j);
            swap(i, j, &temp_data, &temp_mesh);

            println!("Packing S{}E{}", i, j);
            pack_data(i, j, archive_root, &ttarchext, &temp_data);
            pack_mesh(i, j, archive_root, &ttarchext, &temp_mesh);
        }
    }
}
