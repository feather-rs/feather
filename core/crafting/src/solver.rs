//! Solves a crafting output given a set of recipes
//! and the input grid.

use crate::{
    recipe::{Recipe, ShapedRecipe, ShapelessRecipe},
    Grid, TABLE_SIZE, TABLE_WIDTH,
};
use ahash::AHashMap;
use arrayvec::ArrayVec;
use feather_items::{Item, ItemStack};

/// Stores the set of all known recipes.
#[derive(Debug, Clone, Default)]
pub struct Solver {
    /// Mapping from the input grid of a shaped recipe
    /// to the output stack.
    /// Grids are normalized to allow for an efficient O(1) lookup.
    shaped: AHashMap<Grid, ItemStack>,
    /// Mapping from the input set of ingredients for a shapeless
    /// recipe to the output stack.
    ///
    /// Input sets should be sorted using the `Ord` impl for `ItemStack`.
    shapeless: AHashMap<ArrayVec<[Item; TABLE_SIZE]>, ItemStack>,
}

impl Solver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Given an input crafting grid, attempts to find
    /// an item to craft. Returns the craftable item stack,
    /// or `None` if the grid satisfies no recipes.
    pub fn solve(&self, input: &mut Grid) -> Option<ItemStack> {
        normalize(input);

        // Try shaped first, then shapeless.
        if let Some(output) = self.shaped.get(input).copied() {
            Some(output)
        } else {
            // Sort inputs to perform a shapeless lookup.
            let mut inputs = ArrayVec::new();
            input
                .iter()
                .flatten()
                .filter_map(|slot| *slot)
                .for_each(|item| inputs.push(item));
            inputs.sort_unstable();

            if let Some(output) = self.shapeless.get(&inputs).copied() {
                Some(output)
            } else {
                None
            }
        }
    }

    /// Registers a recipe with this `Solver`. Future calls to `solve()`
    /// will account for the new recipe.
    pub fn register(&mut self, recipe: Recipe) {
        match recipe {
            Recipe::Shaped(shaped) => self.register_shaped(shaped),
            Recipe::Shapeless(shapeless) => self.register_shapeless(shapeless),
        }
    }

    fn register_shaped(&mut self, shaped: ShapedRecipe) {
        self.shaped.insert(shaped.input, shaped.output);
    }

    fn register_shapeless(&mut self, shapeless: ShapelessRecipe) {
        self.shapeless.insert(shapeless.input, shapeless.output);
    }
}

/// Normalizes a crafting grid to remove the upper empty rows and the
/// leftmost empty columns.
pub fn normalize(grid: &mut Grid) {
    // Find number of empty upper rows
    let mut y = 0;
    while is_empty(grid, (0, y), (1, y), (2, y)) && y < TABLE_WIDTH - 1 {
        y += 1;
    }

    // Find number of empty leftmost columns
    let mut x = 0;
    while is_empty(grid, (x, 0), (x, 1), (x, 2)) && x < TABLE_WIDTH - 1 {
        x += 1;
    }

    translate(grid, x, y);
}

/// Translates a crafting grid by the given amount
/// upward and to the left.
/// Items translated outside of the grid will be eliminated.
pub fn translate(grid: &mut Grid, x: usize, y: usize) {
    // Translate to the left
    grid.copy_within(x..TABLE_WIDTH, 0);
    // Upward
    for column in grid.iter_mut() {
        column.copy_within(y..TABLE_WIDTH, 0);
    }

    // Fill in newly unused space with empty slots
    for item in grid.iter_mut().skip(TABLE_WIDTH - x) {
        *item = [None, None, None];
    }
    for column in TABLE_WIDTH - y..TABLE_WIDTH {
        for row in grid.iter_mut() {
            row[column] = None;
        }
    }
}

/// Transposes a grid (converts between row-major and column-major format.)
pub fn transpose(grid: &Grid) -> Grid {
    [
        [grid[0][0], grid[1][0], grid[2][0]],
        [grid[0][1], grid[1][1], grid[2][1]],
        [grid[0][2], grid[1][2], grid[2][2]],
    ]
}

fn is_empty(
    grid: &Grid,
    coord1: (usize, usize),
    coord2: (usize, usize),
    coord3: (usize, usize),
) -> bool {
    grid[coord1.0][coord1.1].is_none()
        && grid[coord2.0][coord2.1].is_none()
        && grid[coord3.0][coord3.1].is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_items::Item;

    #[test]
    fn solve_shaped() {
        let mut solver = Solver::new();

        let recipe = Recipe::Shaped(ShapedRecipe {
            input: transpose(&[
                [Some(Item::OakPlanks), None, None],
                [Some(Item::OakPlanks), None, None],
                [None, None, None],
            ]),
            output: ItemStack::new(Item::Stick, 4),
        });
        solver.register(recipe);

        let mut input = transpose(&[
            [None, None, None],
            [None, None, Some(Item::OakPlanks)],
            [None, None, Some(Item::OakPlanks)],
        ]);
        assert_eq!(
            solver.solve(&mut input),
            Some(ItemStack::new(Item::Stick, 4))
        );

        let mut input = transpose(&[
            [None, Some(Item::OakPlanks), Some(Item::OakPlanks)],
            [None, None, None],
            [None, None, None],
        ]);
        assert_eq!(solver.solve(&mut input), None);
    }

    #[test]
    fn solve_shapeless() {
        let mut solver = Solver::new();

        let mut input = vec![Item::Flint, Item::IronIngot];
        input.sort_unstable();
        let recipe = Recipe::Shapeless(ShapelessRecipe {
            input: input.into_iter().collect(),
            output: ItemStack::new(Item::FlintAndSteel, 1),
        });
        solver.register(recipe);

        let mut input = [
            [None, None, Some(Item::Flint)],
            [None, Some(Item::IronIngot), None],
            [None, None, None],
        ];
        assert_eq!(
            solver.solve(&mut input),
            Some(ItemStack::new(Item::FlintAndSteel, 1))
        );

        let mut input = [
            [None, None, Some(Item::Flint)],
            [None, Some(Item::IronIngot), None],
            [None, Some(Item::IronIngot), None],
        ];
        assert_eq!(solver.solve(&mut input), None);
    }

    #[test]
    fn test_normalize() {
        let stack = Item::Stone;
        let stack2 = Item::DiamondSword;
        let mut grid = transpose(&[
            [None, None, None],
            [None, None, Some(stack)],
            [None, None, Some(stack2)],
        ]);

        normalize(&mut grid);

        assert_eq!(
            grid,
            transpose(&[
                [Some(stack), None, None],
                [Some(stack2), None, None],
                [None, None, None],
            ]),
        );
    }

    #[test]
    fn test_normalize_empty() {
        let mut grid = [[None, None, None], [None, None, None], [None, None, None]];
        normalize(&mut grid);

        for row in &mut grid {
            for slot in row {
                assert_eq!(*slot, None);
            }
        }
    }

    #[test]
    fn test_translate() {
        let stack = Item::Cactus;
        let stack2 = Item::WoodenHoe;
        let mut grid = [
            [None, Some(stack), None],
            [None, Some(stack2), Some(stack)],
            [None, None, Some(stack)],
        ];

        translate(&mut grid, 1, 1);

        assert_eq!(
            grid,
            [
                [Some(stack2), Some(stack), None],
                [None, Some(stack), None],
                [None, None, None],
            ]
        );
    }
}
