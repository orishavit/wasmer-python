use crate::{errors::to_py_err, wasmer_inner::wasmer};
use pyo3::{
    conversion::{FromPyObject, IntoPy},
    exceptions::ValueError,
    prelude::*,
};
use std::{convert::TryFrom, slice};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Type {
    I32 = 1,
    I64 = 2,
    F32 = 3,
    F64 = 4,
    V128 = 5,
    ExternRef = 6,
    FuncRef = 7,
}

impl Type {
    pub fn iter() -> slice::Iter<'static, Type> {
        static VARIANTS: [Type; 7] = [
            Type::I32,
            Type::I64,
            Type::F32,
            Type::F64,
            Type::V128,
            Type::ExternRef,
            Type::FuncRef,
        ];

        VARIANTS.iter()
    }
}

impl From<&Type> for &'static str {
    fn from(value: &Type) -> Self {
        match value {
            Type::I32 => "I32",
            Type::I64 => "I64",
            Type::F32 => "F32",
            Type::F64 => "F64",
            Type::V128 => "V128",
            Type::ExternRef => "EXTERN_REF",
            Type::FuncRef => "FUNC_REF",
        }
    }
}

impl ToPyObject for Type {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}

impl IntoPy<PyObject> for Type {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for Type {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let variant = u8::extract(obj)?;

        Ok(match variant {
            1 => Self::I32,
            2 => Self::I64,
            3 => Self::F32,
            4 => Self::F64,
            5 => Self::V128,
            6 => Self::ExternRef,
            7 => Self::FuncRef,
            _ => {
                return Err(to_py_err::<ValueError, _>(
                    "Failed to extract `Type` from `PyAny`",
                ))
            }
        })
    }
}

impl From<wasmer::Type> for Type {
    fn from(value: wasmer::Type) -> Self {
        match value {
            wasmer::Type::I32 => Self::I32,
            wasmer::Type::I64 => Self::I64,
            wasmer::Type::F32 => Self::F32,
            wasmer::Type::F64 => Self::F64,
            wasmer::Type::V128 => Self::V128,
            wasmer::Type::ExternRef => Self::ExternRef,
            wasmer::Type::FuncRef => Self::FuncRef,
        }
    }
}

impl Into<wasmer::Type> for Type {
    fn into(self) -> wasmer::Type {
        match self {
            Self::I32 => wasmer::Type::I32,
            Self::I64 => wasmer::Type::I64,
            Self::F32 => wasmer::Type::F32,
            Self::F64 => wasmer::Type::F64,
            Self::V128 => wasmer::Type::V128,
            Self::ExternRef => wasmer::Type::ExternRef,
            Self::FuncRef => wasmer::Type::FuncRef,
        }
    }
}

#[pyclass]
pub struct FunctionType {
    #[pyo3(get)]
    pub params: Vec<Type>,

    #[pyo3(get)]
    pub results: Vec<Type>,
}

#[pymethods]
impl FunctionType {
    #[new]
    fn new(params: Vec<Type>, results: Vec<Type>) -> Self {
        Self { params, results }
    }
}

