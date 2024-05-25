use std::fmt::format;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Label(String);

#[derive(Debug, PartialEq, Eq)]
pub struct MakeLabelError(String);

impl Label {
    pub fn make(s: String) -> Result<Label, MakeLabelError> {
        //TODO: verify the string here
        Ok(Label(s))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier(String);

pub struct MakeIdentifierError(String);

impl Identifier {
    pub fn make(s: String) -> Result<Identifier, MakeIdentifierError> {
        //TODO: verify the string here
        Ok(Identifier(s))
    }
}

pub struct MakeRecordError(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<T>(Vec<(Label, T)>);
impl<T> Record<T> {
    pub fn make(v: Vec<(Label, T)>) -> Result<Record<T>, MakeRecordError> {
        //TODO: Vectors should:
        //- be lexicografically sorted
        //- labels must be unique
        Ok(Record(v))
    }
}

fn unsafe_make_record<T>(v: Vec<(Label, T)>) -> Record<T> {
    Record(v)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Uint,
    String,
    Variable(u64),
    ExternalVariable(Identifier),
    // forall a b c . a -> (a-> b) -> c
    // Forall(3, Arrow(2,Arrow(Arrow(2,1), 0)))
    // Forall (0,t) means just t without binded variables
    Forall(u64, Box<Type>),
    Arrow(Box<Type>, Box<Type>),
    Record(Record<Type>),
    // data T a = (a,T)
    // Recursive(1, Forall (1, Tuple(0,1)))
    // Recursive(0,t) is just t
    Recursive(u64, Box<Type>),
    Application(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    //The constructor name is the index of the type in the sum
    //data w a b c = S1 | S2 a | S3 b a c
    //Forall(3,Sum([(Sum([])),[2],[1,2,0]]))
    Sum(Vec<Type>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CaseAlternative {
    constructor: u64,
    arguments: u64,
    value: Term,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Uint(u64),
    String(String),
    Variable(u64, Type),
    ExternalVariable(Identifier),
    GlobalVariable(Identifier, Type),
    TermFunction(Box<Term>, Type),
    Application(Box<Term>, Box<Term>, Type),
    Record(Record<Term>, Type),
    Tuple(Vec<Term>, Type),
    Case(Box<Term>, Vec<CaseAlternative>, Type),
}

trait ContextInterface<contextType, valuesType> {
    fn find(&self, name: &str) -> &Vec<valuesType>;
    fn update(&mut self, name: &str, value: valuesType) -> &mut contextType;
}

pub struct LocalContext();
pub struct ExternalContext();
pub struct ModuleContext();
pub struct Context<'a> {
    local: &'a LocalContext,
    external: &'a ExternalContext,
    module: &'a ModuleContext,
}

/*
(if e then x else z):T

becomes

data Bool = True | False
(case e of
    True => x
    False => y):T

Then in core it became

Bool = Sum(0,[[],[]])

Case(e,[CaseAlternative(0,[],x),CaseAlternative(1,[],y)],T)

impl Type {
    fn shift(t:Self,amount:u64, nest_level:u64)->Self{
        match t {
            Type::Uint=> t,
            Type::String=>t,
            Type::Variable(x)=> if x >= nest_level {Type::Variable(x+amount)} else {t},
            Type::Forall(new_levels,t2) => shift(),
            Type::Arrow(Box<Type>, Box<Type>),
            Type::Record(Record<Type>),
            Type::RecursiveType(Box<Type>),
            Type::Application(Box<Type>, Box<Type>),
            Type::Tuple(Vec<Type>),
            Type::Sum(u64, Vec<Vec<Type>>),
        }
    }


    fn substitute_type_var(
        type1: Type,
        variable_number: u64,
        type2: Type,
    ) -> Type {
        match type1 {
            Type::Uint => type1,
            Type::String => type1,
            Type::Variable(x) => {
                if x == variable_number {
                    type2
                } else {
                    type1
                }
            }
            Type::Forall(t1) => {
                Type::substitute_type_var(*t1, variable_number + 1, type2)
            }
            Type::Arrow(t1, t2) => Type::Arrow(
                Box::from(Type::substitute_type_var(
                    *t1,
                    variable_number,
                    type2.clone(),
                )),
                Box::from(Type::substitute_type_var(
                    *t2,
                    variable_number,
                    type2,
                )),
            ),
            Type::Record(Record(v)) => Type::Record(unsafe_make_record(
                v.iter()
                    .map(|(label, t)| {
                        (
                            (*label).clone(),
                            Type::substitute_type_var(
                                (*t).clone(),
                                variable_number,
                                type2.clone(),
                            ),
                        )
                    })
                    .collect(),
            )),
            Type::RecursiveType(t1) => Type::RecursiveType(Box::from(
                Type::substitute_type_var(*t1, variable_number + 1, type2),
            )),
            Type::Application(t1, t2) => Type::Application(
                Box::from(Type::substitute_type_var(
                    *t1,
                    variable_number,
                    type2.clone(),
                )),
                Box::from(Type::substitute_type_var(
                    *t2,
                    variable_number,
                    type2,
                )),
            ),
            Type::Tuple(v) => Type::Tuple(
                v.iter()
                    .map(|t| {
                        Type::substitute_type_var(
                            (*t).clone(),
                            variable_number,
                            type2.clone(),
                        )
                    })
                    .collect(),
            ),
            Type::Sum(binded_vars, v1) => Type::Sum(
                binded_vars,
                v1.iter()
                    .map(|v2| {
                        v2.iter()
                            .map(|t| {
                                Type::substitute_type_var(
                                    (*t).clone(),
                                    variable_number + binded_vars,
                                    type2.clone(),
                                )
                            })
                            .collect()
                    })
                    .collect(),
            ),
        }
    }

    fn reduction(type_: Type) -> Type {
        match type_ {
            Type::Uint => type_,
            Type::String => type_,
            Type::Variable(_) => type_,
            Type::Forall(t1) => Type::Forall(Box::from(Type::reduction(*t1))),
            Type::Arrow(t1, t2) => Type::Arrow(
                Box::from(Type::reduction(*t1)),
                Box::from(Type::reduction(*t2)),
            ),
            Type::Record(Record(v)) => Type::Record(unsafe_make_record(
                v.iter()
                    .map(|(label, t)| {
                        ((*label).clone(), Type::reduction((*t).clone()))
                    })
                    .collect(),
            )),
            Type::RecursiveType(t) => {
                Type::RecursiveType(Box::from(Type::reduction(*t)))
            }
            Type::Application(t1, t2) => match *t1 {
                Type::Forall(t3) => Type::substitute_type_var(*t3, 0, *t2),
                _ => Type::Application(
                    Box::from(Type::reduction(*t1)),
                    Box::from(Type::reduction(*t2)),
                ),
            },
            Type::Tuple(v) => Type::Tuple(
                v.iter().map(|t| Type::reduction((*t).clone())).collect(),
            ),
            Type::Sum(binded_vars, v1) => Type::Sum(
                binded_vars,
                v1.iter()
                    .map(|v2| {
                        v2.iter()
                            .map(|t| Type::reduction((*t).clone()))
                            .collect()
                    })
                    .collect(),
            ),
        }
    }
    fn unify(type1: Type, type2: Type) -> Result<(), String> {
        let simplified1 = Type::reduction(type1.clone());
        let simplified2 = Type::reduction(type2.clone());
        if simplified1 == simplified2 {
            Ok(())
        } else {
            Err(format!("Can't unify types {:?} and {:?}", type1, type2))
        }
    }
}

impl Term {
    pub fn check(
        self,
        t: Type,
        term_stack: &mut Vec<Type>,
        type_stack: &mut Vec<Type>,
    ) -> Result<(), String> {
        match self {
            Term::Uint(_) => Type::unify(t, Type::Uint),
            Term::String(_) => Type::unify(t, Type::String),
            Term::Variable(_, t2) => Type::unify(t, t2),
            Term::GlobalVariable(_, t2) => Type::unify(t, t2),
            Term::TermFunction(term, t2) => {
                let rt2 = Type::reduction(t2);
                let rt = Type::reduction(t);
                match (rt2.clone(), rt.clone()) {
                    (Type::Arrow(t3, t4), Type::Arrow(t5, t6)) => {
                        match Type::unify(*t3, (*t5).clone()).and_then(|x| {
                            Type::unify((*t4).clone(), (*t6).clone())
                        }) {
                            Ok(_) => {
                                term_stack.push(*t5);
                                Term::check(*term, *t6, term_stack, type_stack)
                            }
                            Err(msg) => Err(msg),
                        }
                    }
                    (Type::Forall(t3),Type::Forall(t4))=>
                    _ => {
                        Err(format!("Can't unify types {:?} and {:?}", rt2, rt))
                    }
                }
            }
            Term::TypeFunction(term, t2) => {
                let rt2 = Type::reduction(t2);
                let rt = Type::reduction(t);
                match (rt2.clone(), rt.clone()) {
                    (Type::Forall(t3), Type::Forall(t4)) => {
                        Type::unify((*t3).clone(), *t4).and_then(|_| {
                            type_stack.push()
                            Term::check(*term, *t3, term_stack, type_stack)
                        })
                    }
                    _ => {
                        Err(format!("Can't unify types {:?} and {:?}", rt2, rt))
                    }
                }
            }
            //Beware: stack can be mutated by the first term, so you need to avoid mutating it in
            //second term
            //Application(Box<Term>, Box<Term>, t2),
            //Record(Record<Term>, Type),
            //Tuple(Vec<Term>, Type),
            //Case(Term, Vec<CaseAlternative>, Type),
            _ => Err(String::from(format!("not implemented case {:?}", self))),
        }
    }
}
*/
