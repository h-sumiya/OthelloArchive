mod base;
mod calc;
mod connect;
mod display;
mod kif;
mod mask;
mod pos;
use std::borrow::Cow;

use pyo3::prelude::*;

#[pyfunction]
fn _load_kif(data: &str) -> PyResult<Cow<[u8]>> {
    let res = kif::load_kif(data);
    match res {
        Ok(v) => {
            let mut res = Vec::with_capacity(v.len() * 16);
            for i in v {
                res.extend_from_slice(&i.pysave());
            }
            Ok(res.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
    }
}

#[pyfunction]
fn _mk_data(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let b = base::Board::from_pydata(data);
    let res = b.pysave();
    Ok(res.to_vec().into())
}

#[pyfunction]
fn _put(data: &[u8], p: usize) -> PyResult<Cow<[u8]>> {
    let b = base::Board::pyload(data);
    let p = pos::Pos::from_py(p);
    let next = unsafe { b.put(p) };
    let res = next.pass().pysave();
    Ok(res.to_vec().into())
}

#[pyfunction]
fn _me_legal_moves(data: &[u8]) -> PyResult<Vec<u8>> {
    let b = base::Board::pyload(data);
    let moves = unsafe { b.legal_moves() };
    Ok(connect::u64_to_pydata(moves))
}

#[pyfunction]
fn _opp_legal_moves(data: &[u8]) -> PyResult<Vec<u8>> {
    let b = base::Board::pyload(data);
    let moves = unsafe { b.opp_legal_moves() };
    Ok(connect::u64_to_pydata(moves))
}

#[pyfunction]
fn _legal_moves(data: &[u8]) -> PyResult<[Vec<u8>; 2]> {
    let b = base::Board::pyload(data);
    let me_moves = unsafe { b.legal_moves() };
    let opp_moves = unsafe { b.opp_legal_moves() };
    Ok([
        connect::u64_to_pydata(me_moves),
        connect::u64_to_pydata(opp_moves),
    ])
}

#[pyfunction]
fn _get(data: &[u8], p: usize) -> PyResult<u8> {
    let b = base::Board::pyload(data);
    Ok(b.get(p))
}

#[pyfunction]
fn _decode(data: &[u8]) -> PyResult<Vec<u8>> {
    let b = base::Board::pyload(data);
    let res = b.to_pydata();
    Ok(res.to_vec())
}

#[pyfunction]
fn _make<'a>() -> PyResult<Cow<'a, [u8]>> {
    let b = base::Board::new();
    let res = b.pysave();
    Ok(res.to_vec().into())
}

#[pyfunction]
fn _count(data: &[u8]) -> PyResult<u32> {
    let b = base::Board::pyload(data);
    Ok(b.count())
}

#[pyfunction]
fn _counts(data: &[u8]) -> PyResult<(u32, u32)> {
    let b = base::Board::pyload(data);
    Ok(b.counts())
}

#[pyfunction]
fn _cns(data: &[u8]) -> PyResult<(u32, u32)> {
    let b = base::Board::pyload(data);
    Ok(b.cns())
}

#[pymodule]
fn pyothello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_load_kif, m)?)?;
    m.add_function(wrap_pyfunction!(_mk_data, m)?)?;
    m.add_function(wrap_pyfunction!(_put, m)?)?;
    m.add_function(wrap_pyfunction!(_me_legal_moves, m)?)?;
    m.add_function(wrap_pyfunction!(_opp_legal_moves, m)?)?;
    m.add_function(wrap_pyfunction!(_legal_moves, m)?)?;
    m.add_function(wrap_pyfunction!(_get, m)?)?;
    m.add_function(wrap_pyfunction!(_decode, m)?)?;
    m.add_function(wrap_pyfunction!(_make, m)?)?;
    m.add_function(wrap_pyfunction!(_count, m)?)?;
    m.add_function(wrap_pyfunction!(_counts, m)?)?;
    m.add_function(wrap_pyfunction!(_cns, m)?)?;
    Ok(())
}
