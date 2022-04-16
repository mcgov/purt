use super::block_group::*;
use super::extattrs::*;
use super::extent::*;
use super::inode::*;
use super::superblock::Superblock;
use super::*;
use crate::headers::reader::read_bytes_from_file;
use colored::*;

pub struct Part {
    pub file: String,
    pub start: u64,
    pub s: Superblock,
    pub bg: Vec<Bg>,
}
pub mod part;

pub struct Bg {
    pub start: u64,
    pub b32: Option<BlockGroupDescriptor32>,
    pub b64: Option<BlockGroupDescriptor64>,
    pub ino: Vec<Ino>,
}
pub mod bg;

pub struct Ino {
    pub id: u32,
    pub inode: Inode,
    pub attr: Option<Exatt>,
    pub extents: Vec<Extent>,
    // can also have a hash tree
}
pub mod ino;

pub struct Exatt {
    blk: ExtendedAttrBlock,
    attrs: Vec<ExtendedAttrEntry>,
}

pub struct Extent {
    pub hdr: ExtentHeader,
    pub branches: Vec<ExtentNode>,
    pub leafs: Vec<ExtentLeaf>,
    pub tail: Option<ExtentTail>,
}
