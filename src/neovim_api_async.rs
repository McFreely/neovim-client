// Auto generated 2021-03-19 10:35:07.086552

use crate::neovim::*;
use crate::neovim_api::*;
use crate::r#async::AsyncCall;
use crate::rpc::*;

pub trait NeovimApiAsync {
    /// since: 1
    fn command_output_async(&mut self, command: &str) -> AsyncCall<'_, String>;
    /// since: 3
    fn execute_lua_async(&mut self, code: &str, args: Vec<Value>) -> AsyncCall<'_, Value>;
    /// since: 1
    fn ui_detach_async(&mut self) -> AsyncCall<'_, ()>;
    /// since: 1
    fn ui_try_resize_async(&mut self, width: i64, height: i64) -> AsyncCall<'_, ()>;
    /// since: 1
    fn ui_set_option_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()>;
    /// since: 6
    fn ui_try_resize_grid_async(&mut self, grid: i64, width: i64, height: i64)
        -> AsyncCall<'_, ()>;
    /// since: 6
    fn ui_pum_set_height_async(&mut self, height: i64) -> AsyncCall<'_, ()>;
    /// since: 7
    fn exec_async(&mut self, src: &str, output: bool) -> AsyncCall<'_, String>;
    /// since: 1
    fn command_async(&mut self, command: &str) -> AsyncCall<'_, ()>;
    /// since: 3
    fn get_hl_by_name_async(&mut self, name: &str, rgb: bool)
        -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 3
    fn get_hl_by_id_async(&mut self, hl_id: i64, rgb: bool) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 7
    fn get_hl_id_by_name_async(&mut self, name: &str) -> AsyncCall<'_, i64>;
    /// since: 7
    fn set_hl_async(
        &mut self,
        ns_id: i64,
        name: &str,
        val: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
    /// since: 7
    fn set_hl_ns_async(&mut self, ns_id: i64) -> AsyncCall<'_, ()>;
    /// since: 1
    fn feedkeys_async(&mut self, keys: &str, mode: &str, escape_csi: bool) -> AsyncCall<'_, ()>;
    /// since: 1
    fn input_async(&mut self, keys: &str) -> AsyncCall<'_, i64>;
    /// since: 6
    fn input_mouse_async(
        &mut self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: i64,
        row: i64,
        col: i64,
    ) -> AsyncCall<'_, ()>;
    /// since: 1
    fn replace_termcodes_async(
        &mut self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> AsyncCall<'_, String>;
    /// since: 1
    fn eval_async(&mut self, expr: &str) -> AsyncCall<'_, Value>;
    /// since: 7
    fn exec_lua_async(&mut self, code: &str, args: Vec<Value>) -> AsyncCall<'_, Value>;
    /// since: 7
    fn notify_async(
        &mut self,
        msg: &str,
        log_level: i64,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Value>;
    /// since: 1
    fn call_function_async(&mut self, fname: &str, args: Vec<Value>) -> AsyncCall<'_, Value>;
    /// since: 4
    fn call_dict_function_async(
        &mut self,
        dict: Value,
        fname: &str,
        args: Vec<Value>,
    ) -> AsyncCall<'_, Value>;
    /// since: 1
    fn strwidth_async(&mut self, text: &str) -> AsyncCall<'_, i64>;
    /// since: 1
    fn list_runtime_paths_async(&mut self) -> AsyncCall<'_, Vec<String>>;
    /// since: 7
    fn get_runtime_file_async(&mut self, name: &str, all: bool) -> AsyncCall<'_, Vec<String>>;
    /// since: 1
    fn set_current_dir_async(&mut self, dir: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn get_current_line_async(&mut self) -> AsyncCall<'_, String>;
    /// since: 1
    fn set_current_line_async(&mut self, line: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn del_current_line_async(&mut self) -> AsyncCall<'_, ()>;
    /// since: 1
    fn get_var_async(&mut self, name: &str) -> AsyncCall<'_, Value>;
    /// since: 1
    fn set_var_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()>;
    /// since: 1
    fn del_var_async(&mut self, name: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn get_vvar_async(&mut self, name: &str) -> AsyncCall<'_, Value>;
    /// since: 6
    fn set_vvar_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()>;
    /// since: 1
    fn get_option_async(&mut self, name: &str) -> AsyncCall<'_, Value>;
    /// since: 7
    fn get_all_options_info_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 7
    fn get_option_info_async(&mut self, name: &str) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 1
    fn set_option_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()>;
    /// since: 7
    fn echo_async(
        &mut self,
        chunks: Vec<Value>,
        history: bool,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
    /// since: 1
    fn out_write_async(&mut self, str: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn err_write_async(&mut self, str: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn err_writeln_async(&mut self, str: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn list_bufs_async(&mut self) -> AsyncCall<'_, Vec<Buffer>>;
    /// since: 1
    fn get_current_buf_async(&mut self) -> AsyncCall<'_, Buffer>;
    /// since: 1
    fn set_current_buf_async(&mut self, buffer: &Buffer) -> AsyncCall<'_, ()>;
    /// since: 1
    fn list_wins_async(&mut self) -> AsyncCall<'_, Vec<Window>>;
    /// since: 1
    fn get_current_win_async(&mut self) -> AsyncCall<'_, Window>;
    /// since: 1
    fn set_current_win_async(&mut self, window: &Window) -> AsyncCall<'_, ()>;
    /// since: 6
    fn create_buf_async(&mut self, listed: bool, scratch: bool) -> AsyncCall<'_, Buffer>;
    /// since: 7
    fn open_term_async(&mut self, buffer: &Buffer, opts: Vec<(Value, Value)>)
        -> AsyncCall<'_, i64>;
    /// since: 7
    fn chan_send_async(&mut self, chan: i64, data: &str) -> AsyncCall<'_, ()>;
    /// since: 6
    fn open_win_async(
        &mut self,
        buffer: &Buffer,
        enter: bool,
        config: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Window>;
    /// since: 1
    fn list_tabpages_async(&mut self) -> AsyncCall<'_, Vec<Tabpage>>;
    /// since: 1
    fn get_current_tabpage_async(&mut self) -> AsyncCall<'_, Tabpage>;
    /// since: 1
    fn set_current_tabpage_async(&mut self, tabpage: &Tabpage) -> AsyncCall<'_, ()>;
    /// since: 5
    fn create_namespace_async(&mut self, name: &str) -> AsyncCall<'_, i64>;
    /// since: 5
    fn get_namespaces_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 6
    fn paste_async(&mut self, data: &str, crlf: bool, phase: i64) -> AsyncCall<'_, bool>;
    /// since: 6
    fn put_async(
        &mut self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> AsyncCall<'_, ()>;
    /// since: 1
    fn subscribe_async(&mut self, event: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn unsubscribe_async(&mut self, event: &str) -> AsyncCall<'_, ()>;
    /// since: 1
    fn get_color_by_name_async(&mut self, name: &str) -> AsyncCall<'_, i64>;
    /// since: 1
    fn get_color_map_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 6
    fn get_context_async(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 6
    fn load_context_async(&mut self, dict: Vec<(Value, Value)>) -> AsyncCall<'_, Value>;
    /// since: 2
    fn get_mode_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 3
    fn get_keymap_async(&mut self, mode: &str) -> AsyncCall<'_, Vec<Vec<(Value, Value)>>>;
    /// since: 6
    fn set_keymap_async(
        &mut self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
    /// since: 6
    fn del_keymap_async(&mut self, mode: &str, lhs: &str) -> AsyncCall<'_, ()>;
    /// since: 4
    fn get_commands_async(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 1
    fn get_api_info_async(&mut self) -> AsyncCall<'_, Vec<Value>>;
    /// since: 4
    fn set_client_info_async(
        &mut self,
        name: &str,
        version: Vec<(Value, Value)>,
        typ: &str,
        methods: Vec<(Value, Value)>,
        attributes: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
    /// since: 4
    fn get_chan_info_async(&mut self, chan: i64) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 4
    fn list_chans_async(&mut self) -> AsyncCall<'_, Vec<Value>>;
    /// since: 1
    fn call_atomic_async(&mut self, calls: Vec<Value>) -> AsyncCall<'_, Vec<Value>>;
    /// since: 4
    fn parse_expression_async(
        &mut self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> AsyncCall<'_, Vec<(Value, Value)>>;
    /// since: 4
    fn list_uis_async(&mut self) -> AsyncCall<'_, Vec<Value>>;
    /// since: 4
    fn get_proc_children_async(&mut self, pid: i64) -> AsyncCall<'_, Vec<Value>>;
    /// since: 4
    fn get_proc_async(&mut self, pid: i64) -> AsyncCall<'_, Value>;
    /// since: 6
    fn select_popupmenu_item_async(
        &mut self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
    /// since: 7
    fn set_decoration_provider_async(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()>;
}

impl NeovimApiAsync for Neovim {
    fn command_output_async(&mut self, command: &str) -> AsyncCall<'_, String> {
        self.session
            .call_async::<String>("nvim_command_output", call_args![command])
    }

    fn execute_lua_async(&mut self, code: &str, args: Vec<Value>) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_execute_lua", call_args![code, args])
    }

    fn ui_detach_async(&mut self) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_ui_detach", call_args![])
    }

    fn ui_try_resize_async(&mut self, width: i64, height: i64) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_ui_try_resize", call_args![width, height])
    }

    fn ui_set_option_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_ui_set_option", call_args![name, value])
    }

    fn ui_try_resize_grid_async(
        &mut self,
        grid: i64,
        width: i64,
        height: i64,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_ui_try_resize_grid", call_args![grid, width, height])
    }

    fn ui_pum_set_height_async(&mut self, height: i64) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_ui_pum_set_height", call_args![height])
    }

    fn exec_async(&mut self, src: &str, output: bool) -> AsyncCall<'_, String> {
        self.session
            .call_async::<String>("nvim_exec", call_args![src, output])
    }

    fn command_async(&mut self, command: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_command", call_args![command])
    }

    fn get_hl_by_name_async(
        &mut self,
        name: &str,
        rgb: bool,
    ) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_hl_by_name", call_args![name, rgb])
    }

    fn get_hl_by_id_async(&mut self, hl_id: i64, rgb: bool) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_hl_by_id", call_args![hl_id, rgb])
    }

    fn get_hl_id_by_name_async(&mut self, name: &str) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_get_hl_id_by_name", call_args![name])
    }

    fn set_hl_async(
        &mut self,
        ns_id: i64,
        name: &str,
        val: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_hl", call_args![ns_id, name, val])
    }

    fn set_hl_ns_async(&mut self, ns_id: i64) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_hl_ns", call_args![ns_id])
    }

    fn feedkeys_async(&mut self, keys: &str, mode: &str, escape_csi: bool) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_feedkeys", call_args![keys, mode, escape_csi])
    }

    fn input_async(&mut self, keys: &str) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_input", call_args![keys])
    }

    fn input_mouse_async(
        &mut self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: i64,
        row: i64,
        col: i64,
    ) -> AsyncCall<'_, ()> {
        self.session.call_async::<()>(
            "nvim_input_mouse",
            call_args![button, action, modifier, grid, row, col],
        )
    }

    fn replace_termcodes_async(
        &mut self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> AsyncCall<'_, String> {
        self.session.call_async::<String>(
            "nvim_replace_termcodes",
            call_args![str, from_part, do_lt, special],
        )
    }

    fn eval_async(&mut self, expr: &str) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_eval", call_args![expr])
    }

    fn exec_lua_async(&mut self, code: &str, args: Vec<Value>) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_exec_lua", call_args![code, args])
    }

    fn notify_async(
        &mut self,
        msg: &str,
        log_level: i64,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_notify", call_args![msg, log_level, opts])
    }

    fn call_function_async(&mut self, fname: &str, args: Vec<Value>) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_call_function", call_args![fname, args])
    }

    fn call_dict_function_async(
        &mut self,
        dict: Value,
        fname: &str,
        args: Vec<Value>,
    ) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_call_dict_function", call_args![dict, fname, args])
    }

    fn strwidth_async(&mut self, text: &str) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_strwidth", call_args![text])
    }

    fn list_runtime_paths_async(&mut self) -> AsyncCall<'_, Vec<String>> {
        self.session
            .call_async::<Vec<String>>("nvim_list_runtime_paths", call_args![])
    }

