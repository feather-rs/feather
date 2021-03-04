//! Frontend parser for RON data files.
//!
//! Uses a query-like design inspired by rustc.

use crate::model::{Model, ModelFile, Type};
use anyhow::Context as _;
use heck::CamelCase;
use itertools::Either;
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use std::rc::Rc;

pub struct DataFile {
    pub contents: String,
    pub name: String,
}

/// Context of a running compilation.
struct Context<'a> {
    /// All parsed models, taken from `files`,
    /// plus the file names in which they're defined.
    models: &'a [(&'a str, Model<'a>)],
}

impl<'a> Context<'a> {
    /// Performs a query.
    pub fn query<Q>(&self, query: Q) -> anyhow::Result<Q::Output>
    where
        Q: Query<'a>,
    {
        // TODO: memoize query results
        // if compilation becomes too slow.
        query.execute(self)
    }

    /// Finds an enum model matching the
    /// given name.
    pub fn find_enum_model(&self, name: &str) -> Option<(&'a str, &'a Model<'a>)> {
        self.models
            .iter()
            .find(|(_, model)| match model {
                Model::Enum { name: _name, .. } => *_name == name,
                _ => false,
            })
            .map(|(file_name, model)| (*file_name, model))
    }

    /// Finds a property model matching the given name.
    pub fn find_property_model(&self, name: &str, on: &str) -> Option<&'a Model<'a>> {
        self.models
            .iter()
            .find(|(_, model)| match model {
                Model::Property {
                    name: _name,
                    on: _on,
                    ..
                } => *_name == name && *_on == on,
                _ => false,
            })
            .map(|(_, model)| model)
    }
}

trait Query<'a> {
    type Output;

    fn execute(&self, cx: &Context<'a>) -> anyhow::Result<Self::Output>;
}

/// A query which compiles an enum.
#[derive(Debug)]
struct CompileEnum<'a> {
    /// Name of enum to be compiled.
    name: &'a str,
}

impl<'a, 'b> Query<'a> for CompileEnum<'b> {
    type Output = Enum<'a>;

    fn execute(&self, cx: &Context<'a>) -> anyhow::Result<Self::Output> {
        let (_, model) = cx
            .find_enum_model(self.name)
            .ok_or_else(|| anyhow::anyhow!("no enum matched the name `{}`", self.name))?;

        let (name, variants) = match model {
            Model::Enum { variants, name } => (*name, variants.as_slice()),
            _ => unreachable!(),
        };

        let mut actual_variants = vec![];
        for variant in variants {
            actual_variants.extend(
                expand_expressions(cx, &[*variant], |_, _, _| true)
                    .with_context(|| format!("failed to expand expression `{}`", variant))?
                    .into_iter()
                    .map(|mut vec| vec.remove(0).1),
            );
        }

        let e = Enum {
            name,
            name_camel_case: self.name.to_camel_case(),
            variants_camel_case: actual_variants.iter().map(|s| s.to_camel_case()).collect(),
            variants: actual_variants,
        };

        Ok(e)
    }
}

/// A query which compiles a property.
struct CompileProperty<'a> {
    /// Name of the property to compile
    name: &'a str,
    /// Which enum this property is defined
    /// for (used to avoid issues with
    /// duplicate property names)
    on: &'a str,
}

impl<'a, 'b> Query<'b> for CompileProperty<'a> {
    type Output = Property<'b>;

    fn execute(&self, cx: &Context<'b>) -> anyhow::Result<Self::Output> {
        let model = cx
            .find_property_model(self.name, self.on)
            .with_context(|| format!("no property matched the name `{}`", self.name))?;

        let (on, mapping, name, typ, reverse) = match model {
            Model::Property {
                on,
                mapping,
                name,
                typ,
                reverse,
            } => (on, mapping, name, typ, *reverse),
            _ => unreachable!(),
        };

        let mapping = mapping
            .iter()
            .flat_map(|(keys, value)| {
                keys.iter().copied().zip(std::iter::repeat_with(move || {
                    Value::from_ron(value.clone(), typ.clone()).unwrap()
                }))
            })
            .collect::<BTreeMap<_, _>>();

        let on_enum =
            Rc::new(cx.query(CompileEnum { name: *on }).with_context(|| {
                format!("failed to compile `on: {}` for property {}", on, name)
            })?);

        // Expand expressions
        let mut actual_mapping = BTreeMap::new();
        for (key, value) in &mapping {
            let expressions = if let Value::Custom(x) = value {
                vec![*key, x.as_str()]
            } else {
                vec![*key]
            };

            let filter: Box<dyn Fn(&str) -> bool> = if let Type::Custom(enum_name) = typ {
                let e = cx.query(CompileEnum { name: enum_name })?;
                Box::new(move |variant| e.variants.contains(&Cow::Borrowed(variant)))
            } else {
                Box::new(|_| true)
            };

            let on_enum = Rc::clone(&on_enum);
            let filter2 = move |variant: &str| on_enum.variants.contains(&Cow::Borrowed(variant));

            let expanded = expand_expressions(cx, &expressions, |_, i, variant| {
                if i == 0 {
                    filter2(variant)
                } else {
                    filter(variant)
                }
            })?;

            let new_pairs = expanded.into_iter().map(|mut vec| {
                let (original_expression, key) = vec.remove(0);
                let value = if let Type::Custom(_) = typ {
                    Value::Custom(vec.remove(0).1.to_mut().clone())
                } else {
                    mapping[original_expression].clone()
                };
                (key, value)
            });

            actual_mapping.extend(new_pairs);
        }

        Ok(Property {
            on,
            name,
            reverse,
            typ: typ.clone(),
            mapping: actual_mapping
                .into_iter()
                .map(|(mut key, value)| (Cow::from(key.to_mut().clone()), value))
                .collect(),
        })
    }
}

