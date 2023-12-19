use crate::model::IsValue;

pub trait Source<V: IsValue> {
    fn output(&self, t: f64) -> V;
}

pub trait SourceMut<V: IsValue> {
    fn output_mut(&mut self, t: f64) -> V;
}

impl<T, V> SourceMut<V> for T
where
    T: Source<V>,
    V: IsValue,
{
    fn output_mut(&mut self, t: f64) -> V {
        self.output(t)
    }
}

pub trait Sink<V: IsValue> {
    fn input(&self, t: f64, value: &V);
}

pub trait SinkMut<V: IsValue> {
    fn input_mut(&mut self, t: f64, value: &V);
}

impl<T, V> SinkMut<V> for T
where
    T: Sink<V>,
    V: IsValue,
{
    fn input_mut(&mut self, t: f64, value: &V) {
        self.input(t, value)
    }
}

pub trait Transfer<Vi: IsValue, Vo: IsValue> {
    fn transfer(&self, t: f64, input: &Vi) -> Vo;
}

pub trait TransferMut<Vi: IsValue, Vo: IsValue> {
    fn transfer_mut(&mut self, t: f64, input: &Vi) -> Vo;
}

impl<T, Vi, Vo> TransferMut<Vi, Vo> for T
where
    T: Transfer<Vi, Vo>,
    Vi: IsValue,
    Vo: IsValue,
{
    fn transfer_mut(&mut self, t: f64, input: &Vi) -> Vo {
        self.transfer(t, input)
    }
}
