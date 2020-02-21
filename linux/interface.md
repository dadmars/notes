The Linux Programming inTerface
    --- by Michael KerrisK

The kernel provid a software layer to manage the limited resources of a computer.

/boot/vmlinuz
    The Linux kernel executable

Memory management
    Processes are isolated from one another and from the kernel, so that one process can’t read or modify the memory of another process or the kernel.
    Only part of a process needs to be kept in memory, thereby lowering the memory requirements of each process and allowing more processes to be held in RAM simultaneously. This leads to better CPU utilization, since it increases the likelihood that, at any moment in time, there is at least one process that the CPU(s) can execute

/etc/passwd
    mtk:x:1000:100:Michael Kerrisk:/home/mtk:/bin/bash

    Login name
    Encrypted password
        if shadow passwords have been enabled, the password field in contains the letter x
        If the password field in /etc/passwd is empty, then no password is required
        User ID (UID)
            If this field has the value 0, then this account has superuser privileges (root)
        Group ID (GID)
        Comment
        Home directory
        Login shell

/etc/shadow

/etc/group
    users:x:100:
    jambit:x:106:claus,felli,frank,harti,markus,martin,mtk,paul

    Group name
    Group ID (GID)
    User list

/etc/gshadow

File types
    regular or plain files
    devices
    pipes
    sockets
    directories
        a special file whose contents take the form of a table of filenames coupled with references to the corresponding files.
        This filename-plus-reference association is called a link(hard link)
        files may have multiple links, and thus multiple names, in the same or in different directories.

        a hard link must reside on the same file system as the file to which it refers.
        A hard link can’t be made to a directory. This prevents the creation of circular links

    symbolic links
        a specially marked file containing the name of another file

Process memory layout
    text segment
        the machine-language instructions of the program run by the process.
        read-only so that a process doesn’t accidentally modify its own instructions 
        made sharable so that a single copy of the program code can be mapped into the virtual address space of all of the processes.

    initialized data segment
        contains global and static variables that are explicitly initialized.
        The values of these variables are read from the executable file when the program is loaded into memory.

    uninitialized data segment
        contains global and static variables that are not explicitly initialized.
        Before starting the program, the system initializes all memory in this segment to 0.
        when a program is stored on disk, it is not necessary to allocate space for the uninitialized data.

    stack
        dynamically growing and shrinking segment containing stack frames.
        One stack frame is allocated for each currently called function.
        A frame stores the function’s local variables, arguments, and return value. 

    heap
        an area from which memory can be dynamically allocated at run time
        The top end of the heap is called the program break.

An execve() call destroys the existing text, data, stack, and heap segments, replacing them with new segments based on the code of the new program.

If a process is killed by a signal, the termination status is set according to the type of signal that caused the death of the process.

By convention, a termination status of 0 indicates that the process succeeded, and a nonzero status indicates that some error occurred

Each process has a number of associated user IDs (UIDs) and group IDs (GIDs).
    Real user ID and real group ID
        These identify the user and group to which the process belongs.
        A new process inherits these IDs from its parent.
        A login shell gets its real user ID and real group ID from the corresponding fields in the system password file.

    Effective user ID and effective group ID
        These two IDs are used in determining the permissions that the process has when accessing protected resources such as files and interprocess communication objects.
    Changing the effective IDs is a mechanism that allows a process to assume the privileges of another user or group

    Supplementary group IDs
        These IDs identify additional groups to which a process belongs.
        A new process inherits its supplementary group IDs from its parent.
        A login shell gets its supplementary group IDs from the system group file.

Privileged processes
    UID = 0 (root)

/sbin/init
    The init process
    parent of all processes
    PID = 1
    runs with superuser privileges
    can’t be killed (not even by the superuser), and it terminates only when the system is shut down.

Memory Mappings
    A file mapping
        maps a region of a file into the calling process’s virtual memory.
   anonymous mapping
        doesn’t have a corresponding file. Instead, the pages of the mapping are initialized to 0.

Memory mappings serve
    initialization of a process’s text segment from the corresponding segment of an executable file
    allocation of new (zero-filled) memory
    file I/O (memory-mapped I/O)
    and interprocess communication (via a shared mapping)

interprocess communication (IPC)
    signals
        software interrupts
        the kernel may send a signal to a process when one of the following occurs:
            the user typed the interrupt character (usually Control-C) on the keyboard;
            one of the process’s children has terminated;
            a timer (alarm clock) set by the process has expired; or
            the process attempted to access an invalid memory address.

            Normally, a pending signal is delivered as soon as the receiving process is next scheduled to run, or immediately if the pro cess is already running.
            However, it is also possible to block a signal by adding it to the process’s signal mask.
            If a signal is generated while it is blocked, it remains pending until it is later unblocked.
    pipes
    sockets
    file locking
    message queues
    semaphores
    shared memory

/proc

