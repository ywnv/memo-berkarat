fn main(){
    let nama = "Fransiska";
    let mut umur = 21; // variable mutable
    
    println!("Nama: {}", nama); 

    let nama = &[&nama,"Puspita"].join(" "); // variable shadowing dan pengabungan string 
    println!("Nama Lengkap: {}",&nama);
    println!("Umur: {}",umur);

    // mutable variable
    umur = umur + 1; // pengubahan nilai pada variable mutable
    println!("Umur tahun depan: {}",umur);
}

