/**
 * SARPRAS CARE - FRONTEND APPLICATION LOGIC
 * UJI KOMPETENSI KEAHLIAN (UKK) RPL 2025/2026
 */

// Global State
let currentAspirasiList = [];
let categoriesList = [];
let currentAdminUser = null;
let currentImageBase64 = "";

// Initial Setup
document.addEventListener("DOMContentLoaded", () => {
    initTheme();
    loadCategories();
    checkAuthSession();
});

// ==========================================================
// 1. TEMA & NAVIGASI
// ==========================================================

function initTheme() {
    const savedTheme = localStorage.getItem("sarpras_theme") || "light";
    document.documentElement.setAttribute("data-theme", savedTheme);
    updateThemeIcon(savedTheme);
}

function toggleTheme() {
    const current = document.documentElement.getAttribute("data-theme") || "light";
    const next = current === "dark" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", next);
    localStorage.setItem("sarpras_theme", next);
    updateThemeIcon(next);
}

function updateThemeIcon(theme) {
    const btn = document.getElementById("theme-toggle");
    if (btn) {
        btn.innerHTML = theme === "dark" 
            ? '<i class="fa-solid fa-sun"></i>' 
            : '<i class="fa-solid fa-moon"></i>';
    }
}

function switchView(viewName) {
    const tabSiswa = document.getElementById("tab-btn-siswa");
    const tabAdmin = document.getElementById("tab-btn-admin");
    const viewSiswa = document.getElementById("view-siswa");
    const viewAdmin = document.getElementById("view-admin");

    if (viewName === "siswa") {
        tabSiswa.classList.add("active");
        tabAdmin.classList.remove("active");
        viewSiswa.classList.add("active");
        viewAdmin.classList.remove("active");
    } else {
        tabSiswa.classList.remove("active");
        tabAdmin.classList.add("active");
        viewSiswa.classList.remove("active");
        viewAdmin.classList.add("active");
        
        if (currentAdminUser) {
            loadDashboardStats();
            loadAdminAspirasiList();
        }
    }
}

// ==========================================================
// 2. KATEGORI SARANA
// ==========================================================

const DEFAULT_CATEGORIES = [
    { id: 1, nama_kategori: "Ruang Kelas & Perabot", deskripsi: "Meja, kursi siswa/guru, papan tulis, lemari, pintu, jendela kelas" },
    { id: 2, nama_kategori: "Kelistrikan & Elektronik", deskripsi: "Lampu penerangan, stop kontak, saklar, kipas angin, AC pendingin ruang" },
    { id: 3, nama_kategori: "Laboratorium & Komputer", deskripsi: "PC/Laptop lab, LCD proyektor, kabel VGA/HDMI, headset, jaringan internet/LAN" },
    { id: 4, nama_kategori: "Sanitasi & Toilet", deskripsi: "Kloset, keran air, wastafel, saluran pembuangan, pintu toilet, bak air" },
    { id: 5, nama_kategori: "Olahraga & Lapangan", deskripsi: "Tiang basket, gawang futsal, net voli/badminton, lapangan retak/licin" },
    { id: 6, nama_kategori: "Perpustakaan & Sarana Lain", deskripsi: "Rak buku, karpet ruang baca, AC perpus, tempat sampah, koridor/taman" }
];

async function loadCategories() {
    try {
        const res = await fetch("/api/kategori");
        if (res.ok) {
            const json = await res.json();
            if (json.success && Array.isArray(json.data) && json.data.length > 0) {
                categoriesList = json.data;
                populateCategoryDropdowns(categoriesList);
                populateCategoryTable(categoriesList);
                return;
            }
        }
    } catch (err) {
        console.warn("Koneksi API /api/kategori bermasalah, menggunakan daftar kategori default:", err);
    }

    // Fallback jika API belum aktif atau mengembalikan data kosong
    if (!categoriesList || categoriesList.length === 0) {
        categoriesList = DEFAULT_CATEGORIES;
        populateCategoryDropdowns(categoriesList);
        populateCategoryTable(categoriesList);
    }
}

