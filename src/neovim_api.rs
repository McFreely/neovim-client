// Auto generated 2021-03-19 10:35:07.372181

use crate::neovim::*;
use crate::rpc::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Buffer {
    code_data: Value,
}

impl Buffer {
    pub fn new(code_data: Value) -> Buffer {
        Buffer { code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn line_count(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call("nvim_buf_line_count", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 4
    pub fn attach(
        &self,
        neovim: &mut Neovim,
        send_buffer: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<bool, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_attach",
                call_args![self.code_data.clone(), send_buffer, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 4
    pub fn detach(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_buf_detach", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_lines(
        &self,
        neovim: &mut Neovim,
        start: i64,
        end: i64,
        strict_indexing: bool,
    ) -> Result<Vec<String>, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_lines",
                call_args![self.code_data.clone(), start, end, strict_indexing],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_lines(
        &self,
        neovim: &mut Neovim,
        start: i64,
        end: i64,
        strict_indexing: bool,
        replacement: Vec<String>,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_lines",
                call_args![
                    self.code_data.clone(),
                    start,
                    end,
                    strict_indexing,
                    replacement
                ],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn set_text(
        &self,
        neovim: &mut Neovim,
        start_row: i64,
        start_col: i64,
        end_row: i64,
        end_col: i64,
        replacement: Vec<String>,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_text",
                call_args![
                    self.code_data.clone(),
                    start_row,
                    start_col,
                    end_row,
                    end_col,
                    replacement
                ],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 5
    pub fn get_offset(&self, neovim: &mut Neovim, index: i64) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_offset",
                call_args![self.code_data.clone(), index],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_buf_get_var", call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 2
    pub fn get_changedtick(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_changedtick",
                call_args![self.code_data.clone()],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 3
    pub fn get_keymap(
        &self,
        neovim: &mut Neovim,
        mode: &str,
    ) -> Result<Vec<Vec<(Value, Value)>>, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_keymap",
                call_args![self.code_data.clone(), mode],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 6
    pub fn set_keymap(
        &self,
        neovim: &mut Neovim,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_keymap",
                call_args![self.code_data.clone(), mode, lhs, rhs, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 6
    pub fn del_keymap(&self, neovim: &mut Neovim, mode: &str, lhs: &str) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_del_keymap",
                call_args![self.code_data.clone(), mode, lhs],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 4
    pub fn get_commands(
        &self,
        neovim: &mut Neovim,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_commands",
                call_args![self.code_data.clone(), opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_var",
                call_args![self.code_data.clone(), name, value],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_del_var", call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_option",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_option(
        &self,
        neovim: &mut Neovim,
        name: &str,
        value: Value,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_option",
                call_args![self.code_data.clone(), name, value],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_name(&self, neovim: &mut Neovim) -> Result<String, CallError> {
        neovim
            .session
            .call("nvim_buf_get_name", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_name(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_name",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 5
    pub fn is_loaded(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_buf_is_loaded", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn delete(&self, neovim: &mut Neovim, opts: Vec<(Value, Value)>) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_delete", call_args![self.code_data.clone(), opts])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_buf_is_valid", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_mark(&self, neovim: &mut Neovim, name: &str) -> Result<(i64, i64), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_mark",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn get_extmark_by_id(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        id: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<i64>, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_extmark_by_id",
                call_args![self.code_data.clone(), ns_id, id, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn get_extmarks(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        start: Value,
        end: Value,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<Value>, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_get_extmarks",
                call_args![self.code_data.clone(), ns_id, start, end, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn set_extmark(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        line: i64,
        col: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_extmark",
                call_args![self.code_data.clone(), ns_id, line, col, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 7
    pub fn del_extmark(&self, neovim: &mut Neovim, ns_id: i64, id: i64) -> Result<bool, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_del_extmark",
                call_args![self.code_data.clone(), ns_id, id],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn add_highlight(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        hl_group: &str,
        line: i64,
        col_start: i64,
        col_end: i64,
    ) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_add_highlight",
                call_args![
                    self.code_data.clone(),
                    ns_id,
                    hl_group,
                    line,
                    col_start,
                    col_end
                ],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 5
    pub fn clear_namespace(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_clear_namespace",
                call_args![self.code_data.clone(), ns_id, line_start, line_end],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 5
    pub fn set_virtual_text(
        &self,
        neovim: &mut Neovim,
        src_id: i64,
        line: i64,
        chunks: Vec<Value>,
        opts: Vec<(Value, Value)>,
    ) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_buf_set_virtual_text",
                call_args![self.code_data.clone(), src_id, line, chunks, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call("nvim_buf_get_number", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn clear_highlight(
        &self,
        neovim: &mut Neovim,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_buf_clear_highlight",
                call_args![self.code_data.clone(), ns_id, line_start, line_end],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Window {
    code_data: Value,
}

impl Window {
    pub fn new(code_data: Value) -> Window {
        Window { code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn get_buf(&self, neovim: &mut Neovim) -> Result<Buffer, CallError> {
        neovim
            .session
            .call("nvim_win_get_buf", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 5
    pub fn set_buf(&self, neovim: &mut Neovim, buffer: &Buffer) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_buf",
                call_args![self.code_data.clone(), buffer],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_cursor(&self, neovim: &mut Neovim) -> Result<(i64, i64), CallError> {
        neovim
            .session
            .call("nvim_win_get_cursor", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_cursor(&self, neovim: &mut Neovim, pos: (i64, i64)) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_cursor",
                call_args![self.code_data.clone(), pos],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_height(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call("nvim_win_get_height", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_height(&self, neovim: &mut Neovim, height: i64) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_height",
                call_args![self.code_data.clone(), height],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_width(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call("nvim_win_get_width", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_width(&self, neovim: &mut Neovim, width: i64) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_width",
                call_args![self.code_data.clone(), width],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_win_get_var", call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_var",
                call_args![self.code_data.clone(), name, value],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_del_var", call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call(
                "nvim_win_get_option",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_option(
        &self,
        neovim: &mut Neovim,
        name: &str,
        value: Value,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_option",
                call_args![self.code_data.clone(), name, value],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_position(&self, neovim: &mut Neovim) -> Result<(i64, i64), CallError> {
        neovim
            .session
            .call("nvim_win_get_position", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_tabpage(&self, neovim: &mut Neovim) -> Result<Tabpage, CallError> {
        neovim
            .session
            .call("nvim_win_get_tabpage", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call("nvim_win_get_number", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_win_is_valid", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 6
    pub fn set_config(
        &self,
        neovim: &mut Neovim,
        config: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_win_set_config",
                call_args![self.code_data.clone(), config],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 6
    pub fn get_config(&self, neovim: &mut Neovim) -> Result<Vec<(Value, Value)>, CallError> {
        neovim
            .session
            .call("nvim_win_get_config", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 6
    pub fn close(&self, neovim: &mut Neovim, force: bool) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_close", call_args![self.code_data.clone(), force])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Tabpage {
    code_data: Value,
}

impl Tabpage {
    pub fn new(code_data: Value) -> Tabpage {
        Tabpage { code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn list_wins(&self, neovim: &mut Neovim) -> Result<Vec<Window>, CallError> {
        neovim
            .session
            .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call(
                "nvim_tabpage_get_var",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_tabpage_set_var",
                call_args![self.code_data.clone(), name, value],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call(
                "nvim_tabpage_del_var",
                call_args![self.code_data.clone(), name],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_win(&self, neovim: &mut Neovim) -> Result<Window, CallError> {
        neovim
            .session
            .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<i64, CallError> {
        neovim
            .session
            .call(
                "nvim_tabpage_get_number",
                call_args![self.code_data.clone()],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_tabpage_is_valid", call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Buffer {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Window {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Tabpage {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}

pub trait NeovimApi {
    /// since: 1
    fn command_output(&mut self, command: &str) -> Result<String, CallError>;
    /// since: 3
    fn execute_lua(&mut self, code: &str, args: Vec<Value>) -> Result<Value, CallError>;
    /// since: 1
    fn ui_detach(&mut self) -> Result<(), CallError>;
    /// since: 1
    fn ui_try_resize(&mut self, width: i64, height: i64) -> Result<(), CallError>;
    /// since: 1
    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 6
    fn ui_try_resize_grid(&mut self, grid: i64, width: i64, height: i64) -> Result<(), CallError>;
    /// since: 6
    fn ui_pum_set_height(&mut self, height: i64) -> Result<(), CallError>;
    /// since: 7
    fn exec(&mut self, src: &str, output: bool) -> Result<String, CallError>;
    /// since: 1
    fn command(&mut self, command: &str) -> Result<(), CallError>;
    /// since: 3
    fn get_hl_by_name(&mut self, name: &str, rgb: bool) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 3
    fn get_hl_by_id(&mut self, hl_id: i64, rgb: bool) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 7
    fn get_hl_id_by_name(&mut self, name: &str) -> Result<i64, CallError>;
    /// since: 7
    fn set_hl(&mut self, ns_id: i64, name: &str, val: Vec<(Value, Value)>)
        -> Result<(), CallError>;
    /// since: 7
    fn set_hl_ns(&mut self, ns_id: i64) -> Result<(), CallError>;
    /// since: 1
    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError>;
    /// since: 1
    fn input(&mut self, keys: &str) -> Result<i64, CallError>;
    /// since: 6
    fn input_mouse(
        &mut self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: i64,
        row: i64,
        col: i64,
    ) -> Result<(), CallError>;
    /// since: 1
    fn replace_termcodes(
        &mut self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String, CallError>;
    /// since: 1
    fn eval(&mut self, expr: &str) -> Result<Value, CallError>;
    /// since: 7
    fn exec_lua(&mut self, code: &str, args: Vec<Value>) -> Result<Value, CallError>;
    /// since: 7
    fn notify(
        &mut self,
        msg: &str,
        log_level: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Value, CallError>;
    /// since: 1
    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError>;
    /// since: 4
    fn call_dict_function(
        &mut self,
        dict: Value,
        fname: &str,
        args: Vec<Value>,
    ) -> Result<Value, CallError>;
    /// since: 1
    fn strwidth(&mut self, text: &str) -> Result<i64, CallError>;
    /// since: 1
    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError>;
    /// since: 7
    fn get_runtime_file(&mut self, name: &str, all: bool) -> Result<Vec<String>, CallError>;
    /// since: 1
    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_current_line(&mut self) -> Result<String, CallError>;
    /// since: 1
    fn set_current_line(&mut self, line: &str) -> Result<(), CallError>;
    /// since: 1
    fn del_current_line(&mut self) -> Result<(), CallError>;
    /// since: 1
    fn get_var(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 1
    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 1
    fn del_var(&mut self, name: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 6
    fn set_vvar(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 1
    fn get_option(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 7
    fn get_all_options_info(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 7
    fn get_option_info(&mut self, name: &str) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 1
    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 7
    fn echo(
        &mut self,
        chunks: Vec<Value>,
        history: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError>;
    /// since: 1
    fn out_write(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn err_write(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn err_writeln(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError>;
    /// since: 1
    fn get_current_buf(&mut self) -> Result<Buffer, CallError>;
    /// since: 1
    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError>;
    /// since: 1
    fn list_wins(&mut self) -> Result<Vec<Window>, CallError>;
    /// since: 1
    fn get_current_win(&mut self) -> Result<Window, CallError>;
    /// since: 1
    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError>;
    /// since: 6
    fn create_buf(&mut self, listed: bool, scratch: bool) -> Result<Buffer, CallError>;
    /// since: 7
    fn open_term(&mut self, buffer: &Buffer, opts: Vec<(Value, Value)>) -> Result<i64, CallError>;
    /// since: 7
    fn chan_send(&mut self, chan: i64, data: &str) -> Result<(), CallError>;
    /// since: 6
    fn open_win(
        &mut self,
        buffer: &Buffer,
        enter: bool,
        config: Vec<(Value, Value)>,
    ) -> Result<Window, CallError>;
    /// since: 1
    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError>;
    /// since: 1
    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError>;
    /// since: 1
    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError>;
    /// since: 5
    fn create_namespace(&mut self, name: &str) -> Result<i64, CallError>;
    /// since: 5
    fn get_namespaces(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 6
    fn paste(&mut self, data: &str, crlf: bool, phase: i64) -> Result<bool, CallError>;
    /// since: 6
    fn put(
        &mut self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> Result<(), CallError>;
    /// since: 1
    fn subscribe(&mut self, event: &str) -> Result<(), CallError>;
    /// since: 1
    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_color_by_name(&mut self, name: &str) -> Result<i64, CallError>;
    /// since: 1
    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 6
    fn get_context(&mut self, opts: Vec<(Value, Value)>) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 6
    fn load_context(&mut self, dict: Vec<(Value, Value)>) -> Result<Value, CallError>;
    /// since: 2
    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 3
    fn get_keymap(&mut self, mode: &str) -> Result<Vec<Vec<(Value, Value)>>, CallError>;
    /// since: 6
    fn set_keymap(
        &mut self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError>;
    /// since: 6
    fn del_keymap(&mut self, mode: &str, lhs: &str) -> Result<(), CallError>;
    /// since: 4
    fn get_commands(&mut self, opts: Vec<(Value, Value)>)
        -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 1
    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError>;
    /// since: 4
    fn set_client_info(
        &mut self,
        name: &str,
        version: Vec<(Value, Value)>,
        typ: &str,
        methods: Vec<(Value, Value)>,
        attributes: Vec<(Value, Value)>,
    ) -> Result<(), CallError>;
    /// since: 4
    fn get_chan_info(&mut self, chan: i64) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 4
    fn list_chans(&mut self) -> Result<Vec<Value>, CallError>;
    /// since: 1
    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError>;
    /// since: 4
    fn parse_expression(
        &mut self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 4
    fn list_uis(&mut self) -> Result<Vec<Value>, CallError>;
    /// since: 4
    fn get_proc_children(&mut self, pid: i64) -> Result<Vec<Value>, CallError>;
    /// since: 4
    fn get_proc(&mut self, pid: i64) -> Result<Value, CallError>;
    /// since: 6
    fn select_popupmenu_item(
        &mut self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError>;
    /// since: 7
    fn set_decoration_provider(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError>;
}

impl NeovimApi for Neovim {
    fn command_output(&mut self, command: &str) -> Result<String, CallError> {
        self.session
            .call("nvim_command_output", call_args![command])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn execute_lua(&mut self, code: &str, args: Vec<Value>) -> Result<Value, CallError> {
        self.session
            .call("nvim_execute_lua", call_args![code, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_detach(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_detach", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_try_resize(&mut self, width: i64, height: i64) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_try_resize", call_args![width, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_set_option", call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_try_resize_grid(&mut self, grid: i64, width: i64, height: i64) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_try_resize_grid", call_args![grid, width, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_pum_set_height(&mut self, height: i64) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_pum_set_height", call_args![height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn exec(&mut self, src: &str, output: bool) -> Result<String, CallError> {
        self.session
            .call("nvim_exec", call_args![src, output])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command(&mut self, command: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_command", call_args![command])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_hl_by_name(&mut self, name: &str, rgb: bool) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_hl_by_name", call_args![name, rgb])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_hl_by_id(&mut self, hl_id: i64, rgb: bool) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_hl_by_id", call_args![hl_id, rgb])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_hl_id_by_name(&mut self, name: &str) -> Result<i64, CallError> {
        self.session
            .call("nvim_get_hl_id_by_name", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_hl(
        &mut self,
        ns_id: i64,
        name: &str,
        val: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call("nvim_set_hl", call_args![ns_id, name, val])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_hl_ns(&mut self, ns_id: i64) -> Result<(), CallError> {
        self.session
            .call("nvim_set_hl_ns", call_args![ns_id])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError> {
        self.session
            .call("nvim_feedkeys", call_args![keys, mode, escape_csi])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn input(&mut self, keys: &str) -> Result<i64, CallError> {
        self.session
            .call("nvim_input", call_args![keys])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn input_mouse(
        &mut self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: i64,
        row: i64,
        col: i64,
    ) -> Result<(), CallError> {
        self.session
            .call(
                "nvim_input_mouse",
                call_args![button, action, modifier, grid, row, col],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn replace_termcodes(
        &mut self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String, CallError> {
        self.session
            .call(
                "nvim_replace_termcodes",
                call_args![str, from_part, do_lt, special],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn eval(&mut self, expr: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_eval", call_args![expr])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn exec_lua(&mut self, code: &str, args: Vec<Value>) -> Result<Value, CallError> {
        self.session
            .call("nvim_exec_lua", call_args![code, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn notify(
        &mut self,
        msg: &str,
        log_level: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<Value, CallError> {
        self.session
            .call("nvim_notify", call_args![msg, log_level, opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError> {
        self.session
            .call("nvim_call_function", call_args![fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_dict_function(
        &mut self,
        dict: Value,
        fname: &str,
        args: Vec<Value>,
    ) -> Result<Value, CallError> {
        self.session
            .call("nvim_call_dict_function", call_args![dict, fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn strwidth(&mut self, text: &str) -> Result<i64, CallError> {
        self.session
            .call("nvim_strwidth", call_args![text])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError> {
        self.session
            .call("nvim_list_runtime_paths", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_runtime_file(&mut self, name: &str, all: bool) -> Result<Vec<String>, CallError> {
        self.session
            .call("nvim_get_runtime_file", call_args![name, all])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_dir", call_args![dir])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_line(&mut self) -> Result<String, CallError> {
        self.session
            .call("nvim_get_current_line", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_line(&mut self, line: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_line", call_args![line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_current_line(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_del_current_line", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_var(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_var", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_var", call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_var(&mut self, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_del_var", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_vvar", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_vvar(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_vvar", call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_option(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_option", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_all_options_info(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_all_options_info", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_option_info(&mut self, name: &str) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_option_info", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_option", call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn echo(
        &mut self,
        chunks: Vec<Value>,
        history: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call("nvim_echo", call_args![chunks, history, opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn out_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_out_write", call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_write", call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_writeln(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_writeln", call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError> {
        self.session
            .call("nvim_list_bufs", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_buf(&mut self) -> Result<Buffer, CallError> {
        self.session
            .call("nvim_get_current_buf", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_buf", call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_wins(&mut self) -> Result<Vec<Window>, CallError> {
        self.session
            .call("nvim_list_wins", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_win(&mut self) -> Result<Window, CallError> {
        self.session
            .call("nvim_get_current_win", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_win", call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn create_buf(&mut self, listed: bool, scratch: bool) -> Result<Buffer, CallError> {
        self.session
            .call("nvim_create_buf", call_args![listed, scratch])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn open_term(&mut self, buffer: &Buffer, opts: Vec<(Value, Value)>) -> Result<i64, CallError> {
        self.session
            .call("nvim_open_term", call_args![buffer, opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn chan_send(&mut self, chan: i64, data: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_chan_send", call_args![chan, data])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn open_win(
        &mut self,
        buffer: &Buffer,
        enter: bool,
        config: Vec<(Value, Value)>,
    ) -> Result<Window, CallError> {
        self.session
            .call("nvim_open_win", call_args![buffer, enter, config])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError> {
        self.session
            .call("nvim_list_tabpages", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError> {
        self.session
            .call("nvim_get_current_tabpage", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_tabpage", call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn create_namespace(&mut self, name: &str) -> Result<i64, CallError> {
        self.session
            .call("nvim_create_namespace", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_namespaces(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_namespaces", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn paste(&mut self, data: &str, crlf: bool, phase: i64) -> Result<bool, CallError> {
        self.session
            .call("nvim_paste", call_args![data, crlf, phase])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn put(
        &mut self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> Result<(), CallError> {
        self.session
            .call("nvim_put", call_args![lines, typ, after, follow])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn subscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_subscribe", call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_unsubscribe", call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_by_name(&mut self, name: &str) -> Result<i64, CallError> {
        self.session
            .call("nvim_get_color_by_name", call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_color_map", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_context(&mut self, opts: Vec<(Value, Value)>) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_context", call_args![opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn load_context(&mut self, dict: Vec<(Value, Value)>) -> Result<Value, CallError> {
        self.session
            .call("nvim_load_context", call_args![dict])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_mode", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_keymap(&mut self, mode: &str) -> Result<Vec<Vec<(Value, Value)>>, CallError> {
        self.session
            .call("nvim_get_keymap", call_args![mode])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_keymap(
        &mut self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call("nvim_set_keymap", call_args![mode, lhs, rhs, opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_keymap(&mut self, mode: &str, lhs: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_del_keymap", call_args![mode, lhs])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_commands(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_commands", call_args![opts])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_get_api_info", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_client_info(
        &mut self,
        name: &str,
        version: Vec<(Value, Value)>,
        typ: &str,
        methods: Vec<(Value, Value)>,
        attributes: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call(
                "nvim_set_client_info",
                call_args![name, version, typ, methods, attributes],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_chan_info(&mut self, chan: i64) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_chan_info", call_args![chan])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_chans(&mut self) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_list_chans", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_call_atomic", call_args![calls])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn parse_expression(
        &mut self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_parse_expression", call_args![expr, flags, highlight])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_uis(&mut self) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_list_uis", call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_proc_children(&mut self, pid: i64) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_get_proc_children", call_args![pid])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_proc(&mut self, pid: i64) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_proc", call_args![pid])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn select_popupmenu_item(
        &mut self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call(
                "nvim_select_popupmenu_item",
                call_args![item, insert, finish, opts],
            )
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_decoration_provider(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> Result<(), CallError> {
        self.session
            .call("nvim_set_decoration_provider", call_args![ns_id, opts])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
