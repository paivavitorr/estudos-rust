fn selection_sort(lista: &mut [i32]) {
    let n = lista.len();
    
    for i in 0..n {
        
        let mut indice_menor = i;
        
        
        for j in (i + 1)..n {
            if lista[j] < lista[indice_menor] {
                indice_menor = j;
            }
        }
        
        if indice_menor != i {
            lista.swap(i, indice_menor);
        }
    }
}

fn main() {
    let mut dados = vec![64, 25, 12, 22, 11];
    
    println!("Antes da ordenação: {:?}", dados);
    
/    selection_sort(&mut dados);
    
    println!("Depois da ordenação: {:?}", dados);
}
