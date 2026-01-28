use super::error::TableError;
use crate::target::driver::XilVitisNetP4ReturnType_XIL_VITIS_NET_P4_SUCCESS as XIL_VITIS_NET_P4_SUCCESS;
use crate::target::driver::XilVitisNetP4TableCtx;
use crate::target::driver::XilVitisNetP4TableDelete;
use crate::target::driver::XilVitisNetP4TableGetActionId;
use crate::target::driver::XilVitisNetP4TableInsert;
use crate::target::driver::XilVitisNetP4TargetCtx;
use crate::target::driver::XilVitisNetP4TargetGetTableByName;
use crate::target::schema::TargetTableConfigSchema;
use crate::utils::driver::code_to_name;
use p4runtime::p4::v1::TableEntry;
use p4runtime::p4::v1::update::Type as UpdateType;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[derive(Debug, Clone)]
pub struct Table {
    pub schema: TargetTableConfigSchema,
    pub table_context_ptr: Option<*mut XilVitisNetP4TableCtx>,
    pub entries: HashMap<Vec<u8>, TableEntry>,
}

impl Table {
    pub fn new(schema: TargetTableConfigSchema, target_context: *mut XilVitisNetP4TargetCtx) -> Result<Self, TableError> {
        let mut table_context_ptr: *mut XilVitisNetP4TableCtx = null_mut();
        let code = unsafe { XilVitisNetP4TargetGetTableByName(target_context, schema.name_string.as_ptr() as *mut c_char, &mut table_context_ptr as *mut *mut XilVitisNetP4TableCtx) };
        if code != XIL_VITIS_NET_P4_SUCCESS {
            return Err(TableError::Driver { name: code_to_name(code), code: code });
        }
        Ok(Self {
            schema: schema,
            table_context_ptr: Some(table_context_ptr),
            entries: HashMap::new(),
        })
    }

    pub fn apply(&mut self, entry: TableEntry, update: UpdateType) -> Result<(), TableError> {
        match update {
            UpdateType::Insert => self.insert(entry)?,
            UpdateType::Delete => self.delete(entry)?,
            _ => return Err(TableError::NotSupported),
        }

        Ok(())
    }

    pub fn insert(&mut self, entry: TableEntry) -> Result<(), TableError> {
        let field_match = match entry.r#match.as_slice() {
            [field_match] => field_match,
            _ => return Err(TableError::NotSupported),
        };

        let field_match_exact = match &field_match.field_match_type {
            Some(p4runtime::p4::v1::field_match::FieldMatchType::Exact(field_match_exact)) => field_match_exact,
            _ => return Err(TableError::NotSupported),
        };

        let table_action = match &entry.action {
            Some(table_action) => table_action,
            _ => return Err(TableError::NotSupported),
        };

        let action = match &table_action.r#type {
            Some(p4runtime::p4::v1::table_action::Type::Action(action)) => action,
            _ => return Err(TableError::NotSupported),
        };

        let param = match action.params.as_slice() {
            [param] => param,
            _ => return Err(TableError::NotSupported),
        };

        let action_schema = match self.schema.config.action_list.get(&action.action_id) {
            Some(action_schema) => action_schema,
            _ => return Err(TableError::NotFoundAction),
        };

        let mut action_id: u32 = 0;

        let code = unsafe { XilVitisNetP4TableGetActionId(self.table_context_ptr.unwrap(), action_schema.name_string.as_ptr() as *mut c_char, &mut action_id as *mut u32) };

        if code != XIL_VITIS_NET_P4_SUCCESS {
            return Err(TableError::Driver { name: code_to_name(code), code: code });
        }

        let code = unsafe {
            XilVitisNetP4TableInsert(
                self.table_context_ptr.unwrap(),
                field_match_exact.value.as_ptr() as *mut u8,
                null_mut(),
                0,
                action_id,
                param.value.as_ptr() as *mut u8,
            )
        };

        if code != XIL_VITIS_NET_P4_SUCCESS {
            return Err(TableError::Driver { name: code_to_name(code), code: code });
        }

        self.entries.insert(field_match_exact.value.clone(), entry);
        Ok(())
    }

    pub fn delete(&mut self, entry: TableEntry) -> Result<(), TableError> {
        let field_match = match entry.r#match.as_slice() {
            [field_match] => field_match,
            _ => return Err(TableError::NotSupported),
        };

        let field_match_exact = match &field_match.field_match_type {
            Some(p4runtime::p4::v1::field_match::FieldMatchType::Exact(field_match_exact)) => field_match_exact,
            _ => return Err(TableError::NotSupported),
        };

        if !self.entries.contains_key(&field_match_exact.value) {
            return Err(TableError::NotFoundKey);
        }

        let code = unsafe { XilVitisNetP4TableDelete(self.table_context_ptr.unwrap(), field_match_exact.value.as_ptr() as *mut u8, null_mut()) };

        if code != XIL_VITIS_NET_P4_SUCCESS {
            return Err(TableError::Driver { name: code_to_name(code), code: code });
        }

        self.entries.remove(&field_match_exact.value);

        Ok(())
    }
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}