function populateCategoryDropdowns(categories) {
    const studentSelect = document.getElementById("input-kategori");
    const filterSelect = document.getElementById("filter-kategori");

    let studentOptions = '<option value="">-- Pilih Kategori Sarana --</option>';
    let filterOptions = '<option value="0">Semua Kategori</option>';

    categories.forEach(k => {
        studentOptions += `<option value="${k.id}">${escapeHtml(k.nama_kategori)}</option>`;
        filterOptions += `<option value="${k.id}">${escapeHtml(k.nama_kategori)}</option>`;
    });

    if (studentSelect) {
        const curVal = studentSelect.value;
        studentSelect.innerHTML = studentOptions;
        if (curVal && categories.some(k => String(k.id) === String(curVal))) {
            studentSelect.value = curVal;
        }
    }
    if (filterSelect) {
        const curVal = filterSelect.value;
        filterSelect.innerHTML = filterOptions;
        if (curVal && (curVal === "0" || categories.some(k => String(k.id) === String(curVal)))) {
            filterSelect.value = curVal;
        }
    }
}

function populateCategoryTable(categories) {
    const tbody = document.getElementById("table-kategori-body");
    if (!tbody) return;

    if (categories.length === 0) {
        tbody.innerHTML = `<tr><td colspan="4" class="text-center">Belum ada kategori.</td></tr>`;
        return;
    }

    tbody.innerHTML = categories.map(k => `
        <tr>
            <td><strong>#${k.id}</strong></td>
            <td><strong>${escapeHtml(k.nama_kategori)}</strong></td>
            <td>${escapeHtml(k.deskripsi || '-')}</td>
            <td>
                <button class="btn btn-sm btn-secondary" onclick="hapusKategori(${k.id})" title="Hapus Kategori">
                    <i class="fa-solid fa-trash text-danger"></i>
                </button>
            </td>
        </tr>
    `).join("");
}

function openKategoriModal() {
    populateCategoryTable(categoriesList);
    document.getElementById("modal-kategori").classList.remove("hidden");
}

async function submitTambahKategori(e) {
    e.preventDefault();
    const nama = document.getElementById("kat-nama").value.trim();
    const deskripsi = document.getElementById("kat-deskripsi").value.trim();

    try {
        const res = await fetch("/api/kategori", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ nama_kategori: nama, deskripsi: deskripsi || null }),
        });
        const json = await res.json();
        if (json.success) {
            showToast("Kategori baru berhasil ditambahkan!", "success");
            document.getElementById("form-tambah-kategori").reset();
            loadCategories();
        } else {
            showToast(json.message, "error");
        }
    } catch (err) {
        showToast("Terjadi error koneksi.", "error");
    }
}

async function hapusKategori(id) {
    if (!confirm("Apakah Anda yakin ingin menghapus kategori ini?")) return;

    try {
        const res = await fetch(`/api/kategori/${id}`, { method: "DELETE" });
        const json = await res.json();
        if (json.success) {
            showToast("Kategori berhasil dihapus!", "success");
            loadCategories();
        } else {
            showToast(json.message, "error");
        }
    } catch (err) {
        showToast("Gagal menghapus kategori.", "error");
    }
}

// ==========================================================
// 3. FITUR SISWA (SUBMIT ASPIRASI & CEK HISTORI)
// ==========================================================

function previewImage(event) {
    const file = event.target.files[0];
    if (!file) return;

    if (file.size > 2 * 1024 * 1024) {
        showToast("Ukuran foto maksimal 2MB", "error");
        event.target.value = "";
        return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
        currentImageBase64 = e.target.result;
        document.getElementById("image-preview-img").src = currentImageBase64;
        document.getElementById("image-preview-container").classList.remove("hidden");
        document.getElementById("upload-drop-zone").classList.add("hidden");
    };
    reader.readAsDataURL(file);
}

function removeImagePreview() {
    currentImageBase64 = "";
    const input = document.getElementById("input-foto");
    if (input) input.value = "";
    document.getElementById("image-preview-container")?.classList.add("hidden");
    document.getElementById("upload-drop-zone")?.classList.remove("hidden");
}