type ExpandResult<'a> = Vec<(&'a str, Cow<'a, str>)>;

/// Expands one or more associated expressions in a data file.
///
/// # Format
/// Expressions may specify an expansion clause in the form `${var}`.
/// This clause will cause the expression to expand, with each
/// new expression corresponding to a variant of the enum `var`.
/// For example, consider an `enum color { red, green, blue }`.
/// `${color}` would expand to three new expressions: `[red, green, blue]`.
/// Similarly, `${color}_wool` would expand to the expressions `[red_wool, green_wool, blue_wool]`.
///
/// When there are multiple expansion clauses in a single expression,
/// then all pairs of variant names will be evauluated. For example,
/// let's add an `enum animal { dog, cat, chicken }`. With this context,
/// `${color}_${animal}` expands to the expressions
///
/// [red_dog, green_dog, blue_dog, red_cat, green_cat, blue_cat,
/// red_chicken, green_chicken, blue_chicken].
///
/// Multiple expressions may exist in the same context. For example, consider
/// the case of property mappings. A mapping might look like this:
/// `"${animal}_${color}": "${color}"`. The result of this would
/// be that for each expanded value, the two `color` clauses will
/// _match_ rather than expanding to all possible combinations of two colors.
///
/// # Filters
/// An optional filter function may be specified which filters
/// the expanded expressions by some predicate. We use this
/// to filter expressions by those which point to valid enum variants,
/// for convenience.
fn expand_expressions<'a>(
    cx: &Context<'a>,
    expressions: &[&'a str],
    mut filter: impl FnMut(&Context<'a>, usize, &str) -> bool,
) -> anyhow::Result<Vec<ExpandResult<'a>>> {
    // Determine the locations of expansion clauses in the expressions.
    // This is currently handled using regexes, which is unlikely to be optimal.
    static EXPR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\$\\{[^}]+}").unwrap());

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct MatchLocation {
        /// The index into `expressions` of this match
        expr: usize,
        /// The byte range of characters in the expression
        /// which from an expansion clause
        range: Range<usize>,
    }

    // Compute the locations of expansion clauses.
    let mut matches = vec![];
    for (i, expression) in expressions.iter().copied().enumerate() {
        let mut offset = 0;
        while let Some(m) = EXPR_REGEX.find(&expression[offset..]) {
            let location = MatchLocation {
                expr: i,
                range: Range {
                    start: m.start() + offset,
                    end: m.end() + offset,
                },
            };
            matches.push(location);
            offset += m.end();
        }
    }

    // Determine the variables being expanded in each clause.
    // This is a mapping from variable name => match locations for this variable.
    let mut variables: HashMap<&'a str, Vec<MatchLocation>> = HashMap::new();
    for m in &matches {
        let variable: &'a str = &(expressions[m.expr])[m.range.start + 2..m.range.end - 1];
        variables.entry(variable).or_default().push(m.clone());
    }

    // Perform expansion.
    let mut results: Vec<Vec<(&'a str, Cow<'a, str>)>> = vec![expressions
        .iter()
        .copied()
        .zip(expressions.iter().copied().map(Cow::from))
        .collect()];

    for (variable, _) in variables {
        // Determine the variants of the enum being expanded.
        let e = cx
            .query(CompileEnum { name: variable })
            .with_context(|| format!("failed to compile enum `{}`", variable))?;
        let variants = e.variants.as_slice();

        // Expand the pattern in `results`
        // to include each variant.
        let replace_pattern = Rc::new(format!("${{{}}}", variable));
        results = results
            .into_iter()
            .flat_map(|mut result| {
                let replace_pattern = Rc::clone(&replace_pattern);
                variants.iter().map(move |variant| {
                    result
                        .iter_mut()
                        .map(|(original, result)| {
                            (
                                *original,
                                result
                                    .to_mut()
                                    .clone()
                                    .replace(replace_pattern.as_str(), variant)
                                    .into(),
                            )
                        })
                        .collect()
                })
            })
            .collect();
    }

    results.retain(|result_set| {
        result_set
            .iter()
            .enumerate()
            .all(|(i, (_, res))| filter(cx, i, res))
    });

    Ok(results)
}

