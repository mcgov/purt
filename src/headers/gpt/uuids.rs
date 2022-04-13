use crate::headers::fs::disk::PartitionType;
use compiled_uuid::uuid;
use lazy_static::lazy_static;
use std::collections::HashMap;
use uuid::Uuid;

lazy_static! {
    pub static ref GUID_TYPE_ENUM_MAP: HashMap<Uuid, PartitionType> = HashMap::from([
        (
            uuid!("00000000-0000-0000-0000-000000000000"),
            PartitionType::Unused
        ),
        (
            uuid!("C12A7328-F81F-11D2-BA4B-00A0C93EC93B"),
            PartitionType::EfiSystem,
        ),
        (
            uuid!("21686148-6449-6E6F-744E-656564454649"),
            PartitionType::BiosBoot,
        ),
        (
            uuid!("0FC63DAF-8483-4772-8E79-3D69D8477DE4"),
            PartitionType::LinuxFsTBD,
        )
    ]);
}
lazy_static! {
    pub static ref GUID_TYPE_MAP: HashMap<Uuid, &'static str> = HashMap::from([
        (
            uuid!("00000000-0000-0000-0000-000000000000"),
            "Unused entry",
        ),
        (
            uuid!("024DEE41-33E7-11D3-9D69-0008C781F39F"),
            "MBR partition scheme",
        ),
        (
            uuid!("C12A7328-F81F-11D2-BA4B-00A0C93EC93B"),
            "EFI System partition",
        ),
        (
            uuid!("21686148-6449-6E6F-744E-656564454649"),
            "BIOS boot partition",
        ),
        (
            uuid!("D3BFE2DE-3DAF-11DF-BA40-E3A556D89593"),
            "Intel Fast Flash iFFS partition for Intel Rapid Start technology",
        ),
        (
            uuid!("F4019732-066E-4E12-8273-346C5641494F"),
            "Sony boot partition",
        ),
        (
            uuid!("BFBFAFE7-A34F-448A-9A5B-6213EB736C22"),
            "Lenovo boot partition",
        ),
        (
            uuid!("E3C9E316-0B5C-4DB8-817D-F92DF00215AE"),
            "Microsoft Reserved Partition MSR",
        ),
        (
            uuid!("EBD0A0A2-B9E5-4433-87C0-68B6B72699C7"),
            "Basic data partition",
        ),
        (
            uuid!("5808C8AA-7E8F-42E0-85D2-E1E90434CFB3"),
            "Logical Disk Manager LDM metadata partition",
        ),
        (
            uuid!("AF9B60A0-1431-4F62-BC68-3311714A69AD"),
            "Logical Disk Manager data partition",
        ),
        (
            uuid!("DE94BBA4-06D1-4D40-A16A-BFD50179D6AC"),
            "Windows Recovery Environment",
        ),
        (
            uuid!("37AFFC90-EF7D-4E96-91C3-2D7AE055B174"),
            "IBM General Parallel File System GPFS partition",
        ),
        (
            uuid!("E75CAF8F-F680-4CEE-AFA3-B001E56EFC2D"),
            "Storage Spaces partition",
        ),
        (
            uuid!("558D43C5-A1AC-43C0-AAC8-D1472B2923D1"),
            "Storage Replica partition",
        ),
        (
            uuid!("75894C1E-3AEB-11D3-B7C1-7B03A0000000"),
            "HP-UX 	Data partition",
        ),
        (
            uuid!("E2A1E728-32E3-11D6-A682-7B03A0000000"),
            "Service partition",
        ),
        (
            uuid!("0FC63DAF-8483-4772-8E79-3D69D8477DE4"),
            "Linux filesystem data",
        ),
        (
            uuid!("A19D880F-05FC-4D3B-A006-743F0F84911E"),
            "RAID partition",
        ),
        (
            uuid!("44479540-F297-41B2-9AF7-D131D5F0458A"),
            "Root partition x86",
        ),
        (
            uuid!("4F68BCE3-E8CD-4DB1-96E7-FBCAF984B709"),
            "Root partition x86-64",
        ),
        (
            uuid!("69DAD710-2CE4-4E3C-B16C-21A1D49ABED3"),
            "Root partition 32-bit ARM",
        ),
        (
            uuid!("B921B045-1DF0-41C3-AF44-4C6F280D3FAE"),
            "Root partition 64-bit AArch64",
        ),
        (
            uuid!("BC13C2FF-59E6-4262-A352-B275FD6F7172"),
            "/boot partition",
        ),
        (
            uuid!("0657FD6D-A4AB-43C4-84E5-0933C84B4F4F"),
            "Swap partition",
        ),
        (
            uuid!("E6D6D379-F507-44C2-A23C-238F2A3DF928"),
            "Logical Volume Manager LVM partition",
        ),
        (
            uuid!("933AC7E1-2EB4-4F13-B844-0E14E2AEF915"),
            "/home partition",
        ),
        (
            uuid!("3B8F8425-20E0-4F3B-907F-1A25A76F98E8"),
            "/srv server data partition",
        ),
        (
            uuid!("7FFEC5C9-2D00-49B7-8941-3EA10A5586B7"),
            "Plain dm-crypt partition",
        ),
        (uuid!("CA7D7CCB-63ED-4C53-861C-1742536059CC"),"LUKS partition",),
        (uuid!("8DA63339-0007-60C0-C436-083AC8230908"), "Reserved"),
        (uuid!("83BD6B9D-7F41-11DC-BE0B-001560B84F0F"), "Boot partition"),
        (uuid!("516E7CB4-6ECF-11D6-8FF8-00022D09712B"), "BSD disklabel partition"),
        (uuid!("516E7CB5-6ECF-11D6-8FF8-00022D09712B"), "Swap partition"),
        (uuid!("516E7CB6-6ECF-11D6-8FF8-00022D09712B"), "Unix File System (UFS) partition"),
        (uuid!("516E7CB8-6ECF-11D6-8FF8-00022D09712B"), "Vinum volume manager partition"),
        (uuid!("516E7CBA-6ECF-11D6-8FF8-00022D09712B"), "ZFS partition"),
        (uuid!("74BA7DD9-A689-11E1-BD04-00E081286ACF"), "nandfs partition"),
        //macOS
        (uuid!("48465300-0000-11AA-AA11-00306543ECAC"), "Darwin Hierarchical File System Plus (HFS+) partition"),
        //Apple APFS container
        (uuid!("7C3457EF-0000-11AA-AA11-00306543ECAC"), "APFS FileVault volume container"),
        (uuid!("55465300-0000-11AA-AA11-00306543ECAC"), "Apple UFS container"),
        (uuid!("6A898CC3-1DD2-11B2-99A6-080020736631"), "ZFS[j]"),
        (uuid!("52414944-0000-11AA-AA11-00306543ECAC"), "Apple RAID partition"),
        (uuid!("52414944-5F4F-11AA-AA11-00306543ECAC"), "Apple RAID partition, offline"),
        (uuid!("426F6F74-0000-11AA-AA11-00306543ECAC"), "Apple Boot partition (Recovery HD)"),
        (uuid!("4C616265-6C00-11AA-AA11-00306543ECAC"), "Apple Label"),
        (uuid!("5265636F-7665-11AA-AA11-00306543ECAC"), "Apple TV Recovery partition"),
        //Apple Core Storage Container
        (uuid!("53746F72-6167-11AA-AA11-00306543ECAC"), "HFS+ FileVault volume container"),
        (uuid!("69646961-6700-11AA-AA11-00306543ECAC"), "Apple APFS Preboot partition"),
        (uuid!("52637672-7900-11AA-AA11-00306543ECAC"), "Apple APFS Recovery partition"),
        //Solaris
        (uuid!("6A82CB45-1DD2-11B2-99A6-080020736631"), "illumos 	Boot partition"),
        (uuid!("6A85CF4D-1DD2-11B2-99A6-080020736631"), "(Solaris) Root partition"),
        (uuid!("6A87C46F-1DD2-11B2-99A6-080020736631"), "(Solaris) Swap partition"),
        (uuid!("6A8B642B-1DD2-11B2-99A6-080020736631"), "(Solaris) Backup partition"),
        (uuid!("6A898CC3-1DD2-11B2-99A6-080020736631"), "(Solaris) /usr partition"),
        (uuid!("6A8EF2E9-1DD2-11B2-99A6-080020736631"), "(Solaris) /var partition"),
        (uuid!("6A90BA39-1DD2-11B2-99A6-080020736631"), "(Solaris) /home partition"),
        (uuid!("6A9283A5-1DD2-11B2-99A6-080020736631"), "Alternate sector"),
        (uuid!("6A945A3B-1DD2-11B2-99A6-080020736631"), "Reserved partition"),
        (uuid!("49F48D32-B10E-11DC-B99B-0019D1879648"), "NetBSD[k] 	Swap partition"),
        (uuid!("49F48D5A-B10E-11DC-B99B-0019D1879648"), "FFS partition"),
        (uuid!("49F48D82-B10E-11DC-B99B-0019D1879648"), "LFS partition"),
        (uuid!("49F48DAA-B10E-11DC-B99B-0019D1879648"), "RAID partition"),
        (uuid!("2DB519C4-B10F-11DC-B99B-0019D1879648"), "Concatenated partition"),
        (uuid!("2DB519EC-B10F-11DC-B99B-0019D1879648"), "Encrypted partition"),
        (uuid!("FE3A2A5D-4F32-41A7-B725-ACCC3285A309"), "Chrome OS 	Chrome OS kernel"),
        (uuid!("3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC"), "Chrome OS rootfs"),
        (uuid!("CAB6E88E-ABF3-4102-A07A-D4BB9BE3C1D3"), "Chrome OS firmware"),
        (uuid!("2E0A753D-9E48-43B0-8337-B15192CB1B5E"), "Chrome OS future use"),
        (uuid!("09845860-705F-4BB5-B16C-8A8A099CAF52"), "Chrome OS miniOS"),
        (uuid!("3F0F8318-F146-4E6B-8222-C28C8F02E0D5"), "Chrome OS hibernate"),
        (uuid!("5DFBF5F4-2848-4BAC-AA5E-0D9A20B745A6"), "Container Linux by CoreOS 	/usr partition (coreos-usr)"),
        (uuid!("3884DD41-8582-4404-B9A8-E9B84F2DF50E"), "Resizable rootfs (coreos-resize)"),
        (uuid!("C95DC21A-DF0E-4340-8D7B-26CBFA9A03E0"), "OEM customizations (coreos-reserved)"),
        (uuid!("BE9067B9-EA49-4F15-B4F6-F36F8C9E1818"), "Root filesystem on RAID (coreos-root-raid)"),
        (uuid!("42465331-3BA3-10F1-802A-4861696B7521"), "Haiku 	Haiku BFS"),
        (uuid!("85D5E45E-237C-11E1-B4B3-E89A8F7FC3A7"), "MidnightBSD[k] 	Boot partition"),
        (uuid!("85D5E45A-237C-11E1-B4B3-E89A8F7FC3A7"), "BSD Data partition"),
        (uuid!("85D5E45B-237C-11E1-B4B3-E89A8F7FC3A7"), "BSD Swap partition"),
        (uuid!("0394EF8B-237E-11E1-B4B3-E89A8F7FC3A7"), "Unix File System (UFS) partition"),
        (uuid!("85D5E45C-237C-11E1-B4B3-E89A8F7FC3A7"), "Vinum volume manager partition"),
        (uuid!("85D5E45D-237C-11E1-B4B3-E89A8F7FC3A7"), "ZFS partition"),
        (uuid!("45B0969E-9B03-4F30-B4C6-B4B80CEFF106"), "Ceph[l] 	Journal"),
        (uuid!("45B0969E-9B03-4F30-B4C6-5EC00CEFF106"), "dm-crypt journal"),
        (uuid!("4FBD7E29-9D25-41B8-AFD0-062C0CEFF05D"), "OSD"),
        (uuid!("4FBD7E29-9D25-41B8-AFD0-5EC00CEFF05D"), "dm-crypt OSD"),
        (uuid!("89C57F98-2FE5-4DC0-89C1-F3AD0CEFF2BE"), "Disk in creation"),
        (uuid!("89C57F98-2FE5-4DC0-89C1-5EC00CEFF2BE"), "dm-crypt disk in creation"),
        (uuid!("CAFECAFE-9B03-4F30-B4C6-B4B80CEFF106"), "Block"),
        (uuid!("30CD0809-C2B2-499C-8879-2D6B78529876"), "Block DB"),
        (uuid!("5CE17FCE-4087-4169-B7FF-056CC58473F9"), "Block write-ahead log"),
        (uuid!("FB3AABF9-D25F-47CC-BF5E-721D1816496B"), "Lockbox for dm-crypt keys"),
        (uuid!("4FBD7E29-8AE0-4982-BF9D-5A8D867AF560"), "Multipath OSD"),
        (uuid!("45B0969E-8AE0-4982-BF9D-5A8D867AF560"), "Multipath journal"),
        (uuid!("CAFECAFE-8AE0-4982-BF9D-5A8D867AF560"), "Multipath block"),
        (uuid!("7F4A666A-16F3-47A2-8445-152EF4D03F6C"), "Multipath block"),
        (uuid!("EC6D6385-E346-45DC-BE91-DA2A7C8B3261"), "Multipath block DB"),
        (uuid!("01B41E1B-002A-453C-9F17-88793989FF8F"), "Multipath block write-ahead log"),
        (uuid!("CAFECAFE-9B03-4F30-B4C6-5EC00CEFF106"), "dm-crypt block"),
        (uuid!("93B0052D-02D9-4D8A-A43B-33A3EE4DFBC3"), "dm-crypt block DB"),
        (uuid!("306E8683-4FE2-4330-B7C0-00A917C16966"), "dm-crypt block write-ahead log"),
        (uuid!("45B0969E-9B03-4F30-B4C6-35865CEFF106"), "dm-crypt LUKS journal"),
        (uuid!("CAFECAFE-9B03-4F30-B4C6-35865CEFF106"), "dm-crypt LUKS block"),
        (uuid!("166418DA-C469-4022-ADF4-B30AFD37F176"), "dm-crypt LUKS block DB"),
        (uuid!("86A32090-3647-40B9-BBBD-38D8C573AA86"), "dm-crypt LUKS block write-ahead log"),
        (uuid!("4FBD7E29-9D25-41B8-AFD0-35865CEFF05D"), "dm-crypt LUKS OSD"),
        (uuid!("824CC7A0-36A8-11E3-890A-952519AD3F61"), "OpenBSD 	Data partition"),
        (uuid!("CEF5A9AD-73BC-4601-89F3-CDEEEEE321A1"), "QNX 	Power-safe (QNX6) file system"),
        (uuid!("C91818F9-8025-47AF-89D2-F030D7000C2C"), "Plan 9 	Plan 9 partition"),
        (uuid!("9D275380-40AD-11DB-BF97-000C2911D1B8"), "VMware ESX 	vmkcore (coredump partition)"),
        (uuid!("AA31E02A-400F-11DB-9590-000C2911D1B8"), "VMFS filesystem partition"),
        (uuid!("9198EFFC-31C0-11DB-8F78-000C2911D1B8"), "VMware Reserved"),
        (uuid!("2568845D-2332-4675-BC39-8FA5A4748D15"), "Android-IA 	Bootloader"),
        (uuid!("114EAFFE-1552-4022-B26E-9B053604CF84"), "Android Bootloader2"),
        (uuid!("49A4D17F-93A3-45C1-A0DE-F50B2EBE2599"), "Android Boot"),
        (uuid!("4177C722-9E92-4AAB-8644-43502BFD5506"), "Android Recovery"),
        (uuid!("EF32A33B-A409-486C-9141-9FFB711F6266"), "Android Misc"),
        (uuid!("20AC26BE-20B7-11E3-84C5-6CFDB94711E9"), "Android Metadata"),
        (uuid!("38F428E6-D326-425D-9140-6E0EA133647C"), "Android System"),
        (uuid!("A893EF21-E428-470A-9E55-0668FD91A2D9"), "Android Cache"),
        (uuid!("DC76DDA9-5AC1-491C-AF42-A82591580C0D"), "Android Data"),
        (uuid!("EBC597D0-2053-4B15-8B64-E0AAC75F4DB1"), "Android Persistent"),
        (uuid!("C5A0AEEC-13EA-11E5-A1B1-001E67CA0C3C"), "Android Vendor"),
        (uuid!("BD59408B-4514-490D-BF12-9878D963F378"), "Android Config"),
        (uuid!("8F68CC74-C5E5-48DA-BE91-A0C8C15E9C80"), "Android Factory"),
        (uuid!("9FDAA6EF-4B3F-40D2-BA8D-BFF16BFB887B"), "Android Factory (alt)"),
        (uuid!("767941D0-2085-11E3-AD3B-6CFDB94711E9"), "Android Fastboot / Tertiary"),
        (uuid!("AC6D7924-EB71-4DF8-B48D-E267B27148FF"), "Android OEM"),
        (uuid!("19A710A2-B3CA-11E4-B026-10604B889DCF"), "Android 6.0+ ARM 	Android Meta"),
        (uuid!("193D1EA4-B3CA-11E4-B075-10604B889DCF"), "Android EXT"),
        (uuid!("7412F7D5-A156-4B13-81DC-867174929325"), "Open Network Install Environment (ONIE) 	Boot"),
        (uuid!("D4E6E2CD-4469-46F3-B5CB-1BFF57AFC149"), "Config"),
        (uuid!("9E1A2D38-C612-4316-AA26-8B49521E5A8B"), "PowerPC 	PReP boot"),
        (uuid!("734E5AFE-F61A-11E6-BC64-92361F002671"), "Atari TOS 	Basic data partition (GEM, BGM, F32)"),
        (uuid!("8C8F8EFF-AC95-4770-814A-21994F2DBC8F"), "VeraCrypt 	Encrypted data partition"),
        (uuid!("90B6FF38-B98F-4358-A21F-48F35B4A8AD3"), "OS/2 	ArcaOS Type 1"),
        (uuid!("7C5222BD-8F5D-4087-9C00-BF9843C7B58C"), "Storage Performance Development Kit (SPDK) 	SPDK block device"),
        (uuid!("4778ED65-BF42-45FA-9C5B-287A1DC4AAB1"), "barebox bootloader 	barebox-state"),
        (uuid!("3DE21764-95BD-54BD-A5C3-4ABE786F38A8"), "U-Boot bootloader 	U-Boot environment"),
        (uuid!("B6FA30DA-92D2-4A9A-96F1-871EC6486200"), "SoftRAID Status"),
        (uuid!("2E313465-19B9-463F-8126-8A7993773801"), "SoftRAID Scratch"),
        (uuid!("FA709C7E-65B1-4593-BFD5-E71D61DE9B02"), "SoftRAID Volume"),
        (uuid!("BBBA6DF5-F46F-4A89-8F59-8765B2727503"), "SoftRAID Cache"),
        (uuid!("FE8A2634-5E2E-46BA-99E3-3A192091A350"), "Fuchsia standard partitions 	Bootloader (slot A/B/R)"),
        (uuid!("D9FD4535-106C-4CEC-8D37-DFC020CA87CB"), "Durable mutable encrypted system data"),
        (uuid!("A409E16B-78AA-4ACC-995C-302352621A41"), "Durable mutable bootloader data (including A/B/R metadata)"),
        (uuid!("F95D940E-CABA-4578-9B93-BB6C90F29D3E"), "Factory-provisioned read-only system data"),
        (uuid!("10B8DBAA-D2BF-42A9-98C6-A7C5DB3701E7"), "Factory-provisioned read-only bootloader data"),
        (uuid!("49FD7CB8-DF15-4E73-B9D9-992070127F0F"), "Fuchsia Volume Manager"),
        (uuid!("421A8BFC-85D9-4D85-ACDA-B64EEC0133E9"), "Verified boot metadata (slot A/B/R)"),
        (uuid!("9B37FFF6-2E58-466A-983A-F7926D0B04E0"), "Zircon boot image (slot A/B/R)"),
        //Fuchsia legacy partitions
        (uuid!("606B000B-B7C7-4653-A7D5-B737332C899D"), "fuchsia-system"),
        (uuid!("08185F0C-892D-428A-A789-DBEEC8F55E6A"), "fuchsia-data"),
        (uuid!("48435546-4953-2041-494E-5354414C4C52"), "fuchsia-install"),
        (uuid!("2967380E-134C-4CBB-B6DA-17E7CE1CA45D"), "fuchsia-blob"),
        (uuid!("41D0E340-57E3-954E-8C1E-17ECAC44CFF5"), "fuchsia-fvm"),
        (uuid!("DE30CC86-1F4A-4A31-93C4-66F147D33E05"), "Zircon boot image (slot A)"),
        (uuid!("23CC04DF-C278-4CE7-8471-897D1A4BCDF7"), "Zircon boot image (slot B)"),
        (uuid!("A0E5CF57-2DEF-46BE-A80C-A2067C37CD49"), "Zircon boot image (slot R)"),
        (uuid!("4E5E989E-4C86-11E8-A15B-480FCF35F8E6"), "sys-config"),
        (uuid!("5A3A90BE-4C86-11E8-A15B-480FCF35F8E6"), "factory-config"),
        (uuid!("5ECE94FE-4C86-11E8-A15B-480FCF35F8E6"), "fuchsia bootloader"),
        (uuid!("8B94D043-30BE-4871-9DFA-D69556E8C1F3"), "guid-test"),
        (uuid!("A13B4D9A-EC5F-11E8-97D8-6C3BE52705BF"), "Fuchsia Verified boot metadata (slot A)"),
        (uuid!("A288ABF2-EC5F-11E8-97D8-6C3BE52705BF"), "Fuchsia Verified boot metadata (slot B)"),
        (uuid!("6A2460C3-CD11-4E8B-80A8-12CCE268ED0A"), "Fuchsia Verified boot metadata (slot R)"),
        (uuid!("1D75395D-F2C6-476B-A8B7-45CC1C97B476"), "Fuchsia misc"),
        (uuid!("900B0FC5-90CD-4D4F-84F9-9F8ED579DB88"), "Fuchsia emmc-boot1"),
        (uuid!("B2B2E8D1-7C10-4EBC-A2D0-4614568260AD"), "Fuchsia emmc-boot2"),
    ]);
}
