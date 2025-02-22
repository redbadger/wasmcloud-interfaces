// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// Response to get request
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetResponse {
    /// whether or not the value existed
    #[serde(default)]
    pub exists: bool,
    /// the value, if it existed
    #[serde(default)]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct IncrementRequest {
    /// name of value to increment
    #[serde(default)]
    pub key: String,
    /// amount to add to value
    pub value: i32,
}

/// Parameter to ListAdd operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListAddRequest {
    /// name of the list to modify
    #[serde(rename = "listName")]
    #[serde(default)]
    pub list_name: String,
    /// value to append to the list
    #[serde(default)]
    pub value: String,
}

/// Removes an item from the list. If the item occurred more than once,
/// removes only the first item.
/// Returns true if the item was found.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListDelRequest {
    /// name of list to modify
    #[serde(rename = "listName")]
    #[serde(default)]
    pub list_name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListRangeRequest {
    /// name of list
    #[serde(rename = "listName")]
    #[serde(default)]
    pub list_name: String,
    /// start index of the range, 0-based, inclusive.
    pub start: i32,
    /// end index of the range, 0-based, inclusive.
    pub stop: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetAddRequest {
    /// name of the set
    #[serde(rename = "setName")]
    #[serde(default)]
    pub set_name: String,
    /// value to add to the set
    #[serde(default)]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetDelRequest {
    #[serde(rename = "setName")]
    #[serde(default)]
    pub set_name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetRequest {
    /// expiration time in seconds 0 for no expiration
    pub expires: u32,
    /// the key name to change (or create)
    #[serde(default)]
    pub key: String,
    /// the new value
    #[serde(default)]
    pub value: String,
}

/// list of strings
pub type StringList = Vec<String>;

/// wasmbus.contractId: wasmcloud:keyvalue
/// wasmbus.providerReceive
#[async_trait]
pub trait KeyValue {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:keyvalue"
    }
    /// Increments a numeric value, returning the new value
    async fn increment(&self, ctx: &Context, arg: &IncrementRequest) -> RpcResult<i32>;
    /// returns whether the store contains the key
    async fn contains<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool>;
    /// Deletes a key, returning true if the key was deleted
    async fn del<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool>;
    /// Gets a value for a specified key. If the key exists,
    /// the return structure contains exists: true and the value,
    /// otherwise the return structure contains exists == false.
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse>;
    /// Append a value onto the end of a list. Returns the new list size
    async fn list_add(&self, ctx: &Context, arg: &ListAddRequest) -> RpcResult<u32>;
    /// Deletes a list and its contents
    /// input: list name
    /// output: true if the list existed and was deleted
    async fn list_clear<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool>;
    /// Deletes a value from a list. Returns true if the item was removed.
    async fn list_del(&self, ctx: &Context, arg: &ListDelRequest) -> RpcResult<bool>;
    /// Retrieves a range of values from a list using 0-based indices.
    /// Start and end values are inclusive, for example, (0,10) returns
    /// 11 items if the list contains at least 11 items. If the stop value
    /// is beyond the end of the list, it is treated as the end of the list.
    async fn list_range(&self, ctx: &Context, arg: &ListRangeRequest) -> RpcResult<StringList>;
    /// Sets the value of a key.
    /// expires is an optional number of seconds before the value should be automatically deleted,
    /// or 0 for no expiration.
    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()>;
    /// Add an item into a set. Returns number of items added (1 or 0)
    async fn set_add(&self, ctx: &Context, arg: &SetAddRequest) -> RpcResult<u32>;
    /// Deletes an item from the set. Returns number of items removed from the set (1 or 0)
    async fn set_del(&self, ctx: &Context, arg: &SetDelRequest) -> RpcResult<u32>;
    /// perform intersection of sets and returns values from the intersection.
    /// input: list of sets for performing intersection (at least two)
    /// output: values
    async fn set_intersection(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList>;
    /// Retrieves all items from a set
    /// input: String
    /// output: set members
    async fn set_query<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<StringList>;
    /// perform union of sets and returns values from the union
    /// input: list of sets for performing union (at least two)
    /// output: union of values
    async fn set_union(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList>;
    /// clears all values from the set and removes it
    /// input: set name
    /// output: true if the set existed and was deleted
    async fn set_clear<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool>;
}

/// KeyValueReceiver receives messages defined in the KeyValue service trait
#[doc(hidden)]
#[async_trait]
pub trait KeyValueReceiver: MessageDispatch + KeyValue {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Increment" => {
                let value: IncrementRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::increment(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.Increment",
                    arg: buf,
                })
            }
            "Contains" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::contains(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.Contains",
                    arg: buf,
                })
            }
            "Del" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::del(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.Del",
                    arg: buf,
                })
            }
            "Get" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::get(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.Get",
                    arg: buf,
                })
            }
            "ListAdd" => {
                let value: ListAddRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::list_add(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.ListAdd",
                    arg: buf,
                })
            }
            "ListClear" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::list_clear(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.ListClear",
                    arg: buf,
                })
            }
            "ListDel" => {
                let value: ListDelRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::list_del(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.ListDel",
                    arg: buf,
                })
            }
            "ListRange" => {
                let value: ListRangeRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::list_range(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.ListRange",
                    arg: buf,
                })
            }
            "Set" => {
                let value: SetRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.Set",
                    arg: buf,
                })
            }
            "SetAdd" => {
                let value: SetAddRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_add(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetAdd",
                    arg: buf,
                })
            }
            "SetDel" => {
                let value: SetDelRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_del(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetDel",
                    arg: buf,
                })
            }
            "SetIntersection" => {
                let value: StringList = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_intersection(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetIntersection",
                    arg: buf,
                })
            }
            "SetQuery" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_query(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetQuery",
                    arg: buf,
                })
            }
            "SetUnion" => {
                let value: StringList = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_union(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetUnion",
                    arg: buf,
                })
            }
            "SetClear" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = KeyValue::set_clear(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "KeyValue.SetClear",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "KeyValue::{}",
                message.method
            ))),
        }
    }
}

