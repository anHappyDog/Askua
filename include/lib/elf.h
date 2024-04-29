#ifndef __ELF_H__
#define __ELF_H__
#include <types.h>

struct elf_ident {
  u8 ei_mag[4];
  u8 ei_class;
  u8 ei_data;
  u8 ei_version;
  u8 ei_osabi;
  u8 ei_abiversion;
  u8 ei_pad[7];
};

struct elf32_fhdr {
  struct elf_ident e_ident;
  u16 e_type;
  u16 e_machine;
  u32 e_version;
  u32 e_entry;
  u32 e_phoff;
  u32 e_shoff;
  u32 e_flags;
  u16 e_ehsize;
  u16 e_phentsize;
  u16 e_phnum;
  u16 e_shentsize;
  u16 e_shnum;
  u16 e_shstrndx;
};

struct elf64_fhdr {
  struct elf_ident e_ident;
  u16 e_type;
  u16 e_machine;
  u32 e_version;
  u64 e_entry;
  u64 e_phoff;
  u64 e_shoff;
  u32 e_flags;
  u16 e_ehsize;
  u16 e_phentsize;
  u16 e_phnum;
  u16 e_shentsize;
  u16 e_shnum;
  u16 e_shstrndx;
};

struct elf32_phdr {
  u32 p_type;
  u32 p_offset;
  u32 p_vaddr;
  u32 p_paddr;
  u32 p_filesz;
  u32 p_memsz;
  u32 p_flags;
  u32 p_align;
};

struct elf64_phdr {
  u32 p_type;
  u32 p_flags;
  u64 p_offset;
  u64 p_vaddr;
  u64 p_paddr;
  u64 p_filesz;
  u64 p_memsz;
  u64 p_align;
};

struct elf32_shdr {
  u32 sh_name;
  u32 sh_type;
  u32 sh_flags;
  u32 sh_addr;
  u32 sh_offset;
  u32 sh_size;
  u32 sh_link;
  u32 sh_info;
  u32 sh_addralign;
  u32 sh_entsize;
};

struct elf64_shdr {
  u32 sh_name;
  u32 sh_type;
  u64 sh_flags;
  u64 sh_addr;
  u64 sh_offset;
  u64 sh_size;
  u32 sh_link;
  u32 sh_info;
  u64 sh_addralign;
  u64 sh_entsize;
};

#define EI_CLASS_32 1 /* 32-bit objects */
#define EI_CLASS_64 2 /* 64-bit objects */

#define EI_DATA_LSB 1 /* 2's complement, little endian */
#define EI_DATA_MSB 2 /* 2's complement, big endian */

/* Legal values for e_type (object file type).  */
#define ET_NONE 0        /* No file type */
#define ET_REL 1         /* Relocatable file */
#define ET_EXEC 2        /* Executable file */
#define ET_DYN 3         /* Shared object file */
#define ET_CORE 4        /* Core file */
#define ET_NUM 5         /* Number of defined types */
#define ET_LOOS 0xfe00   /* OS-specific range start */
#define ET_HIOS 0xfeff   /* OS-specific range end */
#define ET_LOPROC 0xff00 /* Processor-specific range start */
#define ET_HIPROC 0xffff /* Processor-specific range end */

/* Legal values for sh_type (section type).  */

#define SHT_NULL 0                    /* Section header table entry unused */
#define SHT_PROGBITS 1                /* Program data */
#define SHT_SYMTAB 2                  /* Symbol table */
#define SHT_STRTAB 3                  /* String table */
#define SHT_RELA 4                    /* Relocation entries with addends */
#define SHT_HASH 5                    /* Symbol hash table */
#define SHT_DYNAMIC 6                 /* Dynamic linking information */
#define SHT_NOTE 7                    /* Notes */
#define SHT_NOBITS 8                  /* Program space with no data (bss) */
#define SHT_REL 9                     /* Relocation entries, no addends */
#define SHT_SHLIB 10                  /* Reserved */
#define SHT_DYNSYM 11                 /* Dynamic linker symbol table */
#define SHT_INIT_ARRAY 14             /* Array of constructors */
#define SHT_FINI_ARRAY 15             /* Array of destructors */
#define SHT_PREINIT_ARRAY 16          /* Array of pre-constructors */
#define SHT_GROUP 17                  /* Section group */
#define SHT_SYMTAB_SHNDX 18           /* Extended section indices */
#define SHT_NUM 19                    /* Number of defined types.  */
#define SHT_LOOS 0x60000000           /* Start OS-specific.  */
#define SHT_GNU_ATTRIBUTES 0x6ffffff5 /* Object attributes.  */
#define SHT_GNU_HASH 0x6ffffff6       /* GNU-style hash table.  */
#define SHT_GNU_LIBLIST 0x6ffffff7    /* Prelink library list */
#define SHT_CHECKSUM 0x6ffffff8       /* Checksum for DSO content.  */
#define SHT_LOSUNW 0x6ffffffa         /* Sun-specific low bound.  */
#define SHT_SUNW_move 0x6ffffffa
#define SHT_SUNW_COMDAT 0x6ffffffb
#define SHT_SUNW_syminfo 0x6ffffffc
#define SHT_GNU_verdef 0x6ffffffd  /* Version definition section.  */
#define SHT_GNU_verneed 0x6ffffffe /* Version needs section.  */
#define SHT_GNU_versym 0x6fffffff  /* Version symbol table.  */
#define SHT_HISUNW 0x6fffffff      /* Sun-specific high bound.  */
#define SHT_HIOS 0x6fffffff        /* End OS-specific type */
#define SHT_LOPROC 0x70000000      /* Start of processor-specific */
#define SHT_HIPROC 0x7fffffff      /* End of processor-specific */
#define SHT_LOUSER 0x80000000      /* Start of application-specific */
#define SHT_HIUSER 0x8fffffff      /* End of application-specific */