async function submitAspirasi(e) {
    e.preventDefault();
    const btn = document.getElementById("btn-submit-aspirasi");
    btn.disabled = true;
    btn.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Mengirim Pengaduan...`;

    const payload = {
        nisn: document.getElementById("input-nisn").value.trim(),
        nama_siswa: document.getElementById("input-nama").value.trim(),
        kelas: document.getElementById("input-kelas").value,
        id_kategori: parseInt(document.getElementById("input-kategori").value, 10),
        lokasi_sarana: document.getElementById("input-lokasi").value.trim(),
        deskripsi: document.getElementById("input-deskripsi").value.trim(),
        foto_url: currentImageBase64 || null,
    };

    try {
        const res = await fetch("/api/aspirasi", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        const json = await res.json();
        if (json.success) {
            showToast("Pengaduan berhasil diajukan! Tiket: " + json.data.kode_tiket, "success");
            document.getElementById("form-aspirasi").reset();
            removeImagePreview();

            // Set input pencarian dengan NISN yang baru saja submit lalu cari histori
            document.getElementById("track-search-input").value = payload.nisn;
            searchStudentHistory();
        } else {
            showToast(json.message, "error");
        }
    } catch (err) {
        showToast("Terjadi gangguan koneksi: " + err, "error");
    } finally {
        btn.disabled = false;
        btn.innerHTML = `<i class="fa-solid fa-paper-plane"></i> Kirim Aspirasi Sekarang`;
    }
}

async function searchStudentHistory() {
    const query = document.getElementById("track-search-input").value.trim();
    const container = document.getElementById("student-history-list");

    if (!query) {
        container.innerHTML = `
            <div class="empty-state">
                <i class="fa-solid fa-clipboard-question"></i>
                <p>Silakan masukkan NISN atau Kode Tiket Anda.</p>
            </div>`;
        return;
    }

    container.innerHTML = `<div class="text-center" style="padding: 1.5rem;"><i class="fa-solid fa-spinner fa-spin"></i> Mencari riwayat aspirasi...</div>`;

    try {
        const res = await fetch(`/api/aspirasi?search=${encodeURIComponent(query)}`);
        const json = await res.json();

        if (json.success && json.data && json.data.length > 0) {
            container.innerHTML = json.data.map(item => `
                <div class="history-card" onclick="viewDetailAspirasi(${item.id})">
                    <div class="history-header">
                        <span class="ticket-tag">${item.kode_tiket}</span>
                        ${getStatusBadge(item.status)}
                    </div>
                    <div class="history-meta">
                        <i class="fa-solid fa-calendar"></i> ${item.created_at} | 
                        <i class="fa-solid fa-shapes"></i> ${escapeHtml(item.nama_kategori || 'Umum')} | 
                        <i class="fa-solid fa-location-dot"></i> ${escapeHtml(item.lokasi_sarana)}
                    </div>
                    <div class="history-desc">
                        ${escapeHtml(item.deskripsi)}
                    </div>
                    <div>
                        <small style="font-weight: 600; color: var(--text-muted);">Progres Perbaikan: ${item.progres_terakhir || 0}%</small>
                        <div class="progress-container">
                            <div class="progress-bar" style="width: ${item.progres_terakhir || 0}%;"></div>
                        </div>
                    </div>
                </div>
            `).join("");
        } else {
            container.innerHTML = `
                <div class="empty-state">
                    <i class="fa-solid fa-circle-exclamation text-warning"></i>
                    <p>Tidak ditemukan pengaduan dengan pencarian "<strong>${escapeHtml(query)}</strong>".</p>
                </div>`;
        }
    } catch (err) {
        container.innerHTML = `<div class="text-danger text-center">Gagal memuat histori.</div>`;
    }
}

// ==========================================================
// 4. AUTENTIKASI ADMIN & PETUGAS
// ==========================================================

function checkAuthSession() {
    const saved = localStorage.getItem("sarpras_user");
    if (saved) {
        try {
            currentAdminUser = JSON.parse(saved);
            updateAdminUI(true);
        } catch (e) {
            localStorage.removeItem("sarpras_user");
            updateAdminUI(false);
        }
    } else {
        updateAdminUI(false);
    }
}

function updateAdminUI(isLoggedIn) {
    const loginCard = document.getElementById("admin-login-card");
    const dashboard = document.getElementById("admin-dashboard");
    const profileBadge = document.getElementById("user-profile-badge");
    const profileName = document.getElementById("profile-name");
    const adminNavLabel = document.getElementById("admin-nav-label");

    if (isLoggedIn && currentAdminUser) {
        loginCard.classList.add("hidden");
        dashboard.classList.remove("hidden");
        profileBadge.classList.remove("hidden");
        profileName.innerText = currentAdminUser.nama_lengkap;
        adminNavLabel.innerText = `Portal (${currentAdminUser.role.toUpperCase()})`;
    } else {
        loginCard.classList.remove("hidden");
        dashboard.classList.add("hidden");
        profileBadge.classList.add("hidden");
        adminNavLabel.innerText = "Portal Admin / Petugas";
    }
}

async function submitLogin(e) {
    e.preventDefault();
    const btn = document.getElementById("btn-login");
    btn.disabled = true;
    btn.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Masuk...`;

    const username = document.getElementById("login-username").value.trim();
    const password = document.getElementById("login-password").value.trim();

    try {
        const res = await fetch("/api/auth/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username, password }),
        });

        const json = await res.json();
        if (json.success && json.data) {
            currentAdminUser = json.data;
            localStorage.setItem("sarpras_user", JSON.stringify(currentAdminUser));
            updateAdminUI(true);
            showToast("Selamat datang, " + currentAdminUser.nama_lengkap, "success");
            loadDashboardStats();
            loadAdminAspirasiList();
        } else {
            showToast(json.message, "error");
        }
    } catch (err) {
        showToast("Gagal melakukan login: " + err, "error");
    } finally {
        btn.disabled = false;
        btn.innerHTML = `<i class="fa-solid fa-right-to-bracket"></i> Masuk Dashboard`;
    }
}

