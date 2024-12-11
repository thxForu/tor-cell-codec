use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tor_cell::chancell::{codec::ChannelCodec, msg::AnyChanMsg, ChanMsg, CELL_DATA_LEN};
use bytes::BytesMut;


#[pymodule]
fn tor_cell_codec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_channel_cell, m)?)?;
    m.add_function(wrap_pyfunction!(encode_channel_cell, m)?)?;
    Ok(())
}

/// decode cell data into (circ_id, command, body)
#[pyfunction]
fn decode_channel_cell(py: Python, data: &[u8]) -> PyResult<(u32, u8, PyObject)> {
    validate_cell_length(data)?;
    let mut codec = ChannelCodec::new(4);
    let mut bytes = BytesMut::from(data);
    
    match codec.decode_cell::<AnyChanMsg>(&mut bytes) {
        Ok(Some(cell)) => {
            let circid = cell.circid().map(|id| u32::from(id)).unwrap_or(0);
            let cmd: u8 = cell.msg().cmd().into();
            let body = extract_cell_body(data, cmd)?;
            Ok((circid, cmd, PyBytes::new(py, body).to_object(py)))
        },
        Ok(None) => Err(pyo3::exceptions::PyValueError::new_err("incomplete cell")),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("decode error: {}", e)))
    }
}

/// encode cell parameters into bytes
#[pyfunction]
fn encode_channel_cell(py: Python, circ_id: u32, command: u8, body: &[u8]) -> PyResult<PyObject> {
    validate_cell_params(circ_id, command, body)?;

    let circid = if circ_id == 0 {
        None  
    } else {
        tor_cell::chancell::CircId::new(circ_id)
    };

    let mut codec = ChannelCodec::new(4);
    let mut out = BytesMut::new();
    
    let cell = tor_cell::chancell::ChanCell::new(
        circid, 
        AnyChanMsg::Unrecognized(tor_cell::chancell::msg::Unrecognized::new(command.into(), body))
    );
    
    match codec.write_cell(cell, &mut out) {
        Ok(()) => Ok(PyBytes::new(py, &out).to_object(py)),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("encode error: {}", e)))
    }
}

// validation helpers 

fn validate_cell_length(data: &[u8]) -> PyResult<()> {
    if data.len() < 5 {
        return Err(pyo3::exceptions::PyValueError::new_err("cell too short"));
    }
    Ok(())
}

fn validate_cell_params(circ_id: u32, command: u8, body: &[u8]) -> PyResult<()> {
    if !is_var_len_cmd(command) && body.len() > CELL_DATA_LEN {
        return Err(pyo3::exceptions::PyValueError::new_err("body too long"));
    }
    validate_circid_requirements(command, circ_id != 0)?;
    Ok(())
}

fn validate_circid_requirements(cmd: u8, has_circid: bool) -> PyResult<()> {
    match cmd {
        0 | 7 | 128 if has_circid => {
            Err(pyo3::exceptions::PyValueError::new_err("command requires no circuit id"))
        },
        1..=6 | 8..=127 if !has_circid => {
            Err(pyo3::exceptions::PyValueError::new_err("command requires circuit id"))
        },
        _ => Ok(())
    }
}

// helper functions

fn extract_cell_body<'a>(data: &'a [u8], cmd: u8) -> PyResult<&'a [u8]> {
    if is_var_len_cmd(cmd) {
        extract_var_body(data)
    } else {
        extract_fixed_body(data)
    }
}

fn extract_var_body<'a>(data: &'a [u8]) -> PyResult<&'a [u8]> {
    if data.len() < 7 {
        return Err(pyo3::exceptions::PyValueError::new_err("truncated var-len cell"));
    }
    let body_len = ((data[5] as usize) << 8) | (data[6] as usize);
    if data.len() < 7 + body_len {
        return Err(pyo3::exceptions::PyValueError::new_err("truncated var-len body"));
    }
    Ok(&data[7..7+body_len])
}

fn extract_fixed_body<'a>(data: &'a [u8]) -> PyResult<&'a [u8]> {
    if data.len() < 5 + CELL_DATA_LEN {
        return Err(pyo3::exceptions::PyValueError::new_err("truncated fixed-len cell"));
    }
    let body = &data[5..5+CELL_DATA_LEN];
    let len = body.iter().rposition(|&b| b != 0).map_or(0, |p| p + 1); 
    Ok(&body[..len])
}

fn is_var_len_cmd(cmd: u8) -> bool {
    cmd == 7 || cmd >= 128
}