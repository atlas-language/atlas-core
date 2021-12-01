@0x90d5fb2b0b8b5f13;

using import "value.capnp".Primitive;

enum Type {
    unit @0;
    bool @1;
    int @2;
    float @3;
    string @4;
    buffer @5;
    # non-primitive types
    list @6;
    tuple @7;
    record @8;
}

struct Symbol {
    name @0 :Text;
    disam @1 :UInt32;
}

struct Case {
    expr @0 :Expr;
    union {
        default @1 :Void;
        tag @2 :Text; # for enum cases
        listCons @3 :Void; 
        listEmpty @4 :Void;
        tuple @5 :UInt64;
        record :group {
            keys @6 :List(Text);
            exact @7 :Bool;
        }
        eq @8 :Primitive;
        of @9 :Type; # TODO: Revisit
    }
}

struct Expr {
    struct Arg {
        value @0 :Expr;
        union {
            pos @1 :Void;
            byName @2 :Text;
            varPos @3 :Void;
            varKeys @4 :Void;
        }
    }

    struct Param {
        symbol @0 :Symbol;
        union {
            # Not not generated by ast, but for builtins
            pos @1 :Void;
            named @2 :Text; # fn foo(a) will be named argument "a" with symbol a
            optional @3 :Text; # fn(a?) will be optional argument "a"
            varPos @4 :Void; # fn(*a) will be varpos argument /w symbol "a"
            varKeys @5 :Void; # fn(**a) will be varkeys argument a
        }
    }

    struct Binds {
        struct Bind {
            symbol @0 :Symbol;
            value @1 :Expr;
        }
        binds @0 :List(Bind);
        # non-recursive binds 
        # later symbols depend on earlier symbols
        # recursive binds they can depend
        # in any order
        rec @1 :Bool;
    }

    union {
        id @0 :Symbol;
        literal @1 :Primitive;
        app :group {
            lam @2 :Expr;
            args @3 :List(Arg);
        }
        call :group {
            lam @4 :Expr;
            args @5 :List(Arg);
        }
        match :group {
            expr @6 :Expr;
            bindTo @7 :Symbol;
            cases @8 :List(Case);
        }
        lam :group {
            params @9 :List(Param);
            body @10 :Expr;
        }
        let :group {
            binds @11 :Binds;
            body @12 :Expr;
        }
    }
}