function logoutAdmin() {
    if (!confirm("Apakah Anda yakin ingin keluar?")) return;
    localStorage.removeItem("sarpras_user");
    currentAdminUser = null;
    updateAdminUI(false);
    switchView("siswa");
    showToast("Anda telah berhasil keluar.", "success");
}

// ==========================================================
// 5. ADMIN DASHBOARD & LIST ASPIRASI (MULTI-FILTER)
// ==========================================================

async function loadDashboardStats() {
    try {
        const res = await fetch("/api/stats");
        const json = await res.json();
        if (json.success && json.data) {
            const d = json.data;
            document.getElementById("stat-total").innerText = d.total_aspirasi;
            document.getElementById("stat-menunggu").innerText = d.menunggu;
            document.getElementById("stat-diproses").innerText = d.diproses;
            document.getElementById("stat-selesai").innerText = d.selesai;
            document.getElementById("stat-ditolak").innerText = d.ditolak;
        }
    } catch (err) {
        console.error("Gagal memuat statistik:", err);
    }
}

async function loadAdminAspirasiList() {
    const tbody = document.getElementById("table-aspirasi-body");
    tbody.innerHTML = `<tr><td colspan="9" class="text-center"><i class="fa-solid fa-spinner fa-spin"></i> Memuat data aspirasi...</td></tr>`;

    // Ambil parameter filter sesuai kebutuhan UKK
    const status = document.getElementById("filter-status")?.value || "semua";
    const kategori = document.getElementById("filter-kategori")?.value || "0";
    const bulan = document.getElementById("filter-bulan")?.value || "";
    const tglMulai = document.getElementById("filter-tgl-mulai")?.value || "";
    const tglSelesai = document.getElementById("filter-tgl-selesai")?.value || "";
    const search = document.getElementById("filter-search")?.value.trim() || "";

    const params = new URLSearchParams();
    if (status && status !== "semua") params.append("status", status);
    if (kategori && kategori !== "0") params.append("id_kategori", kategori);
    if (bulan) params.append("bulan", bulan);
    if (tglMulai) params.append("tanggal_mulai", tglMulai);
    if (tglSelesai) params.append("tanggal_selesai", tglSelesai);
    if (search) params.append("search", search);

    try {
        const res = await fetch(`/api/aspirasi?${params.toString()}`);
        const json = await res.json();

        if (json.success && json.data) {
            currentAspirasiList = json.data;
            renderAdminTable(currentAspirasiList);
        } else {
            tbody.innerHTML = `<tr><td colspan="9" class="text-center text-danger">Gagal memuat data.</td></tr>`;
        }
    } catch (err) {
        tbody.innerHTML = `<tr><td colspan="9" class="text-center text-danger">Koneksi terputus: ${err}</td></tr>`;
    }
}

