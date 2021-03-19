use crate::utils::get_query_params;
use crate::Params;

#[derive(Debug, Clone)]
pub struct CurrentRoute {
    path: String,
    params: Params,
    query: Params,
}

impl CurrentRoute {
    pub(crate) fn new(path: String, params: impl Into<Params>) -> Self {
        Self {
            path,
            params: params.into(),
            query: get_query_params().into(),
        }
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline]
    pub fn parmas(&self) -> &Params {
        &self.params
    }

    #[inline]
    pub fn query(&self) -> &Params {
        &self.query
    }
}
