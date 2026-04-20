#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

#[test]
fn test_sorolance_escrow_flow() {
    // 1. Inisialisasi Environment (Simulasi Blockchain)
    let env = Env::default();
    
    // Karena kontrak kita menggunakan `require_auth()` untuk verifikasi dompet,
    // kita harus menyuruh environment tes untuk menyetujui semua otorisasi (mocking).
    env.mock_all_auths();

    // 2. Mendaftarkan kontrak ke environment simulasi
    let contract_id = env.register_contract(None, SorolanceContract);
    let client = SorolanceContractClient::new(&env, &contract_id);

    // 3. Membuat dompet (Address) palsu untuk Klien dan Freelancer
    let client_address = Address::generate(&env);
    let freelancer_address = Address::generate(&env);

    // ==========================================
    // SKENARIO 1: KLIEN MEMBUAT PROYEK
    // ==========================================
    
    // Siapkan daftar milestone
    let mut milestones = Vec::new(&env);
    milestones.push_back(Milestone {
        description: String::from_str(&env, "Tahap 1: Desain UI/UX"),
        amount: 300, 
        status: MilestoneStatus::Pending,
    });
    milestones.push_back(Milestone {
        description: String::from_str(&env, "Tahap 2: Koding Smart Contract"),
        amount: 700,
        status: MilestoneStatus::Pending,
    });

    let project_name = String::from_str(&env, "Pembuatan dApp Web3");

    // Panggil fungsi create_project
    let create_result = client.create_project(
        &client_address,
        &freelancer_address,
        &project_name,
        &milestones,
    );

    // Validasi apakah pesan sukses sesuai
    assert_eq!(create_result, String::from_str(&env, "Proyek Escrow berhasil dibuat"));

    // Ambil data proyek untuk mengecek apakah benar tersimpan
    let projects = client.get_projects();
    assert_eq!(projects.len(), 1); // Harus ada 1 proyek

    let active_project = projects.get(0).unwrap();
    assert_eq!(active_project.client, client_address);
    assert_eq!(active_project.milestones.len(), 2); // Harus ada 2 milestone

    let project_id = active_project.id;

    // ==========================================
    // SKENARIO 2: FREELANCER SUBMIT PEKERJAAN
    // ==========================================
    
    // Panggil fungsi submit_milestone untuk index 0 (Tahap 1)
    let submit_result = client.submit_milestone(&freelancer_address, &project_id, &0);
    assert_eq!(submit_result, String::from_str(&env, "Pekerjaan berhasil disubmit"));

    // Validasi apakah status milestone 0 sudah berubah menjadi Submitted
    let projects_after_submit = client.get_projects();
    let milestone_0 = projects_after_submit.get(0).unwrap().milestones.get(0).unwrap();
    assert_eq!(milestone_0.status, MilestoneStatus::Submitted);

    // ==========================================
    // SKENARIO 3: KLIEN APPROVE & CAIRKAN DANA
    // ==========================================
    
    // Panggil fungsi approve_and_pay untuk index 0 (Tahap 1)
    let approve_result = client.approve_and_pay(&client_address, &project_id, &0);
    assert_eq!(approve_result, String::from_str(&env, "Dana berhasil dicairkan ke Freelancer"));

    // Validasi apakah status milestone 0 sudah berubah menjadi Paid
    let projects_after_approve = client.get_projects();
    let milestone_0_final = projects_after_approve.get(0).unwrap().milestones.get(0).unwrap();
    assert_eq!(milestone_0_final.status, MilestoneStatus::Paid);
}

// Opsional: Anda bisa menambahkan fungsi #[test] lain di bawah ini 
// untuk menguji skenario gagal, misalnya "bagaimana jika yang menekan 
// tombol approve adalah Freelancer, bukan Klien?" (Harus panic/error).