    fn get_runtime_file_async(&mut self, name: &str, all: bool) -> AsyncCall<'_, Vec<String>> {
        self.session
            .call_async::<Vec<String>>("nvim_get_runtime_file", call_args![name, all])
    }

    fn set_current_dir_async(&mut self, dir: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_current_dir", call_args![dir])
    }

    fn get_current_line_async(&mut self) -> AsyncCall<'_, String> {
        self.session
            .call_async::<String>("nvim_get_current_line", call_args![])
    }

    fn set_current_line_async(&mut self, line: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_current_line", call_args![line])
    }

    fn del_current_line_async(&mut self) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_del_current_line", call_args![])
    }

    fn get_var_async(&mut self, name: &str) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_get_var", call_args![name])
    }

    fn set_var_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_var", call_args![name, value])
    }

    fn del_var_async(&mut self, name: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_del_var", call_args![name])
    }

    fn get_vvar_async(&mut self, name: &str) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_get_vvar", call_args![name])
    }

    fn set_vvar_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_vvar", call_args![name, value])
    }

    fn get_option_async(&mut self, name: &str) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_get_option", call_args![name])
    }

    fn get_all_options_info_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_all_options_info", call_args![])
    }

    fn get_option_info_async(&mut self, name: &str) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_option_info", call_args![name])
    }

    fn set_option_async(&mut self, name: &str, value: Value) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_option", call_args![name, value])
    }

    fn echo_async(
        &mut self,
        chunks: Vec<Value>,
        history: bool,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_echo", call_args![chunks, history, opts])
    }

    fn out_write_async(&mut self, str: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_out_write", call_args![str])
    }

    fn err_write_async(&mut self, str: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_err_write", call_args![str])
    }

    fn err_writeln_async(&mut self, str: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_err_writeln", call_args![str])
    }

    fn list_bufs_async(&mut self) -> AsyncCall<'_, Vec<Buffer>> {
        self.session
            .call_async::<Vec<Buffer>>("nvim_list_bufs", call_args![])
    }

    fn get_current_buf_async(&mut self) -> AsyncCall<'_, Buffer> {
        self.session
            .call_async::<Buffer>("nvim_get_current_buf", call_args![])
    }

    fn set_current_buf_async(&mut self, buffer: &Buffer) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_current_buf", call_args![buffer])
    }

    fn list_wins_async(&mut self) -> AsyncCall<'_, Vec<Window>> {
        self.session
            .call_async::<Vec<Window>>("nvim_list_wins", call_args![])
    }

    fn get_current_win_async(&mut self) -> AsyncCall<'_, Window> {
        self.session
            .call_async::<Window>("nvim_get_current_win", call_args![])
    }

    fn set_current_win_async(&mut self, window: &Window) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_current_win", call_args![window])
    }

    fn create_buf_async(&mut self, listed: bool, scratch: bool) -> AsyncCall<'_, Buffer> {
        self.session
            .call_async::<Buffer>("nvim_create_buf", call_args![listed, scratch])
    }

    fn open_term_async(
        &mut self,
        buffer: &Buffer,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_open_term", call_args![buffer, opts])
    }

    fn chan_send_async(&mut self, chan: i64, data: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_chan_send", call_args![chan, data])
    }

    fn open_win_async(
        &mut self,
        buffer: &Buffer,
        enter: bool,
        config: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Window> {
        self.session
            .call_async::<Window>("nvim_open_win", call_args![buffer, enter, config])
    }

    fn list_tabpages_async(&mut self) -> AsyncCall<'_, Vec<Tabpage>> {
        self.session
            .call_async::<Vec<Tabpage>>("nvim_list_tabpages", call_args![])
    }

    fn get_current_tabpage_async(&mut self) -> AsyncCall<'_, Tabpage> {
        self.session
            .call_async::<Tabpage>("nvim_get_current_tabpage", call_args![])
    }

    fn set_current_tabpage_async(&mut self, tabpage: &Tabpage) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_current_tabpage", call_args![tabpage])
    }

    fn create_namespace_async(&mut self, name: &str) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_create_namespace", call_args![name])
    }

    fn get_namespaces_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_namespaces", call_args![])
    }

    fn paste_async(&mut self, data: &str, crlf: bool, phase: i64) -> AsyncCall<'_, bool> {
        self.session
            .call_async::<bool>("nvim_paste", call_args![data, crlf, phase])
    }

    fn put_async(
        &mut self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_put", call_args![lines, typ, after, follow])
    }

    fn subscribe_async(&mut self, event: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_subscribe", call_args![event])
    }

    fn unsubscribe_async(&mut self, event: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_unsubscribe", call_args![event])
    }

    fn get_color_by_name_async(&mut self, name: &str) -> AsyncCall<'_, i64> {
        self.session
            .call_async::<i64>("nvim_get_color_by_name", call_args![name])
    }

    fn get_color_map_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_color_map", call_args![])
    }

    fn get_context_async(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_context", call_args![opts])
    }

    fn load_context_async(&mut self, dict: Vec<(Value, Value)>) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_load_context", call_args![dict])
    }

    fn get_mode_async(&mut self) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_mode", call_args![])
    }

    fn get_keymap_async(&mut self, mode: &str) -> AsyncCall<'_, Vec<Vec<(Value, Value)>>> {
        self.session
            .call_async::<Vec<Vec<(Value, Value)>>>("nvim_get_keymap", call_args![mode])
    }

    fn set_keymap_async(
        &mut self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_keymap", call_args![mode, lhs, rhs, opts])
    }

    fn del_keymap_async(&mut self, mode: &str, lhs: &str) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_del_keymap", call_args![mode, lhs])
    }

    fn get_commands_async(
        &mut self,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_commands", call_args![opts])
    }

    fn get_api_info_async(&mut self) -> AsyncCall<'_, Vec<Value>> {
        self.session
            .call_async::<Vec<Value>>("nvim_get_api_info", call_args![])
    }

    fn set_client_info_async(
        &mut self,
        name: &str,
        version: Vec<(Value, Value)>,
        typ: &str,
        methods: Vec<(Value, Value)>,
        attributes: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session.call_async::<()>(
            "nvim_set_client_info",
            call_args![name, version, typ, methods, attributes],
        )
    }

    fn get_chan_info_async(&mut self, chan: i64) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session
            .call_async::<Vec<(Value, Value)>>("nvim_get_chan_info", call_args![chan])
    }

    fn list_chans_async(&mut self) -> AsyncCall<'_, Vec<Value>> {
        self.session
            .call_async::<Vec<Value>>("nvim_list_chans", call_args![])
    }

    fn call_atomic_async(&mut self, calls: Vec<Value>) -> AsyncCall<'_, Vec<Value>> {
        self.session
            .call_async::<Vec<Value>>("nvim_call_atomic", call_args![calls])
    }

    fn parse_expression_async(
        &mut self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> AsyncCall<'_, Vec<(Value, Value)>> {
        self.session.call_async::<Vec<(Value, Value)>>(
            "nvim_parse_expression",
            call_args![expr, flags, highlight],
        )
    }

    fn list_uis_async(&mut self) -> AsyncCall<'_, Vec<Value>> {
        self.session
            .call_async::<Vec<Value>>("nvim_list_uis", call_args![])
    }

    fn get_proc_children_async(&mut self, pid: i64) -> AsyncCall<'_, Vec<Value>> {
        self.session
            .call_async::<Vec<Value>>("nvim_get_proc_children", call_args![pid])
    }

    fn get_proc_async(&mut self, pid: i64) -> AsyncCall<'_, Value> {
        self.session
            .call_async::<Value>("nvim_get_proc", call_args![pid])
    }

    fn select_popupmenu_item_async(
        &mut self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session.call_async::<()>(
            "nvim_select_popupmenu_item",
            call_args![item, insert, finish, opts],
        )
    }

    fn set_decoration_provider_async(
        &mut self,
        ns_id: i64,
        opts: Vec<(Value, Value)>,
    ) -> AsyncCall<'_, ()> {
        self.session
            .call_async::<()>("nvim_set_decoration_provider", call_args![ns_id, opts])
    }
}
