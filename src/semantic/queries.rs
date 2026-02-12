pub const RUST_QUERY: &str = r#"
    (function_item name: (identifier) @function.name) @function
    (struct_item name: (type_identifier) @struct.name) @struct
    (impl_item type: (type_identifier) @impl.name) @impl
    (trait_item name: (type_identifier) @trait.name) @trait
    (mod_item name: (identifier) @module.name) @module
    (type_item name: (type_identifier) @type.name) @type
    (enum_item name: (type_identifier) @enum.name) @enum
    (call_expression function: (identifier) @call.name) @call
    (call_expression function: (field_expression field: (field_identifier) @call.name)) @call
"#;

pub const PYTHON_QUERY: &str = r#"
    (function_definition name: (identifier) @function.name) @function
    (class_definition name: (identifier) @class.name) @class
"#;

pub const JAVASCRIPT_QUERY: &str = r#"
    (function_declaration name: (identifier) @function.name) @function
    (method_definition name: (property_identifier) @method.name) @method
    (class_declaration name: (identifier) @class.name) @class
    (variable_declarator name: (identifier) @variable.name value: (arrow_function)) @function
"#;

pub const TYPESCRIPT_QUERY: &str = r#"
    (function_declaration name: (identifier) @function.name) @function
    (method_definition name: (property_identifier) @method.name) @method
    (class_declaration name: (identifier) @class.name) @class
    (interface_declaration name: (type_identifier) @interface.name) @interface
    (type_alias_declaration name: (type_identifier) @type.name) @type
    (enum_declaration name: (identifier) @enum.name) @enum
"#;

pub const GO_QUERY: &str = r#"
    (function_declaration name: (identifier) @function.name) @function
    (method_declaration name: (field_identifier) @method.name) @method
    (type_declaration (type_spec name: (type_identifier) @type.name)) @type
"#;

pub const C_QUERY: &str = r#"
    (function_definition declarator: (function_declarator declarator: (identifier) @function.name)) @function
    (struct_specifier name: (type_identifier) @struct.name) @struct
    (type_definition declarator: (type_identifier) @type.name) @type
"#;

pub const CPP_QUERY: &str = r#"
    (function_definition declarator: (function_declarator declarator: (identifier) @function.name)) @function
    (class_specifier name: (type_identifier) @class.name) @class
    (struct_specifier name: (type_identifier) @struct.name) @struct
    (namespace_definition name: (identifier) @namespace.name) @namespace
"#;

pub const JAVA_QUERY: &str = r#"
    (class_declaration name: (identifier) @class.name) @class
    (interface_declaration name: (identifier) @interface.name) @interface
    (method_declaration name: (identifier) @method.name) @method
    (constructor_declaration name: (identifier) @method.name) @method
"#;

pub const RUBY_QUERY: &str = r#"
    (method name: (identifier) @method.name) @method
    (class name: [
        (constant) @class.name
        (scope_resolution name: (constant) @class.name)
    ]) @class
    (module name: [
        (constant) @module.name
        (scope_resolution name: (constant) @module.name)
    ]) @module
"#;

pub const C_SHARP_QUERY: &str = r#"
    (class_declaration name: (identifier) @class.name) @class
    (interface_declaration name: (identifier) @interface.name) @interface
    (method_declaration name: (identifier) @method.name) @method
    (struct_declaration name: (identifier) @struct.name) @struct
    (enum_declaration name: (identifier) @enum.name) @enum
    (namespace_declaration name: [
        (identifier) @namespace.name
        (qualified_name) @namespace.name
    ]) @namespace
"#;

pub const PHP_QUERY: &str = r#"
    (function_definition name: (identifier) @function.name) @function
    (method_definition name: (identifier) @method.name) @method
    (class_declaration name: (identifier) @class.name) @class
    (interface_declaration name: (identifier) @interface.name) @interface
    (trait_declaration name: (identifier) @trait.name) @trait
"#;

pub const JSON_QUERY: &str = r#"
    (pair key: (string (string_content) @key.name)) @pair
"#;

pub const HTML_QUERY: &str = r#"
    (tag_name) @tag.name
    (attribute_name) @attr.name
"#;

pub const CSS_QUERY: &str = r#"
    (class_selector (class_name) @class.name) @class
    (id_selector (id_name) @id.name) @id
    (declaration property: (property_name) @prop.name) @decl
"#;

pub const MARKDOWN_QUERY: &str = r#"
    (atx_heading (atx_h1_marker) (heading_content) @h1.name) @h1
    (atx_heading (atx_h2_marker) (heading_content) @h2.name) @h2
    (atx_heading (atx_h3_marker) (heading_content) @h3.name) @h3
"#;
