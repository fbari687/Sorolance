#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

// Enum untuk melacak status masing-masing milestone
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Paid,
}

// Struktur data untuk Milestone
#[contracttype]
#[derive(Clone, Debug)]
pub struct Milestone {
    pub description: String,
    pub amount: u64, // Nominal pembayaran dalam representasi angka
    pub status: MilestoneStatus,
}

// Struktur data yang akan menyimpan detail Proyek Escrow
#[contracttype]
#[derive(Clone, Debug)]
pub struct Project {
    pub id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub name: String,
    pub milestones: Vec<Milestone>,
    pub is_completed: bool,
}

// Storage key untuk data projects
const PROJECT_DATA: Symbol = symbol_short!("PROJ_DATA");

#[contract]
pub struct SorolanceContract;

#[contractimpl]
impl SorolanceContract {
    // Fungsi untuk mendapatkan semua proyek
    pub fn get_projects(env: Env) -> Vec<Project> {
        // 1. ambil data proyek dari storage
        return env.storage().instance().get(&PROJECT_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat proyek escrow baru
    pub fn create_project(
        env: Env, 
        client: Address, 
        freelancer: Address, 
        name: String, 
        milestones: Vec<Milestone>
    ) -> String {
        // Memastikan bahwa transaksi ini benar-benar ditandatangani oleh dompet klien
        client.require_auth();

        // 1. ambil data proyek dari storage
        let mut projects: Vec<Project> = env.storage().instance().get(&PROJECT_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object project baru
        let project = Project {
            id: env.prng().gen::<u64>(),
            client: client,
            freelancer: freelancer,
            name: name,
            milestones: milestones,
            is_completed: false,
        };
        
        // 3. tambahkan project baru ke data lama
        projects.push_back(project);
        
        // 4. simpan data ke storage
        env.storage().instance().set(&PROJECT_DATA, &projects);
        
        // Catatan: Pada implementasi tingkat lanjut, di titik ini kita akan 
        // memanggil Token Contract (seperti XLM/USDC) untuk menarik dana 
        // dari `client` ke alamat Smart Contract ini (Escrow).
        
        return String::from_str(&env, "Proyek Escrow berhasil dibuat");
    }

    // Fungsi untuk freelancer mengirimkan hasil kerja
    pub fn submit_milestone(env: Env, freelancer: Address, project_id: u64, milestone_index: u32) -> String {
        // Memastikan bahwa yang menekan tombol submit adalah dompet freelancer
        freelancer.require_auth();

        // 1. ambil data proyek dari storage 
        let mut projects: Vec<Project> = env.storage().instance().get(&PROJECT_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index project yang akan diupdate menggunakan perulangan
        for i in 0..projects.len() {
            let mut project = projects.get(i).unwrap();
            
            if project.id == project_id {
                // Keamanan: Pastikan hanya freelancer yang ditugaskan yang bisa submit
                if project.freelancer != freelancer {
                    panic!("Hanya freelancer proyek ini yang dapat melakukan submit");
                }

                // Ambil dan ubah status milestone menjadi Submitted
                let mut milestone = project.milestones.get(milestone_index).unwrap();
                milestone.status = MilestoneStatus::Submitted;
                project.milestones.set(milestone_index, milestone);
                
                // Update project di array
                projects.set(i, project);
                
                // Simpan kembali ke storage
                env.storage().instance().set(&PROJECT_DATA, &projects);
                
                return String::from_str(&env, "Pekerjaan berhasil disubmit");
            }
        }

        return String::from_str(&env, "Proyek tidak ditemukan")
    }

    // Fungsi untuk klien menyetujui pekerjaan dan mencairkan dana
    pub fn approve_and_pay(env: Env, client: Address, project_id: u64, milestone_index: u32) -> String {
        // Memastikan yang menekan tombol approve adalah klien yang bersangkutan
        client.require_auth();

        // 1. ambil data proyek dari storage
        let mut projects: Vec<Project> = env.storage().instance().get(&PROJECT_DATA).unwrap_or(Vec::new(&env));

        // 2. cari project yang akan diupdate
        for i in 0..projects.len() {
            let mut project = projects.get(i).unwrap();
            
            if project.id == project_id {
                // Keamanan: Pastikan yang melakukan approve adalah klien pembuat proyek
                if project.client != client {
                    panic!("Hanya klien pembuat proyek yang dapat menyetujui pencairan");
                }

                let mut milestone = project.milestones.get(milestone_index).unwrap();
                
                // Pastikan freelancer sudah submit sebelum klien bisa approve
                if milestone.status != MilestoneStatus::Submitted {
                    panic!("Freelancer belum melakukan submit pada milestone ini");
                }

                // Ubah status milestone menjadi Paid
                milestone.status = MilestoneStatus::Paid;
                project.milestones.set(milestone_index, milestone);
                
                // Update project di array
                projects.set(i, project);
                
                // Simpan kembali ke storage
                env.storage().instance().set(&PROJECT_DATA, &projects);

                // Catatan: Pada implementasi tingkat lanjut, di titik ini Smart Contract 
                // akan mengeksekusi transfer token (XLM/USDC) dari Escrow ke alamat `freelancer`.

                return String::from_str(&env, "Dana berhasil dicairkan ke Freelancer");
            }
        }

        return String::from_str(&env, "Proyek tidak ditemukan");
    }
}

mod test;