function renderAdminTable(data) {
    const tbody = document.getElementById("table-aspirasi-body");
    if (!tbody) return;

    if (data.length === 0) {
        tbody.innerHTML = `<tr><td colspan="9" class="text-center" style="padding: 2rem;">Tidak ada data pengaduan yang sesuai dengan filter.</td></tr>`;
        return;
    }

    tbody.innerHTML = data.map(item => `
        <tr>
            <td><strong class="ticket-tag">${item.kode_tiket}</strong></td>
            <td><small>${item.created_at}</small></td>
            <td><strong>${escapeHtml(item.nama_siswa)}</strong><br><small class="text-muted">NISN: ${item.nisn}</small></td>
            <td>${escapeHtml(item.kelas)}</td>
            <td><strong>${escapeHtml(item.nama_kategori || 'Fasilitas')}</strong><br><small class="text-muted">${escapeHtml(item.lokasi_sarana)}</small></td>
            <td><div style="max-width: 250px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${escapeHtml(item.deskripsi)}</div></td>
            <td>
                <small>${item.progres_terakhir || 0}%</small>
                <div class="progress-container" style="width: 80px;">
                    <div class="progress-bar" style="width: ${item.progres_terakhir || 0}%;"></div>
                </div>
            </td>
            <td>${getStatusBadge(item.status)}</td>
            <td class="text-center">
                <button class="btn btn-sm btn-primary" onclick="viewDetailAspirasi(${item.id})" title="Lihat Detail & Umpan Balik">
                    <i class="fa-solid fa-eye"></i> Detail
                </button>
            </td>
        </tr>
    `).join("");
}

function resetFilters() {
    document.getElementById("filter-status").value = "semua";
    document.getElementById("filter-kategori").value = "0";
    document.getElementById("filter-bulan").value = "";
    document.getElementById("filter-tgl-mulai").value = "";
    document.getElementById("filter-tgl-selesai").value = "";
    document.getElementById("filter-search").value = "";
    loadAdminAspirasiList();
}

// ==========================================================
// 6. DETAIL ASPIRASI, UMPAN BALIK & STATUS UPDATE
// ==========================================================

async function viewDetailAspirasi(id) {
    try {
        const res = await fetch(`/api/aspirasi/${id}`);
        const json = await res.json();
        if (json.success && json.data) {
            renderModalDetail(json.data);
            document.getElementById("modal-detail").classList.remove("hidden");
        } else {
            showToast("Aspirasi tidak ditemukan", "error");
        }
    } catch (err) {
        showToast("Gagal mengambil detail aspirasi: " + err, "error");
    }
}

