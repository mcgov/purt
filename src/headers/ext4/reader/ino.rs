/*
if inode.get_ext_attrs_addr() != 0 {
    let extoffset = get_offset_from_block_number(
        ext4_block_0,
        inode.get_ext_attrs_addr() as u64,
        block_size,
    );
    type HdrType = ext4::extattrs::ExtendedAttrBlock;
    let extadd = read_header_from_offset::<HdrType>(&file_arg, extoffset);
    println!("EXTATTR: {:#X?}", extadd);
    println!("size of header: 0x{:x?}", size_of::<HdrType>());
    let size_of_hdr = size_of::<HdrType>() as u64;
    let mut entry_offset = 0;
    loop {
        let extblockbytes = read_bytes_from_file(
            &file_arg,
            extoffset + entry_offset + size_of_hdr,
            0xff + ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME,
        );
        //println!("{:X?}", extblockbytes);
        let extblock = ext4::extattrs::get_extended_attr_entry(&extblockbytes);
        if extblock.is_empty() {
            println!(
                "{}",
                "Next extended attr entry was empty.".cyan().to_string()
            );
            break;
        }
        println!("{:#X?}", extblock);
        entry_offset +=
            ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME + extblock.name_len as u64;
    }
}
pub struct Ino {
    pub inode: Inode,
    pub attr: Option<Exatt>,
    pub extents: Vec<Extent>,
    // can also have a hash tree
}

// print the timestamp is not zero while we're debugging
if inode.inode_uses_extents() {
    let extent = inode.get_extent();
    println!("Extent: {:#X?}", extent);
    let read_block = extent.leaf.get_block();
    let offset = get_offset_from_block_number(ext4_block_0, read_block, block_size);
    let mut table_offset = 0;
    if j + 1 != superblock.journal_inum
        && bitfield_fetch::<u16>(
            inode.mode,
            inode::filemode_bitflags::mutex::S_IFREG,
        ) {
        let bytes = read_bytes_from_file(&file_arg, offset, inode.get_file_size());
        println!("Found file content... ");
        println!(
            "{}",
            String::from_utf8(bytes).unwrap().bright_green().to_string()
        );
    } else if bitfield_fetch::<u16>(
        inode.mode,
        inode::filemode_bitflags::mutex::S_IFDIR,
    ) {
        if inode.uses_hash_tree_directories() {
            println!(
                "{}",
                "Hash tree directories not implemented. Probably going to miss reading some directories here 😢".red().to_string()
            );
        }
        loop {
            let bytes = read_bytes_from_file(&file_arg, offset + table_offset, 263);
            let dirent = get_dir_ent(&bytes[..]);
            println!("dirent: {:x?}", dirent);
            println!("file_type: {}", dirent.filetype_to_str());
            // this logic isn't right yet
            if dirent.inode == 0
                || dirent.rec_len as u64 + table_offset == block_size
                || table_offset == block_size || dirent.filetype
                == dirent::file_type::FAKE_TAIL_ENTRY_CHECKSUM
            {
                break;
            }
            table_offset += dirent.record_size() as u64;
            //honestly most of this logic *waves* isn't right
        }
    }
}

*/
use crate::headers::ext4;
use crate::headers::ext4::dirent;
use crate::headers::ext4::reader::Exatt;
use crate::headers::ext4::reader::Ino;
use crate::headers::ext4::superblock::Superblock;
use crate::headers::reader::*;
use colored::*;
use std::mem::size_of;
impl Ino {
    pub fn populate_ext_attrs(&mut self, reader: &mut OnDisk, s: &Superblock, block0: u64) {
        if self.inode.get_ext_attrs_addr() != 0 {
            let extoffset = get_offset_from_block_number(
                block0,
                self.inode.get_ext_attrs_addr() as u64,
                s.block_size_bytes(),
            );
            type HdrType = ext4::extattrs::ExtendedAttrBlock;
            let extadd = reader.read_header_from_offset::<HdrType>(extoffset);
            //println!("EXTATTR: {:#X?}", extadd);
            //println!("size of header: 0x{:x?}", size_of::<HdrType>());
            let size_of_hdr = size_of::<HdrType>() as u64;
            let mut entry_offset = 0;
            let mut exat = Exatt {
                blk: extadd,
                attrs: vec![],
            };
            loop {
                let extblockbytes = reader.read_bytes_from_file(
                    extoffset + entry_offset + size_of_hdr,
                    0xff + ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME,
                );
                //println!("{:X?}", extblockbytes);
                let extblock = ext4::extattrs::get_extended_attr_entry(&extblockbytes);
                if !extblock.is_empty() {
                    //println!("{:#X?}", extblock);
                    entry_offset +=
                        ext4::extattrs::EXTATTR_ENTRY_SIZE_WO_NAME + extblock.name_len as u64;
                    exat.attrs.push(extblock);
                } else {
                    break;
                }
            }
            self.attr = Some(exat);
        }
    }

    pub fn populate_extents(&mut self, reader: &mut OnDisk, s: &Superblock, block0: u64) {
        let inode = self.inode;
        //inode.print_fields();

        if !inode.inode_uses_extents() {
            return;
        }
        let mut extent = inode.get_extent();
        extent.ascend(reader, block0, s.block_size_bytes());
        self.extent = Some(extent);
    }

    pub fn get_file_content(&self, reader: &mut OnDisk, s: &Superblock, block0: u64) -> Vec<u8> {
        if !self.inode.inode_uses_extents() {
            return vec![];
            //panic!("This inode doesn't use extents, this method shouldn't have been called")
        }
        let mut tree = self.extent.clone().unwrap();
        tree.walk(
            reader,
            block0,
            s.block_size_bytes(),
            self.inode.get_file_size() as usize,
        )
    }
    /*
    let read_block = extent.leaf.get_block();
        let block_size = s.block_size_bytes();
        let offset = get_offset_from_block_number(block0, read_block, block_size);
        let mut table_offset = 0;

        // skip dealing with the journal for now
        if self.id != s.journal_inum && inode.regular_file() {
            let bytes = reader.read_bytes_from_file(offset, inode.get_file_size());
            println!("Found file content... ");
            println!(
                "{}",
                String::from_utf8(bytes).unwrap().to_string().bright_green()
            );
        } else if inode.directory() {
            if inode.uses_hash_tree_directories() {
                println!(
                    "{}",
                    "Hash tree directories not implemented. Probably going to miss reading some directories here 😢"
                );
            }
            loop {
                let bytes = reader.read_bytes_from_file(offset + table_offset, 263);
                let dirent = dirent::get_dir_ent(&bytes[..]);
                println!("dirent: {:x?}", dirent);
                println!("file_type: {}", dirent.filetype_to_str());
                // this logic isn't right yet
                if dirent.inode == 0
                    || dirent.rec_len as u64 + table_offset == block_size
                    || table_offset == block_size
                    || dirent.filetype == dirent::file_type::FAKE_TAIL_ENTRY_CHECKSUM
                {
                    break;
                }
                table_offset += dirent.record_size() as u64;
                //honestly most of this logic *waves* isn't right
            }
        }
    */
}