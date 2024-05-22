// SPDX-FileCopyrightText: © 2024 Xronos Inc.
// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;

use lexpr::datum;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Reactor,
    Import,
    Preamble,
    Target,
    Reactors,
    Imports,
    Preambles,
    Instantiations,
    IsMain,
    IsFederated,
    IsRealtime,
    TodoUnrecognized,
}
#[derive(Debug)]
pub struct LfFile {
    pub path: PathBuf,
    pub model: Model,
}
#[derive(Debug)]
pub struct Model {
    pub target: Target,
    pub imports: Vec<Import>,
    pub preambles: Vec<Preamble>,
    pub reactors: Vec<Reactor>,
}
#[derive(Debug)]
pub struct Target {
    // cst: Datum,
}
#[derive(Debug)]
pub struct Import {
    // cst: Datum,
}
#[derive(Debug)]
pub struct Preamble {
    // cst: Datum,
}
#[derive(Debug)]
pub struct Instantiation {
    pub name: String,
}

// sList(
//     "reactor",
//     new SAtom<>(object.getName()),
//     fingerprint(object),
//     sList("attributes", object.getAttributes()),
//     sList("is-main", object.isMain()),
//     sList("is-federated", object.isFederated()),
//     sList("is-realtime", object.isRealtime()),
//     sList("typeParms", object.getTypeParms()),
//     sList("parameters", object.getParameters()),
//     sList("host", object.getHost()),
//     sList("extends", object.getSuperClasses()),
//     sList("preambles", object.getPreambles()),
//     sList("state", object.getStateVars()),
//     sList("methods", object.getMethods()),
//     sList("inputs", object.getInputs()),
//     sList("outputs", object.getOutputs()),
//     sList("timers", object.getTimers()),
//     sList("actions", object.getActions()),
//     sList("watchdogs", object.getWatchdogs()),
//     sList("instantiations", object.getInstantiations()),
//     sList("connections", object.getConnections()),
//     sList("reactions", object.getReactions()),
//     sList("modes", object.getModes()));
#[derive(Debug)]
pub struct Reactor {
    pub name: String,
    // fingerprint: [u8],
    // attributes: Vec<Attribute>,
    pub is_main: bool,
    pub is_federated: bool,
    // is_realtime: bool,
    // type_parms: Vec<TypeParm>,
    // parameters: Vec<Parameter>,
    // host: String,
    // extends: Vec<String>,
    // preambles: Vec<Preamble>,
    // state: Vec<StateVar>,
    // methods: Vec<Method>,
    // inputs: Vec<Input>,
    // outputs: Vec<Output>,
    // timers: Vec<Timer>,
    // actions: Vec<Action>,
    // watchdogs: Vec<Watchdog>,
    pub instantiations: Vec<Instantiation>,
    // connections: Vec<Connection>,
    // reactions: Vec<Reaction>,
    // modes: Vec<Mode>,
}

pub fn parse(cst: &str) -> LfFile {
    let cst = lexpr::datum::from_str(&cst).unwrap();
    let model = parse_model(cst.as_ref());
    LfFile {
        path: PathBuf::from(
            cst.list_iter()
                .unwrap()
                .next()
                .unwrap()
                .list_iter()
                .unwrap()
                .next()
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ),
        model,
    }
}

fn parse_model(cst: datum::Ref) -> Model {
    let mut iter = cst.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let mut rest = iter.next().unwrap();
    assert!(iter.next().is_none());
    let mut iter = rest.list_iter().unwrap();
    let checkme = iter.next().unwrap();
    let mut next = iter.next().unwrap();
    let mut target = parse_target(next);
    let mut imports = Option::None;
    let mut preambles = Option::None;
    let mut reactors = Option::None;
    let mut maybe_next = iter.next();
    while let Some(next) = maybe_next {
        let (keyword, cst) = get_field(next);
        dbg!("DEBUG: found keyword {:?}", keyword);
        match keyword {
            Keyword::Reactors => {
                assert!(reactors.is_none());
                reactors = Some(parse_reactors(cst));
            }
            Keyword::Imports => {
                assert!(imports.is_none());
                imports = Some(parse_imports(cst));
            }
            Keyword::Preambles => {
                assert!(preambles.is_none());
                preambles = Some(parse_preambles(cst));
            }
            _ => panic!("unexpected keyword {:?}", keyword),
        }
        maybe_next = iter.next();
    }
    assert!(iter.next().is_none());
    Model {
        target,
        imports: imports.unwrap_or_default(),
        preambles: preambles.unwrap_or_default(),
        reactors: reactors.unwrap_or_default(),
    }
}