function renderModalDetail(item) {
    document.getElementById("modal-detail-ticket").innerText = item.kode_tiket;
    document.getElementById("modal-detail-nama").innerText = item.nama_siswa;
    document.getElementById("modal-detail-nisn").innerText = item.nisn;
    document.getElementById("modal-detail-kelas").innerText = item.kelas;
    document.getElementById("modal-detail-waktu").innerText = item.created_at;
    document.getElementById("modal-detail-kategori").innerText = item.nama_kategori || 'Umum';
    document.getElementById("modal-detail-lokasi").innerText = item.lokasi_sarana;
    document.getElementById("modal-detail-status").innerHTML = getStatusBadge(item.status);
    document.getElementById("modal-detail-progres").innerText = `${item.progres_terakhir || 0}%`;
    document.getElementById("modal-detail-deskripsi").innerText = item.deskripsi;

    // Foto Kerusakan
    const fotoContainer = document.getElementById("modal-detail-foto-container");
    const fotoImg = document.getElementById("modal-detail-foto");
    if (item.foto_url && item.foto_url.trim() !== "") {
        fotoImg.src = item.foto_url;
        fotoContainer.classList.remove("hidden");
    } else {
        fotoContainer.classList.add("hidden");
    }

    // Riwayat Status (Timeline)
    const historiContainer = document.getElementById("modal-detail-histori");
    if (item.histori_list && item.histori_list.length > 0) {
        historiContainer.innerHTML = item.histori_list.map(h => `
            <div class="timeline-item">
                <div style="display:flex; justify-content:space-between; font-weight:700;">
                    <span>${h.status_lama.toUpperCase()} &rarr; ${h.status_baru.toUpperCase()}</span>
                    <small class="text-muted">${h.created_at}</small>
                </div>
                <div style="font-size: 0.78rem; color: var(--text-muted);">Oleh: ${escapeHtml(h.diubah_oleh)}</div>
                ${h.catatan ? `<div style="margin-top: 2px;">${escapeHtml(h.catatan)}</div>` : ''}
            </div>
        `).join("");
    } else {
        historiContainer.innerHTML = `<p class="text-muted">Belum ada riwayat status.</p>`;
    }

    // Umpan Balik List
    const feedbackContainer = document.getElementById("modal-detail-umpan-balik");
    if (item.umpan_balik_list && item.umpan_balik_list.length > 0) {
        feedbackContainer.innerHTML = item.umpan_balik_list.map(ub => `
            <div class="feedback-item">
                <div style="display:flex; justify-content:space-between; font-weight:700;">
                    <span class="text-primary">${escapeHtml(ub.nama_petugas)}</span>
                    <small class="text-muted">${ub.created_at}</small>
                </div>
                <div style="margin-top: 3px;">${escapeHtml(ub.tanggapan)}</div>
                <div style="font-size: 0.75rem; margin-top: 4px; font-weight: 600;">
                    Progres: ${ub.progres_persen}%
                </div>
            </div>
        `).join("");
    } else {
        feedbackContainer.innerHTML = `<p class="text-muted">Belum ada tanggapan/umpan balik dari petugas.</p>`;
    }

    // Tampilkan Form Umpan Balik jika Admin / Petugas login
    const adminActionWrapper = document.getElementById("admin-feedback-form-wrapper");
    if (currentAdminUser && (currentAdminUser.role === 'admin' || currentAdminUser.role === 'petugas')) {
        adminActionWrapper.classList.remove("hidden");
        document.getElementById("ub-id-aspirasi").value = item.id;
        document.getElementById("ub-status-ke").value = item.status;
        const currentProg = item.progres_terakhir || 50;
        document.getElementById("ub-progres").value = currentProg;
        document.getElementById("ub-progres-val").innerText = currentProg + '%';
        document.getElementById("ub-tanggapan").value = "";
    } else {
        adminActionWrapper.classList.add("hidden");
    }
}

