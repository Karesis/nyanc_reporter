use nyanc_core::errors::CompilerError;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct DiagnosticsEngine {
    pub errors: RefCell<Vec<CompilerError>>,
}

impl DiagnosticsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    // 这是暴露给编译器其他部分的公共 API
    pub fn add_error(&self, error: CompilerError) {
        self.errors.borrow_mut().push(error);
    }

    // 检查是否有错误发生
    pub fn has_errors(&self) -> bool {
        !self.errors.borrow().is_empty()
    }

    // 未来，这里将会有打印错误的逻辑
    // pub fn report_all(&self, source_manager: &SourceManager) { ... }
}
