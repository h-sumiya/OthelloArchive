mod base;
mod calc;
mod cell_name;
mod connect;
mod display;
mod kif;
mod mask;
mod pack;
mod pos;
mod pyindex;
mod score;
use std::borrow::Cow;

use once_cell::sync::Lazy;
use pyo3::prelude::*;

static EVAL: Lazy<score::EvalData> = Lazy::new(|| score::EvalData::load());

#[pyfunction]
fn _parse_kif(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let kif = kif::Kif::pyload(data);
    let res = kif.parse();
    let mut data = Vec::with_capacity(res.len() * 16);
    for i in res {
        data.extend_from_slice(&i.pysave());
    }
    Ok(data.into())
}

#[pyfunction]
fn _parse_kif_with_check(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let kif = kif::Kif::pyload(data);
    let res = kif.parse_with_check();
    match res {
        Ok(v) => {
            let mut data = Vec::with_capacity(v.len() * 16);
            for i in v {
                data.extend_from_slice(&i.pysave());
            }
            Ok(data.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
    }
}

#[pyfunction]
fn _mk_kif(data: &str) -> PyResult<Cow<[u8]>> {
    let kif = kif::Kif::new(data);
    match kif {
        Ok(k) => {
            let res = k.pysave();
            Ok(res.to_vec().into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
    }
}

#[pyfunction]
fn _read_kif(data: &[u8]) -> PyResult<String> {
    let kif = kif::Kif::pyload(data);
    let res = format!("{}", kif);
    Ok(res)
}

#[pyfunction]
fn _rotate_kif(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let kif = kif::Kif::pyload(data).rotate();
    let res = kif.pysave();
    Ok(res.into())
}

#[pyfunction]
fn _miror_kif(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let kif = kif::Kif::pyload(data).miror();
    let res = kif.pysave();
    Ok(res.into())
}

#[pyfunction]
fn _mk_data<'a>(data: Vec<u8>) -> PyResult<Cow<'a, [u8]>> {
    let b = base::Board::from_pydata(data.as_slice());
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
fn _put_opp(data: &[u8], p: usize) -> PyResult<Cow<[u8]>> {
    let b = base::Board::pyload(data).pass();
    let p = pos::Pos::from_py(p);
    let next = unsafe { b.put(p) };
    let res = next.pysave();
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

#[pyfunction]
fn _edax(data: &[u8]) -> PyResult<f64> {
    let b = base::Board::pyload(data);
    let res = EVAL.evaluate(&b);
    Ok(res)
}

#[pyfunction]
fn _pyindex(data: &[u8], index: &[u8]) -> PyResult<Vec<i32>> {
    let b = base::Board::pyload(data);
    let res = b.pyindex(index);
    Ok(res)
}

#[pymodule]
fn pyothello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_parse_kif, m)?)?;
    m.add_function(wrap_pyfunction!(_parse_kif_with_check, m)?)?;
    m.add_function(wrap_pyfunction!(_mk_kif, m)?)?;
    m.add_function(wrap_pyfunction!(_read_kif, m)?)?;
    m.add_function(wrap_pyfunction!(_rotate_kif, m)?)?;
    m.add_function(wrap_pyfunction!(_miror_kif, m)?)?;
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
    m.add_function(wrap_pyfunction!(_edax, m)?)?;
    m.add_function(wrap_pyfunction!(_put_opp, m)?)?;
    m.add_function(wrap_pyfunction!(_pyindex, m)?)?;
    Ok(())
}