/dev/fd --> /proc/self/fd
    Opening one of the files is equivalent to duplicating the corresponding file descriptor
        fd = open("/dev/fd/1", O_WRONLY);
        fd = dup(1);

    ls | diff /dev/fd/0 oldfilelist

Determining the version of glibc on the system
    /lib/libc.so.6
    ldd myprog | grep libc

if open() succeeds, it is guaranteed to use the lowest-numbered unused file descriptor for the process.

One use of fcntl() is to retrieve or modify the access mode and open file status flags of an open file.

three data structures maintained by the kernel
    the per-process file descriptor table;
        For each process, the kernel maintains a table of open file descriptors. Each entry:
            a set of flags controlling the operation of the file descriptor (there is just one such flag, close-on-exec)
            a reference to the open file description.

    the system-wide table of open file descriptions
        the current file offset
        status flags specified when opening the file
        the file access mode
        settings relating to signal-driven I/O
        a reference to the i-node object for this file.

    the file system i-node table.
        file type and permissions;
        a pointer to a list of locks held on this file
        various properties of the file, including its size and timestamps relating to different types of file operations.

$ ./myscript > results.log 2>&1
    duplicating file descriptor 2 so that it refers to the same open file description as file descriptor 1

O_NONBLOCK is generally ignored for regular files, because the kernel buffer cache ensures that I/O on regular files does not block

A program is a file containing a range of information that describes how to construct a process at run time:
    Binary format identification: 
        a.out (“assembler output”)
        COFF (Common Object File Format)
        Executable and Linking Format (ELF)

    Machine-language instructions
        These encode the algorithm of the program.

    Program entry-point address
        the location of the instruction at which execution of the program should commence.

    Data
        The program file contains values used to initialize variables and also literal constants used by the program 

    Symbol and relocation tables
        the locations and names of functions and variables within the program. for debugging and dynamic linking

    Shared-library and dynamic-linking information
        The program file includes fields listing the shared libraries that the program needs to use at run time and the pathname of the dynamic linker that should be used to load these libraries.

    Other information
        other information that describes how to construct a process.

pstree
    process's family tree

size
    displays the size of the text, initialized data, and uninitialized data (bss)

/proc/kallsyms
    provides addresses of kernel symbols in this region 

/proc/self/cmdline
/proc/PID/cmdline
    command-line arguments

/proc/PID/environ
    The environment list of any process

/proc/PID/status
    The Uid and Gid lines list the identifiers in the order real, effective, saved set, and file system.

printenv
    displays the current environment list

Set-User-ID and Set-Group-ID Programs
    A program allows a process to gain privileges it would not normally have, by setting the process’s effective user ID to the same value as the user ID (owner) of the executable file.

    chmod u+s prog
    chmod g+s prog

Saved Set-User-ID and Saved Set-Group-ID
    use with set-user-ID and set-group-ID programs.
        If the set-user-ID (set-group-ID) permission bit is enabled on the executable, then the effective user (group) ID of the process is made the same as the owner of the executable.
        The values for the saved set-user-ID and saved set-group-ID are copied from the corresponding effective IDs. This copying occurs regardless of whether the set-user-ID or set-group-ID bit is set on the file being executed.

usr/share/zoneinfo
    timezone information

time
    obtain both process time values, as well as the real time required to run the program

uname
    a range of identifying information about the host system on which an application is running,

The API provided by device drivers is fixed, and includes operations corresponding to the system calls
    open()
    close()
    read()
    write()
    mmap()
    ioctl()

/dev
    Device files appear within the file system, just like other files

/sys
    The udev program relies on the sysfs file system
    information about devices and other kernel objects

mknod
    The superuser create a device file 

A device’s major and minor IDs are recorded in the i-node for the device file
    The major ID
        identifies the general class of device, and is used by the kernel to look up the appropriate driver for this type of device.
    
    The minor ID
        uniquely identifies a particular device within a general class.

A file system contains the following parts:
    Boot block
        This is always the first block in a file system.
        The boot block is not used by the file system
        contains information used to boot the operating system.
        all file systems have a boot block (most of which are unused).

    Superblock
        a single block, immediately following the boot block
        contains parameter information about the file system
            the size of the i-node table;
            the size of logical blocks in this file system
            the size of the file system in logical blocks.

    I-node table
        Each file or directory in the file system has a unique entry in the i-node table
        This entry records various information about the file.

    Data blocks
        used for the blocks of data that form the files and directories residing in the file system.

i-node table contains one i-node (index node) for each file residing in the file system.
    I-nodes are identified numerically by their sequential location in the i-node table.
        File type (e.g., regular file, directory, symbolic link, character device).
        Owner (UID) for the file.
        Group (GID) for the file.
        Access permissions for three categories of user: owner, group, other 
        Three timestamps: 
        Number of hard links to the file.
        Size of the file in bytes.
        Number of blocks actually allocated to the file, measured in units of 512-byte blocks.
        Pointers to the data blocks of the file.

