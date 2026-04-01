#![allow(dead_code)]

use crate::lexer::token::Span;

// ──────────────────────────────────────────────
// Core types
// ──────────────────────────────────────────────

/// A declared name, referencing a span in the source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub span: Span,
    pub value: String,
}

/// A qualified name: `A::B::C`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedName {
    pub span: Span,
    pub segments: Vec<Name>,
}

/// Visibility of a membership or import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Direction of a feature (parameter, port, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureDirection {
    In,
    Out,
    Inout,
}
tr
/// Portion kind for occurrence usages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortionKind {
    Snapshot,
    Timeslice,
}

/// Kind of constraint within a requirement body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementConstraintKind {
    Assumption,
    Requirement,
}

/// Kind of subaction within a state body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSubactionKind {
    Entry,
    Do,
    Exit,
}

/// Kind of feature within a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionFeatureKind {
    Trigger,
    Guard,
    Effect,
}

/// Kind of trigger invocation expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKind {
    At,
    After,
    When,
}

// ──────────────────────────────────────────────
// Multiplicity
// ──────────────────────────────────────────────

/// Multiplicity bounds: `[1..5]`, `[*]`, `[0..*]`, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Multiplicity {
    pub span: Span,
    pub lower: Option<Box<ExprNode>>,
    pub upper: Option<Box<ExprNode>>,
}

// ──────────────────────────────────────────────
// Top-level AST Node
// ──────────────────────────────────────────────

/// The root AST node. Every construct in a SysML v2 / KerML model is an Element.
#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    pub span: Span,
    pub kind: NodeKind,
    pub name: Option<Name>,
    pub short_name: Option<Name>,
    pub visibility: Option<Visibility>,
    pub annotations: Vec<AnnotationNode>,
}

/// Top-level dispatch enum for all node kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    // ── Namespace / Package ──
    Namespace(NamespaceNode),
    Package(PackageNode),
    LibraryPackage(PackageNode),

    // ── KerML Classifiers ──
    Classifier(ClassifierNode),

    // ── SysML Definitions ──
    Definition(DefinitionNode),

    // ── SysML Usages ──
    Usage(UsageNode),

    // ── Action Nodes ──
    ActionNode(ActionNodeNode),

    // ── Connectors as Usages ──
    ConnectorAsUsage(ConnectorAsUsageNode),

    // ── Relationships ──
    Relationship(RelationshipNode),

    // ── Memberships ──
    Membership(MembershipNode),

    // ── Imports ──
    Import(ImportNode),

    // ── Expressions ──
    Expr(ExprNode),

    // ── Literals ──
    Literal(LiteralNode),

    // ── Annotations ──
    Annotation(AnnotationNode),

    // ── Features (KerML level) ──
    Feature(FeatureNode),

    // ── Multiplicity ──
    Multiplicity(Multiplicity),

    // ── Alias ──
    Alias(AliasNode),

    /// Placeholder for error recovery — a node that failed to parse.
    Error,
}