/* Legal values for sh_flags (section flags).  */

#define SHF_WRITE (1 << 0)      /* Writable */
#define SHF_ALLOC (1 << 1)      /* Occupies memory during execution */
#define SHF_EXECINSTR (1 << 2)  /* Executable */
#define SHF_MERGE (1 << 4)      /* Might be merged */
#define SHF_STRINGS (1 << 5)    /* Contains nul-terminated strings */
#define SHF_INFO_LINK (1 << 6)  /* `sh_info' contains SHT index */
#define SHF_LINK_ORDER (1 << 7) /* Preserve order after combining */
#define SHF_OS_NONCONFORMING                                                   \
  (1 << 8)                       /* Non-standard OS specific handling          \
                                    required */
#define SHF_GROUP (1 << 9)       /* Section is member of a group.  */
#define SHF_TLS (1 << 10)        /* Section hold thread-local data.  */
#define SHF_COMPRESSED (1 << 11) /* Section with compressed data. */
#define SHF_MASKOS 0x0ff00000    /* OS-specific.  */
#define SHF_MASKPROC 0xf0000000  /* Processor-specific */
#define SHF_GNU_RETAIN (1 << 21) /* Not to be GCed by linker.  */
#define SHF_ORDERED                                                            \
  (1 << 30) /* Special ordering requirement                                    \
               (Solaris).  */
#define SHF_EXCLUDE                                                            \
  (1U << 31) /* Section is excluded unless                                     \
                referenced or allocated (Solaris).*/

/* Legal values for p_type (segment type).  */

#define PT_NULL 0                  /* Program header table entry unused */
#define PT_LOAD 1                  /* Loadable program segment */
#define PT_DYNAMIC 2               /* Dynamic linking information */
#define PT_INTERP 3                /* Program interpreter */
#define PT_NOTE 4                  /* Auxiliary information */
#define PT_SHLIB 5                 /* Reserved */
#define PT_PHDR 6                  /* Entry for header table itself */
#define PT_TLS 7                   /* Thread-local storage segment */
#define PT_NUM 8                   /* Number of defined types */
#define PT_LOOS 0x60000000         /* Start of OS-specific */
#define PT_GNU_EH_FRAME 0x6474e550 /* GCC .eh_frame_hdr segment */
#define PT_GNU_STACK 0x6474e551    /* Indicates stack executability */
#define PT_GNU_RELRO 0x6474e552    /* Read-only after relocation */
#define PT_GNU_PROPERTY 0x6474e553 /* GNU property */
#define PT_LOSUNW 0x6ffffffa
#define PT_SUNWBSS 0x6ffffffa   /* Sun Specific segment */
#define PT_SUNWSTACK 0x6ffffffb /* Stack segment */
#define PT_HISUNW 0x6fffffff
#define PT_HIOS 0x6fffffff   /* End of OS-specific */
#define PT_LOPROC 0x70000000 /* Start of processor-specific */
#define PT_HIPROC 0x7fffffff /* End of processor-specific */

/* Legal values for p_flags (segment flags).  */

#define PF_X (1 << 0)          /* Segment is executable */
#define PF_W (1 << 1)          /* Segment is writable */
#define PF_R (1 << 2)          /* Segment is readable */
#define PF_MASKOS 0x0ff00000   /* OS-specific */
#define PF_MASKPROC 0xf0000000 /* Processor-specific */

/* RISC-V ELF Flags */
#define EF_RISCV_RVC 0x0001
#define EF_RISCV_FLOAT_ABI 0x0006
#define EF_RISCV_FLOAT_ABI_SOFT 0x0000
#define EF_RISCV_FLOAT_ABI_SINGLE 0x0002
#define EF_RISCV_FLOAT_ABI_DOUBLE 0x0004
#define EF_RISCV_FLOAT_ABI_QUAD 0x0006

#endif // __ELF_H__