/// Creates a `Data` from a slice
/// of data files.
pub fn from_slice(files: &[DataFile]) -> anyhow::Result<Data> {
    let models: Vec<_> = files
        .iter()
        .map(parse_file)
        .collect::<anyhow::Result<Vec<_>>>()
        .context("failed to parse data files")?
        .into_iter()
        .flatten()
        .collect();

    // The borrow checker has stopped me, and I'm tired.
    // This is a build script. Memory leaks are fine.
    // FIXME
    let models = Box::leak(models.into_boxed_slice());

    let cx = Context { models };

    let mut data = Data::default();
    for (file_name, model) in cx.models {
        if let Model::Enum { name, .. } = model {
            let e = cx.query(CompileEnum { name: *name }).with_context(|| {
                format!(
                    "failed to compile enum `{}` defined in `{}`",
                    name, file_name
                )
            })?;
            data.files
                .entry(*file_name)
                .or_insert_with(|| FileData {
                    file_name: *file_name,
                    ..Default::default()
                })
                .enums
                .insert(e.name, e);
        } else if let Model::Property { name, on, .. } = model {
            let p = cx
                .query(CompileProperty {
                    name: *name,
                    on: *on,
                })
                .with_context(|| {
                    format!(
                        "failed to compile property `{}` defined in `{}`",
                        name, file_name
                    )
                })?;
            data.files
                .entry(*file_name)
                .or_insert_with(|| FileData {
                    file_name: *file_name,
                    ..Default::default()
                })
                .properties
                .push(p);
        }
    }

    Ok(data)
}

fn parse_file<'a>(
    file: &'a DataFile,
) -> anyhow::Result<impl Iterator<Item = (&'a str, Model)> + 'a> {
    Ok(
        match crate::model::from_str(&file.contents)
            .with_context(|| format!("failed to parse file `{}`", file.name))?
        {
            ModelFile::Single(model) => Either::Left(std::iter::once((file.name.as_str(), model))),
            ModelFile::Multiple(models) => {
                Either::Right(std::iter::repeat(file.name.as_str()).zip(models))
            }
        },
    )
}

#[derive(Default, Debug)]
pub struct Data<'a> {
    /// Mapping from file name => file contents
    pub files: BTreeMap<&'a str, FileData<'a>>,
}

#[derive(Debug, Default)]
pub struct FileData<'a> {
    /// File name (without extension)
    pub file_name: &'a str,
    /// The enums defined in this file
    ///
    /// Mapping from enum names => enum
    pub enums: BTreeMap<&'a str, Enum<'a>>,
    /// The properties defined in this file
    pub properties: Vec<Property<'a>>,
}

#[derive(Debug, Default)]
pub struct Enum<'a> {
    pub name: &'a str,
    pub name_camel_case: String,

    pub variants: Vec<Cow<'a, str>>,
    pub variants_camel_case: Vec<String>,
}

#[derive(Debug)]
pub struct Property<'a> {
    pub on: &'a str,
    pub name: &'a str,
    pub typ: Type<'a>,
    pub reverse: bool,
    /// Mapping from variant names => values
    pub mapping: BTreeMap<Cow<'a, str>, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    U32(u32),
    F64(f64),
    String(String),
    Slice(Vec<Value>),
    Bool(bool),
    /// custom type - name of enum variant
    Custom(String),
}

impl Value {
    pub fn from_ron(r: ron::Value, typ: Type) -> anyhow::Result<Self> {
        use ron::Value as Ron;

        Ok(match r {
            Ron::Number(n) => match typ {
                Type::U32 => Value::U32(n.as_i64().unwrap() as u32),
                Type::F64 => Value::F64(n.as_f64().unwrap_or_else(|| n.as_i64().unwrap() as f64)),
                t => anyhow::bail!("value {:?} is not a valid instance of type {:?}", t, r),
            },
            Ron::String(s) if typ == Type::String => Value::String(s),
            Ron::String(s) => Value::Custom(s),
            Ron::Seq(values) => Value::Slice(
                values
                    .into_iter()
                    .map(|v| Value::from_ron(v, typ.clone()))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ),
            Ron::Bool(x) => Value::Bool(x),
            r => anyhow::bail!("value {:?} is not supported for type {:?}", r, typ),
        })
    }
}
