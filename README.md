# Basic System Dashboard

Windows masaüstünde CPU ve RAM kullanımını gösteren küçük, sürekli üstte kalan bir **Rust** widget'ı. Python kurulumu gerekmez.

## Özellikler

- CPU ve RAM kullanım yüzdesi ile kullanılan/toplam bellek
- Her saniye otomatik yenileme
- 300 × 150 piksel, çerçevesiz ve sürekli üstte pencere
- Üst bölümden sürükleyerek taşıma ve `X` ile kapatma
- Tek dosyalı Windows EXE

## Çalıştırma

GitHub'daki `release/SystemResourceWidget.exe` dosyasını indirin ve açın. Derleme sonrası en güncel dosya **Build Windows Rust EXE** iş akışındaki `SystemResourceWidget-Windows-Rust` artifact'ında da bulunur. EXE dijital olarak imzalanmamıştır.

Rust kaynak kodundan derlemek için:

```powershell
git clone https://github.com/musaoguzdeniz-cc/Basic-System-Dashboard.git
cd Basic-System-Dashboard
cargo check --locked
cargo build --release --locked
.\target\release\SystemResourceWidget.exe
```

Windows 10/11 x64 ve Rust stable gerekir. Kaynak dosya `src/main.rs`, bağımlılıklar `Cargo.toml` içindedir. Eski Python sürümü geçiş dönemi için depoda tutulmuştur; uygulamanın ana derleme yolu Rust'tır.

## English

Basic System Dashboard is a compact, always-on-top **Rust** widget for Windows. It displays CPU usage, RAM usage, and used/total memory, refreshed every second. Drag the header to move it and click `X` to close. Run `release/SystemResourceWidget.exe` without installing Python, or build it with `cargo build --release --locked`. The executable is unsigned.

## License

[MIT](LICENSE)