// ──────────────────────────────────────────────
// Namespace / Package
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceNode {
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PackageNode {
    pub is_standard: bool,
    pub members: Vec<ASTNode>,
    pub imports: Vec<ImportNode>,
    pub filters: Vec<ExprNode>,
}

// ──────────────────────────────────────────────
// KerML Classifiers
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ClassifierNode {
    pub kind: ClassifierKind,
    pub is_abstract: bool,
    pub specializations: Vec<SpecializationItem>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassifierKind {
    Type,
    Classifier,
    Class,
    DataType,
    Structure,
    Association,
    AssociationStructure,
    Metaclass,
    Behavior,
    Function,
    Predicate,
    Interaction,
}

// ──────────────────────────────────────────────
// SysML Definitions
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionNode {
    pub kind: DefinitionKind,
    pub is_abstract: bool,
    pub is_variation: bool,
    pub specializations: Vec<SpecializationItem>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionKind {
    /// `attribute def`
    Attribute,
    /// `enum def`
    Enumeration,
    /// `occurrence def`
    Occurrence,
    /// `item def`
    Item,
    /// `part def`
    Part,
    /// `port def`
    Port,
    /// Implied conjugated port definition
    ConjugatedPort,
    /// `connection def`
    Connection,
    /// `interface def`
    Interface,
    /// `allocation def`
    Allocation,
    /// `flow def`
    Flow,
    /// `action def`
    Action,
    /// `state def`
    State,
    /// `calc def`
    Calculation,
    /// `constraint def`
    Constraint,
    /// `requirement def`
    Requirement,
    /// `concern def`
    Concern,
    /// `viewpoint def`
    Viewpoint,
    /// `case def`
    Case,
    /// `analysis def`
    AnalysisCase,
    /// `verification def`
    VerificationCase,
    /// `use case def`
    UseCase,
    /// `view def`
    View,
    /// `rendering def`
    Rendering,
    /// `metadata def`
    Metadata,
}

// ──────────────────────────────────────────────
// SysML Usages
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct UsageNode {
    pub kind: UsageKind,
    pub is_ref: bool,
    pub is_variation: bool,
    pub is_individual: bool,
    pub is_derived: bool,
    pub is_readonly: bool,
    pub is_end: bool,
    pub direction: Option<FeatureDirection>,
    pub portion_kind: Option<PortionKind>,
    pub specializations: Vec<SpecializationItem>,
    pub multiplicity: Option<Multiplicity>,
    pub value: Option<FeatureValue>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageKind {
    /// `attribute`
    Attribute,
    /// `enum`
    Enumeration,
    /// `occurrence`
    Occurrence,
    /// `item`
    Item,
    /// `part`
    Part,
    /// `port`
    Port,
    /// `ref`
    Reference,
    /// `connection` / `connect`
    Connection,
    /// `interface`
    Interface,
    /// `allocation` / `allocate`
    Allocation,
    /// `flow`
    Flow,
    /// `succession flow`
    SuccessionFlow,
    /// `action`
    Action,
    /// `state`
    State,
    /// `transition`
    Transition,
    /// `calc`
    Calculation,
    /// `constraint`
    Constraint,
    /// `requirement`
    Requirement,
    /// `concern`
    Concern,
    /// `viewpoint`
    Viewpoint,
    /// `case`
    Case,
    /// `analysis`
    AnalysisCase,
    /// `verification`
    VerificationCase,
    /// `use case`
    UseCase,
    /// `view`
    View,
    /// `rendering`
    Rendering,
    /// `metadata` / `@`
    Metadata,
    /// `event occurrence` / `event`
    EventOccurrence,
}

// ──────────────────────────────────────────────
// Action Nodes
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ActionNodeNode {
    pub kind: ActionNodeKind,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionNodeKind {
    /// `if ... then ... else ...`
    IfAction,
    /// `while` / `loop`
    WhileLoop,
    /// `for ... in ...`
    ForLoop,
    /// `fork`
    Fork,
    /// `join`
    Join,
    /// `merge`
    Merge,
    /// `decide`
    Decision,
    /// `accept`
    Accept,
    /// `send ... via/to ...`
    Send,
    /// `assign ... := ...`
    Assign,
    /// `terminate`
    Terminate,
    /// `perform`
    Perform,
    /// `exhibit`
    ExhibitState,
    /// `include`
    IncludeUseCase,
    /// `assert`
    AssertConstraint,
    /// `satisfy`
    SatisfyRequirement,
}

// ──────────────────────────────────────────────
// Connector-as-Usage
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectorAsUsageNode {
    pub kind: ConnectorAsUsageKind,
    pub ends: Vec<ASTNode>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorAsUsageKind {
    /// `bind x = y`
    Binding,
    /// `first x then y`
    Succession,
}

// ──────────────────────────────────────────────
// Features (KerML)
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureNode {
    pub kind: FeatureKind,
    pub is_composite: bool,
    pub is_portion: bool,
    pub is_readonly: bool,
    pub is_derived: bool,
    pub is_end: bool,
    pub direction: Option<FeatureDirection>,
    pub specializations: Vec<SpecializationItem>,
    pub multiplicity: Option<Multiplicity>,
    pub value: Option<FeatureValue>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureKind {
    /// `feature`
    Feature,
    /// `step`
    Step,
    /// `expr`
    Expression,
    /// `bool`
    BooleanExpression,
    /// `inv`
    Invariant,
    /// `connector`
    Connector,
    /// `binding`
    BindingConnector,
    /// `succession`
    Succession,
    /// `flow`
    Flow,
    /// `succession flow` (KerML level)
    SuccessionFlow,
}

// ──────────────────────────────────────────────
// Feature Value
// ──────────────────────────────────────────────

/// A feature value: `= expr`, `:= expr`, or `default = expr`, `default := expr`.
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureValue {
    pub span: Span,
    pub kind: FeatureValueKind,
    pub value: Box<ExprNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureValueKind {
    /// `= expr` — bound value
    Bind,
    /// `:= expr` — initial value (assignment)
    Initial,
    /// `default = expr` — default bound
    DefaultBind,
    /// `default := expr` — default initial
    DefaultInitial,
}

// ──────────────────────────────────────────────
// Specialization / Typing relationships
// ──────────────────────────────────────────────

/// A specialization, typing, subsetting, or redefinition appearing in a declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecializationItem {
    pub span: Span,
    pub kind: SpecializationKind,
    pub target: QualifiedName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializationKind {
    /// `:` or `typed by`
    Typing,
    /// `:` or `defined by`
    DefinedBy,
    /// `:>` or `specializes`
    Specializes,
    /// `:>` or `subsets`
    Subsets,
    /// `::>` or `references`
    References,
    /// `=>` or `crosses`
    Crosses,
    /// `:>>` or `redefines`
    Redefines,
    /// `~` or `conjugates`
    Conjugates,
}

// ──────────────────────────────────────────────
// Relationships (standalone)
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct RelationshipNode {
    pub span: Span,
    pub kind: RelationshipKind,
    pub source: Option<QualifiedName>,
    pub target: Option<QualifiedName>,
    pub members: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipKind {
    /// General specialization relationship (KerML)
    Specialization,
    /// Classifier-to-classifier specialization
    Subclassification,
    /// Feature typed by a type
    FeatureTyping,
    /// Typing via conjugated port definition
    ConjugatedPortTyping,
    /// Feature subsetting
    Subsetting,
    /// Non-owning subsetting
    ReferenceSubsetting,
    /// Cross-feature subsetting
    CrossSubsetting,
    /// Feature redefinition
    Redefinition,
    /// Type conjugation
    Conjugation,
    /// Port-specific conjugation
    PortConjugation,
    /// Types declared disjoint
    Disjoining,
    /// Feature direction inversion
    FeatureInverting,
    /// Feature featured by a type
    TypeFeaturing,
    /// Feature chain link
    FeatureChaining,
    /// Type difference
    Differencing,
    /// Type intersection
    Intersecting,
    /// Type union
    Unioning,
    /// `dependency`
    Dependency,
}

// ──────────────────────────────────────────────
// Memberships
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct MembershipNode {
    pub span: Span,
    pub kind: MembershipKind,
    pub visibility: Option<Visibility>,
    pub member: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipKind {
    /// Basic membership (names an element in a namespace)
    Membership,
    /// Owning membership
    Owning,
    /// Feature membership (feature owned by a type)
    Feature,
    /// End feature of connector/association
    EndFeature,
    /// Feature value
    Value,
    /// Parameter of behavior/step
    Parameter,
    /// Return parameter
    ReturnParameter,
    /// Result expression of function body
    ResultExpression,
    /// Variant in a variation
    Variant,
    /// Filter expression in package
    ElementFilter,
    /// `objective` in case body
    Objective,
    /// `subject` in requirement/case
    Subject,
    /// `actor` in requirement/case
    Actor,
    /// `stakeholder` in requirement
    Stakeholder,
    /// `assume` / `require` in requirement body
    RequirementConstraint(RequirementConstraintKind),
    /// `frame` in requirement
    FramedConcern,
    /// `verify` in requirement
    RequirementVerification,
    /// `entry` / `do` / `exit` in state
    StateSubaction(StateSubactionKind),
    /// `accept` / `if` / `do` in transition
    TransitionFeature(TransitionFeatureKind),
    /// `render` in view
    ViewRendering,
}

// ──────────────────────────────────────────────
// Imports
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ImportNode {
    pub span: Span,
    pub kind: ImportKind,
    pub visibility: Option<Visibility>,
    pub target: QualifiedName,
    pub is_all: bool,
    pub is_recursive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    /// `import X;` — import a single membership
    Membership,
    /// `import X::*;` — import all from namespace
    Namespace,
    /// `expose X;` — expose a single membership (in views)
    MembershipExpose,
    /// `expose X::*;` — expose all from namespace (in views)
    NamespaceExpose,
}

// ──────────────────────────────────────────────
// Expressions
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ExprNode {
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Binary or unary operator expression: `a + b`, `!a`, `a and b`, etc.
    Operator {
        operator: String,
        operands: Vec<ExprNode>,
    },
    /// Function/behavior invocation: `f(a, b)`
    Invocation {
        target: Box<ExprNode>,
        arguments: Vec<ExprNode>,
    },
    /// `new T(...)` constructor expression
    Constructor {
        target: QualifiedName,
        arguments: Vec<ExprNode>,
    },
    /// Reference to a feature by name
    FeatureReference(QualifiedName),
    /// Feature chain expression: `a.b`
    FeatureChain {
        source: Box<ExprNode>,
        member: Box<ExprNode>,
    },
    /// Index expression: `a#(i)`
    Index {
        source: Box<ExprNode>,
        index: Box<ExprNode>,
    },
    /// Collect expression: `a.{...}`
    Collect {
        source: Box<ExprNode>,
        body: Box<ExprNode>,
    },
    /// Select expression: `a.?{...}`
    Select {
        source: Box<ExprNode>,
        body: Box<ExprNode>,
    },
    /// Metadata access: `x.metadata`
    MetadataAccess {
        source: Box<ExprNode>,
    },
    /// `null`
    Null,
    /// Conditional: `test ? if_true : if_false`
    Conditional {
        condition: Box<ExprNode>,
        if_true: Box<ExprNode>,
        if_false: Box<ExprNode>,
    },
    /// Null coalescing: `a ?? b`
    NullCoalescing {
        lhs: Box<ExprNode>,
        rhs: Box<ExprNode>,
    },
    /// Range expression: `lower..upper`
    Range {
        lower: Box<ExprNode>,
        upper: Box<ExprNode>,
    },
    /// Trigger invocation: `at`, `after`, or `when` expression
    TriggerInvocation {
        trigger_kind: TriggerKind,
        argument: Box<ExprNode>,
    },
    /// Cast expression: `expr as Type`
    Cast {
        operand: Box<ExprNode>,
        target: QualifiedName,
    },
    /// Type test: `expr istype Type`, `expr hastype Type`
    TypeTest {
        operator: TypeTestOp,
        operand: Box<ExprNode>,
        target: QualifiedName,
    },
    /// `expr implies expr`
    Implies {
        lhs: Box<ExprNode>,
        rhs: Box<ExprNode>,
    },
    /// A literal value
    Literal(LiteralNode),
}

/// Type test operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeTestOp {
    /// `istype`
    IsType,
    /// `hastype`
    HasType,
    /// `instanceof` (KerML)
    InstanceOf,
}

// ──────────────────────────────────────────────
// Literals
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNode {
    pub span: Span,
    pub kind: LiteralKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    /// `true` / `false`
    Boolean(bool),
    /// `"text"`
    String(String),
    /// `42`
    Integer(i64),
    /// `3.14` (called LiteralRational in the metamodel)
    Rational(f64),
    /// `*` (unbounded multiplicity)
    Infinity,
}

// ──────────────────────────────────────────────
// Annotations
// ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationNode {
    pub span: Span,
    pub kind: AnnotationKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationKind {
    /// `comment about X /* ... */`
    Comment {
        about: Vec<QualifiedName>,
        locale: Option<String>,
        body: String,
    },
    /// `doc /* ... */`
    Documentation {
        locale: Option<String>,
        body: String,
    },
    /// `rep language "lang" /* ... */`
    TextualRepresentation {
        language: String,
        body: String,
    },
    /// `@Metadata { ... }` or `#Metadata { ... }`
    MetadataUsage {
        target: QualifiedName,
        members: Vec<ASTNode>,
    },
    /// Annotation relationship linking an annotating element to its target.
    Annotation {
        target: QualifiedName,
    },
}

// ──────────────────────────────────────────────
// Alias
// ──────────────────────────────────────────────

/// `alias <name> for <target>;`
#[derive(Debug, Clone, PartialEq)]
pub struct AliasNode {
    pub span: Span,
    pub name: Name,
    pub target: QualifiedName,
}
