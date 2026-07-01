# System Snapshot

Generated: 2026-07-01T04:50:42+01:00

## Host
```text
 Static hostname: saltbox26
       Icon name: computer-vm
         Chassis: vm 🖴
      Machine ID: 96b18368420745dcab3c3ab14be41af2
         Boot ID: 1ba8040b6ac04dc2bbb10697c81c2dcc
  Virtualization: kvm
Operating System: Ubuntu 24.04.4 LTS
          Kernel: Linux 6.8.0-124-generic
    Architecture: x86-64
 Hardware Vendor: QEMU
  Hardware Model: Standard PC _Q35 + ICH9, 2009_
Firmware Version: 4.2025.05-2
   Firmware Date: Thu 2025-11-13
    Firmware Age: 7month 2w 3d
```
## Kernel
```text
Linux saltbox26 6.8.0-124-generic #124-Ubuntu SMP PREEMPT_DYNAMIC Tue May 26 13:00:45 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux
```
## CPU
```text
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           48 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  28
On-line CPU(s) list:                     0-27
Vendor ID:                               AuthenticAMD
Model name:                              AMD Ryzen 9 5950X 16-Core Processor
CPU family:                              25
Model:                                   33
Thread(s) per core:                      1
Core(s) per socket:                      28
Socket(s):                               1
Stepping:                                2
BogoMIPS:                                6800.37
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm rep_good nopl cpuid extd_apicid tsc_known_freq pni pclmulqdq ssse3 fma cx16 sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw perfctr_core ssbd ibrs ibpb stibp vmmcall fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr wbnoinvd arat npt lbrv nrip_save tsc_scale vmcb_clean flushbyasid pausefilter pfthreshold v_vmsave_vmload vgif umip pku ospke vaes vpclmulqdq rdpid overflow_recov succor fsrm
Virtualization:                          AMD-V
Hypervisor vendor:                       KVM
Virtualization type:                     full
L1d cache:                               1.8 MiB (28 instances)
L1i cache:                               1.8 MiB (28 instances)
L2 cache:                                14 MiB (28 instances)
L3 cache:                                448 MiB (28 instances)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0-27
Vulnerability Gather data sampling:      Not affected
Vulnerability Indirect target selection: Not affected
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Not affected
Vulnerability Mds:                       Not affected
Vulnerability Meltdown:                  Not affected
Vulnerability Mmio stale data:           Not affected
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Not affected
Vulnerability Spec rstack overflow:      Mitigation; Safe RET
Vulnerability Spec store bypass:         Mitigation; Speculative Store Bypass disabled via prctl
Vulnerability Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:                Mitigation; Retpolines; IBPB conditional; IBRS_FW; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Mitigation; Clear CPU buffers
Vulnerability Tsx async abort:           Not affected
Vulnerability Vmscape:                   Not affected
```
## Memory
```text
               total        used        free      shared  buff/cache   available
Mem:            46Gi        37Gi       856Mi       342Mi       9.7Gi       9.6Gi
Swap:           47Gi        23Gi        24Gi
```
## Block Devices
```text
NAME                      MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
loop0                       7:0    0     4K  1 loop /snap/bare/5
loop1                       7:1    0  63.8M  1 loop /snap/core20/2769
loop2                       7:2    0  63.8M  1 loop /snap/core20/2866
loop3                       7:3    0    74M  1 loop /snap/core22/2339
loop4                       7:4    0    74M  1 loop /snap/core22/2411
loop5                       7:5    0  66.8M  1 loop /snap/core24/1643
loop6                       7:6    0 252.4M  1 loop /snap/firefox/8521
loop7                       7:7    0 252.4M  1 loop /snap/firefox/8504
loop8                       7:8    0 606.1M  1 loop /snap/gnome-46-2404/153
loop9                       7:9    0  91.7M  1 loop /snap/gtk-common-themes/1535
loop10                      7:10   0   395M  1 loop /snap/mesa-2404/1165
loop11                      7:11   0  48.4M  1 loop /snap/snapd/26382
loop12                      7:12   0  49.3M  1 loop /snap/snapd/26865
sda                         8:0    0   1.5T  0 disk 
├─sda1                      8:1    0     1M  0 part 
├─sda2                      8:2    0     2G  0 part /boot
└─sda3                      8:3    0   1.5T  0 part 
  └─ubuntu--vg-ubuntu--lv 252:0    0   1.5T  0 lvm  /mnt/remote
                                                    /mnt/remote
                                                    /mnt
                                                    /
sdb                         8:16   0     1G  0 disk 
└─sdb1                      8:17   0  1022M  0 part /boot/efi
sdc                         8:32   0   1.8T  0 disk 
└─sdc1                      8:33   0   1.8T  0 part 
sdd                         8:48   0   1.8T  0 disk 
└─sdd1                      8:49   0   1.8T  0 part /opt/whisparr
                                                    /opt/decypharr
                                                    /mnt/local
                                                    /mnt/local
                                                    /mnt/symlinks
                                                    /mnt/symlinks
                                                    /mnt/remote/decypharr
                                                    /mnt/remote/decypharr
                                                    /mnt/remote/decypharr
                                                    /mnt/remote/decypharr
                                                    /mnt/fast
                                                    /mnt/fast
                                                    /mnt/decypharr
                                                    /mnt/decypharr
                                                    /mnt/bulk
                                                    /mnt/bulk
                                                    /mnt/altmount
                                                    /mnt/altmount
                                                    /mnt/nvme2
                                                    /mnt/nvme2
sr0                        11:0    1  1024M  0 rom  
```
## Filesystems - Safe Root Check
```text
Filesystem                         Size  Used Avail Use% Mounted on
/dev/mapper/ubuntu--vg-ubuntu--lv  1.5T  1.1T  399G  73% /
```
## Mounts
```text
TARGET                                                                                             SOURCE                                                                                                             FSTYPE            OPTIONS
/                                                                                                  /dev/mapper/ubuntu--vg-ubuntu--lv                                                                                  ext4              rw,nodiratime,relatime
├─/sys                                                                                             sysfs                                                                                                              sysfs             rw,nosuid,nodev,noexec,relatime
│ ├─/sys/firmware/efi/efivars                                                                      efivarfs                                                                                                           efivarfs          rw,nosuid,nodev,noexec,relatime
│ ├─/sys/kernel/security                                                                           securityfs                                                                                                         securityfs        rw,nosuid,nodev,noexec,relatime
│ ├─/sys/fs/cgroup                                                                                 cgroup2                                                                                                            cgroup2           rw,nosuid,nodev,noexec,relatime,nsdelegate,memory_recursiveprot
│ ├─/sys/fs/pstore                                                                                 pstore                                                                                                             pstore            rw,nosuid,nodev,noexec,relatime
│ ├─/sys/fs/bpf                                                                                    bpf                                                                                                                bpf               rw,nosuid,nodev,noexec,relatime,mode=700
│ ├─/sys/kernel/debug                                                                              debugfs                                                                                                            debugfs           rw,nosuid,nodev,noexec,relatime
│ │ └─/sys/kernel/debug/tracing                                                                    tracefs                                                                                                            tracefs           rw,nosuid,nodev,noexec,relatime
│ ├─/sys/kernel/tracing                                                                            tracefs                                                                                                            tracefs           rw,nosuid,nodev,noexec,relatime
│ ├─/sys/fs/fuse/connections                                                                       fusectl                                                                                                            fusectl           rw,nosuid,nodev,noexec,relatime
│ └─/sys/kernel/config                                                                             configfs                                                                                                           configfs          rw,nosuid,nodev,noexec,relatime
├─/proc                                                                                            proc                                                                                                               proc              rw,nosuid,nodev,noexec,relatime
│ └─/proc/sys/fs/binfmt_misc                                                                       systemd-1                                                                                                          autofs            rw,relatime,fd=32,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=32159
│   └─/proc/sys/fs/binfmt_misc                                                                     binfmt_misc                                                                                                        binfmt_misc       rw,nosuid,nodev,noexec,relatime
├─/dev                                                                                             udev                                                                                                               devtmpfs          rw,nosuid,relatime,size=24586148k,nr_inodes=6146537,mode=755,inode64
│ ├─/dev/pts                                                                                       devpts                                                                                                             devpts            rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000
│ ├─/dev/shm                                                                                       tmpfs                                                                                                              tmpfs             rw,nosuid,nodev,inode64
│ ├─/dev/hugepages                                                                                 hugetlbfs                                                                                                          hugetlbfs         rw,nosuid,nodev,relatime,pagesize=2M
│ └─/dev/mqueue                                                                                    mqueue                                                                                                             mqueue            rw,nosuid,nodev,noexec,relatime
├─/run                                                                                             tmpfs                                                                                                              tmpfs             rw,nosuid,nodev,noexec,relatime,size=4925612k,mode=755,inode64
│ ├─/run/lock                                                                                      tmpfs                                                                                                              tmpfs             rw,nosuid,nodev,noexec,relatime,size=5120k,inode64
│ ├─/run/qemu                                                                                      tmpfs                                                                                                              tmpfs             rw,nosuid,nodev,relatime,mode=755,inode64
│ ├─/run/user/1000                                                                                 tmpfs                                                                                                              tmpfs             rw,nosuid,nodev,relatime,size=4925608k,nr_inodes=1231402,mode=700,uid=1000,gid=1000,inode64
│ │ ├─/run/user/1000/gvfs                                                                          gvfsd-fuse                                                                                                         fuse.gvfsd-fuse   rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ │ └─/run/user/1000/doc                                                                           portal                                                                                                             fuse.portal       rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/run/docker/netns/d556956b9d91                                                                 nsfs[net:[4026534626]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/5ec860a3b086                                                                 nsfs[net:[4026533695]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/fd4e1e2044c2                                                                 nsfs[net:[4026534444]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/c96e7808fe7d                                                                 nsfs[net:[4026534013]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/2f68466d5cb9                                                                 nsfs[net:[4026534566]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/2e2d239b5340                                                                 nsfs[net:[4026534507]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/5fe4434eeb67                                                                 nsfs[net:[4026533950]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/6d704deecd2a                                                                 nsfs[net:[4026534139]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/15faad78baf6                                                                 nsfs[net:[4026534684]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/5fb629888952                                                                 nsfs[net:[4026534263]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/b1e75f97c0da                                                                 nsfs[net:[4026534324]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/4263d5f87c2c                                                                 nsfs[net:[4026534074]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/54bfaf0b382f                                                                 nsfs[net:[4026534202]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/e3c5a5a9a85c                                                                 nsfs[net:[4026534385]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/ec9c76db2adb                                                                 nsfs[net:[4026534747]]                                                                                             nsfs              rw
│ ├─/run/snapd/ns                                                                                  tmpfs[/snapd/ns]                                                                                                   tmpfs             rw,nosuid,nodev,noexec,relatime,size=4925612k,mode=755,inode64
│ │ └─/run/snapd/ns/firefox.mnt                                                                    nsfs[mnt:[4026534959]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/87a1171f17ba                                                                 nsfs[net:[4026535151]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/26f0539a4861                                                                 nsfs[net:[4026534980]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/3ef0a4d975f6                                                                 nsfs[net:[4026535586]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/25023360d1fc                                                                 nsfs[net:[4026535462]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/e1635e181696                                                                 nsfs[net:[4026535107]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/848026b1df9d                                                                 nsfs[net:[4026535299]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/92e3065511f4                                                                 nsfs[net:[4026535545]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/f278cb13f4ee                                                                 nsfs[net:[4026534840]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/3e5a1595f6f8                                                                 nsfs[net:[4026534901]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/8c6fe6ced30d                                                                 nsfs[net:[4026535026]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/15be6c2cc76b                                                                 nsfs[net:[4026536698]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/2520dd56a43f                                                                 nsfs[net:[4026535212]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/c927fd99f1f6                                                                 nsfs[net:[4026535336]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/60dcbaa7ba3d                                                                 nsfs[net:[4026535399]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/249ca79bfcb4                                                                 nsfs[net:[4026535680]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/193693c6087e                                                                 nsfs[net:[4026535710]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/7c4333ccf087                                                                 nsfs[net:[4026535810]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/230677d3ba3f                                                                 nsfs[net:[4026535945]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/f156c0f95ceb                                                                 nsfs[net:[4026536947]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/b2c42a6a62a2                                                                 nsfs[net:[4026538316]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/133506ded160                                                                 nsfs[net:[4026537491]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/719b527ff458                                                                 nsfs[net:[4026535834]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/03823ca11d01                                                                 nsfs[net:[4026536450]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/efd579241abb                                                                 nsfs[net:[4026536755]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/cbb1112de6fd                                                                 nsfs[net:[4026536076]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/d5a43959ac73                                                                 nsfs[net:[4026536383]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/0a3485a5ea28                                                                 nsfs[net:[4026535958]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/69f136385fe0                                                                 nsfs[net:[4026538060]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/02dc9923b31a                                                                 nsfs[net:[4026537179]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/2caacbb820fd                                                                 nsfs[net:[4026537988]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/933a701c27bf                                                                 nsfs[net:[4026537243]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/d3afc77938fb                                                                 nsfs[net:[4026537920]]                                                                                             nsfs              rw
│ ├─/run/docker/netns/88dd95741048                                                                 nsfs[net:[4026537670]]                                                                                             nsfs              rw
│ └─/run/docker/netns/7ed0bec29582                                                                 nsfs[net:[4026536140]]                                                                                             nsfs              rw
├─/mnt                                                                                             /dev/mapper/ubuntu--vg-ubuntu--lv[/mnt]                                                                            ext4              rw,relatime
│ ├─/mnt/remote                                                                                    /dev/mapper/ubuntu--vg-ubuntu--lv[/mnt/remote]                                                                     ext4              rw,relatime
│ │ ├─/mnt/remote/decypharr                                                                        /dev/sdd1[/remote-decypharr-mnt]                                                                                   ext4              rw,noatime
│ │ └─/mnt/remote/google                                                                           google{-m2zs}:                                                                                                     fuse.rclone       rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other
│ ├─/mnt/nvme2                                                                                     /dev/sdd1                                                                                                          ext4              rw,noatime
│ │ └─/mnt/nvme2/altmount-mnt/altmount                                                             altmount                                                                                                           fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
│ ├─/mnt/altmount                                                                                  /dev/sdd1[/altmount-mnt]                                                                                           ext4              rw,noatime
│ │ └─/mnt/altmount/altmount                                                                       altmount                                                                                                           fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
│ ├─/mnt/bulk                                                                                      /dev/sdd1[/bulk]                                                                                                   ext4              rw,noatime
│ ├─/mnt/decypharr                                                                                 /dev/sdd1[/decypharr-mnt]                                                                                          ext4              rw,noatime
│ ├─/mnt/fast                                                                                      /dev/sdd1[/fast]                                                                                                   ext4              rw,noatime
│ ├─/mnt/remote/decypharr                                                                          /dev/sdd1[/remote-decypharr-mnt]                                                                                   ext4              rw,noatime
│ ├─/mnt/symlinks                                                                                  /dev/sdd1[/symlinks-mnt]                                                                                           ext4              rw,noatime
│ ├─/mnt/local                                                                                     /dev/sdd1[/bulk]                                                                                                   ext4              rw,noatime
│ ├─/mnt/x32-roms                                                                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/saturn-roms                                                                               FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/remote/google                                                                             google{-m2zs}:                                                                                                     fuse.rclone       rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other
│ ├─/mnt/unionfs                                                                                   mergerfs                                                                                                           fuse.mergerfs     rw,nosuid,nodev,noatime,user_id=0,group_id=0,default_permissions,allow_other
│ ├─/mnt/gba-roms                                                                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc1.cdi                                                   altmount[/complete/games/_Dreamcast_Shenmue/cd1/2003-11-23-Shenmue-cd1-pal-DCP/2003-11-23-Shenmue-cd1-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
│ ├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc2.cdi                                                   altmount[/complete/games/_Dreamcast_Shenmue/cd2/2003-11-23-Shenmue-cd2-pal-DCP/2003-11-23-Shenmue-cd2-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
│ ├─/mnt/x68000-roms                                                                               FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc3.cdi                                                   altmount[/complete/games/_Dreamcast_Shenmue/cd3/2003-11-23-Shenmue-cd3-pal-DCP/2003-11-23-Shenmue-cd3-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
│ ├─/mnt/psx-roms                                                                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Baten Kaitos Origins (USA) (Disc 1)                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Madden NFL 2005 (USA)                                                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Metal Gear Solid - The Twin Snakes (USA) (Disc 2)                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/ESPN International Winter Sports 2002 (USA)                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Tiger Woods PGA Tour 2004 (USA) (Disc 1)                                         FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/NBA Live 2004 (USA)                                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/NCAA Football 2003 (USA)                                                         FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/NBA 2K2 (USA)                                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Metroid Prime 2 - Echoes (USA)                                                   FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/F1 2002 (USA) (En,Fr,De,It)                                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Nickelodeon Tak 2 - The Staff of Dreams (USA)                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/NASCAR 2005 - Chase for the Cup (USA)                                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/MLB SlugFest 2003 (USA)                                                          FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Dakar 2 - The World's Ultimate Rally (USA) (En,Fr,De,Es,It)                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Tony Hawk's Pro Skater 4 (USA)                                                   FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc Version 11 (USA)                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Digimon World 4 (USA)                                                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc - January 2002 (USA)                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Resident Evil Zero (USA) (Disc 1)                                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Advance Game Port (USA) (Unl) (Rev 1)                                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Rayman 3 - Hoodlum Havoc (USA) (En,Fr,De,Es,It)                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Kao the Kangaroo - Round 2 (USA)                                                 FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Tom Clancy's Rainbow Six 3 (USA)                                                 FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Resident Evil 4 (USA) (Disc 1)                                                   FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/FIFA Street 2 (USA) (En,Es)                                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Knockout Kings 2003 (USA)                                                        FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Sims 2, The - Pets (USA)                                                         FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/4x4 Evo 2 (USA)                                                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ ├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc Version 30 (USA)                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
│ └─/mnt/ngc-roms/Mat Hoffman's Pro BMX 2 (USA)                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/remote                                                                                      /dev/mapper/ubuntu--vg-ubuntu--lv[/mnt/remote]                                                                     ext4              rw,relatime
│ ├─/mnt/remote/decypharr                                                                          /dev/sdd1[/remote-decypharr-mnt]                                                                                   ext4              rw,noatime
│ └─/mnt/remote/google                                                                             google{-m2zs}:                                                                                                     fuse.rclone       rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other
├─/snap/bare/5                                                                                     /dev/loop0                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/core20/2769                                                                                /dev/loop1                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/core20/2866                                                                                /dev/loop2                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/core22/2339                                                                                /dev/loop3                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/core22/2411                                                                                /dev/loop4                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/core24/1643                                                                                /dev/loop5                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/firefox/8521                                                                               /dev/loop6                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/firefox/8504                                                                               /dev/loop7                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/gnome-46-2404/153                                                                          /dev/loop8                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/gtk-common-themes/1535                                                                     /dev/loop9                                                                                                         squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/snap/mesa-2404/1165                                                                             /dev/loop10                                                                                                        squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/boot                                                                                            /dev/sda2                                                                                                          ext4              rw,relatime
│ └─/boot/efi                                                                                      /dev/sdb1                                                                                                          vfat              rw,relatime,fmask=0077,dmask=0077,codepage=437,iocharset=iso8859-1,shortname=mixed,errors=remount-ro
├─/mnt/nvme2                                                                                       /dev/sdd1                                                                                                          ext4              rw,noatime
│ └─/mnt/nvme2/altmount-mnt/altmount                                                               altmount                                                                                                           fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
├─/mnt/altmount                                                                                    /dev/sdd1[/altmount-mnt]                                                                                           ext4              rw,noatime
│ └─/mnt/altmount/altmount                                                                         altmount                                                                                                           fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
├─/mnt/bulk                                                                                        /dev/sdd1[/bulk]                                                                                                   ext4              rw,noatime
├─/mnt/decypharr                                                                                   /dev/sdd1[/decypharr-mnt]                                                                                          ext4              rw,noatime
├─/mnt/fast                                                                                        /dev/sdd1[/fast]                                                                                                   ext4              rw,noatime
├─/mnt/remote/decypharr                                                                            /dev/sdd1[/remote-decypharr-mnt]                                                                                   ext4              rw,noatime
├─/snap/snapd/26382                                                                                /dev/loop11                                                                                                        squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/mnt/symlinks                                                                                    /dev/sdd1[/symlinks-mnt]                                                                                           ext4              rw,noatime
├─/snap/snapd/26865                                                                                /dev/loop12                                                                                                        squashfs          ro,nodev,relatime,errors=continue,threads=single
├─/mnt/local                                                                                       /dev/sdd1[/bulk]                                                                                                   ext4              rw,noatime
├─/opt/decypharr                                                                                   /dev/sdd1[/bulk/opt/decypharr]                                                                                     ext4              rw,noatime
├─/opt/whisparr                                                                                    /dev/sdd1[/bulk/opt/whisparr]                                                                                      ext4              rw,noatime
├─/var/lib/docker/overlay2/5cd13898be2aa2709be6f103bf3d3e386949ad349ed99770c827a127c418636c/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/B2AHLIXDWZ5NR4EPCPUR42D247:/var/lib/docker/overlay2/l/DEQJILPSA3ZS7EKVPRTLHDPJCR:/var/lib/docker/overlay2/l/JYYKNF4NUFR4HVI5TWTXODQKJG:/var/lib/docker/overlay2/l/OOOETE22PIVZNULSNT2PRLM56D:/var/lib/docker/overlay2/l/52HFVAKIC7H4DEMRRFOPQFKGD7:/var/lib/docker/overlay2/l/RMCKAQKE7KYVWYQITHDPO7C3QM:/var/lib/docker/overlay2/l/GUPRMG47D5KS7PJSKQTZZCDX7T,upperdir=/var/lib/docker/overlay2/5cd13898be2aa2709be6f103bf3d3e386949ad349ed99770c827a127c418636c/diff,workdir=/var/lib/docker/overlay2/5cd13898be2aa2709be6f103bf3d3e386949ad349ed99770c827a127c418636c/work,nouserxattr
├─/var/lib/docker/overlay2/28a39d8416bd3ae9b1eb1c27f312ca36194b7f4da422941c8154e179a26ec03e/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/UTPHORDAVFO34Y4BDHNYICXXUP:/var/lib/docker/overlay2/l/PLWJXKT4JSPLMB3POPC6DPQKZV:/var/lib/docker/overlay2/l/D6IPRE4BXIXUXDDI6QAONNGOF5:/var/lib/docker/overlay2/l/F2DPESFUGA3TGUPE36J7QXKVWS:/var/lib/docker/overlay2/l/Z3CXIU43GHIH3BUU6LWEAZZZ6H:/var/lib/docker/overlay2/l/FMYVUVM344HHWHE4DO53FRKWAC:/var/lib/docker/overlay2/l/32PYQIMNNT42RNBZKJK54MUAI3:/var/lib/docker/overlay2/l/GUPRMG47D5KS7PJSKQTZZCDX7T,upperdir=/var/lib/docker/overlay2/28a39d8416bd3ae9b1eb1c27f312ca36194b7f4da422941c8154e179a26ec03e/diff,workdir=/var/lib/docker/overlay2/28a39d8416bd3ae9b1eb1c27f312ca36194b7f4da422941c8154e179a26ec03e/work,nouserxattr
├─/var/lib/docker/overlay2/ddee13443ed643524486dfeb71eb87e291eeef1f3046a0325dd3bf49b9941b59/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/WT7JL7XPMRGWENXSPWAG4WJMRE:/var/lib/docker/overlay2/l/NXVXKLF5232P5CG5QKEIA63IA3:/var/lib/docker/overlay2/l/L54HZ6EP3LVQQAO6OBF266WMTJ:/var/lib/docker/overlay2/l/GQ7YRMRUBW55W7PV756JUAHWV7:/var/lib/docker/overlay2/l/FLH3QN22W4RDVNZNQVZ5I4EAZO:/var/lib/docker/overlay2/l/HOA33AMBT2YJ74PFRENDGAOKOR:/var/lib/docker/overlay2/l/QDIKJHDNNTPXHPMWRDE2RIKNYX:/var/lib/docker/overlay2/l/XHYYZDHKP2KPC4TI5PHBH5OJK7:/var/lib/docker/overlay2/l/BNBQOGZJXJ7QQGXVTHK65R6LDO:/var/lib/docker/overlay2/l/GUPRMG47D5KS7PJSKQTZZCDX7T,upperdir=/var/lib/docker/overlay2/ddee13443ed643524486dfeb71eb87e291eeef1f3046a0325dd3bf49b9941b59/diff,workdir=/var/lib/docker/overlay2/ddee13443ed643524486dfeb71eb87e291eeef1f3046a0325dd3bf49b9941b59/work,nouserxattr
├─/mnt/x32-roms                                                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/var/lib/docker/overlay2/725273fdd2238642a53056a74d42853b637402221ac8d744e2624568995a1b16/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/SHFJLIHXRZOHHJZNVGHC4Z5MDZ:/var/lib/docker/overlay2/l/OUUP2H44535CXHH6A7M554RKXF:/var/lib/docker/overlay2/l/LMOX4YMNBIO4OYZ5YP2DLJK5T7:/var/lib/docker/overlay2/l/IWZ6KBSHEQMVBRBJ7CPRWOB5EN:/var/lib/docker/overlay2/l/Y6P53GQHXXEPCT3XURS66J234S:/var/lib/docker/overlay2/l/QLPXDAUPN3Q3JHZN6YZJVKRETE:/var/lib/docker/overlay2/l/MEIA37XMDE6KWK4EPAWPPMMFMJ:/var/lib/docker/overlay2/l/5PZCUSEEODG73O623YCN3V5LDS:/var/lib/docker/overlay2/l/OKH6N6MXGTR23KZSEELJQR7QUN:/var/lib/docker/overlay2/l/O3CG3QCVNTAM6EEABITREXGJ6O,upperdir=/var/lib/docker/overlay2/725273fdd2238642a53056a74d42853b637402221ac8d744e2624568995a1b16/diff,workdir=/var/lib/docker/overlay2/725273fdd2238642a53056a74d42853b637402221ac8d744e2624568995a1b16/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/0eec312571706d67539df38f7e400dfdccbfd416b317be94ae4b67aad9d5f42e/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/53XDVXV5N3LBPBZ5IMARZ5FHWW:/var/lib/docker/overlay2/l/B5TDRQ4TLNTQB2GWTBI74XCREB:/var/lib/docker/overlay2/l/2X76GDM7TEWI2CH7T46FDYYRMZ:/var/lib/docker/overlay2/l/SQGANR4Z6WNO57HDGJMOY7OFET:/var/lib/docker/overlay2/l/IJM4IQJXBFSZ46VTEGIKSSVPNU:/var/lib/docker/overlay2/l/RLKLZCEZGJKTLJGUGHGIDESDS5:/var/lib/docker/overlay2/l/5S7I3RO2FHFOOSNTUKRPQDQ4GN:/var/lib/docker/overlay2/l/BMK2TVQPCDIBONAMXES536XZT6:/var/lib/docker/overlay2/l/O3QYJOQXJ44LD4C6TJAJ5LU4ZU:/var/lib/docker/overlay2/l/U5LNSLTSISAAMBITQTTUXN3NLU,upperdir=/var/lib/docker/overlay2/0eec312571706d67539df38f7e400dfdccbfd416b317be94ae4b67aad9d5f42e/diff,workdir=/var/lib/docker/overlay2/0eec312571706d67539df38f7e400dfdccbfd416b317be94ae4b67aad9d5f42e/work,nouserxattr
├─/var/lib/docker/overlay2/5133d8b6cd815b99be6dc93dbbc572d5388814f02ac2200d3cadf628789331cb/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/3JG6VVHHZYGSAHZF4A25ARGYSO:/var/lib/docker/overlay2/l/RTRVJQ2QOBI5KXZKAYBU764UCR:/var/lib/docker/overlay2/l/IFHMWNMUVUBEELVUG7GQC3RUII:/var/lib/docker/overlay2/l/DYL566IZFL7FMWE37X2PLG7LES:/var/lib/docker/overlay2/l/GKXDGCGAOSQ6ZZPHL3LS574R3J:/var/lib/docker/overlay2/l/3LIDZ5KECXL24QQIRVXKFCCWWW:/var/lib/docker/overlay2/l/AB4DWJD52VR6TPSPBXMAH7OSWZ:/var/lib/docker/overlay2/l/CDXVOXN3QJE2DQWLKVWVHC5JL6,upperdir=/var/lib/docker/overlay2/5133d8b6cd815b99be6dc93dbbc572d5388814f02ac2200d3cadf628789331cb/diff,workdir=/var/lib/docker/overlay2/5133d8b6cd815b99be6dc93dbbc572d5388814f02ac2200d3cadf628789331cb/work,nouserxattr
├─/var/lib/docker/overlay2/71629f39fdc313fbdb7250c973b59f4621d450485c38c17356b535fe9ed0069c/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/J4PPBXRTSM2YKDA5QWPBLMYBNS:/var/lib/docker/overlay2/l/RTRVJQ2QOBI5KXZKAYBU764UCR:/var/lib/docker/overlay2/l/IFHMWNMUVUBEELVUG7GQC3RUII:/var/lib/docker/overlay2/l/DYL566IZFL7FMWE37X2PLG7LES:/var/lib/docker/overlay2/l/GKXDGCGAOSQ6ZZPHL3LS574R3J:/var/lib/docker/overlay2/l/3LIDZ5KECXL24QQIRVXKFCCWWW:/var/lib/docker/overlay2/l/AB4DWJD52VR6TPSPBXMAH7OSWZ:/var/lib/docker/overlay2/l/CDXVOXN3QJE2DQWLKVWVHC5JL6,upperdir=/var/lib/docker/overlay2/71629f39fdc313fbdb7250c973b59f4621d450485c38c17356b535fe9ed0069c/diff,workdir=/var/lib/docker/overlay2/71629f39fdc313fbdb7250c973b59f4621d450485c38c17356b535fe9ed0069c/work,nouserxattr
├─/mnt/saturn-roms                                                                                 FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/remote/google                                                                               google{-m2zs}:                                                                                                     fuse.rclone       rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other
├─/var/lib/docker/overlay2/b9642abe0e0c7b62d420538f7c8393cdc060ff1d1ef3825eb337bce53af35ad3/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/OKQ62P4YDFIUKRRGEDMW3Q2QJK:/var/lib/docker/overlay2/l/QDZU3MW33TKRP3FZLV2HHNIAK2:/var/lib/docker/overlay2/l/WVMCQJAGSLPWH3W5ZK6UJRPZ3V:/var/lib/docker/overlay2/l/Y6PNSALA35OQBQ6BCV22DTXIA7:/var/lib/docker/overlay2/l/VASUO4X3GTQY73AHADDKGXBICN:/var/lib/docker/overlay2/l/SJZFBXMFD6XXLOQMIGMNYK5T72:/var/lib/docker/overlay2/l/D7ZYR67QPRB4VI3VKGLJT7H535:/var/lib/docker/overlay2/l/HUUKLLPOOMYCO6JIYZ3LX7FEP7:/var/lib/docker/overlay2/l/V3LBMCSLAGVIGGP3OVSSYHFICQ,upperdir=/var/lib/docker/overlay2/b9642abe0e0c7b62d420538f7c8393cdc060ff1d1ef3825eb337bce53af35ad3/diff,workdir=/var/lib/docker/overlay2/b9642abe0e0c7b62d420538f7c8393cdc060ff1d1ef3825eb337bce53af35ad3/work,nouserxattr
├─/var/lib/docker/overlay2/fa8cc8636f44e161fd83cfc45cfdb9ffc63db6e2d2877d1731ac0f3bc98e1729/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/XH3KOQTPLGAA5BSQO55DH5456M:/var/lib/docker/overlay2/l/XCVJI2YFT3CWPOETREGMLQSXVV:/var/lib/docker/overlay2/l/YHVK6GC66DMGXOT4AMTR4H74EP:/var/lib/docker/overlay2/l/WDP7C3N4RAIQYYOYGOMV4R5ARF:/var/lib/docker/overlay2/l/GGHDODTYLAQ2TYCV3FOG6WJB72:/var/lib/docker/overlay2/l/H7O7F2UBC5PVIXNW5BFC623FTR:/var/lib/docker/overlay2/l/NSAVAZPMYDDF6WWPGRYSP6OYXE:/var/lib/docker/overlay2/l/O2U5GEP5ZOA3PXXK7BXKJ7ZVGI:/var/lib/docker/overlay2/l/7JNYKE4AGEJ6OAR4MTFTRQWXOP:/var/lib/docker/overlay2/l/6BKRVG3P5GXFCCOCSGCJSYRWHS:/var/lib/docker/overlay2/l/3RCWZGHHUXER3LMMWA3NFQIYO2:/var/lib/docker/overlay2/l/4PBVH4MUOHWM257HKC7BG3V3H4:/var/lib/docker/overlay2/l/UU3V5OW3ZVWB5PGXC7ZAK2VQ5F:/var/lib/docker/overlay2/l/MGYGNLHCSK52PODBKYPWTQFTWW:/var/lib/docker/overlay2/l/5D74FYKP7TEFDSUW6YX4SFU4LA:/var/lib/docker/overlay2/l/NWUZTXOSVJBSLA6ZYRPIPQCHTZ,upperdir=/var/lib/docker/overlay2/fa8cc8636f44e161fd83cfc45cfdb9ffc63db6e2d2877d1731ac0f3bc98e1729/diff,workdir=/var/lib/docker/overlay2/fa8cc8636f44e161fd83cfc45cfdb9ffc63db6e2d2877d1731ac0f3bc98e1729/work,nouserxattr
├─/mnt/unionfs                                                                                     mergerfs                                                                                                           fuse.mergerfs     rw,nosuid,nodev,noatime,user_id=0,group_id=0,default_permissions,allow_other
├─/var/lib/docker/overlay2/7b11d182d3c57a8606f97902870e6813ede89dd2e61feae0cd01a816adecbeb6/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/5GZENJILO546DEVTL6CSABJ2GV:/var/lib/docker/overlay2/l/N57543HS2Z7AASIQORUGIDMPGJ:/var/lib/docker/overlay2/l/RQ3AIJJNQCSA3V2RE3DA6IAY7S:/var/lib/docker/overlay2/l/6XDP3JAUN4KF7USB2ZKPSWUCS3:/var/lib/docker/overlay2/l/6EKGT7B7LSZKOZXEYQ7YRGYJGJ:/var/lib/docker/overlay2/l/FXLGYCQ67MS744QRM7TYLN2LWL:/var/lib/docker/overlay2/l/SLKCG533SPLSBPFCER63P6QYPV:/var/lib/docker/overlay2/l/BCGEEXINSQOEVPYFBYLS43XV7O:/var/lib/docker/overlay2/l/NHZDD7NIMAI5WUDNGFN3H4ZNJW:/var/lib/docker/overlay2/l/3V3NU7I2RYS55H6OFYBUD6FL7Z:/var/lib/docker/overlay2/l/BNLBBB2YT7PMV62XUUKJQ3JSAJ,upperdir=/var/lib/docker/overlay2/7b11d182d3c57a8606f97902870e6813ede89dd2e61feae0cd01a816adecbeb6/diff,workdir=/var/lib/docker/overlay2/7b11d182d3c57a8606f97902870e6813ede89dd2e61feae0cd01a816adecbeb6/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/827a61d2e3c3af7f1e6153f2a6bb5bf842011e6e0be0870daa200abf075aa15b/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/OEWWJXT5Q3WSFOZNRC6IJ6YOTU:/var/lib/docker/overlay2/l/3KHPQYF2XXU3YGURMQBJJM6VKS:/var/lib/docker/overlay2/l/5HRX7JDTWEXPHZ3JCALDHGXDSD:/var/lib/docker/overlay2/l/2SKCB6NPUU5QN3MELB43V6GGVX:/var/lib/docker/overlay2/l/MRVEN4LBM7GGNHOIXMR4SALUBL:/var/lib/docker/overlay2/l/Z6SC5SVXARDYPLYKDUVF2XRJKV:/var/lib/docker/overlay2/l/FGROWV2RXPPRAFD4ISZXRTIUOU:/var/lib/docker/overlay2/l/YPF532KYAQXDQAIKZRINHEIE7Z:/var/lib/docker/overlay2/l/VGCBW2THMHW7UKT66435MC7KMD:/var/lib/docker/overlay2/l/PR2IJTD7WSXZLFOZQRLHZRYZWI:/var/lib/docker/overlay2/l/TMMSGOHK2GEHDUVR5GPJFL7D3A:/var/lib/docker/overlay2/l/GUPRMG47D5KS7PJSKQTZZCDX7T,upperdir=/var/lib/docker/overlay2/827a61d2e3c3af7f1e6153f2a6bb5bf842011e6e0be0870daa200abf075aa15b/diff,workdir=/var/lib/docker/overlay2/827a61d2e3c3af7f1e6153f2a6bb5bf842011e6e0be0870daa200abf075aa15b/work,nouserxattr
├─/var/lib/docker/overlay2/8f4032af047c67b5712ab5aa144d0a39b995f6f757570d87d90b510eef4a3a2b/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/VGZBGNCPN2OZ75VKCU2YP7MRM6:/var/lib/docker/overlay2/l/FZJ526NGCK6IV4ANPIJRFTD2FO:/var/lib/docker/overlay2/l/IJR3QAA7WFT2KMQYE3A63GRLGK:/var/lib/docker/overlay2/l/7F7UIKOBQK6IUVX3Z5NS64TWBU:/var/lib/docker/overlay2/l/AFWD2WCPPJLBBIOY7CHNZDOK64:/var/lib/docker/overlay2/l/NQTRL3WBFL3OEI43RTUPFUUPHS:/var/lib/docker/overlay2/l/SRXJ6VOAVUDZK3XYHTZU6ZRQKC:/var/lib/docker/overlay2/l/6CSKJJR5EYGW7DVVJ2KXDE4BN6:/var/lib/docker/overlay2/l/CKX2FXSLMGWZBDWS6DNBU4OTAO:/var/lib/docker/overlay2/l/PN663UHI7MR6KGIQVT5KA7GQSH,upperdir=/var/lib/docker/overlay2/8f4032af047c67b5712ab5aa144d0a39b995f6f757570d87d90b510eef4a3a2b/diff,workdir=/var/lib/docker/overlay2/8f4032af047c67b5712ab5aa144d0a39b995f6f757570d87d90b510eef4a3a2b/work,nouserxattr
├─/var/lib/docker/overlay2/bc7b1772a4ffe011303b52321f34ab44dfb1684680c45369ecb2f26afee8df20/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/22337SY5CMEMDGGPQ4UVIN5LGF:/var/lib/docker/overlay2/l/WOAJHDGTKMXDH3EYQVWYWWA5O7:/var/lib/docker/overlay2/l/TAP6HTO5PEDJBJGAYBVOCUOBKV:/var/lib/docker/overlay2/l/2IPX2IEIZFFO3QDXRWWHT4XT6D:/var/lib/docker/overlay2/l/J25YKTCZ4XWOCTJN6QE6XXLZSC:/var/lib/docker/overlay2/l/7WVOVVNDSJYGX6X7BCVYT6Y52N:/var/lib/docker/overlay2/l/GAQT4XTMMFQD43BXXLSUDB753E:/var/lib/docker/overlay2/l/H66KG6O34C6BOGDI7WHR3GRYOP:/var/lib/docker/overlay2/l/JRSSENPK7VBVQPD2R5KUW2QEPE:/var/lib/docker/overlay2/l/DEIJDM7P3SPYD7XBQPAKM6L465:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/bc7b1772a4ffe011303b52321f34ab44dfb1684680c45369ecb2f26afee8df20/diff,workdir=/var/lib/docker/overlay2/bc7b1772a4ffe011303b52321f34ab44dfb1684680c45369ecb2f26afee8df20/work,nouserxattr
├─/var/lib/docker/overlay2/bb54643f6f18ad9bcd458f040b60256ab632ea85c1aa9655bd4aec02cb26cc00/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/YC3LMRJJIS5D4HMGAE2ORRG6Q5:/var/lib/docker/overlay2/l/XRHBTWATI5NKCKOEANF4JNZ4YY:/var/lib/docker/overlay2/l/7TC33WG7ZUXQC5HGQ4ZJ67U4XV:/var/lib/docker/overlay2/l/IJ4SWG2UNOAULF66PV7MCXR45D:/var/lib/docker/overlay2/l/AYLRZQTFGOQDPRAETBBFSUTUHP:/var/lib/docker/overlay2/l/OH5TB7HTQIPKIB4OY3WT4UXJZA:/var/lib/docker/overlay2/l/HEDE7J73VTLJPC6CG7SZ7IVIBZ:/var/lib/docker/overlay2/l/OODOVRQJ7ZNH6PIZVJX7AT6BPL:/var/lib/docker/overlay2/l/VS2EM6QJC6E6SH47KCKTOFUAFY:/var/lib/docker/overlay2/l/SB5VP545PLWFTWPDRUQYJQ3QG2:/var/lib/docker/overlay2/l/F6DNS35SXEE3ZKK2K2Q5ELJYMD:/var/lib/docker/overlay2/l/RIWXUKBTGGVDSXREYECFJVZFZN:/var/lib/docker/overlay2/l/Z4J5QLCZULAL7UNHHFQU6GJHWI:/var/lib/docker/overlay2/l/DZESRRKQR47KV3M4WJT77EGBKN,upperdir=/var/lib/docker/overlay2/bb54643f6f18ad9bcd458f040b60256ab632ea85c1aa9655bd4aec02cb26cc00/diff,workdir=/var/lib/docker/overlay2/bb54643f6f18ad9bcd458f040b60256ab632ea85c1aa9655bd4aec02cb26cc00/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/31c39c9a46cb346664b520fef568c5d8ce77f66bc6a6d8e764d1ab45093dab11/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/YVK6BRBF43QB2ZXLL4S33OCB3D:/var/lib/docker/overlay2/l/2FLVXF6IIVJYJQ7V3NWEUIHJ32:/var/lib/docker/overlay2/l/2FZ7VARENYFJXB6PVHLWEWCPN7:/var/lib/docker/overlay2/l/YDJEW6LN77QPDO36HVXLAQKXWM:/var/lib/docker/overlay2/l/YXLWJZJGT2YOHJJ4IAXRSUJDE5:/var/lib/docker/overlay2/l/YFVLF4IBWCJXZI3XVC5XPAZGKA:/var/lib/docker/overlay2/l/YGFG6QNL5KCEDTODKQ6XIRUA6R:/var/lib/docker/overlay2/l/3F2V2ZZNTMD3S4XIEJO6X4HZVE:/var/lib/docker/overlay2/l/I2CQ7DFORIEYKLX3F2YUTZSALI:/var/lib/docker/overlay2/l/BP3JUYX6MNKMZJ2B5ZNAQGIBOD:/var/lib/docker/overlay2/l/W5YFIZFBXXWD6E7PWP4PVSXUB5:/var/lib/docker/overlay2/l/2XGKX3L4GOE4727JWCHK5HTHVZ:/var/lib/docker/overlay2/l/7GLV5ZUX3SLQDKXL3JHGZAODNH:/var/lib/docker/overlay2/l/XMT5OQOZ6OHE76BZYYIMH42APZ:/var/lib/docker/overlay2/l/2DQIQ7KVWFO4RQUIECBSM2TNRX:/var/lib/docker/overlay2/l/YJRDS2AHOURDCLX34VVGH7U7V4:/var/lib/docker/overlay2/l/Z4T4WRBZU23XC6VEXOFLPMUZEO:/var/lib/docker/overlay2/l/FGD3H6L6TKN2764YDSEKH4WERL:/var/lib/docker/overlay2/l/4X2EGLDABO56WYAEVYTCEXE545:/var/lib/docker/overlay2/l/KXBR4OGVFAQU7R7DIKHDIKXRFU:/var/lib/docker/overlay2/l/7EU7ZBQ7KDV7AOHHFOD7DT7ZMI:/var/lib/docker/overlay2/l/JVM7TG6ESCG2ILEYQS62INS3ZS:/var/lib/docker/overlay2/l/J3YM2C5TRH6K4SEX4GYY3DQ5NQ:/var/lib/docker/overlay2/l/4G7ATLYOBKDH4P5IXEELQDGV32,upperdir=/var/lib/docker/overlay2/31c39c9a46cb346664b520fef568c5d8ce77f66bc6a6d8e764d1ab45093dab11/diff,workdir=/var/lib/docker/overlay2/31c39c9a46cb346664b520fef568c5d8ce77f66bc6a6d8e764d1ab45093dab11/work,nouserxattr
├─/var/lib/docker/overlay2/def5c0feba605d5d34706bdd33a22dee98e272932648b3cde410400119066e77/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/L2MJTP5SWXXRUI2DLD7O7MI6HG:/var/lib/docker/overlay2/l/ISOJNE2U4EKOM6JZBJ3TE2SITG:/var/lib/docker/overlay2/l/HJQRQS5P32NKW6BIAUIUINXYMD:/var/lib/docker/overlay2/l/B5EJAGEPOKZ7URI7KRFLWZ5KHZ:/var/lib/docker/overlay2/l/YKJBHGEUS7LSLYVEWXRLQQ5S7I,upperdir=/var/lib/docker/overlay2/def5c0feba605d5d34706bdd33a22dee98e272932648b3cde410400119066e77/diff,workdir=/var/lib/docker/overlay2/def5c0feba605d5d34706bdd33a22dee98e272932648b3cde410400119066e77/work,nouserxattr
├─/tmp/.mount_SRM.ApBATqYW                                                                         SRM.AppImage                                                                                                       fuse.SRM.AppImage ro,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/var/lib/docker/overlay2/222248c1856d039827700bc5862b7e340ef84f4f5f98b59be252376eee74ecbe/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/4ZILWDFYI7O2CPKE7KKQZ7JT2S:/var/lib/docker/overlay2/l/GVSZPQWTBNV4CAI6YZYL2RKSP2:/var/lib/docker/overlay2/l/5G4A67AK7HANAT7CZKIZUXDIC3:/var/lib/docker/overlay2/l/F4WB52D6JN5HFLWFXXBXGAZXX5:/var/lib/docker/overlay2/l/IXTU234UKARXCPBJIOJLOQ3BJW:/var/lib/docker/overlay2/l/ZE7I2MOHFAMQLPUEDNJMTR26N7,upperdir=/var/lib/docker/overlay2/222248c1856d039827700bc5862b7e340ef84f4f5f98b59be252376eee74ecbe/diff,workdir=/var/lib/docker/overlay2/222248c1856d039827700bc5862b7e340ef84f4f5f98b59be252376eee74ecbe/work,nouserxattr
├─/var/lib/docker/overlay2/f14f63f38bbf4c7e6b4bfd8c0a8c5f787c98e3924d2bfd25f80d395aca2c80d5/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/PGHXRU3YXNGNV5NK6UAQOP66LV:/var/lib/docker/overlay2/l/AHS4ZM5XX6U7TNHGMGD7WISORU:/var/lib/docker/overlay2/l/OQT2MDMXFGTLAZONPUOD3T3YVC:/var/lib/docker/overlay2/l/SYC2SJRNMTLTO3WCOWKU5276BE:/var/lib/docker/overlay2/l/DXYEKUEGF5JPMZGFMBEIAO7JLE:/var/lib/docker/overlay2/l/PN4VXHQNRIWTE24V36VN2M7R7Z:/var/lib/docker/overlay2/l/ZZ7D4SOIM55K4QAZQP3USWHJLZ,upperdir=/var/lib/docker/overlay2/f14f63f38bbf4c7e6b4bfd8c0a8c5f787c98e3924d2bfd25f80d395aca2c80d5/diff,workdir=/var/lib/docker/overlay2/f14f63f38bbf4c7e6b4bfd8c0a8c5f787c98e3924d2bfd25f80d395aca2c80d5/work,nouserxattr
├─/var/lib/docker/overlay2/3457f4f726d6ed4bbb162c81a1146ea25fc2e4d52debe29036fc4f2d4e702fcf/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/2K63A6ZITDAYCBLG4YXKOP3KRG:/var/lib/docker/overlay2/l/PGWSXT5UJ7XQC25KLMQWELFOWH:/var/lib/docker/overlay2/l/H4YH7SRJV7XVJSFDBMCDPIAG2R:/var/lib/docker/overlay2/l/RDBIATVCSSMBNLKDZ2KHESSK3L:/var/lib/docker/overlay2/l/K3E6J3E4LPMGQ6SMVEDMJUIYRI:/var/lib/docker/overlay2/l/3FPYRCC3DTFKDRNKW2SGPSZTWU:/var/lib/docker/overlay2/l/G32ZDXB4SUX6PWXAIL6ZR2B3KZ:/var/lib/docker/overlay2/l/X45RWUUTKLFW4UPUZ7DEI7WXUW,upperdir=/var/lib/docker/overlay2/3457f4f726d6ed4bbb162c81a1146ea25fc2e4d52debe29036fc4f2d4e702fcf/diff,workdir=/var/lib/docker/overlay2/3457f4f726d6ed4bbb162c81a1146ea25fc2e4d52debe29036fc4f2d4e702fcf/work,nouserxattr
├─/var/lib/docker/overlay2/b5db126d140987a30b42ce5937a94b4701a1df20f63a503cb9424c2ebda83c58/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/DUWGEAQ2HMXJYOUTW2QOUPAQMT:/var/lib/docker/overlay2/l/7NW4ROPIWQWXQ3NK552OL34N5B:/var/lib/docker/overlay2/l/RBWBLMMFTA2O47GF37Y5KSAHQJ:/var/lib/docker/overlay2/l/YXMNRLOA356C4YNPCDKJI2JSLL:/var/lib/docker/overlay2/l/4OJQAETIIMLUOVOJZZTIUYWFPZ:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/b5db126d140987a30b42ce5937a94b4701a1df20f63a503cb9424c2ebda83c58/diff,workdir=/var/lib/docker/overlay2/b5db126d140987a30b42ce5937a94b4701a1df20f63a503cb9424c2ebda83c58/work,nouserxattr
├─/var/lib/docker/overlay2/b83b0cd1c54aee206c487574285575ca5ce167b7a0190d346b1b92c5e34fd07f/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/HJBQUGWZAK7LFPDIPIUUTZYYHH:/var/lib/docker/overlay2/l/3U7MCIVLNRDFLS6ACSYDXAAJKX:/var/lib/docker/overlay2/l/5BLM4A5VBEWDPMPQGXNEF777QT:/var/lib/docker/overlay2/l/2MK7GJ4Q5QEV6QVEB5LQDJJG3J:/var/lib/docker/overlay2/l/IZ7MQL5RJTWIXRUOXT4XCOV52Y:/var/lib/docker/overlay2/l/ZWV2ASOUJ7J7IXOOXYRQEM6CUO:/var/lib/docker/overlay2/l/G3L2ZJ7LSQF7J73HAJOHUJLSQB:/var/lib/docker/overlay2/l/YKJBHGEUS7LSLYVEWXRLQQ5S7I,upperdir=/var/lib/docker/overlay2/b83b0cd1c54aee206c487574285575ca5ce167b7a0190d346b1b92c5e34fd07f/diff,workdir=/var/lib/docker/overlay2/b83b0cd1c54aee206c487574285575ca5ce167b7a0190d346b1b92c5e34fd07f/work,nouserxattr
├─/var/lib/docker/overlay2/296f17437310678b5fe0ea0371b76795b42ecb170f791a9140d2d62c4d4c4906/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/DE7IW6A5K2OPB7G2BTNSIPLX3Q:/var/lib/docker/overlay2/l/DWPPGVS3UJZUY6XZYKUMJWW6F2:/var/lib/docker/overlay2/l/5VBJ6AZRWOOLMBFGOQQY4XTO73:/var/lib/docker/overlay2/l/NHUDB66JKM76YLKPLDZV3KQBYE:/var/lib/docker/overlay2/l/ZZ7D4SOIM55K4QAZQP3USWHJLZ,upperdir=/var/lib/docker/overlay2/296f17437310678b5fe0ea0371b76795b42ecb170f791a9140d2d62c4d4c4906/diff,workdir=/var/lib/docker/overlay2/296f17437310678b5fe0ea0371b76795b42ecb170f791a9140d2d62c4d4c4906/work,nouserxattr
├─/var/lib/docker/overlay2/7f746301fcc65d17447f248dc40d559b4c1f9bff4eae3e996a31ddc3b9d5a44f/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/ZDUML5EC2WW72T73NUIJ3OSTLV:/var/lib/docker/overlay2/l/M46TKML422GOML4P6OUTZEP3ZX:/var/lib/docker/overlay2/l/KLJXB2Q4YKU4GDXCLS6HQVG5T5:/var/lib/docker/overlay2/l/WB3PNSR5WETKF2W7BSWEDOPPZB:/var/lib/docker/overlay2/l/PTI6K7WBS3ZI4YIU2S5S56466Q:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/7f746301fcc65d17447f248dc40d559b4c1f9bff4eae3e996a31ddc3b9d5a44f/diff,workdir=/var/lib/docker/overlay2/7f746301fcc65d17447f248dc40d559b4c1f9bff4eae3e996a31ddc3b9d5a44f/work,nouserxattr
├─/var/lib/docker/overlay2/b6e34fd95ca78fdbb102250063c7952c04ac4986bec15f3dbccb2c042eb7828d/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/DQ7JPIB7TUWJKAIKQXYGODRVX6:/var/lib/docker/overlay2/l/K6WVLPFT6QYX56XIRADQYIPWRJ:/var/lib/docker/overlay2/l/LGEVP3ZVUIFDXFWJLQFHLHXNP5:/var/lib/docker/overlay2/l/5M7EVVEJ3T4XVWK5ZYLNQVLO6P:/var/lib/docker/overlay2/l/CMR6V74NSH6PVIYJLB7J37NWNA:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/b6e34fd95ca78fdbb102250063c7952c04ac4986bec15f3dbccb2c042eb7828d/diff,workdir=/var/lib/docker/overlay2/b6e34fd95ca78fdbb102250063c7952c04ac4986bec15f3dbccb2c042eb7828d/work,nouserxattr
├─/var/lib/docker/overlay2/6898cd17dc1fd9a8adb22b054bad8aafe75889a77c3d09e889ca6b44069e4624/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/MNNAVTXJDDLTNIF2EE6D7BDA5Y:/var/lib/docker/overlay2/l/ESYJUXNPTJW2BCBFBS4GXL6QMF:/var/lib/docker/overlay2/l/5IPY3XURSFDUJ4VMY5WJX6GUFR:/var/lib/docker/overlay2/l/GZO5M6RAFFQKV5YRDDINOS2RGV:/var/lib/docker/overlay2/l/TAVNTPIJ7L6L5BGU6CXDFHD6WA:/var/lib/docker/overlay2/l/HFJNBGO22NYN2NLELZRJT44BU2:/var/lib/docker/overlay2/l/CG2DG6WRW76FY3TIDPMAP2YR6N:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/6898cd17dc1fd9a8adb22b054bad8aafe75889a77c3d09e889ca6b44069e4624/diff,workdir=/var/lib/docker/overlay2/6898cd17dc1fd9a8adb22b054bad8aafe75889a77c3d09e889ca6b44069e4624/work,nouserxattr
├─/var/lib/docker/overlay2/8cd87e468c465e92b854eef51711cada9322c948a85e1c4b2d26f061f042b413/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/3KV2PAXFFI7KIY4WVIF4NX6BPH:/var/lib/docker/overlay2/l/JA43BCEX77ROUTJ5IETZVU575P:/var/lib/docker/overlay2/l/AGDSPU2VOWB227ITFWIU47KB5V:/var/lib/docker/overlay2/l/DCI75WGQUQFXUIJRGSNLRN5GUF:/var/lib/docker/overlay2/l/RI4D7VT64UJFJW3ETKUA6DAQY3:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/8cd87e468c465e92b854eef51711cada9322c948a85e1c4b2d26f061f042b413/diff,workdir=/var/lib/docker/overlay2/8cd87e468c465e92b854eef51711cada9322c948a85e1c4b2d26f061f042b413/work,nouserxattr
├─/var/lib/docker/overlay2/0bf1035a35f499c79e5a519360f27db261738958860ea25568505e385228d671/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/22VCHKNI3EKBFS73PXEFM23EKW:/var/lib/docker/overlay2/l/MSWGEMVNOCBWVBC2U7HR47L5MS:/var/lib/docker/overlay2/l/T3TISVEW32VXRJIAERUHFKITV5:/var/lib/docker/overlay2/l/JKMXOH4LE5TH3DG5GGU6A4KLAS:/var/lib/docker/overlay2/l/DLQIK6HR77EJETKQJ6BP6FRKDL:/var/lib/docker/overlay2/l/3MNAO6ABV4NSQTP7MV3NXGQYNG:/var/lib/docker/overlay2/l/SFSZXAWN2LC7BUOBFVQCEF5JJN:/var/lib/docker/overlay2/l/CJ4JHST667K7MCJDKU5CO6KP3F:/var/lib/docker/overlay2/l/CDXVOXN3QJE2DQWLKVWVHC5JL6,upperdir=/var/lib/docker/overlay2/0bf1035a35f499c79e5a519360f27db261738958860ea25568505e385228d671/diff,workdir=/var/lib/docker/overlay2/0bf1035a35f499c79e5a519360f27db261738958860ea25568505e385228d671/work,nouserxattr
├─/var/lib/docker/overlay2/e0ae7465c4e97f7f4b98c40938990334e1ddb314ee81e5fe41b3c88fd805fdb5/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/EC7BOM23R7X6S2GAKDP3MW6JAG:/var/lib/docker/overlay2/l/UU7JZYYKZGIRGQRDAXDOJUUIE4:/var/lib/docker/overlay2/l/FBQHDDQ6D5KYBYX5CLBAUMENCP:/var/lib/docker/overlay2/l/Z7AFXI2ZOOLZJF3ML53EHFF3J2:/var/lib/docker/overlay2/l/UVMLN5HZEJGOBQUXTMVLO4WC77:/var/lib/docker/overlay2/l/KITMAKI2VGMDVCGFHKVV53FBJR,upperdir=/var/lib/docker/overlay2/e0ae7465c4e97f7f4b98c40938990334e1ddb314ee81e5fe41b3c88fd805fdb5/diff,workdir=/var/lib/docker/overlay2/e0ae7465c4e97f7f4b98c40938990334e1ddb314ee81e5fe41b3c88fd805fdb5/work,nouserxattr
├─/var/lib/docker/overlay2/6f7fecd4918ff47212c0771f0ac59691a7a946adc84db04d15e6ad5ad9626956/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/AD7FMAYL5HQ6ZGVQE2O74P3I3M:/var/lib/docker/overlay2/l/VIST6I3C6OFIYHR47AJO4NY4JE:/var/lib/docker/overlay2/l/CH5RMBULE7TEVS53YQDDGLAV6F:/var/lib/docker/overlay2/l/MWH7WCCDVTZC642GASZKJQUBW6:/var/lib/docker/overlay2/l/3Z2JUWOU66UJMOKES2PODZXYYC:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/6f7fecd4918ff47212c0771f0ac59691a7a946adc84db04d15e6ad5ad9626956/diff,workdir=/var/lib/docker/overlay2/6f7fecd4918ff47212c0771f0ac59691a7a946adc84db04d15e6ad5ad9626956/work,nouserxattr
├─/var/lib/docker/overlay2/6fac81432421d2f99c7531fd2648e4c5407e29e1bc46bce46d69610e3d8d0a5e/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/KX6VOLWADNZE7CEJCXA7O6FS4C:/var/lib/docker/overlay2/l/ISOJNE2U4EKOM6JZBJ3TE2SITG:/var/lib/docker/overlay2/l/HJQRQS5P32NKW6BIAUIUINXYMD:/var/lib/docker/overlay2/l/B5EJAGEPOKZ7URI7KRFLWZ5KHZ:/var/lib/docker/overlay2/l/YKJBHGEUS7LSLYVEWXRLQQ5S7I,upperdir=/var/lib/docker/overlay2/6fac81432421d2f99c7531fd2648e4c5407e29e1bc46bce46d69610e3d8d0a5e/diff,workdir=/var/lib/docker/overlay2/6fac81432421d2f99c7531fd2648e4c5407e29e1bc46bce46d69610e3d8d0a5e/work,nouserxattr
├─/var/lib/docker/overlay2/7cfe03e2daea11cd9cdb8f4067e190797b8eb666bff3e54307d48876177d52cf/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/HYNMS52WSPDV7B5EH43SNW2WOD:/var/lib/docker/overlay2/l/AQPWHVOGZAT2SKU2LMVJUK2YS3:/var/lib/docker/overlay2/l/GBIXJWFKUF6GC5V2TJGO7DBR3V:/var/lib/docker/overlay2/l/G2WHUI242PN75FHXZC6JSMYANX:/var/lib/docker/overlay2/l/5KNTNFLBVXOY47BCP234OVWA7H:/var/lib/docker/overlay2/l/6LJYJMTVJMXF4XVYC632XN47ZM:/var/lib/docker/overlay2/l/SK2PFDMQKLYILTPKLLYNJDLZQD:/var/lib/docker/overlay2/l/CGELIE4TK3RQTAHVCLJL3MPPO7:/var/lib/docker/overlay2/l/K4SOWYAX6IFWFINDDUSYVC5VRE:/var/lib/docker/overlay2/l/5KSRCQKOC7GA7XS5SMOZ5MTX5L:/var/lib/docker/overlay2/l/VMV6ZDA6UZ55QX6L4GKZ3VWETZ:/var/lib/docker/overlay2/l/V3LBMCSLAGVIGGP3OVSSYHFICQ,upperdir=/var/lib/docker/overlay2/7cfe03e2daea11cd9cdb8f4067e190797b8eb666bff3e54307d48876177d52cf/diff,workdir=/var/lib/docker/overlay2/7cfe03e2daea11cd9cdb8f4067e190797b8eb666bff3e54307d48876177d52cf/work,nouserxattr
├─/var/lib/docker/overlay2/d270f927085799ba8039c66012f5467229071099cd37b947c0936dd98fa7c717/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/XF7ODQ3D5XKWRPRRCBG2TYVBVW:/var/lib/docker/overlay2/l/CKDFNOOIBOQ4Q5IN6A6GO43GUN:/var/lib/docker/overlay2/l/3FIQ46AXLTCERIK2MPISKM4VHN:/var/lib/docker/overlay2/l/RHNAJISVYESVONJW57L4JLWHDP:/var/lib/docker/overlay2/l/5PG42YYMY5UJ4ITLPVIW2FFMZ4:/var/lib/docker/overlay2/l/NIHFSLKMOJGIFZYNZOIR6UPCXJ:/var/lib/docker/overlay2/l/6LJYJMTVJMXF4XVYC632XN47ZM:/var/lib/docker/overlay2/l/SK2PFDMQKLYILTPKLLYNJDLZQD:/var/lib/docker/overlay2/l/CGELIE4TK3RQTAHVCLJL3MPPO7:/var/lib/docker/overlay2/l/K4SOWYAX6IFWFINDDUSYVC5VRE:/var/lib/docker/overlay2/l/5KSRCQKOC7GA7XS5SMOZ5MTX5L:/var/lib/docker/overlay2/l/VMV6ZDA6UZ55QX6L4GKZ3VWETZ:/var/lib/docker/overlay2/l/V3LBMCSLAGVIGGP3OVSSYHFICQ,upperdir=/var/lib/docker/overlay2/d270f927085799ba8039c66012f5467229071099cd37b947c0936dd98fa7c717/diff,workdir=/var/lib/docker/overlay2/d270f927085799ba8039c66012f5467229071099cd37b947c0936dd98fa7c717/work,nouserxattr
├─/var/lib/docker/overlay2/65e2c7925c6e57dd6dccdac491cff398cd9be6973f10bfc99ef3eadf31119a96/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/KRBAIQSAN5GK3HE3VQO7YQRQFD:/var/lib/docker/overlay2/l/TG6FNAQWIGZC34CJUBYOYSRHPT:/var/lib/docker/overlay2/l/7USDENUXJEZADV3IAW7UVG6VP6:/var/lib/docker/overlay2/l/OBBEY54IWYSFO4FT5TYVKAW577:/var/lib/docker/overlay2/l/LYLJ5KS2XPVWU7GUB6IVWVTQCK:/var/lib/docker/overlay2/l/R37A6OUKQEAJLP3TGBUS6OCYAG:/var/lib/docker/overlay2/l/BBW456X2X7AYM5VGB5ZXZV37CB:/var/lib/docker/overlay2/l/VNZH6YTVHAFYUYSUIAII4XCC65:/var/lib/docker/overlay2/l/WKPUWB27JTAD65I24JZPAKPGGZ:/var/lib/docker/overlay2/l/AM5Q45AQNAEHR7AGBZRVAIGCB2:/var/lib/docker/overlay2/l/S2VA2DWOICPIKDWORDFRGWRYBZ,upperdir=/var/lib/docker/overlay2/65e2c7925c6e57dd6dccdac491cff398cd9be6973f10bfc99ef3eadf31119a96/diff,workdir=/var/lib/docker/overlay2/65e2c7925c6e57dd6dccdac491cff398cd9be6973f10bfc99ef3eadf31119a96/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/c7f754a92c1c88f4219740cccb8e7472b282a1f348f435987693d1230287528b/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/3JMUDCAWJCBNJA5LV3KLDMS6FR:/var/lib/docker/overlay2/l/67IB4L3DYHQGZLPD2TUCZKVUT7:/var/lib/docker/overlay2/l/IP6D3LG4MQ7FY2Y4KA3SETVTER:/var/lib/docker/overlay2/l/3PPCERSPW3ZHDZKDEEN6RSMGCN:/var/lib/docker/overlay2/l/5FOWJ4AZTJCQNDDM3NIW6JN6OL:/var/lib/docker/overlay2/l/E2PBSUZWGEUOBLXY6B4VFJYRVU:/var/lib/docker/overlay2/l/ODDTWYAIZIWASFPYY2QDXCU42Y:/var/lib/docker/overlay2/l/WW5EMRTB4H6NPUMJK3JJHAUB2G:/var/lib/docker/overlay2/l/HZ2YEQTIT2UOUVZNIBMV35UU2A:/var/lib/docker/overlay2/l/INMEXU5MADRPTQVRJGVK6PJDCI:/var/lib/docker/overlay2/l/HVDMQHAV6H2GTL5AAQLVUWPJEG:/var/lib/docker/overlay2/l/7OP7UVEJR7OVO25Y622VJNCQ7T:/var/lib/docker/overlay2/l/MCAUXPLOBAZ3MR7BMH3QGX4ROQ:/var/lib/docker/overlay2/l/Q4ZCP5NB4XN6UVAWFUF4GVBACR:/var/lib/docker/overlay2/l/LINQA2P57AZ4YPJRDB5ZYVAOPE:/var/lib/docker/overlay2/l/MFDRVVXP5SVJUIFJEYMADQZG4D:/var/lib/docker/overlay2/l/2KTF5V536LYU54N2OWQJISILXT,upperdir=/var/lib/docker/overlay2/c7f754a92c1c88f4219740cccb8e7472b282a1f348f435987693d1230287528b/diff,workdir=/var/lib/docker/overlay2/c7f754a92c1c88f4219740cccb8e7472b282a1f348f435987693d1230287528b/work,nouserxattr
├─/var/lib/docker/overlay2/2a5abf3c69cca9ce5848686cf1d529038e9f782d559446da298887416f09728d/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/HUT3S4TMTCBJNEM2E6GU6MYIOO:/var/lib/docker/overlay2/l/3KDWTZHESNO2WM3NC4LIFLJ3GU:/var/lib/docker/overlay2/l/QK57XUP75CUDZRSOA3OYDV5OFC:/var/lib/docker/overlay2/l/N7Q345EYUDRH6KZ3BWEOBPQP7B:/var/lib/docker/overlay2/l/ONVBTHI5AP62C6EIJIKKXEQWD6:/var/lib/docker/overlay2/l/KWXRHY7ME6UTWD5OQHHLPRFD3Q:/var/lib/docker/overlay2/l/ZIZIYBWWUKN54LSXMITJRGEBPN:/var/lib/docker/overlay2/l/OZDVJNFHZFV3KBNELU7FGSKMDC:/var/lib/docker/overlay2/l/SJCXCSQBMCBMFTZKJJTRFY6XQI:/var/lib/docker/overlay2/l/P43IZ2LZTDQANLPJOPRO4EOQDD:/var/lib/docker/overlay2/l/SDX7RTYJCM22RZ2WN5QQA2BDAU:/var/lib/docker/overlay2/l/H2F2LBJIAQ7OALB7HAOI7GLR3A:/var/lib/docker/overlay2/l/674ZMS2MIZUPVPNQW63QBNX2TN,upperdir=/var/lib/docker/overlay2/2a5abf3c69cca9ce5848686cf1d529038e9f782d559446da298887416f09728d/diff,workdir=/var/lib/docker/overlay2/2a5abf3c69cca9ce5848686cf1d529038e9f782d559446da298887416f09728d/work,nouserxattr
├─/var/lib/docker/overlay2/b9cebc6fa8372bd9ddc1589af6b7c4acf1b1ced41e90a021ddb2ab6a8f5ec52a/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/CFKB7MJIPWCBDMFGJSSYRCD7DM:/var/lib/docker/overlay2/l/KMDHZ5B5QLVL334I72VOBKK3X7:/var/lib/docker/overlay2/l/YFFG3DN4JPOD3D6CIY7O4CSETU:/var/lib/docker/overlay2/l/FJYJWFT2D2IDOAOKQQHGKAPYLN:/var/lib/docker/overlay2/l/MDICZL66T4WV3LAYYZUCFNXZYC:/var/lib/docker/overlay2/l/XUKJ2OYOL2LPEUDZ2MFEX3DJHF:/var/lib/docker/overlay2/l/JJTEU24TZVEHQGZSNS7ZGOSMHS:/var/lib/docker/overlay2/l/ZSD6FLJ3OAAXEIGC7SXO5RWKK2:/var/lib/docker/overlay2/l/6WP5TLGZW7W33IFTHBODAQBTFR:/var/lib/docker/overlay2/l/GKTIVWHXMQZR5IMVKQK64QG77P:/var/lib/docker/overlay2/l/BK4NC6PX6URONG52NDRAFMCGSZ:/var/lib/docker/overlay2/l/7SHDDX7NRD3OL7HZXCATNTIMP3:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/b9cebc6fa8372bd9ddc1589af6b7c4acf1b1ced41e90a021ddb2ab6a8f5ec52a/diff,workdir=/var/lib/docker/overlay2/b9cebc6fa8372bd9ddc1589af6b7c4acf1b1ced41e90a021ddb2ab6a8f5ec52a/work,nouserxattr
├─/var/lib/docker/overlay2/10ab92b190e9fc9b6e49dce9dcc2a96c1fb033b7ae8cae4312269805f042919a/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/G2PUDJIVVRE4JEBPTF2HRP3GBH:/var/lib/docker/overlay2/l/GLUGK5BFWS7S2JACA34E264BCO:/var/lib/docker/overlay2/l/I6XQ25TCIGI42XBO5PRI3EU5PQ:/var/lib/docker/overlay2/l/IS5JM34RY7WGECGKTE3L5J7LFX:/var/lib/docker/overlay2/l/6FNQ4KXTPLKUKOOKVSWI7PJLI4,upperdir=/var/lib/docker/overlay2/10ab92b190e9fc9b6e49dce9dcc2a96c1fb033b7ae8cae4312269805f042919a/diff,workdir=/var/lib/docker/overlay2/10ab92b190e9fc9b6e49dce9dcc2a96c1fb033b7ae8cae4312269805f042919a/work,nouserxattr
├─/mnt/gba-roms                                                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/var/lib/docker/overlay2/e2e355160e6d39a652fef36ef66c2f455d0963e4f364a8701459cd5259ec1f78/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/AWU6TLPVAA6Q5P4SGBRJEMZ54S:/var/lib/docker/overlay2/l/FZSECO3NN5AEPKDYBIZAP6IA4X:/var/lib/docker/overlay2/l/FILAAGY3RZKR2HTPMAB23FPXV6:/var/lib/docker/overlay2/l/NXU5MGYG4335PMODAEWHY7FVN7:/var/lib/docker/overlay2/l/7UZUOPA6DJEBSKDCJXJTUT6MUF:/var/lib/docker/overlay2/l/RIJCP75PXOW47A63XKI4K73UBN:/var/lib/docker/overlay2/l/3CEMQDY6PO355PE2SZGIMRCWZV:/var/lib/docker/overlay2/l/OBWDZ7GP2HO6JNBLZX47BES76E:/var/lib/docker/overlay2/l/6YXN3ULGVN3UBY5TZ2IAYMF4VI:/var/lib/docker/overlay2/l/C55Z4MV3UXXHP42MQBGJ7TSEOT:/var/lib/docker/overlay2/l/LNCBS3QF4DMWCHZB3CCVZFFPG5:/var/lib/docker/overlay2/l/L6OWJM62AE5UBLBTZ3OZP2T3IO,upperdir=/var/lib/docker/overlay2/e2e355160e6d39a652fef36ef66c2f455d0963e4f364a8701459cd5259ec1f78/diff,workdir=/var/lib/docker/overlay2/e2e355160e6d39a652fef36ef66c2f455d0963e4f364a8701459cd5259ec1f78/work,nouserxattr
├─/var/lib/docker/overlay2/df90eaa5221203df13373311e17f6e3457c9c2011a69df40bfed1bcf3119c63b/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/LA6BYWD7ANQYS47H4UKZKCTBUX:/var/lib/docker/overlay2/l/6C6LDDJCSU7HL35PEPUMRSHD7T:/var/lib/docker/overlay2/l/KDBLWC3YTGSVCLU5AECHYFYT3P:/var/lib/docker/overlay2/l/PD2ICIR2XADZKGEFN7EKQATIT7:/var/lib/docker/overlay2/l/LZY3ZXGJ2EMKQF2LKKCNBDJWBD:/var/lib/docker/overlay2/l/VI2LQZNSF2J3MQAYJEHNEJZAUD:/var/lib/docker/overlay2/l/QQVEKB3H52QXP3HX5ZTLEJ3LVO:/var/lib/docker/overlay2/l/PJPDBGRRS6IIVNG7WS6FWPGCE4:/var/lib/docker/overlay2/l/7H2MNG4QPNDD673TP7G4VF7IQO:/var/lib/docker/overlay2/l/FLJ33U7QI3BAG732SLNHV3ZTAI:/var/lib/docker/overlay2/l/D2QOYCHFS5EQRNQDI67DOZJ273:/var/lib/docker/overlay2/l/HS2VL74HMCBZ3BYCDU36UJMB55:/var/lib/docker/overlay2/l/CKOB5LBQR5NQHHDESRBXRS7MFM:/var/lib/docker/overlay2/l/C6CLQHZWSJI7KR56DS3VANBNW3:/var/lib/docker/overlay2/l/EG2HAU74BKJ2JK7QZN2DCRHZJV:/var/lib/docker/overlay2/l/VZHQ3RHEFV4BHEARPBGPTAYQBO,upperdir=/var/lib/docker/overlay2/df90eaa5221203df13373311e17f6e3457c9c2011a69df40bfed1bcf3119c63b/diff,workdir=/var/lib/docker/overlay2/df90eaa5221203df13373311e17f6e3457c9c2011a69df40bfed1bcf3119c63b/work,nouserxattr
├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc1.cdi                                                     altmount[/complete/games/_Dreamcast_Shenmue/cd1/2003-11-23-Shenmue-cd1-pal-DCP/2003-11-23-Shenmue-cd1-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
├─/var/lib/docker/overlay2/860eec1de852d293c73436e889f6aadcfa5064acc49e8a2e11d2dc03f78457ef/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/JWWLHZNZFA2EIFGGXP43W23CJ3:/var/lib/docker/overlay2/l/6WROEH7F7XKAPDKSFG7F4H5KZ4:/var/lib/docker/overlay2/l/DWWCLF2NPST5WMLARQZCMAWBXE:/var/lib/docker/overlay2/l/VING2LDHQL27KX4NKOXFK65LDG:/var/lib/docker/overlay2/l/6FBMEYEROTDRVAM4X7JPZESPJX:/var/lib/docker/overlay2/l/XITBTLWF4IKJ5NYCMRHQNEOV7N,upperdir=/var/lib/docker/overlay2/860eec1de852d293c73436e889f6aadcfa5064acc49e8a2e11d2dc03f78457ef/diff,workdir=/var/lib/docker/overlay2/860eec1de852d293c73436e889f6aadcfa5064acc49e8a2e11d2dc03f78457ef/work,nouserxattr
├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc2.cdi                                                     altmount[/complete/games/_Dreamcast_Shenmue/cd2/2003-11-23-Shenmue-cd2-pal-DCP/2003-11-23-Shenmue-cd2-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
├─/var/lib/docker/overlay2/c3a5d972d36fe5e7a4eefb7c528c4c74a6beb7180b80ed45e7f9f929dc337138/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/F4KX6T6DPHRCL7CRY6ZA7ZNG32:/var/lib/docker/overlay2/l/D73Z6KOTRZQZFAVYCLOT6SX3H6:/var/lib/docker/overlay2/l/YP2PAAECKFDIKHPVQT42NC4XBP:/var/lib/docker/overlay2/l/PORVK3JJC6ZH6PCNBK3NFQZGTN:/var/lib/docker/overlay2/l/XN7WOIZVDXMMAILOOIOSYQALFT:/var/lib/docker/overlay2/l/6EV2QXIEZF2PBBGS23DYCGGDEG:/var/lib/docker/overlay2/l/FA27LDLTWJSRHX5O6OIP4TUKPW:/var/lib/docker/overlay2/l/FTH5TXOLV6U5SJRQHWTQHCFEL6:/var/lib/docker/overlay2/l/6EIFQEOHTEYE45HUUDFDNU5C2T:/var/lib/docker/overlay2/l/PZ66JMPH2LLPKU6TSJZOWO2GOZ,upperdir=/var/lib/docker/overlay2/c3a5d972d36fe5e7a4eefb7c528c4c74a6beb7180b80ed45e7f9f929dc337138/diff,workdir=/var/lib/docker/overlay2/c3a5d972d36fe5e7a4eefb7c528c4c74a6beb7180b80ed45e7f9f929dc337138/work,nouserxattr
├─/mnt/x68000-roms                                                                                 FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/var/lib/docker/overlay2/32a0adb47576ea356fded3a4e99fcb3ba920a91bb681689bf701f10c261505de/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/4TDEMPLWQN23M53NKQQEBLNEDE:/var/lib/docker/overlay2/l/IKDJXP5KSJMSFOXJVSUMZGTHVY:/var/lib/docker/overlay2/l/VHLDXVOF2J3RJYTXX4AE26NA7L:/var/lib/docker/overlay2/l/WI2O3M63PLDH3BVPDRMM3GSJSV:/var/lib/docker/overlay2/l/7UTEP2RH2SIGCXF3EG4M2SB267,upperdir=/var/lib/docker/overlay2/32a0adb47576ea356fded3a4e99fcb3ba920a91bb681689bf701f10c261505de/diff,workdir=/var/lib/docker/overlay2/32a0adb47576ea356fded3a4e99fcb3ba920a91bb681689bf701f10c261505de/work,uuid=null,nouserxattr
├─/mnt/games/roms/dc/Shenmue/Shenmue_Disc3.cdi                                                     altmount[/complete/games/_Dreamcast_Shenmue/cd3/2003-11-23-Shenmue-cd3-pal-DCP/2003-11-23-Shenmue-cd3-pal-DCP.cdi] fuse.altmount     rw,nosuid,nodev,relatime,user_id=1000,group_id=1000,allow_other,max_read=1048576
├─/var/lib/docker/overlay2/f87b9dbf59cddf3c8097fb8953760a14c5adaeb9fe35b987e62e40d562dc7c15/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/XEXOXLIKD7BNZHKD4BBHZOJT4K:/var/lib/docker/overlay2/l/25IB32IYO4AUTUJ7O3VHQDE4XV:/var/lib/docker/overlay2/l/ETK7Y3G24VYW6FACLUBQ4LMIZE:/var/lib/docker/overlay2/l/37PY64N3V2NS5J57AOONLAZUW5:/var/lib/docker/overlay2/l/C27A4EW3DYBZ5FULK725BN2IHB:/var/lib/docker/overlay2/l/SWTL4JKB7FXCYDPC5IWEUGGU3L:/var/lib/docker/overlay2/l/DTYWVQ6W4X2BTNPQKLXXYPCLSK:/var/lib/docker/overlay2/l/OG2PR4KVIM3B2ZVE4PVJMELTB6:/var/lib/docker/overlay2/l/2ZYJQ2N7ACVKWM3IEEGA4Z7YU3:/var/lib/docker/overlay2/l/VGJ4ANI5QWBGBKHM3PID3O26DT:/var/lib/docker/overlay2/l/B3EEPZOLZ6S23ZKI2LLPNE7ZD7:/var/lib/docker/overlay2/l/FGZXMQUYVGHTAQUVFC4LZECY22:/var/lib/docker/overlay2/l/HG5ZBQXD4B55AD7LYOOSRSPL6S:/var/lib/docker/overlay2/l/2XOEY3VH7RW3CDL6VM3YPYVMK6,upperdir=/var/lib/docker/overlay2/f87b9dbf59cddf3c8097fb8953760a14c5adaeb9fe35b987e62e40d562dc7c15/diff,workdir=/var/lib/docker/overlay2/f87b9dbf59cddf3c8097fb8953760a14c5adaeb9fe35b987e62e40d562dc7c15/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/e5a0c2a3ee2289ddc52ef3eaadcd533c1dd0852c9c4c2b9297b81f645d50105a/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/W4TRLYQCRRCXBEXSQ7GKPTIF42:/var/lib/docker/overlay2/l/NTAARLIXBAUP7ZFULRN24GIV6F:/var/lib/docker/overlay2/l/N7RPQFXGPRPOICO22EFBU2OBG6:/var/lib/docker/overlay2/l/ELLOGHXQM5N7T5B2ZC2C54L2ZF:/var/lib/docker/overlay2/l/AJHX5N6ZRDK725XN5HRQP4EEID:/var/lib/docker/overlay2/l/EGDM6D6QYYVGL2CBX337NKHIZN:/var/lib/docker/overlay2/l/WKSHNMA4XKYSY6E4PNCLP7PSWY:/var/lib/docker/overlay2/l/7MER7NVUG2R23TNQTBSYAW75GW:/var/lib/docker/overlay2/l/3U5OOQIX6UWTD5BXIQ4QAYOEW2:/var/lib/docker/overlay2/l/RYAHHTUOFD6U7YLI5TZEWRBD6K:/var/lib/docker/overlay2/l/WZPOI6SKANJZCMCRG3QCWGZJDX,upperdir=/var/lib/docker/overlay2/e5a0c2a3ee2289ddc52ef3eaadcd533c1dd0852c9c4c2b9297b81f645d50105a/diff,workdir=/var/lib/docker/overlay2/e5a0c2a3ee2289ddc52ef3eaadcd533c1dd0852c9c4c2b9297b81f645d50105a/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/29da7b401bcc4b4c49a0e863d8ac7edc7cd8944912a6ab45286b050ea985ebf1/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/GGAPJ2HTR2XW46MS5DPBATC7C7:/var/lib/docker/overlay2/l/Z5NJVBYFGUO3PFZW7JQEVFR3AA:/var/lib/docker/overlay2/l/3CHVRY5ZH3K4VQBGGAAJMWHRH5:/var/lib/docker/overlay2/l/VSIXWA44JVBYBBN67E6MXTHJRB:/var/lib/docker/overlay2/l/ROL6VEGHZU53DSMVCJYLHKQOWN:/var/lib/docker/overlay2/l/AJEA3ZX7CFLG4RIUU66LV4MO6K,upperdir=/var/lib/docker/overlay2/29da7b401bcc4b4c49a0e863d8ac7edc7cd8944912a6ab45286b050ea985ebf1/diff,workdir=/var/lib/docker/overlay2/29da7b401bcc4b4c49a0e863d8ac7edc7cd8944912a6ab45286b050ea985ebf1/work,nouserxattr
├─/var/lib/docker/overlay2/0587845d5aaa8d37015065bf87a379d66623c0f2cb46c2bb8f853a75acf9afec/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/JMR2SQVQVN3BINVAU4ARKIRSDK:/var/lib/docker/overlay2/l/J66D7BN55DDAR67NSP2WCR2SYZ:/var/lib/docker/overlay2/l/Y6RFR47YQEG3MMFQ7CCOT5NV7U:/var/lib/docker/overlay2/l/ZZ7D4SOIM55K4QAZQP3USWHJLZ,upperdir=/var/lib/docker/overlay2/0587845d5aaa8d37015065bf87a379d66623c0f2cb46c2bb8f853a75acf9afec/diff,workdir=/var/lib/docker/overlay2/0587845d5aaa8d37015065bf87a379d66623c0f2cb46c2bb8f853a75acf9afec/work,nouserxattr
├─/mnt/psx-roms                                                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/var/lib/docker/overlay2/b9b1de73f1437d25660143500eedae7381383fc00baab4b233ebd5a674b2fcb9/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/GAF2RIZ4OEEH6SHBE3N5SU53DM:/var/lib/docker/overlay2/l/WTWNN6K2JGXJL5IOSEH47ULQQA:/var/lib/docker/overlay2/l/2VMQWGSQH5YHORQNDQX7I4QXRT:/var/lib/docker/overlay2/l/3NQZQ3I5GBNYK3RS6NOZGZYPUR:/var/lib/docker/overlay2/l/H2HYVXO6XRKMUUYBADA5ZYRCSS:/var/lib/docker/overlay2/l/SIVUOKVYG65TSUG4GW5SYNKEXK:/var/lib/docker/overlay2/l/IZ2E34IFNFUMVOIU7I67F6IURE:/var/lib/docker/overlay2/l/CBRUGQJXNVN2AOKCMXXZAYLCNA:/var/lib/docker/overlay2/l/UARDH3ESPBHRSIDDTHWPRHM32J:/var/lib/docker/overlay2/l/JWTRAOVMGKXQJ67YTFBGHEZZDW:/var/lib/docker/overlay2/l/42MLHT4XFQCR4O67IKYOLRNQ6F:/var/lib/docker/overlay2/l/4SHSXWMXDII4F27RPKEEJBQK4F:/var/lib/docker/overlay2/l/VZTVQ37BKTA5LYP6Y4TOJMOHNM:/var/lib/docker/overlay2/l/5FVOLRVP77J226IYA4USXTVJAU:/var/lib/docker/overlay2/l/JAIURHZC3Z4TNC7MYBNYTJQMWQ:/var/lib/docker/overlay2/l/V7OZTOSV6ISPKXDP7COTBDHNEO:/var/lib/docker/overlay2/l/PQIF27HA4KQAN7SLSRXYTR5ESP:/var/lib/docker/overlay2/l/QJ4WN64S6UXMNKQAOIAWEQ4YMK,upperdir=/var/lib/docker/overlay2/b9b1de73f1437d25660143500eedae7381383fc00baab4b233ebd5a674b2fcb9/diff,workdir=/var/lib/docker/overlay2/b9b1de73f1437d25660143500eedae7381383fc00baab4b233ebd5a674b2fcb9/work,nouserxattr
├─/var/lib/docker/overlay2/fad67cad24f45bd6238796d91be267e134ddd7bb47670b6a9f92bca2572763a9/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/KNLOUHISOUFRSBMHME4GIKDUCL:/var/lib/docker/overlay2/l/PP4KJSLLFRG6LWS4GVAQROJADN:/var/lib/docker/overlay2/l/UZEWFAXAFTXIGDJUEOKCREUENX:/var/lib/docker/overlay2/l/6TYWUEJJVAG7YYL77THN3ESTF3:/var/lib/docker/overlay2/l/JHBJD5L3RDPCDQUFOKGTM7DUPW:/var/lib/docker/overlay2/l/N7SHOGH2SWYYTLGEHOZBYPNKEX:/var/lib/docker/overlay2/l/X45RWUUTKLFW4UPUZ7DEI7WXUW,upperdir=/var/lib/docker/overlay2/fad67cad24f45bd6238796d91be267e134ddd7bb47670b6a9f92bca2572763a9/diff,workdir=/var/lib/docker/overlay2/fad67cad24f45bd6238796d91be267e134ddd7bb47670b6a9f92bca2572763a9/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/482f851a0fc22f102a7a48408da5dd90c16495b60a5ad2182dedd5f1622d2396/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/GV6OCV43XAPPGDR3V2W4O7NKJS:/var/lib/docker/overlay2/l/3ONEQH2PMIN74MGNAIZX6ZJA2A:/var/lib/docker/overlay2/l/WEYI6YM2V5GXHF72FVD7332OMB:/var/lib/docker/overlay2/l/CT2CXZLTVMDWQNN4LEFBCLOMSY:/var/lib/docker/overlay2/l/TQRSJN22QZ4XPNRML5EYMDXZP3:/var/lib/docker/overlay2/l/7NNNMYL2PE5BZRZCBLBIMN5PZ6:/var/lib/docker/overlay2/l/PV5G7WFNTV74N5OTBW5JRJWZNN:/var/lib/docker/overlay2/l/LC6YFS2KIJACRKPJCJXUEGPOGU:/var/lib/docker/overlay2/l/SLZ3MJ5YPSE7TBEIHLLNQ7MGQJ:/var/lib/docker/overlay2/l/GNUVUV5TBCT6MP2MVPKGUILL76:/var/lib/docker/overlay2/l/QLDK6LFTIFVIOV5FU67IXM3VOK:/var/lib/docker/overlay2/l/S6532PBVSAYTIYROWPFTRFXJEA:/var/lib/docker/overlay2/l/BBSL3Y6YGJDN4RO54E74DU6JRB:/var/lib/docker/overlay2/l/GFVKU5OY57I4QRVR5E7HHE4W2L:/var/lib/docker/overlay2/l/AFH32IDFYHXF2WY7NNV2W5J3UD:/var/lib/docker/overlay2/l/PVT7WDLZO3KXKJZILY6KEUEPWM:/var/lib/docker/overlay2/l/DKX2FD5JRJIAMHSHRXSAAESH4B:/var/lib/docker/overlay2/l/TU7WB7D6DSTFJQIGFWJAGR7NV3:/var/lib/docker/overlay2/l/CO4BUDF4O6TR5HI2AOZYA4YT75:/var/lib/docker/overlay2/l/3NZ5DOPTZKLNWVMLZVMSS2UWOI,upperdir=/var/lib/docker/overlay2/482f851a0fc22f102a7a48408da5dd90c16495b60a5ad2182dedd5f1622d2396/diff,workdir=/var/lib/docker/overlay2/482f851a0fc22f102a7a48408da5dd90c16495b60a5ad2182dedd5f1622d2396/work,uuid=null,nouserxattr
├─/var/lib/docker/overlay2/146a29174a0df2fad9d8f7495550833d6684a4c56a620b6133092fe8e9d1b961/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/2DGWLYCRCMJBKWDT4DUDWKK4UM:/var/lib/docker/overlay2/l/UQG46C254T4S6EQJPLJRQE3QQY:/var/lib/docker/overlay2/l/DREF5CUCOYSR6ZMVRIOOQUKFNL:/var/lib/docker/overlay2/l/247T3ZP2MD4LATF3WU52ZK6AQH:/var/lib/docker/overlay2/l/2SGNZ3KTI5RIN2LKOTZWKTUWIB:/var/lib/docker/overlay2/l/UE7AXQ46X5TSZIXEQNHDBVL7YA:/var/lib/docker/overlay2/l/YDRSSVS5AHEKCRGQ7ZQTSHIWT3:/var/lib/docker/overlay2/l/K2H3AXIQUAJDNT4K65EVZZIPRI:/var/lib/docker/overlay2/l/OH4PLAWPABYVATSLG4P7BI5KOR:/var/lib/docker/overlay2/l/TZVXY3U7AZS2HIKUGO6KZTLLS4:/var/lib/docker/overlay2/l/NPTHMVH5LD35EY5DB36SBTXCZ5:/var/lib/docker/overlay2/l/KGT6QDAUYJ3UN3TJPUPPMTV3OG:/var/lib/docker/overlay2/l/KFC2MBCVHDPGNTVDLF3YZPJWOE:/var/lib/docker/overlay2/l/4Q4YK4UUHTF5DTC5FKAXS7GBQI:/var/lib/docker/overlay2/l/LQG4G4TXSXSLKLAW2TVNYRKSDZ:/var/lib/docker/overlay2/l/PNXUCFAEOT4WXFSMI76VJ5G77Y:/var/lib/docker/overlay2/l/77VX2TTBDBYLMVZ5PW2Y7FJJDK:/var/lib/docker/overlay2/l/W5RI6NVGX5N4OO3A7FDB74OOCK:/var/lib/docker/overlay2/l/TAFMX5WL574YYF7TGICYWANOAL:/var/lib/docker/overlay2/l/VBESMYGMD3GJQTIX5KEO63LTHZ:/var/lib/docker/overlay2/l/6TDDZBM2O7AFJNBTHQONNM5QZQ:/var/lib/docker/overlay2/l/RVETQW2O6KPI7F7S7FEJMAP4C3,upperdir=/var/lib/docker/overlay2/146a29174a0df2fad9d8f7495550833d6684a4c56a620b6133092fe8e9d1b961/diff,workdir=/var/lib/docker/overlay2/146a29174a0df2fad9d8f7495550833d6684a4c56a620b6133092fe8e9d1b961/work,nouserxattr
├─/var/lib/docker/overlay2/dd64b31bc53c8c6f17528a783fa79c34734847b0b2765186118afa4431d7ac0c/merged overlay                                                                                                            overlay           rw,relatime,lowerdir=/var/lib/docker/overlay2/l/5XDDF523YTVQ33V5BTOLL2X6VN:/var/lib/docker/overlay2/l/TH3ZNHE45HGEKCSGEHB5QXROET:/var/lib/docker/overlay2/l/7TJTKVFFKCQM2HH6JLNNWX2DYJ:/var/lib/docker/overlay2/l/QP7T2L5MDHKPIWCV3GYNS4UD6Q:/var/lib/docker/overlay2/l/3DKMO5DNWIZBLMPDDT23GMSUIO,upperdir=/var/lib/docker/overlay2/dd64b31bc53c8c6f17528a783fa79c34734847b0b2765186118afa4431d7ac0c/diff,workdir=/var/lib/docker/overlay2/dd64b31bc53c8c6f17528a783fa79c34734847b0b2765186118afa4431d7ac0c/work,nouserxattr
├─/mnt/ngc-roms/Baten Kaitos Origins (USA) (Disc 1)                                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Madden NFL 2005 (USA)                                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Metal Gear Solid - The Twin Snakes (USA) (Disc 2)                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/ESPN International Winter Sports 2002 (USA)                                        FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Tiger Woods PGA Tour 2004 (USA) (Disc 1)                                           FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/NBA Live 2004 (USA)                                                                FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/NCAA Football 2003 (USA)                                                           FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/NBA 2K2 (USA)                                                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Metroid Prime 2 - Echoes (USA)                                                     FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/F1 2002 (USA) (En,Fr,De,It)                                                        FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Nickelodeon Tak 2 - The Staff of Dreams (USA)                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/NASCAR 2005 - Chase for the Cup (USA)                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/MLB SlugFest 2003 (USA)                                                            FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Dakar 2 - The World's Ultimate Rally (USA) (En,Fr,De,Es,It)                        FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Tony Hawk's Pro Skater 4 (USA)                                                     FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc Version 11 (USA)                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Digimon World 4 (USA)                                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc - January 2002 (USA)                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Resident Evil Zero (USA) (Disc 1)                                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Advance Game Port (USA) (Unl) (Rev 1)                                              FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Rayman 3 - Hoodlum Havoc (USA) (En,Fr,De,Es,It)                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Kao the Kangaroo - Round 2 (USA)                                                   FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Tom Clancy's Rainbow Six 3 (USA)                                                   FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Resident Evil 4 (USA) (Disc 1)                                                     FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/FIFA Street 2 (USA) (En,Es)                                                        FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Knockout Kings 2003 (USA)                                                          FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Sims 2, The - Pets (USA)                                                           FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/4x4 Evo 2 (USA)                                                                    FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
├─/mnt/ngc-roms/Interactive Multi-Game Demo Disc Version 30 (USA)                                  FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
└─/mnt/ngc-roms/Mat Hoffman's Pro BMX 2 (USA)                                                      FuseMount                                                                                                          fuse              rw,nosuid,nodev,relatime,user_id=1000,group_id=1000
```
## Broken Mount Clues
```text
/sys/fs/fuse/connections fusectl
/run/user/1000/gvfs fuse.gvfsd-fuse
/run/user/1000/doc fuse.portal
/mnt/remote/google fuse.rclone
/mnt/remote/google fuse.rclone
/mnt/remote/google fuse.rclone
/mnt/remote/google fuse.rclone
/mnt/unionfs fuse.mergerfs
/mnt/unionfs fuse.mergerfs
/var/lib/docker/overlay2/0eec312571706d67539df38f7e400dfdccbfd416b317be94ae4b67aad9d5f42e/merged overlay
/var/lib/docker/overlay2/5133d8b6cd815b99be6dc93dbbc572d5388814f02ac2200d3cadf628789331cb/merged overlay
/var/lib/docker/overlay2/71629f39fdc313fbdb7250c973b59f4621d450485c38c17356b535fe9ed0069c/merged overlay
/var/lib/docker/overlay2/b9642abe0e0c7b62d420538f7c8393cdc060ff1d1ef3825eb337bce53af35ad3/merged overlay
/var/lib/docker/overlay2/6898cd17dc1fd9a8adb22b054bad8aafe75889a77c3d09e889ca6b44069e4624/merged overlay
/var/lib/docker/overlay2/65e2c7925c6e57dd6dccdac491cff398cd9be6973f10bfc99ef3eadf31119a96/merged overlay
/var/lib/docker/overlay2/10ab92b190e9fc9b6e49dce9dcc2a96c1fb033b7ae8cae4312269805f042919a/merged overlay
/run/docker/netns/60dcbaa7ba3d nsfs
/var/lib/docker/overlay2/860eec1de852d293c73436e889f6aadcfa5064acc49e8a2e11d2dc03f78457ef/merged overlay
/var/lib/docker/overlay2/c3a5d972d36fe5e7a4eefb7c528c4c74a6beb7180b80ed45e7f9f929dc337138/merged overlay
/var/lib/docker/overlay2/f87b9dbf59cddf3c8097fb8953760a14c5adaeb9fe35b987e62e40d562dc7c15/merged overlay
/run/docker/netns/02dc9923b31a nsfs
/var/lib/docker/overlay2/29da7b401bcc4b4c49a0e863d8ac7edc7cd8944912a6ab45286b050ea985ebf1/merged overlay
/var/lib/docker/overlay2/e5a0c2a3ee2289ddc52ef3eaadcd533c1dd0852c9c4c2b9297b81f645d50105a/merged overlay
/mnt/games/roms/dc/Shenmue/Shenmue_Disc1.cdi fuse.altmount
/mnt/games/roms/dc/Shenmue/Shenmue_Disc1.cdi fuse.altmount
/mnt/games/roms/dc/Shenmue/Shenmue_Disc2.cdi fuse.altmount
/mnt/games/roms/dc/Shenmue/Shenmue_Disc2.cdi fuse.altmount
/mnt/games/roms/dc/Shenmue/Shenmue_Disc3.cdi fuse.altmount
/mnt/games/roms/dc/Shenmue/Shenmue_Disc3.cdi fuse.altmount
/tmp/.mount_SRM.ApBATqYW fuse.SRM.AppImage
/var/lib/docker/overlay2/7f746301fcc65d17447f248dc40d559b4c1f9bff4eae3e996a31ddc3b9d5a44f/merged overlay
/var/lib/docker/overlay2/b9cebc6fa8372bd9ddc1589af6b7c4acf1b1ced41e90a021ddb2ab6a8f5ec52a/merged overlay
/var/lib/docker/overlay2/6f7fecd4918ff47212c0771f0ac59691a7a946adc84db04d15e6ad5ad9626956/merged overlay
/mnt/ngc-roms/Baten\x20Kaitos\x20Origins\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Baten\x20Kaitos\x20Origins\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Madden\x20NFL\x202005\x20(USA) fuse
/mnt/ngc-roms/Madden\x20NFL\x202005\x20(USA) fuse
/mnt/ngc-roms/Metal\x20Gear\x20Solid\x20-\x20The\x20Twin\x20Snakes\x20(USA)\x20(Disc\x202) fuse
/mnt/ngc-roms/Metal\x20Gear\x20Solid\x20-\x20The\x20Twin\x20Snakes\x20(USA)\x20(Disc\x202) fuse
/mnt/ngc-roms/ESPN\x20International\x20Winter\x20Sports\x202002\x20(USA) fuse
/mnt/ngc-roms/ESPN\x20International\x20Winter\x20Sports\x202002\x20(USA) fuse
/mnt/ngc-roms/Tiger\x20Woods\x20PGA\x20Tour\x202004\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Tiger\x20Woods\x20PGA\x20Tour\x202004\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/NBA\x20Live\x202004\x20(USA) fuse
/mnt/ngc-roms/NBA\x20Live\x202004\x20(USA) fuse
/mnt/ngc-roms/NCAA\x20Football\x202003\x20(USA) fuse
/mnt/ngc-roms/NCAA\x20Football\x202003\x20(USA) fuse
/mnt/ngc-roms/Metroid\x20Prime\x202\x20-\x20Echoes\x20(USA) fuse
/mnt/ngc-roms/Metroid\x20Prime\x202\x20-\x20Echoes\x20(USA) fuse
/mnt/ngc-roms/NBA\x202K2\x20(USA) fuse
/mnt/ngc-roms/NBA\x202K2\x20(USA) fuse
/mnt/ngc-roms/F1\x202002\x20(USA)\x20(En,Fr,De,It) fuse
/mnt/ngc-roms/F1\x202002\x20(USA)\x20(En,Fr,De,It) fuse
/mnt/ngc-roms/Nickelodeon\x20Tak\x202\x20-\x20The\x20Staff\x20of\x20Dreams\x20(USA) fuse
/mnt/ngc-roms/Nickelodeon\x20Tak\x202\x20-\x20The\x20Staff\x20of\x20Dreams\x20(USA) fuse
/mnt/ngc-roms/NASCAR\x202005\x20-\x20Chase\x20for\x20the\x20Cup\x20(USA) fuse
/mnt/ngc-roms/NASCAR\x202005\x20-\x20Chase\x20for\x20the\x20Cup\x20(USA) fuse
/mnt/ngc-roms/MLB\x20SlugFest\x202003\x20(USA) fuse
/mnt/ngc-roms/MLB\x20SlugFest\x202003\x20(USA) fuse
/mnt/ngc-roms/Dakar\x202\x20-\x20The\x20World's\x20Ultimate\x20Rally\x20(USA)\x20(En,Fr,De,Es,It) fuse
/mnt/ngc-roms/Dakar\x202\x20-\x20The\x20World's\x20Ultimate\x20Rally\x20(USA)\x20(En,Fr,De,Es,It) fuse
/mnt/ngc-roms/Tony\x20Hawk's\x20Pro\x20Skater\x204\x20(USA) fuse
/mnt/ngc-roms/Tony\x20Hawk's\x20Pro\x20Skater\x204\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20Version\x2011\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20Version\x2011\x20(USA) fuse
/mnt/ngc-roms/Digimon\x20World\x204\x20(USA) fuse
/mnt/ngc-roms/Digimon\x20World\x204\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20-\x20January\x202002\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20-\x20January\x202002\x20(USA) fuse
/mnt/ngc-roms/Resident\x20Evil\x20Zero\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Resident\x20Evil\x20Zero\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Advance\x20Game\x20Port\x20(USA)\x20(Unl)\x20(Rev\x201) fuse
/mnt/ngc-roms/Advance\x20Game\x20Port\x20(USA)\x20(Unl)\x20(Rev\x201) fuse
/mnt/ngc-roms/Rayman\x203\x20-\x20Hoodlum\x20Havoc\x20(USA)\x20(En,Fr,De,Es,It) fuse
/mnt/ngc-roms/Rayman\x203\x20-\x20Hoodlum\x20Havoc\x20(USA)\x20(En,Fr,De,Es,It) fuse
/mnt/ngc-roms/Kao\x20the\x20Kangaroo\x20-\x20Round\x202\x20(USA) fuse
/mnt/ngc-roms/Kao\x20the\x20Kangaroo\x20-\x20Round\x202\x20(USA) fuse
/mnt/ngc-roms/Tom\x20Clancy's\x20Rainbow\x20Six\x203\x20(USA) fuse
/mnt/ngc-roms/Tom\x20Clancy's\x20Rainbow\x20Six\x203\x20(USA) fuse
/mnt/ngc-roms/Resident\x20Evil\x204\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/Resident\x20Evil\x204\x20(USA)\x20(Disc\x201) fuse
/mnt/ngc-roms/FIFA\x20Street\x202\x20(USA)\x20(En,Es) fuse
/mnt/ngc-roms/FIFA\x20Street\x202\x20(USA)\x20(En,Es) fuse
/mnt/ngc-roms/Knockout\x20Kings\x202003\x20(USA) fuse
/mnt/ngc-roms/Knockout\x20Kings\x202003\x20(USA) fuse
/mnt/ngc-roms/Sims\x202,\x20The\x20-\x20Pets\x20(USA) fuse
/mnt/ngc-roms/Sims\x202,\x20The\x20-\x20Pets\x20(USA) fuse
/mnt/ngc-roms/4x4\x20Evo\x202\x20(USA) fuse
/mnt/ngc-roms/4x4\x20Evo\x202\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20Version\x2030\x20(USA) fuse
/mnt/ngc-roms/Interactive\x20Multi-Game\x20Demo\x20Disc\x20Version\x2030\x20(USA) fuse
/mnt/ngc-roms/Mat\x20Hoffman's\x20Pro\x20BMX\x202\x20(USA) fuse
/mnt/ngc-roms/Mat\x20Hoffman's\x20Pro\x20BMX\x202\x20(USA) fuse
/mnt/x32-roms fuse
/mnt/x32-roms fuse
/mnt/saturn-roms fuse
/mnt/saturn-roms fuse
/mnt/gba-roms fuse
/mnt/gba-roms fuse
/mnt/x68000-roms fuse
/mnt/x68000-roms fuse
/mnt/psx-roms fuse
/mnt/psx-roms fuse
/mnt/altmount/altmount fuse.altmount
/mnt/nvme2/altmount-mnt/altmount fuse.altmount
/mnt/nvme2/altmount-mnt/altmount fuse.altmount
/mnt/altmount/altmount fuse.altmount
```
## Network
```text
257: vethd786271@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether c2:fa:b8:cb:39:b2 brd ff:ff:ff:ff:ff:ff link-netnsid 9
    inet6 fe80::c0fa:b8ff:fecb:39b2/64 scope link 
       valid_lft forever preferred_lft forever
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host noprefixroute 
       valid_lft forever preferred_lft forever
2: enp6s18: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP group default qlen 1000
    link/ether bc:24:11:a9:1c:c1 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.78/24 brd 192.168.1.255 scope global enp6s18
       valid_lft forever preferred_lft forever
    inet6 fdb2:74b2:6d6d:dba2:be24:11ff:fea9:1cc1/64 scope global dynamic mngtmpaddr noprefixroute 
       valid_lft 1757sec preferred_lft 1757sec
    inet6 2a10:d585:76ae::1/64 scope global 
       valid_lft forever preferred_lft forever
    inet6 fe80::be24:11ff:fea9:1cc1/64 scope link 
       valid_lft forever preferred_lft forever
259: veth7f48ab5@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 7e:15:53:3e:95:92 brd ff:ff:ff:ff:ff:ff link-netnsid 30
    inet6 fe80::7c15:53ff:fe3e:9592/64 scope link 
       valid_lft forever preferred_lft forever
3: br-930dcf26a623: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 0a:31:33:90:bb:7d brd ff:ff:ff:ff:ff:ff
    inet 172.20.0.1/16 brd 172.20.255.255 scope global br-930dcf26a623
       valid_lft forever preferred_lft forever
    inet6 fe80::831:33ff:fe90:bb7d/64 scope link 
       valid_lft forever preferred_lft forever
260: vethcfa8507@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 0a:8d:8c:e2:d7:8d brd ff:ff:ff:ff:ff:ff link-netnsid 37
    inet6 fe80::f49c:5ff:fee9:62f2/64 scope link 
       valid_lft forever preferred_lft forever
4: br-ae22bd8e7962: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether d6:83:d3:a3:9b:15 brd ff:ff:ff:ff:ff:ff
    inet 172.22.0.1/16 brd 172.22.255.255 scope global br-ae22bd8e7962
       valid_lft forever preferred_lft forever
    inet6 fe80::d483:d3ff:fea3:9b15/64 scope link 
       valid_lft forever preferred_lft forever
5: br-bde86ecffe2e: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether ae:6f:52:06:27:36 brd ff:ff:ff:ff:ff:ff
    inet 172.18.0.1/16 brd 172.18.255.255 scope global br-bde86ecffe2e
       valid_lft forever preferred_lft forever
    inet6 fe80::ac6f:52ff:fe06:2736/64 scope link 
       valid_lft forever preferred_lft forever
262: vetha3dc446@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 2e:eb:be:b1:4a:00 brd ff:ff:ff:ff:ff:ff link-netnsid 14
    inet6 fe80::2ceb:beff:feb1:4a00/64 scope link 
       valid_lft forever preferred_lft forever
6: br-c5e4c202334a: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 6e:72:24:92:f6:82 brd ff:ff:ff:ff:ff:ff
    inet 172.21.0.1/16 brd 172.21.255.255 scope global br-c5e4c202334a
       valid_lft forever preferred_lft forever
    inet6 fe80::6c72:24ff:fe92:f682/64 scope link 
       valid_lft forever preferred_lft forever
7: docker0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether 56:d5:34:2d:27:d5 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0
       valid_lft forever preferred_lft forever
    inet6 fd00:d0c::1/80 scope global nodad 
       valid_lft forever preferred_lft forever
8: br-769a326cf30d: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 0e:e6:1b:40:a9:52 brd ff:ff:ff:ff:ff:ff
    inet 172.23.0.1/16 brd 172.23.255.255 scope global br-769a326cf30d
       valid_lft forever preferred_lft forever
    inet6 fe80::ce6:1bff:fe40:a952/64 scope link 
       valid_lft forever preferred_lft forever
265: veth464469f@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether a6:27:f2:ae:c9:db brd ff:ff:ff:ff:ff:ff link-netnsid 4
    inet6 fe80::a427:f2ff:feae:c9db/64 scope link 
       valid_lft forever preferred_lft forever
9: br-8a872bb56623: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 32:c3:4b:94:97:c7 brd ff:ff:ff:ff:ff:ff
    inet 172.19.0.1/16 brd 172.19.255.255 scope global br-8a872bb56623
       valid_lft forever preferred_lft forever
    inet6 fe80::30c3:4bff:fe94:97c7/64 scope link 
       valid_lft forever preferred_lft forever
266: vetha302e6d@if3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-769a326cf30d state UP group default 
    link/ether 72:c4:24:68:c1:b7 brd ff:ff:ff:ff:ff:ff link-netnsid 4
    inet6 fe80::70c4:24ff:fe68:c1b7/64 scope link 
       valid_lft forever preferred_lft forever
10: vethca2fd05@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 0a:20:e3:c1:97:96 brd ff:ff:ff:ff:ff:ff link-netnsid 0
    inet6 fe80::820:e3ff:fec1:9796/64 scope link 
       valid_lft forever preferred_lft forever
11: veth1be4e30@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-769a326cf30d state UP group default 
    link/ether 4e:3d:13:69:29:63 brd ff:ff:ff:ff:ff:ff link-netnsid 1
    inet6 fe80::4c3d:13ff:fe69:2963/64 scope link 
       valid_lft forever preferred_lft forever
268: veth535e684@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether e6:2b:1c:6c:55:94 brd ff:ff:ff:ff:ff:ff link-netnsid 26
    inet6 fe80::e42b:1cff:fe6c:5594/64 scope link 
       valid_lft forever preferred_lft forever
12: veth78969de@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-930dcf26a623 state UP group default 
    link/ether 22:32:d6:da:91:00 brd ff:ff:ff:ff:ff:ff link-netnsid 2
    inet6 fe80::2032:d6ff:feda:9100/64 scope link 
       valid_lft forever preferred_lft forever
13: veth1a983e3@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether f2:31:8a:69:86:ad brd ff:ff:ff:ff:ff:ff link-netnsid 3
    inet6 fe80::f031:8aff:fe69:86ad/64 scope link 
       valid_lft forever preferred_lft forever
270: vethc94a5a1@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 8a:83:85:b6:1f:58 brd ff:ff:ff:ff:ff:ff link-netnsid 39
    inet6 fe80::8883:85ff:feb6:1f58/64 scope link 
       valid_lft forever preferred_lft forever
15: veth9077b95@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-ae22bd8e7962 state UP group default 
    link/ether ee:34:7f:3f:69:58 brd ff:ff:ff:ff:ff:ff link-netnsid 5
    inet6 fe80::ec34:7fff:fe3f:6958/64 scope link 
       valid_lft forever preferred_lft forever
272: vethdefd4a5@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 26:ff:0a:ed:15:9a brd ff:ff:ff:ff:ff:ff link-netnsid 15
    inet6 fe80::24ff:aff:feed:159a/64 scope link 
       valid_lft forever preferred_lft forever
16: veth366d1a6@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether ee:32:e8:6b:e0:8f brd ff:ff:ff:ff:ff:ff link-netnsid 6
    inet6 fe80::ec32:e8ff:fe6b:e08f/64 scope link 
       valid_lft forever preferred_lft forever
17: veth60bbdd7@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-c5e4c202334a state UP group default 
    link/ether 0a:13:a0:2d:0c:5e brd ff:ff:ff:ff:ff:ff link-netnsid 7
    inet6 fe80::813:a0ff:fe2d:c5e/64 scope link 
       valid_lft forever preferred_lft forever
20: vethef9a1ad@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 1e:76:f9:d8:48:39 brd ff:ff:ff:ff:ff:ff link-netnsid 10
    inet6 fe80::1c76:f9ff:fed8:4839/64 scope link 
       valid_lft forever preferred_lft forever
22: vethbe45279@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether e6:2b:67:e9:4a:ee brd ff:ff:ff:ff:ff:ff link-netnsid 12
    inet6 fe80::e42b:67ff:fee9:4aee/64 scope link 
       valid_lft forever preferred_lft forever
23: veth0bf6b4b@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 6e:0a:36:10:5d:cd brd ff:ff:ff:ff:ff:ff link-netnsid 13
    inet6 fe80::6c0a:36ff:fe10:5dcd/64 scope link 
       valid_lft forever preferred_lft forever
26: veth932b192@if3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-ae22bd8e7962 state UP group default 
    link/ether ba:bd:1f:d6:c5:bd brd ff:ff:ff:ff:ff:ff link-netnsid 13
    inet6 fe80::b8bd:1fff:fed6:c5bd/64 scope link 
       valid_lft forever preferred_lft forever
30: vethf33ec3d@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 7a:c1:ca:da:65:7a brd ff:ff:ff:ff:ff:ff link-netnsid 16
    inet6 fe80::78c1:caff:feda:657a/64 scope link 
       valid_lft forever preferred_lft forever
32: veth6b56e4c@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 8e:cd:56:ac:8e:7e brd ff:ff:ff:ff:ff:ff link-netnsid 18
    inet6 fe80::8ccd:56ff:feac:8e7e/64 scope link 
       valid_lft forever preferred_lft forever
35: veth5fac584@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether b2:0c:28:a3:dc:42 brd ff:ff:ff:ff:ff:ff link-netnsid 21
    inet6 fe80::b00c:28ff:fea3:dc42/64 scope link 
       valid_lft forever preferred_lft forever
37: vethe446813@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 22:9c:5d:59:b5:68 brd ff:ff:ff:ff:ff:ff link-netnsid 23
    inet6 fe80::209c:5dff:fe59:b568/64 scope link 
       valid_lft forever preferred_lft forever
38: veth3ebbfd5@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 2a:56:af:b3:85:f3 brd ff:ff:ff:ff:ff:ff link-netnsid 24
    inet6 fe80::2856:afff:feb3:85f3/64 scope link 
       valid_lft forever preferred_lft forever
41: veth01c8312@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 62:da:fa:b3:c1:3c brd ff:ff:ff:ff:ff:ff link-netnsid 27
    inet6 fe80::60da:faff:feb3:c13c/64 scope link 
       valid_lft forever preferred_lft forever
43: veth4e0fe67@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 9a:11:17:a3:ef:34 brd ff:ff:ff:ff:ff:ff link-netnsid 29
    inet6 fe80::9811:17ff:fea3:ef34/64 scope link 
       valid_lft forever preferred_lft forever
45: veth21e3c8a@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 9e:49:b0:fd:a7:c0 brd ff:ff:ff:ff:ff:ff link-netnsid 31
    inet6 fe80::9c49:b0ff:fefd:a7c0/64 scope link 
       valid_lft forever preferred_lft forever
46: veth9f4e3ca@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether fe:28:9c:a1:b1:e1 brd ff:ff:ff:ff:ff:ff link-netnsid 32
    inet6 fe80::fc28:9cff:fea1:b1e1/64 scope link 
       valid_lft forever preferred_lft forever
47: veth2795ab1@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 72:f7:42:10:0d:a1 brd ff:ff:ff:ff:ff:ff link-netnsid 33
    inet6 fe80::70f7:42ff:fe10:da1/64 scope link 
       valid_lft forever preferred_lft forever
49: veth7d1d137@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 9e:2b:d7:20:45:e8 brd ff:ff:ff:ff:ff:ff link-netnsid 35
    inet6 fe80::9c2b:d7ff:fe20:45e8/64 scope link 
       valid_lft forever preferred_lft forever
52: veth4520e0b@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether aa:19:81:dc:41:c8 brd ff:ff:ff:ff:ff:ff link-netnsid 38
    inet6 fe80::a819:81ff:fedc:41c8/64 scope link 
       valid_lft forever preferred_lft forever
56: veth15999dd@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether ea:10:0f:8b:9e:80 brd ff:ff:ff:ff:ff:ff link-netnsid 42
    inet6 fe80::e810:fff:fe8b:9e80/64 scope link 
       valid_lft forever preferred_lft forever
57: vetha263f85@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 26:c4:40:33:41:9c brd ff:ff:ff:ff:ff:ff link-netnsid 43
    inet6 fe80::24c4:40ff:fe33:419c/64 scope link 
       valid_lft forever preferred_lft forever
58: vethbcccc1f@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-930dcf26a623 state UP group default 
    link/ether aa:e5:b3:b4:95:a7 brd ff:ff:ff:ff:ff:ff link-netnsid 44
    inet6 fe80::a8e5:b3ff:feb4:95a7/64 scope link 
       valid_lft forever preferred_lft forever
61: vethe535e91@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 82:a2:64:f5:6c:67 brd ff:ff:ff:ff:ff:ff link-netnsid 47
    inet6 fe80::80a2:64ff:fef5:6c67/64 scope link 
       valid_lft forever preferred_lft forever
63: veth68ba38a@if3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-c5e4c202334a state UP group default 
    link/ether de:2a:ab:bb:23:55 brd ff:ff:ff:ff:ff:ff link-netnsid 44
    inet6 fe80::dc2a:abff:febb:2355/64 scope link 
       valid_lft forever preferred_lft forever
66: vethce3cc0f@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 26:04:7e:f1:d0:7c brd ff:ff:ff:ff:ff:ff link-netnsid 50
    inet6 fe80::2404:7eff:fef1:d07c/64 scope link 
       valid_lft forever preferred_lft forever
71: veth58a06cd@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 16:04:41:79:7c:34 brd ff:ff:ff:ff:ff:ff link-netnsid 55
    inet6 fe80::1404:41ff:fe79:7c34/64 scope link 
       valid_lft forever preferred_lft forever
78: veth51723a7@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 56:14:e9:41:48:38 brd ff:ff:ff:ff:ff:ff link-netnsid 62
    inet6 fe80::5414:e9ff:fe41:4838/64 scope link 
       valid_lft forever preferred_lft forever
79: vethc6e6c23@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 4e:97:61:07:6a:a0 brd ff:ff:ff:ff:ff:ff link-netnsid 63
    inet6 fe80::4c97:61ff:fe07:6aa0/64 scope link 
       valid_lft forever preferred_lft forever
81: veth703b444@if4: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 2a:bb:37:49:fc:51 brd ff:ff:ff:ff:ff:ff link-netnsid 44
    inet6 fe80::28bb:37ff:fe49:fc51/64 scope link 
       valid_lft forever preferred_lft forever
83: veth2a42514@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether ce:cf:9d:1b:4b:16 brd ff:ff:ff:ff:ff:ff link-netnsid 66
    inet6 fe80::cccf:9dff:fe1b:4b16/64 scope link 
       valid_lft forever preferred_lft forever
210: br-f87e1278ea6d: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether 02:e2:b2:77:3e:ff brd ff:ff:ff:ff:ff:ff
    inet 172.24.0.1/16 brd 172.24.255.255 scope global br-f87e1278ea6d
       valid_lft forever preferred_lft forever
    inet6 fe80::e2:b2ff:fe77:3eff/64 scope link 
       valid_lft forever preferred_lft forever
225: veth9c5ef53@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 56:5e:eb:a8:59:b6 brd ff:ff:ff:ff:ff:ff link-netnsid 17
    inet6 fe80::545e:ebff:fea8:59b6/64 scope link 
       valid_lft forever preferred_lft forever
233: veth1a60e33@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 32:ae:b4:10:01:47 brd ff:ff:ff:ff:ff:ff link-netnsid 20
    inet6 fe80::30ae:b4ff:fe10:147/64 scope link 
       valid_lft forever preferred_lft forever
235: veth46bf014@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether f2:87:f0:f1:2e:01 brd ff:ff:ff:ff:ff:ff link-netnsid 22
    inet6 fe80::f087:f0ff:fef1:2e01/64 scope link 
       valid_lft forever preferred_lft forever
237: veth4a851e9@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 12:8f:9a:8b:15:cf brd ff:ff:ff:ff:ff:ff link-netnsid 8
    inet6 fe80::108f:9aff:fe8b:15cf/64 scope link 
       valid_lft forever preferred_lft forever
239: veth96f47cd@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 1e:28:3b:3a:08:36 brd ff:ff:ff:ff:ff:ff link-netnsid 19
    inet6 fe80::1c28:3bff:fe3a:836/64 scope link 
       valid_lft forever preferred_lft forever
241: veth9166bf2@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 72:f6:af:94:5f:e2 brd ff:ff:ff:ff:ff:ff link-netnsid 25
    inet6 fe80::70f6:afff:fe94:5fe2/64 scope link 
       valid_lft forever preferred_lft forever
243: veth0f48ced@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 86:e8:f2:7a:a1:0a brd ff:ff:ff:ff:ff:ff link-netnsid 28
    inet6 fe80::84e8:f2ff:fe7a:a10a/64 scope link 
       valid_lft forever preferred_lft forever
245: veth703d751@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether be:19:26:0f:71:bf brd ff:ff:ff:ff:ff:ff link-netnsid 34
    inet6 fe80::bc19:26ff:fe0f:71bf/64 scope link 
       valid_lft forever preferred_lft forever
253: veth7acda09@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether 4a:43:13:5e:12:6a brd ff:ff:ff:ff:ff:ff link-netnsid 11
    inet6 fe80::4843:13ff:fe5e:126a/64 scope link 
       valid_lft forever preferred_lft forever
255: veth5e3a49e@if2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-8a872bb56623 state UP group default 
    link/ether d6:86:33:d5:d6:bd brd ff:ff:ff:ff:ff:ff link-netnsid 36
    inet6 fe80::d486:33ff:fed5:d6bd/64 scope link 
       valid_lft forever preferred_lft forever
```
## Routes
```text
default via 192.168.1.1 dev enp6s18 proto static 
172.17.0.0/16 dev docker0 proto kernel scope link src 172.17.0.1 linkdown 
172.18.0.0/16 dev br-bde86ecffe2e proto kernel scope link src 172.18.0.1 linkdown 
172.19.0.0/16 dev br-8a872bb56623 proto kernel scope link src 172.19.0.1 
172.20.0.0/16 dev br-930dcf26a623 proto kernel scope link src 172.20.0.1 
172.21.0.0/16 dev br-c5e4c202334a proto kernel scope link src 172.21.0.1 
172.22.0.0/16 dev br-ae22bd8e7962 proto kernel scope link src 172.22.0.1 
172.23.0.0/16 dev br-769a326cf30d proto kernel scope link src 172.23.0.1 
172.24.0.0/16 dev br-f87e1278ea6d proto kernel scope link src 172.24.0.1 linkdown 
192.168.1.0/24 dev enp6s18 proto kernel scope link src 192.168.1.78 
```
## PCI Devices
```text
00:00.0 Host bridge: Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller
00:1a.0 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #4 (rev 03)
00:1a.1 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #5 (rev 03)
00:1a.2 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #6 (rev 03)
00:1a.7 USB controller: Intel Corporation 82801I (ICH9 Family) USB2 EHCI Controller #2 (rev 03)
00:1b.0 Audio device: Intel Corporation 82801I (ICH9 Family) HD Audio Controller (rev 03)
00:1c.0 PCI bridge: Red Hat, Inc. QEMU PCIe Root port
00:1c.1 PCI bridge: Red Hat, Inc. QEMU PCIe Root port
00:1c.2 PCI bridge: Red Hat, Inc. QEMU PCIe Root port
00:1c.3 PCI bridge: Red Hat, Inc. QEMU PCIe Root port
00:1d.0 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #1 (rev 03)
00:1d.1 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #2 (rev 03)
00:1d.2 USB controller: Intel Corporation 82801I (ICH9 Family) USB UHCI Controller #3 (rev 03)
00:1d.7 USB controller: Intel Corporation 82801I (ICH9 Family) USB2 EHCI Controller #1 (rev 03)
00:1e.0 PCI bridge: Intel Corporation 82801 PCI Bridge (rev 92)
00:1f.0 ISA bridge: Intel Corporation 82801IB (ICH9) LPC Interface Controller (rev 02)
00:1f.2 SATA controller: Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] (rev 02)
00:1f.3 SMBus: Intel Corporation 82801I (ICH9 Family) SMBus Controller (rev 02)
01:00.0 VGA compatible controller: NVIDIA Corporation AD104 [GeForce RTX 4070 SUPER] (rev a1)
01:00.1 Audio device: NVIDIA Corporation Device 22bc (rev a1)
05:01.0 PCI bridge: Red Hat, Inc. QEMU PCI-PCI bridge
05:02.0 PCI bridge: Red Hat, Inc. QEMU PCI-PCI bridge
05:03.0 PCI bridge: Red Hat, Inc. QEMU PCI-PCI bridge
05:04.0 PCI bridge: Red Hat, Inc. QEMU PCI-PCI bridge
06:03.0 Unclassified device [00ff]: Red Hat, Inc. Virtio memory balloon
06:08.0 Communication controller: Red Hat, Inc. Virtio console
06:12.0 Ethernet controller: Red Hat, Inc. Virtio network device
09:01.0 SCSI storage controller: Red Hat, Inc. Virtio SCSI
09:05.0 SCSI storage controller: Red Hat, Inc. Virtio SCSI
09:06.0 SCSI storage controller: Red Hat, Inc. Virtio SCSI
09:07.0 SCSI storage controller: Red Hat, Inc. Virtio SCSI
```
## GPU
```text
Wed Jul  1 04:50:42 2026       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 580.159.03             Driver Version: 580.159.03     CUDA Version: 13.0     |
+-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  NVIDIA GeForce RTX 4070 ...    Off |   00000000:01:00.0 Off |                  N/A |
|  0%   52C    P2             38W /  220W |     582MiB /  12282MiB |     20%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+

+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI              PID   Type   Process name                        GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|    0   N/A  N/A            1977      G   /usr/lib/xorg/Xorg                      214MiB |
|    0   N/A  N/A            2045      G   xfwm4                                     3MiB |
|    0   N/A  N/A            2538      C   /usr/bin/sunshine                       260MiB |
|    0   N/A  N/A         3413041      G   ...nstallation/ubuntu12_32/steam          5MiB |
|    0   N/A  N/A         3414019      G   ./steamwebhelper                          7MiB |
|    0   N/A  N/A         3414159      G   ...on/ubuntu12_64/steamwebhelper         55MiB |
+-----------------------------------------------------------------------------------------+
```
## Docker
```text
Docker version 29.6.0, build fb59821
Docker Compose version v5.1.4
NAMES                   STATUS                  PORTS
sab-proxy               Up 4 hours              
jellyfin                Up 4 hours              8096/tcp
lidarr                  Up 7 hours              8686/tcp
comet                   Up 7 hours (healthy)    0.0.0.0:8000->8000/tcp
gameyfin                Up 7 hours              8080/tcp
radarr                  Up 9 hours              7878/tcp
spotwebdb               Up 9 hours              3306/tcp
whisparr                Up 9 hours              6969/tcp
altmount                Up 10 hours (healthy)   127.0.0.1:18080->8080/tcp
seerr                   Up 10 hours             5055/tcp
stash                   Up 10 hours             9999/tcp
prowlarr                Up 10 hours             9696/tcp
jackett                 Up 10 hours             9117/tcp
aiostreams              Up 10 hours (healthy)   0.0.0.0:3002->3000/tcp
cf-proxy                Up 10 hours             
tautulli                Up 11 hours             8181/tcp
dap-manual              Up 27 hours             0.0.0.0:8008->8000/tcp
spotweb                 Up 30 hours             80/tcp
byparr-byparr-1         Up 30 hours (healthy)   8191/tcp
searxng                 Up 30 hours (healthy)   
fourget                 Up 30 hours             
gluetun                 Up 30 hours (healthy)   1080/tcp, 8000/tcp, 1080/udp, 8388/tcp, 8888/tcp, 8388/udp
searxng-redis           Up 30 hours (healthy)   6379/tcp
beets                   Up 30 hours             0.0.0.0:8337->8337/tcp
qbit-proxy              Up 30 hours             
authelia-redis          Up 30 hours             6379/tcp
kometa                  Up 30 hours             
portainer               Up 30 hours             8000/tcp, 9000/tcp, 9443/tcp
gamearr                 Up 30 hours (healthy)   8484/tcp
decypharr               Up 7 hours (healthy)    8282/tcp
authelia                Up 30 hours (healthy)   9091/tcp
traefik                 Up 30 hours             0.0.0.0:80->80/tcp, 0.0.0.0:443->443/tcp, 0.0.0.0:443->443/udp
dockhand-postgres       Up 30 hours (healthy)   5432/tcp
comet-postgres          Up 30 hours (healthy)   5432/tcp
romm                    Up 30 hours (healthy)   6379/tcp, 8080/tcp
jdownloader2            Up 30 hours             3129/tcp, 5800/tcp, 5900/tcp
romm-db                 Up 30 hours (healthy)   3306/tcp
romm-redis              Up 30 hours (healthy)   6379/tcp
karakeep                Up 30 hours (healthy)   3000/tcp
chrome                  Up 30 hours             
warrden                 Up 30 hours             
sonarr                  Up 30 hours             8989/tcp
plex                    Up 30 hours (healthy)   8324/tcp, 1900/udp, 32410/udp, 32400/tcp, 32412-32414/udp, 32469/tcp
dockhand                Up 30 hours (healthy)   3000/tcp
autoscan                Up 30 hours             3030/tcp
vaultwarden             Up 30 hours (healthy)   80/tcp
annatar                 Up 30 hours             
zilean                  Up 30 hours (healthy)   
zileanpostgres          Up 30 hours (healthy)   5432/tcp
gotify                  Up 30 hours (healthy)   80/tcp
dockhand-socket-proxy   Up 30 hours             2375/tcp
```
## Docker Networks
```text
NETWORK ID     NAME                         DRIVER    SCOPE
dfd5100c1ec3   bridge                       bridge    local
769a326cf30d   comet-net                    bridge    local
f87e1278ea6d   dap-manual_default           bridge    local
930dcf26a623   dockhand_dockhand_internal   bridge    local
c5e4c202334a   dockhand_dockhand_socket     bridge    local
cfaffa31d837   host                         host      local
632ddbf21e7e   none                         null      local
bde86ecffe2e   odysseus_odysseus_internal   bridge    local
8a872bb56623   saltbox                      bridge    local
ae22bd8e7962   zilean_default               bridge    local
```
## Listening Ports
```text
Netid State  Recv-Q Send-Q Local Address:Port  Peer Address:PortProcess                                       
udp   UNCONN 0      0         127.0.0.54:53         0.0.0.0:*                                                 
udp   UNCONN 0      0      127.0.0.53%lo:53         0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:443        0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:35566      0.0.0.0:*                                                 
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=190))
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=189))
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=188))
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=187))
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=186))
udp   UNCONN 0      0        224.0.0.251:5353       0.0.0.0:*    users:(("steamwebhelper",pid=3414019,fd=183))
udp   UNCONN 0      0            0.0.0.0:5353       0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:47998      0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:47999      0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:48000      0.0.0.0:*                                                 
udp   UNCONN 0      0            0.0.0.0:27036      0.0.0.0:*    users:(("steam",pid=3413041,fd=148))         
udp   UNCONN 0      0               [::]:33733         [::]:*                                                 
udp   UNCONN 0      0               [::]:5353          [::]:*                                                 
tcp   LISTEN 0      4096      127.0.0.54:53         0.0.0.0:*                                                 
tcp   LISTEN 0      128        127.0.0.1:27060      0.0.0.0:*    users:(("steam",pid=3413041,fd=101))         
tcp   LISTEN 0      5            0.0.0.0:18082      0.0.0.0:*                                                 
tcp   LISTEN 0      128        127.0.0.1:57343      0.0.0.0:*    users:(("steam",pid=3413041,fd=55))          
tcp   LISTEN 0      4096      172.19.0.1:9100       0.0.0.0:*    users:(("node_exporter",pid=24976,fd=4))     
tcp   LISTEN 0      4096   127.0.0.53%lo:53         0.0.0.0:*                                                 
tcp   LISTEN 0      128          0.0.0.0:27036      0.0.0.0:*    users:(("steam",pid=3413041,fd=149))         
tcp   LISTEN 0      4096       127.0.0.1:18080      0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:8000       0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:8008       0.0.0.0:*                                                 
tcp   LISTEN 0      100          0.0.0.0:6082       0.0.0.0:*    users:(("websockify",pid=2101,fd=3))         
tcp   LISTEN 0      4096         0.0.0.0:3002       0.0.0.0:*                                                 
tcp   LISTEN 0      128        127.0.0.1:42531      0.0.0.0:*    users:(("steam",pid=3413041,fd=60))          
tcp   LISTEN 0      4096         0.0.0.0:443        0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:80         0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:22         0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:47990      0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:47989      0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:47984      0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:48010      0.0.0.0:*                                                 
tcp   LISTEN 0      4096       127.0.0.1:8191       0.0.0.0:*                                                 
tcp   LISTEN 0      128        127.0.0.1:37479      0.0.0.0:*    users:(("steam",pid=3413041,fd=63))          
tcp   LISTEN 0      32         127.0.0.1:5900       0.0.0.0:*    users:(("x11vnc",pid=2128,fd=8))             
tcp   LISTEN 0      4096       127.0.0.1:5572       0.0.0.0:*    users:(("rclone",pid=2521,fd=7))             
tcp   LISTEN 0      4096       127.0.0.1:3377       0.0.0.0:*                                                 
tcp   LISTEN 0      4096       127.0.0.1:631        0.0.0.0:*                                                 
tcp   LISTEN 0      2048         0.0.0.0:9119       0.0.0.0:*    users:(("hermes",pid=5094,fd=14))            
tcp   LISTEN 0      128          0.0.0.0:8765       0.0.0.0:*                                                 
tcp   LISTEN 0      4096         0.0.0.0:8337       0.0.0.0:*                                                 
tcp   LISTEN 0      4096           [::1]:631           [::]:*                                                 
tcp   LISTEN 0      32             [::1]:5900          [::]:*    users:(("x11vnc",pid=2128,fd=9))             
tcp   LISTEN 0      4096            [::]:22            [::]:*                                                 
```
