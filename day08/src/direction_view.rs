use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};

pub struct DirectionViews<T> {
    grid: Array2<T>,
}

impl<T> DirectionViews<T> {
    pub fn new(grid: Array2<T>) -> Self {
        DirectionViews { grid }
    }

    pub fn left_to_right(&self) -> ArrayView2<T> {
        self.grid.view()
    }

    pub fn right_to_left(&self) -> ArrayView2<T> {
        let mut view = self.grid.view();
        view.invert_axis(Axis(1));
        view
    }

    pub fn top_to_bottom(&self) -> ArrayView2<T> {
        self.grid.t()
    }

    pub fn bottom_to_top(&self) -> ArrayView2<T> {
        let mut view = self.grid.t();
        view.invert_axis(Axis(1));
        view
    }

    pub fn left_to_right_mut(&mut self) -> ArrayViewMut2<T> {
        self.grid.view_mut()
    }

    pub fn right_to_left_mut(&mut self) -> ArrayViewMut2<T> {
        let mut view = self.grid.view_mut();
        view.invert_axis(Axis(1));
        view
    }

    pub fn top_to_bottom_mut(&mut self) -> ArrayViewMut2<T> {
        self.grid.view_mut().reversed_axes()
    }

    pub fn bottom_to_top_mut(&mut self) -> ArrayViewMut2<T> {
        let mut view = self.grid.view_mut().reversed_axes();
        view.invert_axis(Axis(1));
        view
    }

    pub fn grid(&self) -> &Array2<T> {
        &self.grid
    }
}
