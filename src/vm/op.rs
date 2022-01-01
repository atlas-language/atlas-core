use std::collections::HashMap;

pub use crate::op_capnp::op::{
    Which as OpWhich,
    Reader as OpReader,
    Builder as OpBuilder
};
pub use crate::op_capnp::primitive_op::{
    OpType as PrimOpWType,
    Reader as PrimOpReader,
    Builder as PrimOpBuilder
};
pub use crate::op_capnp::code::{
    Reader as CodeReader,
    Builder as CodeBuilder
};

pub type RegAddr = u32;
pub type CodeHash = u64;
pub type OpAddr = u32;
pub type TargetID = u32;
pub type SegmentID = usize;

// an op arg can directly be a literal
pub enum OpPrimitive<'s> {
    AddrTarget(OpAddr), ExternalTarget(TargetID),
    Unit, Bool(bool), Int(i64),
    Float(f64), Char(char),
    String(&'s str), Buffer(&'s [u8]),
    EmptyList, EmptyTuple, EmptyRecord
}

pub enum BuiltinOp {
    Negate { dest: RegAddr, src: RegAddr },
    Add { dest: RegAddr, left: RegAddr, right: RegAddr },
    Mul { dest: RegAddr, left: RegAddr, right: RegAddr },
    Mod { dest: RegAddr, left: RegAddr, right: RegAddr },
    Or  { dest: RegAddr, left: RegAddr, right: RegAddr },
    And { dest: RegAddr, left: RegAddr, right: RegAddr },

    // List methods
    Decons { head_dest: RegAddr, tail_dest: RegAddr, src: RegAddr },
    Cons { dest: RegAddr, head: RegAddr, tail: RegAddr },

    // Tuple methods
    Index { dest: RegAddr, src: RegAddr, index: RegAddr },
    Append { dest: RegAddr, tuple: RegAddr, item: RegAddr },

    // Variant methods
    Variant { dest: RegAddr, tag: RegAddr, value: RegAddr },
    Unwrap { dest: RegAddr, src: RegAddr }, // unwrap a variant

    // Record methods
    Insert { dest: RegAddr, record: RegAddr, key: RegAddr, value: RegAddr },
    Lookup { dest: RegAddr, src: RegAddr, key: RegAddr },
}

pub enum UnpackOp<'s> {
    Pos(RegAddr),
    Named(RegAddr, &'s str),
    Optional(RegAddr, &'s str),
    VarPos(RegAddr),
    VarKey(RegAddr),
}

pub enum ApplyOp<'s> {
    Pos { dest: RegAddr, tgt: RegAddr, arg: RegAddr },
    ByName { dest: RegAddr, tgt: RegAddr, arg: RegAddr, name: &'s str}, 
    VarPos { dest: RegAddr, tgt: RegAddr, arg: RegAddr },
    VarKey { dest: RegAddr, tgt: RegAddr, arg: RegAddr }
}

// A rusty op representation
// which can be serialized/deserialized
// to the capnproto format.
// Note that the Op has a lifetime, which
// it uses to reference strings in the serialized format
pub enum Op<'s> {
    BuiltinOp(BuiltinOp),
    Unpack(UnpackOp<'s>),
    Apply(ApplyOp<'s>),
    Store(RegAddr, OpPrimitive<'s>), // dest, src
    Invoke(RegAddr, RegAddr), // dest, src
    ScopeSet(RegAddr, RegAddr, RegAddr), // thunk/lambda dest, reg, src
    Force(RegAddr),

    // For case/if-else
    JmpTarget(RegAddr, TargetID),
    JmpAddr(RegAddr, OpAddr),

    Return(RegAddr)
}

// A temporary structure for interacting with a code segment
// core expressions are transpiled into segments, which are then converted
// into the Code values

pub struct Segment<'s> {
    ops: Vec<Op<'s>>,
    targets: Vec<SegmentID> // other segment targets
}

impl<'s> Segment<'s> {
    pub fn new() -> Self {
        Segment { ops: Vec::new(), targets: Vec::new() }
    }

    pub fn add_target(&mut self, seg: SegmentID) -> TargetID {
        let id = self.targets.len() as TargetID;
        self.targets.push(seg);
        id
    }

    pub fn append(&mut self, op: Op<'s>) {
        self.ops.push(op);
    }
}

pub struct Program<'s> {
    segments: HashMap<SegmentID, Segment<'s>>,
    // external: HashMap<SegmentID, Pointer>,
    next_id: SegmentID
}

impl<'s> Program<'s> {
    pub fn register_seg(&mut self, id: SegmentID, seg: Segment<'s>) {
        self.segments.insert(id, seg);
    }

    pub fn gen_id(&mut self) -> SegmentID {
        let id = self.next_id;
        self.next_id = self.next_id + 1;
        id
    }
}

// When loading a program for modification
// the load context keeps track of the pointer
// to segment ID map
// pub struct LoadContext {
//     reverse: HashMap<Pointer, SegmentID>
// }