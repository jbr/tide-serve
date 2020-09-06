use std::fmt::{self, Debug, Display};
use tide::listener::{Listener, ToListener};

pub struct DynListener<T> {
    l: Box<dyn Listener<T>>,
}

impl<T> Display for DynListener<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.l, f)
    }
}

impl<T> Debug for DynListener<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.l, f)
    }
}

impl<T: Clone + Send + Sync + 'static> DynListener<T> {
    pub fn new<TL, L>(tl: TL) -> Self
    where
        TL: ToListener<T, Listener = L>,
        L: Listener<T>,
    {
        Self {
            l: Box::new(tl.to_listener().unwrap()),
        }
    }
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Listener<T> for DynListener<T> {
    async fn listen(&mut self, app: tide::Server<T>) -> std::io::Result<()> {
        self.l.listen(app).await
    }
}

impl<T: Clone + Send + Sync + 'static> ToListener<T> for DynListener<T> {
    type Listener = DynListener<T>;
    fn to_listener(self) -> std::io::Result<Self::Listener> {
        Ok(self)
    }
}