/// KeyValueSender sends messages to a KeyValue service
/// client for sending KeyValue messages
#[derive(Debug)]
pub struct KeyValueSender<T: Transport> {
    transport: T,
}

impl<T: Transport> KeyValueSender<T> {
    /// Constructs a KeyValueSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<'send> KeyValueSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl KeyValueSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a KeyValue provider
    /// implementing the 'wasmcloud:keyvalue' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:keyvalue", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a KeyValue provider
    /// implementing the 'wasmcloud:keyvalue' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:keyvalue", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> KeyValue for KeyValueSender<T> {
    #[allow(unused)]
    /// Increments a numeric value, returning the new value
    async fn increment(&self, ctx: &Context, arg: &IncrementRequest) -> RpcResult<i32> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Increment",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Increment", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// returns whether the store contains the key
    async fn contains<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Contains",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Contains", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Deletes a key, returning true if the key was deleted
    async fn del<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Del",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Del", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Gets a value for a specified key. If the key exists,
    /// the return structure contains exists: true and the value,
    /// otherwise the return structure contains exists == false.
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Get",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Get", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Append a value onto the end of a list. Returns the new list size
    async fn list_add(&self, ctx: &Context, arg: &ListAddRequest) -> RpcResult<u32> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ListAdd",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListAdd", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Deletes a list and its contents
    /// input: list name
    /// output: true if the list existed and was deleted
    async fn list_clear<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ListClear",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListClear", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Deletes a value from a list. Returns true if the item was removed.
    async fn list_del(&self, ctx: &Context, arg: &ListDelRequest) -> RpcResult<bool> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ListDel",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListDel", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Retrieves a range of values from a list using 0-based indices.
    /// Start and end values are inclusive, for example, (0,10) returns
    /// 11 items if the list contains at least 11 items. If the stop value
    /// is beyond the end of the list, it is treated as the end of the list.
    async fn list_range(&self, ctx: &Context, arg: &ListRangeRequest) -> RpcResult<StringList> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ListRange",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListRange", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Sets the value of a key.
    /// expires is an optional number of seconds before the value should be automatically deleted,
    /// or 0 for no expiration.
    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Set",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    /// Add an item into a set. Returns number of items added (1 or 0)
    async fn set_add(&self, ctx: &Context, arg: &SetAddRequest) -> RpcResult<u32> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetAdd",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetAdd", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Deletes an item from the set. Returns number of items removed from the set (1 or 0)
    async fn set_del(&self, ctx: &Context, arg: &SetDelRequest) -> RpcResult<u32> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetDel",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetDel", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// perform intersection of sets and returns values from the intersection.
    /// input: list of sets for performing intersection (at least two)
    /// output: values
    async fn set_intersection(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetIntersection",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetIntersection", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Retrieves all items from a set
    /// input: String
    /// output: set members
    async fn set_query<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<StringList> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetQuery",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetQuery", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// perform union of sets and returns values from the union
    /// input: list of sets for performing union (at least two)
    /// output: union of values
    async fn set_union(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetUnion",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetUnion", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// clears all values from the set and removes it
    /// input: set name
    /// output: true if the set existed and was deleted
    async fn set_clear<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SetClear",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "SetClear", e)))?;
        Ok(value)
    }
}