impl From<&wasmer::FunctionType> for FunctionType {
    fn from(value: &wasmer::FunctionType) -> Self {
        Self {
            params: value
                .params()
                .to_vec()
                .into_iter()
                .map(Into::into)
                .collect(),
            results: value
                .results()
                .to_vec()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl Into<wasmer::FunctionType> for &FunctionType {
    fn into(self) -> wasmer::FunctionType {
        wasmer::FunctionType::new(
            self.params
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
            self.results
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
        )
    }
}

#[pyclass]
pub struct MemoryType {
    #[pyo3(get)]
    pub minimum: u32,

    #[pyo3(get)]
    pub maximum: Option<u32>,

    #[pyo3(get)]
    pub shared: bool,
}

#[pymethods]
impl MemoryType {
    #[new]
    fn new(minimum: u32, maximum: Option<u32>, shared: bool) -> Self {
        Self {
            minimum,
            maximum,
            shared,
        }
    }
}

impl From<&wasmer::MemoryType> for MemoryType {
    fn from(value: &wasmer::MemoryType) -> Self {
        Self {
            minimum: value.minimum.0,
            maximum: value.maximum.map(|pages| pages.0),
            shared: value.shared,
        }
    }
}

impl Into<wasmer::MemoryType> for &MemoryType {
    fn into(self) -> wasmer::MemoryType {
        wasmer::MemoryType::new(self.minimum, self.maximum, self.shared)
    }
}

#[pyclass]
pub struct GlobalType {
    #[pyo3(get)]
    pub r#type: Type,

    #[pyo3(get)]
    pub mutable: bool,
}

#[pymethods]
impl GlobalType {
    #[new]
    fn new(r#type: Type, mutable: bool) -> Self {
        Self { r#type, mutable }
    }
}

impl From<&wasmer::GlobalType> for GlobalType {
    fn from(value: &wasmer::GlobalType) -> Self {
        Self {
            r#type: value.ty.into(),
            mutable: value.mutability.is_mutable(),
        }
    }
}

#[pyclass]
pub struct TableType {
    #[pyo3(get)]
    pub r#type: Type,

    #[pyo3(get)]
    pub minimum: u32,

    #[pyo3(get)]
    pub maximum: Option<u32>,
}

#[pymethods]
impl TableType {
    #[new]
    fn new(r#type: Type, minimum: u32, maximum: Option<u32>) -> Self {
        Self {
            r#type,
            minimum,
            maximum,
        }
    }
}

impl From<&wasmer::TableType> for TableType {
    fn from(value: &wasmer::TableType) -> Self {
        Self {
            r#type: value.ty.into(),
            minimum: value.minimum,
            maximum: value.maximum,
        }
    }
}

impl Into<wasmer::TableType> for &TableType {
    fn into(self) -> wasmer::TableType {
        wasmer::TableType::new(self.r#type.into(), self.minimum, self.maximum)
    }
}

#[pyclass]
pub struct ExportType {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub r#type: PyObject,
}

#[pymethods]
impl ExportType {
    #[new]
    fn new(name: String, r#type: PyObject) -> Self {
        Self { name, r#type }
    }
}

impl TryFrom<wasmer::ExportType> for ExportType {
    type Error = PyErr;

    fn try_from(value: wasmer::ExportType) -> Result<Self, Self::Error> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(Self {
            name: value.name().to_string(),
            r#type: extern_type_to_py_object(py, value.ty())?,
        })
    }
}

#[pyclass]
pub struct ImportType {
    #[pyo3(get)]
    pub module: String,

    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub r#type: PyObject,
}

#[pymethods]
impl ImportType {
    #[new]
    fn new(module: String, name: String, r#type: PyObject) -> Self {
        Self {
            module,
            name,
            r#type,
        }
    }
}

impl TryFrom<wasmer::ImportType> for ImportType {
    type Error = PyErr;

    fn try_from(value: wasmer::ImportType) -> Result<Self, Self::Error> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(Self {
            module: value.module().to_string(),
            name: value.name().to_string(),
            r#type: extern_type_to_py_object(py, value.ty())?,
        })
    }
}

fn extern_type_to_py_object(py: Python, value: &wasmer::ExternType) -> PyResult<PyObject> {
    Ok(match value {
        wasmer::ExternType::Function(t) => Py::new(py, FunctionType::from(t))?.to_object(py),
        wasmer::ExternType::Global(t) => Py::new(py, GlobalType::from(t))?.to_object(py),
        wasmer::ExternType::Table(t) => Py::new(py, TableType::from(t))?.to_object(py),
        wasmer::ExternType::Memory(t) => Py::new(py, MemoryType::from(t))?.to_object(py),
    })
}
