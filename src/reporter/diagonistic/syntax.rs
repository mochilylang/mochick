#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
  UnknownVariable {
    loc  : Span,
    name : String,
    /// If there's a discarded variable with the same name in the same scope
    /// this will contain its location.
    discarded : Option<Span>,
  },

  UnknownType {
    loc  : Span,
    name : EcoString,
    hint : Vec<String>, // edit
  },

  UnknownModule {
    loc  : Span,
    name : String,
    hint : Vec<String>, // edit
  },
  /*
  UnknownModuleValue {
    location: Span,
    name: EcoString,
    module_name: EcoString,
    value_constructors: Vec<EcoString>,
    type_with_same_name: bool,
    context: ModuleValueUsageContext,
  },

  ModuleAliasUsedAsName {
    location: SrcSpan,
    name: EcoString,
  },

  NotFn {
    location: SrcSpan,
    type_: Arc<Type>,
  },

  UnknownRecordField {
    location: SrcSpan,
    type_: Arc<Type>,
    label: EcoString,
    fields: Vec<EcoString>,
    usage: FieldAccessUsage,
    unknown_field: UnknownField,
  },

  IncorrectArity {
    location: SrcSpan,
    expected: usize,
    context: IncorrectArityContext,
    given: usize,
    labels: Vec<EcoString>,
  },

  UnsafeRecordUpdate {
    location: SrcSpan,
    reason: UnsafeRecordUpdateReason,
  },

  UnnecessarySpreadOperator {
    location: SrcSpan,
    arity: usize,
  },

  IncorrectTypeArity {
    location: SrcSpan,
    name: EcoString,
    expected: usize,
    given: usize,
  },

  CouldNotUnify {
    location: SrcSpan,
    situation: Option<UnifyErrorSituation>,
    expected: Arc<Type>,
    given: Arc<Type>,
  },

  RecursiveType {
    location: Span,
  },

  DuplicateName {
    location_a: SrcSpan,
    location_b: SrcSpan,
    name: EcoString,
  },

  DuplicateImport {
    location: Span,
    previous_location: SrcSpan,
    name: EcoString,
  },

    DuplicateTypeName {
        location: SrcSpan,
        previous_location: SrcSpan,
        name: EcoString,
    },

    DuplicateArgument {
        location: SrcSpan,
        label: EcoString,
    },

    DuplicateField {
        location: SrcSpan,
        label: EcoString,
    },

    PrivateTypeLeak {
        location: SrcSpan,
        leaked: Type,
    },

    UnexpectedLabelledArg {
        location: SrcSpan,
        label: EcoString,
    },

    PositionalArgumentAfterLabelled {
        location: SrcSpan,
    },

    IncorrectNumClausePatterns {
        location: SrcSpan,
        expected: usize,
        given: usize,
    },

    NonLocalClauseGuardVariable {
        location: SrcSpan,
        name: EcoString,
    },

    ExtraVarInAlternativePattern {
        location: SrcSpan,
        name: EcoString,
    },

    MissingVarInAlternativePattern {
        location: SrcSpan,
        name: EcoString,
    },

    DuplicateVarInPattern {
        location: SrcSpan,
        name: EcoString,
    },

    OutOfBoundsTupleIndex {
        location: SrcSpan,
        index: u64,
        size: usize,
    },

    NotATuple {
        location: SrcSpan,
        given: Arc<Type>,
    },

    NotATupleUnbound {
        location: SrcSpan,
    },

    RecordAccessUnknownType {
        location: SrcSpan,
    },

    RecordUpdateInvalidConstructor {
        location: SrcSpan,
    },

    UnexpectedTypeHole {
        location: SrcSpan,
    },

    ReservedModuleName {
        name: EcoString,
    },

    KeywordInModuleName {
        name: EcoString,
        keyword: EcoString,
    },

    NotExhaustivePatternMatch {
        location: SrcSpan,
        unmatched: Vec<EcoString>,
        kind: PatternMatchKind,
    },

    /// A function was defined with multiple arguments with the same name
    ///
    /// # Examples
    ///
    /// ```gleam
    /// fn main(x, x) { Nil }
    /// ```
    /// ```gleam
    /// fn main() {
    ///   fn(x, x) { Nil }
    /// }
    /// ```
    ArgumentNameAlreadyUsed {
        location: SrcSpan,
        name: EcoString,
    },

    /// A function was defined with an unlabelled argument after a labelled one.
    UnlabelledAfterlabelled {
        location: SrcSpan,
    },

    /// A type alias was defined directly or indirectly in terms of itself, which would
    /// cause it to expand to infinite size.
    /// e.g.
    ///     type ForkBomb = #(ForkBomb, ForkBomb)
    RecursiveTypeAlias {
        location: SrcSpan,
        cycle: Vec<EcoString>,
    },

    /// A function has been given an external implementation but not all the
    /// type annotations have been given. The annotations are required as we
    /// cannot infer the types of external implementations.
    ExternalMissingAnnotation {
        location: SrcSpan,
        kind: MissingAnnotation,
    },

    /// A function has been given without either a Gleam implementation or an
    /// external one.
    NoImplementation {
        location: SrcSpan,
    },

    /// A function/constant that is used doesn't have an implementation for the
    /// current compilation target.
    UnsupportedExpressionTarget {
        location: SrcSpan,
        target: Target,
    },

    /// A function's JavaScript implementation has been given but it does not
    /// have a valid module name.
    InvalidExternalJavascriptModule {
        location: SrcSpan,
        module: EcoString,
        name: EcoString,
    },

    /// A function's JavaScript implementation has been given but it does not
    /// have a valid function name.
    InvalidExternalJavascriptFunction {
        location: SrcSpan,
        function: EcoString,
        name: EcoString,
    },

    /// A case expression is missing one or more patterns to match all possible
    /// values of the type.
    InexhaustiveCaseExpression {
        location: SrcSpan,
        missing: Vec<EcoString>,
    },

    /// A case expression is missing its body.
    MissingCaseBody {
        location: SrcSpan,
    },

    /// Let assignment's pattern does not match all possible values of the type.
    InexhaustiveLetAssignment {
        location: SrcSpan,
        missing: Vec<EcoString>,
    },

    /// A type alias has a type variable but it is not used in the definition.
    ///
    /// For example, here `unused` is not used
    ///
    /// ```gleam
    /// pub type Wibble(unused) =
    ///   Int
    /// ```
    UnusedTypeAliasParameter {
        location: SrcSpan,
        name: EcoString,
    },

    /// A definition has two type parameters with the same name.
    ///
    /// ```gleam
    /// pub type Wibble(a, a) =
    ///   Int
    /// ```
    /// ```gleam
    /// pub type Wibble(a, a) {
    ///   Wibble
    /// }
    /// ```
    DuplicateTypeParameter {
        location: SrcSpan,
        name: EcoString,
    },

    /// A public function doesn't have an implementation for the current target.
    /// This is only raised when compiling a package with `TargetSupport::Enforced`, which is
    /// typically the root package, deps not being enforced.
    ///
    /// For example, if compiling to Erlang:
    ///
    /// ```gleam
    /// @external(javascript, "one", "two")
    /// pub fn wobble() -> Int
    /// ```
    UnsupportedPublicFunctionTarget {
        target: Target,
        name: EcoString,
        location: SrcSpan,
    },

    /// When there's something that is not a function to the left of the `<-`
    /// operator in a use expression:
    ///
    /// For example:
    ///
    /// ```gleam
    /// use <- "wibble"
    /// todo
    /// ```
    NotFnInUse {
        location: SrcSpan,
        type_: Arc<Type>,
    },

    /// When the function to the right hand side of `<-` in a `use` expression
    /// is called with the wrong number of arguments (given already takes into
    /// account the use callback passed to the function).
    ///
    UseFnIncorrectArity {
        location: SrcSpan,
        expected: usize,
        given: usize,
    },

    /// When on the left hand side of `<-` in a `use` expression there is the
    /// wrong number of patterns.
    ///
    /// For example:
    ///
    /// ```gleam
    /// use _, _ <- result.try(res)
    /// todo
    /// ```
    ///
    UseCallbackIncorrectArity {
        call_location: SrcSpan,
        pattern_location: SrcSpan,
        expected: usize,
        given: usize,
    },

    /// When on the right hand side of use there is a function that doesn't take
    /// a callback function as its last argument.
    ///
    /// For example:
    ///
    /// ```gleam
    /// use <- io.println
    /// ```
    ///
    UseFnDoesntTakeCallback {
        location: SrcSpan,
        actual_type: Option<Type>,
    },

    /// When the name assigned to a variable or function doesn't follow the gleam
    /// naming conventions.
    ///
    /// For example:
    ///
    /// ```gleam
    /// let myBadName = 42
    /// ```
    BadName {
        location: SrcSpan,
        kind: Named,
        name: EcoString,
    },

    /// Occurs when all the variant types of a custom type are deprecated
    ///
    /// ```gleam
    /// type Wibble {
    ///     @deprecated("1")
    ///     Wobble1
    ///     @deprecated("1")
    ///     Wobble1
    /// }
    /// ```
    AllVariantsDeprecated {
        location: SrcSpan,
    },

    /// Occers when any varient of a custom type is deprecated while
    /// the custom type itself is deprecated
    DeprecatedVariantOnDeprecatedType {
        location: SrcSpan,
    },

    ErlangFloatUnsafe {
        location: SrcSpan,
    },

    /// When the echo keyword is not followed by an expression to be printed.
    /// The only place where echo is allowed to appear on its own is as a step
    /// of a pipeline, otherwise omitting the expression will result in this
    /// error. For example:
    ///
    /// ```gleam
    /// call(echo, 1, 2)
    /// //   ^^^^ Error!
    /// ```
    ///
    EchoWithNoFollowingExpression {
        location: SrcSpan,
    },
    /// When someone tries concatenating two string values using the `+` operator.
    ///
    /// ```gleam
    /// "aaa" + "bbb"
    /// //    ^ We wont to suggest using `<>` instead!
    /// ```
    StringConcatenationWithAddInt {
        location: SrcSpan,
    },
    /// When someone tries using an int operator on two floats.
    ///
    /// ```gleam
    /// 1 +. 3
    /// //^ We wont to suggest using `+` instead!
    /// ```
    FloatOperatorOnInts {
        operator: BinOp,
        location: SrcSpan,
    },
    /// When someone tries using an int operator on two floats.
    ///
    /// ```gleam
    /// 1.2 + 1.0
    /// //  ^ We wont to suggest using `+.` instead!
    /// ```
    IntOperatorOnFloats {
        operator: BinOp,
        location: SrcSpan,
    },

    DoubleVariableAssignmentInBitArray {
        location: SrcSpan,
    },

    NonUtf8StringAssignmentInBitArray {
        location: SrcSpan,
    },

    /// This happens when a private type is marked as opaque. Only public types
    /// can be opaque.
    ///
    /// ```gleam
    /// opaque type Wibble {
    ///   Wobble
    /// }
    /// ```
    ///
    PrivateOpaqueType {
        location: SrcSpan,
    },

    SrcImportingDevDependency {
        importing_module: EcoString,
        imported_module: EcoString,
        package: EcoString,
        location: SrcSpan,
    },

    /// This happens when a type has no type parameters (for example `Int`) but
    /// it is being used as a constructor: `Int()`, `Bool(a, b)`.
    ///
    TypeUsedAsAConstructor {
        location: SrcSpan,
        name: EcoString,
    },

    /// The `@external` annotation on custom types can only be used for external
    /// types, types with no constructors.
    ///
    ExternalTypeWithConstructors {
        location: SrcSpan,
    },

    LowercaseBoolPattern {
        location: SrcSpan,
    },
    */
}
