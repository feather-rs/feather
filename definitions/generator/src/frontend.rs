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
                expand_expressions(cx, &[*variant], |_, _| true)
                    .with_context(|| format!("failed to expand expression `{}`", variant))?
                    .remove(0),
            );
        }

        let e = Enum {
            name,
            name_camel_case: self.name.to_camel_case(),
            variants: actual_variants,
            variants_camel_case: variants.iter().map(|s| s.to_camel_case()).collect(),
        };

        Ok(e)
    }
}

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
    mut filter: impl FnMut(&Context<'a>, &str) -> bool,
) -> anyhow::Result<Vec<Vec<Cow<'a, str>>>> {
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
                range: m.range(),
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
    let mut results: Vec<Vec<Cow<'a, str>>> = expressions
        .iter()
        .map(|expr| vec![(*expr).into()])
        .collect();

    for (variable, _) in variables {
        // Determine the variants of the enum being expanded.
        let e = cx
            .query(CompileEnum { name: variable })
            .with_context(|| format!("failed to compile enum `{}`", variable))?;
        let variants = e.variants.as_slice();

        // Expand the pattern in `results`
        // to include each variant.
        let replace_pattern = format!("${{{}}}", variable);
        for variant in variants {
            results = results
                .into_iter()
                .map(|result| {
                    result
                        .into_iter()
                        .map(|mut result| result.to_mut().replace(&replace_pattern, variant))
                        .map(Cow::from)
                        .collect()
                })
                .collect();
        }
    }

    for results in &mut results {
        results.retain(|result| filter(cx, &result));
    }

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
    ///
    /// Mapping from property names => property
    pub properties: BTreeMap<&'a str, Property<'a>>,
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
    /// Mapping from variant names => values
    pub mapping: BTreeMap<&'a str, Value>,
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
                Type::F64 => Value::F64(n.as_f64().unwrap()),
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
