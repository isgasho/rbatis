use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::utils;
use rexpr::runtime::RExprRuntime;
use rexpr::ast::Node;
use crate::interpreter::sql::node::parse_node;

#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub get_collection_func:Node,
}

impl ForEachNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
        if !express.contains("in ") {
            return Err(crate::core::Error::from(
                "[rbatis] parser express fail:".to_string() + source,
            ));
        }
        let express = express[Self::name().len()..].trim();
        let in_index = express.find("in ").unwrap();
        let col = express[in_index + "in ".len()..].trim();
        let mut item = express[..in_index].trim();
        let mut index = "";
        if item.contains(",") {
            let items: Vec<&str> = item.split(",").collect();
            if items.len() != 2 {
                return Err(crate::core::Error::from(format!(
                    "[rbatis][py] parse fail 'for ,' must be 'for arg1,arg2 in ...',value:'{}'",
                    source
                )));
            }
            index = items[0];
            item = items[1];
        }
        return Ok(ForEachNode {
            childs: childs,
            collection: col.to_string(),
            index: index.to_string(),
            item: item.to_string(),
            get_collection_func: parse_node(col)?
        });
    }
}

impl RbatisAST for ForEachNode {
    fn name() -> &'static str {
        "for"
    }
    fn eval(
        &self,
        convert: &crate::core::db::DriverType,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        let collection_value = self.get_collection_func.eval( env)?;
        if collection_value.is_null() {
            return Result::Ok(serde_json::Value::Null);
        }
        if collection_value.is_array() {
            let collection = collection_value.as_array().unwrap();
            let collection_len = collection.len() as i32;
            let mut index = -1;
            for item in collection {
                index = index + 1;
                env[&self.item] = item.clone();
                env[&self.index] = json!(index);
                do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
                env.as_object_mut().unwrap().remove(&self.item);
                env.as_object_mut().unwrap().remove(&self.index);
            }
            return Result::Ok(serde_json::Value::Null);
        } else if collection_value.is_object() {
            let collection = collection_value.as_object().unwrap();
            let collection_len = collection.len() as i32;
            let mut index = -1;
            for (key, item) in collection {
                index = index + 1;
                env[&self.item] = item.clone();
                env[&self.index] = json!(key);
                do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
                env.as_object_mut().unwrap().remove(&self.item);
                env.as_object_mut().unwrap().remove(&self.index);
            }
            return Result::Ok(serde_json::Value::Null);
        } else {
            return Result::Err(crate::core::Error::from(
                "[rbatis] collection name:".to_owned()
                    + self.collection.as_str()
                    + " is not a array or object/map value!",
            ));
        }
    }
}