async function submitUmpanBalik(e) {
    e.preventDefault();
    const btn = document.getElementById("btn-submit-ub");
    btn.disabled = true;
    btn.innerHTML = `<i class="fa-solid fa-spinner fa-spin"></i> Menyimpan...`;

    const idAspirasi = parseInt(document.getElementById("ub-id-aspirasi").value, 10);
    const statusKe = document.getElementById("ub-status-ke").value;
    const progresPersen = parseInt(document.getElementById("ub-progres").value, 10);
    const tanggapan = document.getElementById("ub-tanggapan").value.trim();

    const payload = {
        id_aspirasi: idAspirasi,
        id_petugas: currentAdminUser ? currentAdminUser.id : null,
        nama_petugas: currentAdminUser ? currentAdminUser.nama_lengkap : "Petugas Sarpras",
        tanggapan: tanggapan,
        progres_persen: progresPersen,
        foto_perbaikan: null,
        ubah_status_ke: statusKe,
    };

    try {
        const res = await fetch("/api/umpan-balik", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        const json = await res.json();
        if (json.success) {
            showToast("Umpan balik & status berhasil diperbarui!", "success");
            // Refresh modal detail
            await viewDetailAspirasi(idAspirasi);
            // Refresh table admin & stats
            loadAdminAspirasiList();
            loadDashboardStats();
        } else {
            showToast(json.message, "error");
        }
    } catch (err) {
        showToast("Terjadi kesalahan: " + err, "error");
    } finally {
        btn.disabled = false;
        btn.innerHTML = `<i class="fa-solid fa-paper-plane"></i> Kirim Umpan Balik & Update Status`;
    }
}

function closeModal(modalId) {
    document.getElementById(modalId).classList.add("hidden");
}

// ==========================================================
// 7. CETAK LAPORAN (PRINT REPORT)
// ==========================================================

function printReport() {
    if (currentAspirasiList.length === 0) {
        showToast("Tidak ada data untuk dicetak!", "error");
        return;
    }

    const printTbody = document.getElementById("print-table-body");
    const printFilterInfo = document.getElementById("print-filter-info");
    const printDateNow = document.getElementById("print-date-now");
    const printOfficer = document.getElementById("print-officer-name");

    const statusFilter = document.getElementById("filter-status")?.value || "semua";
    const bulanFilter = document.getElementById("filter-bulan")?.value || "";
    printFilterInfo.innerText = `Periode / Filter: Status (${statusFilter.toUpperCase()}) | Bulan: ${bulanFilter || 'Semua Data'}`;

    const today = new Date();
    const formattedDate = today.toLocaleDateString('id-ID', { year: 'numeric', month: 'long', day: 'numeric' });
    printDateNow.innerText = `Kota, ${formattedDate}`;

    if (currentAdminUser) {
        printOfficer.innerText = `( ${currentAdminUser.nama_lengkap} )`;
    }

    printTbody.innerHTML = currentAspirasiList.map((item, index) => `
        <tr>
            <td>${index + 1}</td>
            <td><strong>${item.kode_tiket}</strong></td>
            <td>${item.created_at}</td>
            <td>${escapeHtml(item.nama_siswa)}<br>(${item.nisn})</td>
            <td>${escapeHtml(item.kelas)}</td>
            <td>${escapeHtml(item.nama_kategori || '-')}</td>
            <td>${escapeHtml(item.lokasi_sarana)}</td>
            <td>${escapeHtml(item.deskripsi)}</td>
            <td>${item.progres_terakhir || 0}%</td>
            <td><strong>${item.status.toUpperCase()}</strong></td>
        </tr>
    `).join("");

    window.print();
}

// ==========================================================
// 8. UTILITIES & HELPERS
// ==========================================================

function getStatusBadge(status) {
    switch (status.toLowerCase()) {
        case "menunggu":
            return `<span class="badge badge-menunggu"><i class="fa-solid fa-clock"></i> Menunggu</span>`;
        case "diproses":
            return `<span class="badge badge-diproses"><i class="fa-solid fa-screwdriver-wrench"></i> Diproses</span>`;
        case "selesai":
            return `<span class="badge badge-selesai"><i class="fa-solid fa-circle-check"></i> Selesai</span>`;
        case "ditolak":
            return `<span class="badge badge-ditolak"><i class="fa-solid fa-circle-xmark"></i> Ditolak</span>`;
        default:
            return `<span class="badge">${escapeHtml(status)}</span>`;
    }
}

function showToast(message, type = "success") {
    const toast = document.getElementById("toast");
    const msg = document.getElementById("toast-message");
    const icon = document.getElementById("toast-icon");

    toast.className = `toast ${type}`;
    msg.innerText = message;

    if (type === "success") {
        icon.className = "fa-solid fa-circle-check";
    } else {
        icon.className = "fa-solid fa-circle-exclamation";
    }

    toast.classList.remove("hidden");
    setTimeout(() => {
        toast.classList.add("hidden");
    }, 4000);
}

function escapeHtml(str) {
    if (!str) return "";
    return str
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}