umount
mount device directory
    attaches the file system on the named device into the directory 

mount
    list the currently mounted file systems

/proc/mounts -> /proc/self/mounts
    A list of the currently mounted file systems

/proc/PID/mounts
    each process that lists the mount points constituting its mount namespace

/proc/ PID /exe
    symbolic link containing the absolute pathname of the executable file being run by the corresponding process.

/etc/mtab
    contains information that is similar to that in /proc/mounts , but slightly more detailed
    mount and umount commands automatically maintain the file /etc/mtab

/etc/fstab
    contains descriptions of all of the available file systems on a system
    maintained manually by the system administrator

mount --bind d1 d2
    A bind mount is somewhat like a hard link, but differs in two respects:
        A bind mount can cross file-system mount points (and even chroot jails).
        It is possible to make a bind mount for a directory

mount --rbind top dir2
    Recursive Bind Mounts

mount -t tmpfs source target
mount -t tmpfs newtmp /tmp
    tmpfs file system
        memory-based file systems 

    The source can be any name, its appears in /proc/mounts and is displayed by the mount and df commands
    target is the mount point for the file system. Note that it is not necessary to use mkfs to create a file system

    An invisible tmpfs file system, mounted internally by the kernel, is used for:
        implementing System V shared memory and shared anonymous memory mappings
    A tmpfs file system mounted at /dev/shm is used for the glibc implementation of POSIX shared memory and POSIX semaphores.

stat
    retrieve information about a file

chown
    change the owner (user ID) and group (group ID) of a file

umask
    process attribute that specifies which permission bits should always be turned off when new files or directories are created

chmod
    changes the permissions of the file

chattr
lsattr
    set and view i-node flags

ACLs(access control lists)
    allow file permissions to be specified per user or per group, for an arbitrary number of users and groups

getfacl
    view the ACL on a file

setfacl
    modifies a file ACL

chroot
    Changing the Root Directory of a Process
    all absolute pathnames are interpreted as starting from that location in the file system.
    the program is then confined to a particular area of the file system.

signal
    software interrupts.

    A signal is said to be generated by some event.
    Once generated, a signal is later delivered to a process, which then takes some action in response to the signal.
    Between the time it is generated and the time it is delivered, a signal is said to be pending.

kill pid 0
    (null signal), no signal is sent
    performs error checking to see if the process can be signaled.
    we can use the null signal to test if a process with a specific process ID exists.
    fails with the error ESRCH , then we know the process doesn’t exist.
    fails with the error EPERM 

core dump
    a file containing a memory image of the process at the time it terminated.
    This memory image can be loaded into a debugger in order to examine the state of a program’s code and data at the moment when the signal arrived.

/proc/ PID /coredump_filter
    used on a per-process basis to determine which types of memory mappings are written to a core dump file. 
        private anonymous mappings (default)
        private file mappings
        shared anonymous mappings (default)
        shared file mappings.

/proc/sys/kernel/core_pattern
    controls the naming of all core dump files produced on the system

the limitations of both nonblocking I/O and the use of multiple threads or processes:
    I/O multiplexing
        allows a process to simultaneously monitor multiple file descriptors to find out whether I/O is possible on any of them.
        select() and poll()
    Signal-driven I/O
        process requests that the kernel send it a signal when input is available or data can be written on a specified file descriptor.
    epoll API
        allows a process to monitor multiple file descriptors to see if I/O is possible on any of them.

    none of these techniques performs I/O.
    They merely tell us that a file descriptor is ready. Some other system call must then be used to actually perform the I/O.

Level-triggered notification
    A file descriptor is considered to be ready if it is possible to perform an I/O system call without blocking.

    select() and poll()
    epoll

Edge-triggered notification
    Notification is provided if there is I/O activity on a file descriptor since it was last monitored.

    Signal-driven I/O
    epoll

Nonblocking I/O (the O_NONBLOCK flag) is often used in conjunction with the I/O models

epoll instance
    The central data structure of the epoll API is an epoll instance

    the interest list: recording a list of file descriptors that this process has declared an interest in monitoring
    the ready list: maintaining a list of file descriptors that are ready for I/O

    For each file descriptor, we can specify a bit mask indicating events that we are interested in knowing about.

inotify mechanism
    allows an application to monitor file events.

    create an inotify instance. a file descriptor that is used to refer to the inotify instance
    watch list, Each watch item consists of a pathname and an associated bit mask.
    performs read() operations on the inotify file descriptor. returns one or more inotify_event structures
    closes the inotify file descriptor

    The inotify monitoring mechanism is not recursive
    An inotify file descriptor can be monitored using select(), poll(), epoll, and signal-driven I/O. If events are available to be read, then these interfaces indicate the inotify file descriptor as being readable.

/proc/sys/fs/inotify
    Queuing inotify events requires kernel memory. the kernel places various limits on the operation of the inotify mechanism.
