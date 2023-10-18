// main.rs
mod block; // Importa el módulo block

use block::Blockchain; // Importa la estructura Blockchain desde el módulo block

fn main() {
    let mut blockchain = Blockchain::new(); // Crea una nueva instancia de la Blockchain

    // Agrega algunos bloques a la cadena de bloques
    blockchain
        .add_block("Data for Block 1".to_string())
        .unwrap();
    blockchain
        .add_block("Data for Block 2".to_string())
        .unwrap();

    // Imprime la cadena de bloques
    for block in &blockchain.blocks {
        println!("Hash: {}", block.get_hash());
        println!("Data: {}", block.transactions);
        println!("Height: {}\n", block.height);
    }
}
