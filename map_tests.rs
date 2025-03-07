#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_map() {
        let map = Map::new(10, 10, 42);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert!(map.grid.len() == 10 && map.grid[0].len() == 10); // Vérifier que la grille est bien de la bonne taille
    }

    #[test]
    fn test_collect_resource() {
        let mut map = Map::new(10, 10, 42);
        
        // Essayer de récolter une ressource sur une cellule
        let x = 0;
        let y = 0;
        map.grid[y][x] = Cell::Energy; // Simuler une cellule d'énergie
        let result = map.collect_resource(x, y);
        
        assert_eq!(result, Some(Cell::Energy)); // Vérifier qu'une ressource d'énergie a bien été collectée
        assert_eq!(map.grid[y][x], Cell::Empty); // Vérifier que la cellule est maintenant vide
    }
}