fn get_field(cst: datum::Ref) -> (Keyword, datum::Ref) {
    let mut iter = cst.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    dbg!(cst.value().to_string());
    let (header, rest) = iter.next().unwrap().as_pair().unwrap();
    let mut iter = header.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    assert!(iter.next().is_none());
    let name = name.as_symbol().unwrap();
    let k = match name {
        "reactors" => Keyword::Reactors,
        "imports" => Keyword::Imports,
        "preambles" => Keyword::Preambles,
        "reactor" => Keyword::Reactor,
        "import" => Keyword::Import,
        "preamble" => Keyword::Preamble,
        "target" => Keyword::Target,
        "instantiations" => Keyword::Instantiations,
        "is-main" => Keyword::IsMain,
        "is-federated" => Keyword::IsFederated,
        "is-realtime" => Keyword::IsRealtime,
        _ => Keyword::TodoUnrecognized,
        // _ => panic!("unexpected symbol \"{}\"", name),
    };
    (k, rest)
}

fn parse_target(cst: datum::Ref) -> Target {
    Target { /*cst: cst.into()*/ }
}

fn parse_imports(cst: datum::Ref) -> Vec<Import> {
    cst.list_iter()
        .unwrap()
        .map(|cst| Import { /*cst: cst.into()*/ })
        .collect()
}

fn parse_preambles(cst: datum::Ref) -> Vec<Preamble> {
    cst.list_iter()
        .unwrap()
        .map(|cst| Preamble { /*cst: cst.into()*/ })
        .collect()
}

fn parse_reactors(cst: datum::Ref) -> Vec<Reactor> {
    cst.list_iter()
        .unwrap()
        .map(|cst| parse_reactor(cst))
        .collect()
}

fn parse_reactor(cst: datum::Ref) -> Reactor {
    let mut iter = cst.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let mut iter = iter.next().unwrap().list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    let fingerprint = iter.next().unwrap();
    dbg!(fingerprint.value().to_string());
    let mut instantiations = Option::None;
    let mut is_main = Option::None;
    let mut is_federated = Option::None;
    while let Some(next) = iter.next() {
        let (keyword, cst) = get_field(next);
        match keyword {
            Keyword::IsMain => {
                assert!(is_main.is_none());
                is_main = Some(parse_bool(cst));
            }
            Keyword::IsFederated => {
                assert!(is_federated.is_none());
                is_federated = Some(parse_bool(cst));
            }
            Keyword::Instantiations => {
                assert!(instantiations.is_none());
                instantiations = Some(parse_instantiations(cst));
            }
            _ => {
                dbg!(keyword);
            } // _ => panic!("unexpected keyword {:?}", keyword),
        }
    }
    dbg!(name.value().to_string());
    let mut iter = name.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    dbg!(name.value());
    let name = name.as_str().unwrap();
    // let fingerprint = fingerprint.as_bytes().unwrap();
    Reactor {
        name: name.to_string(),
        instantiations: instantiations.unwrap_or_default(),
        is_main: is_main.unwrap_or(false),
        is_federated: is_federated.unwrap_or(false),
    }
}

fn parse_instantiations(cst: datum::Ref) -> Vec<Instantiation> {
    cst.list_iter()
        .unwrap()
        .map(|cst| parse_instantiation(cst))
        .collect()
}

fn parse_instantiation(cst: datum::Ref) -> Instantiation {
    let mut iter = cst.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    let mut iter = name.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    let mut iter = name.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let name = iter.next().unwrap();
    // assert!(iter.next().is_none()); // FIXME
    dbg!(name.value().to_string());
    let name = name.value().as_str().unwrap();
    Instantiation {
        name: name.to_string(),
    }
}

fn parse_bool(cst: datum::Ref) -> bool {
    let mut iter = cst.list_iter().unwrap();
    let unboxed = iter.next().unwrap();
    assert!(iter.next().is_none());
    let mut iter = unboxed.list_iter().unwrap();
    let ogspan = iter.next().unwrap();
    let value = iter.next().unwrap();
    dbg!(value.value());
    assert!(iter.next().is_none());
    value.as_bool().unwrap()
}
