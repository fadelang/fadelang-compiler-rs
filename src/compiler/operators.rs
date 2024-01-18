#[derive(Debug)]
pub(crate) enum Operator {
    /* Arithmetic */
    //    Addition,       // +
    Subtraction, // -
    //    Multiplication, // *
    Division, // /
    //    Modulo,         // %

    /* Scoping, Accessing & Statements */
    //    ScopeAccessor,  // ::
    //    MemberAccessor, // .
    StatementTerminator,

    /* Types & Generics */
    TypeSpecifier, // :
    ReturnTypeSpecifier, // ->

                   /* Comparison */
                   //    Equals,      // ==
                   //    NotEquals,   // !=
                   //    LessThan,    // <
                   //    GreaterThan, // >

                       /* Logic */
                   //    LogicalAnd, // &&
                   //    LogicalOr,  // ||
                   //    LogicalNot, // !

                       /* Bitwise */
                   //    BitwiseAnd,        // &
                   //    BitwiseXOr,        // ^
                   //    BitwiseOr,         // |
                   //    BitwiseNot,        // ~
                   //    BitwiseRightShift, // >>
                   //    BitwiseLeftShift,  // <<

                       /* Assignment */
                   //    MethodAssignment,         // =>
                   //    ValueAssignment,          // :=
                   //    AdditionAssignment,       // +=
                   //    SubtractionAssignment,    // -=
                   //    MultiplicationAssignment, // *=
                   //    DivisionAssignment,       // /=
                   //    ModuloAssignment,         // %=

                   //    BitwiseRightShiftAssignment, // >>=
                   //    BitwiseLeftShiftAssignment,  // <<=
                   //    BitwiseAndAssignment,        // &=
                   //    BitwiseXOrAssignment,        // ^=
                   //    BitwiseOrAssignment,         // |=
}
