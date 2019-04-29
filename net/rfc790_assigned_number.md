https://tools.ietf.org/html/rfc790

      The first type, or class a, of address has a 7-bit network number
      and a 24-bit local address.  This allows 128 class a networks.

                                                 1                                     2                                     3
         0  1 2  3  4  5  6 7  8  9  0  1  2 3  4  5  6  7  8 9  0  1  2  3 4  5  6  7  8 9  0  1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       |0 |    NETWORK     |                                        Local Address                                |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                             Class A Address

      The second type, or class b, of address has a 14-bit network
      number and a 16-bit local address.  This allows 16,384 class b
      networks.

                                                  1                                     2                                     3
         0  1 2  3  4  5  6 7  8  9  0  1  2 3  4  5  6  7  8 9  0  1  2  3 4  5  6  7  8 9  0  1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       | 1 0|                NETWORK                      |        Local Address                                |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                             Class B Address

      The third type, or class c, of address has a 21-bit network number
      and a 8-bit local address.  This allows 2,097,152 class c
      networks.

                                                  1                                     2                                     3
         0  1 2  3  4  5  6 7  8  9  0  1  2 3  4  5  6  7  8 9  0  1  2  3 4  5  6  7  8 9  0  1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       | 1  1 0|                          NETWORK                                        |  Local Address      |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                             Class C Address

   The class a networks will have nnn.rrr.rrr.rrr, the class b
   networks will have nnn.nnn.rrr.rrr, and the class c networks will
   have nnn.nnn.nnn.rrr, where nnn represents part or all of a network
   number and rrr represents part or all of a local address or rest
   field.

   Class A Networks

      Internet Address   Name          Network                 References
      
      000.rrr.rrr.rrr                             Reserved                     [JBP]
      001.rrr.rrr.rrr   BBN-PR        BBN Packet Radio Network    [DCA2]
      002.rrr.rrr.rrr   SF-PR-1       SF Packet Radio Network (1)  [JEM]
      003.rrr.rrr.rrr   BBN-RCC       BBN RCC Network              [SGC]
      004.rrr.rrr.rrr   SATNET        Atlantic Satellite Network  [DM11]
      005.rrr.rrr.rrr   SILL-PR       Ft. Sill Packet Radio Network[JEM]
      006.rrr.rrr.rrr   SF-PR-2       SF Packet Radio Network (2)  [JEM]
      007.rrr.rrr.rrr   CHAOS         MIT CHAOS Network           [MOON]
      008.rrr.rrr.rrr   CLARKNET      SATNET subnet for Clarksburg[DM11]
      009.rrr.rrr.rrr   BRAGG-PR      Ft. Bragg Packet Radio Net   [JEM]
      010.rrr.rrr.rrr   ARPANET       ARPANET                 [17,1,VGC]
      011.rrr.rrr.rrr   UCLNET        University College London     [PK]
      012.rrr.rrr.rrr   CYCLADES      CYCLADES                     [VGC]
      013.rrr.rrr.rrr                 Unassigned                   [JBP]
      014.rrr.rrr.rrr   TELENET       TELENET                      [VGC]
      015.rrr.rrr.rrr   EPSS          British Post Office EPSS      [PK]
      016.rrr.rrr.rrr   DATAPAC       DATAPAC                      [VGC]
      017.rrr.rrr.rrr   TRANSPAC      TRANSPAC                     [VGC]
      018.rrr.rrr.rrr   LCSNET        MIT LCS Network       [43,10,DDC2]
      019.rrr.rrr.rrr   TYMNET        TYMNET                       [VGC]
      020.rrr.rrr.rrr   DC-PR         D.C. Packet Radio Network    [VGC]
      021.rrr.rrr.rrr   EDN           DCEC EDN                     [EC5]
      022.rrr.rrr.rrr   DIALNET       DIALNET                [26,16,MRC]
      023.rrr.rrr.rrr   MITRE         MITRE Cablenet            [44,APS]
      024.rrr.rrr.rrr   BBN-LOCAL     BBN Local Network            [SGC]
      025.rrr.rrr.rrr   RSRE-PPSN     RSRE / PPSN                  [BD2]
      026.rrr.rrr.rrr   AUTODIN-II    AUTODIN II                   [EC5]
      027.rrr.rrr.rrr   NOSC-LCCN     NOSC / LCCN                  [KTP]
      028.rrr.rrr.rrr   WIDEBAND      Wide Band Satellite Network [CJW2]
      029.rrr.rrr.rrr   DCN-COMSAT    COMSAT Dist. Comp. Network  [DLM1]
      030.rrr.rrr.rrr   DCN-UCL       UCL Dist. Comp. Network       [PK]
      031.rrr.rrr.rrr   BBN-SAT-TEST  BBN SATNET Test Network     [DM11]
      032.rrr.rrr.rrr   UCL-CR1       UCL Cambridge Ring 1          [PK]
      033.rrr.rrr.rrr   UCL-CR2       UCL Cambridge Ring 2          [PK]
      034.rrr.rrr.rrr   MATNET        Mobile Access Terminal Net  [DM11]
      035.rrr.rrr.rrr   NULL          UCL/RSRE Null Network        [BD2]
      036.rrr.rrr.rrr   SU-NET        Stanford University Ethernet [MRC]
      037.rrr.rrr.rrr   DECNET        Digital Equipment Network    [DRL]
      038.rrr.rrr.rrr   DECNET-TEST   Test Digital Equipment Net   [DRL]
      039.rrr.rrr.rrr   SRINET        SRI Local Network           [GEOF]
      040.rrr.rrr.rrr   CISLNET       CISL Multics Network         [CH2]
      041.rrr.rrr.rrr   BBN-LN-TEST   BBN Local Network Testbed    [KTP]
      042.rrr.rrr.rrr   S1NET         LLL-S1-NET                   [EAK]
      043.rrr.rrr.rrr   INTELPOST     COMSAT INTELPOST            [DLM1]
      044.rrr.rrr.rrr   AMPRNET       Amature Radio Experiment Net  [HM]
      044.rrr.rrr.rrr-126.rrr.rrr.rrr Unassigned                   [JBP]
      127.rrr.rrr.rrr                 Reserved                     [JBP]

   Class B Networks

      Internet Address        Name          Network                 References
      
      128.000.rrr.rrr                 Reserved                     [JBP]
      128.001.rrr.rrr-128.254.rrr.rrr Unassigned                   [JBP]
      191.255.rrr.rrr                 Reserved                     [JBP]

   Class C Networks

      Internet Address    Name          Network                 References
      
      192.000.001.rrr                 Reserved                     [JBP]
      192.000.001.rrr-223.255.254.rrr Unassigned                   [JBP]
      223.255.255.rrr                 Reserved                     [JBP]

   Other Reserved Internet Addresses

      Internet Address  Name          Network                 References
      
      224.000.000.000-255.255.255.255 Reserved                     [JBP]

                   ASSIGNED INTERNET VERSION NUMBERS

   In the Internet Protocol (IP) [33] there is a field to identify the
   version of the internetwork general protocol.  This field is 4 bits
   in size.

   Assigned Internet Version Numbers

      Decimal   Octal      Version                            References
      
          0      0         Reserved                                [JBP]
        1-3    1-3         Unassigned                              [JBP]
          4      4         Internet Protocol                    [33,JBP]
          5      5         ST Datagram Mode                     [20,JWF]
       6-14   6-16         Unassigned                              [JBP]
         15     17         Reserved                                [JBP]

Internet Protocol Numbers

                   ASSIGNED INTERNET PROTOCOL NUMBERS

   In the Internet Protocol (IP) [33] there is a field, called Protocol,
   to identify the the next level protocol.  This is an 8 bit field.

   Assigned Internet Protocol Numbers

      Decimal    Octal      Protocol Numbers                  References
      
           0       0         Reserved                              [JBP]
           1       1         ICMP                               [53,JBP]
           2       2         Unassigned                            [JBP]
           3       3         Gateway-to-Gateway              [48,49,VMS]
           4       4         CMCC Gateway Monitoring Message [18,19,DFP]
           5       5         ST                                 [20,JWF]
           6       6         TCP                                [34,JBP]
           7       7         UCL                                    [PK]
           8      10         Unassigned                            [JBP]
           9      11         Secure                                [VGC]
          10      12         BBN RCC Monitoring                    [VMS]
          11      13         NVP                                 [12,DC]
          12      14         PUP                                [4,EAT3]
          13      15         Pluribus                             [RDB2]
          14      16         Telenet                              [RDB2]
          15      17         XNET                              [25,JFH2]
          16      20         Chaos                                [MOON]
          17      21         User Datagram                      [42,JBP]
          18      22         Multiplexing                       [13,JBP]
          19      23         DCN                                  [DLM1]
          20      24         TAC Monitoring                     [55,RH6]
       21-62   25-76         Unassigned                            [JBP]
          63      77         any local network                     [JBP]
          64     100         SATNET and Backroom EXPAK            [DM11]
          65     101         MIT Subnet Support                    [NC3]
       66-68 102-104         Unassigned                            [JBP]
          69     105         SATNET Monitoring                    [DM11]
          70     106         Unassigned                            [JBP]
          71     107         Internet Packet Core Utility         [DM11]
       72-75 110-113         Unassigned                            [JBP]
          76     114         Backroom SATNET Monitoring           [DM11]
          77     115         Unassigned                            [JBP]
          78     116         WIDEBAND Monitoring                  [DM11]
          79     117         WIDEBAND EXPAK                       [DM11]
      80-254 120-376         Unassigned                            [JBP]
         255     377         Reserved                              [JBP]

                    ASSIGNED PORT or SOCKET NUMBERS

   Ports are used in the TCP [34] and sockets are used in the AHHP
   [28,17] to name the ends of logical connections which carry long term
   conversations.  For the purpose of providing services to unknown
   callers a service contact socket is defined.  This list specifies the
   port or socket used by the server process as its contact socket.  In
   the AHHP an Initial Connection Procedure ICP [39,17] is used between
   the user process and the server process to make the initial contact
   and establish the long term connections leaving the contact socket
   free to handle other callers.  In the TCP no ICP is necessary since a
   port may engage in many simultaneous connections.

   To the extent possible these same port assignments are used with UDP
   [42].

   The assigned ports/sockets use a small part of the possible
   port/socket numbers.  The assigned ports/sockets have all except the
   low order eight bits cleared to zero.  The low order eight bits are
   specified here.

   Socket Assignments:

      General Assignments:

         Decimal   Octal     Description
         -------   -----     -----------
         0-63      0-77      Network Wide Standard Function
         64-131    100-203   Hosts Specific Functions
         132-223   204-337   Reserved for Future Use
         224-255   340-377   Any Experimental Function

                                                        Assigned Numbers
Port or Socket Numbers


      Specific Assignments:

         Network Standard Functions

         Decimal   Octal     Description                      References
         -------   -----     -----------                      ----------
         1         1         Old Telnet                         [40,JBP]
         3         3         Old File Transfer            [27,11,24,JBP]
         5         5         Remote Job Entry                 [6,17,JBP]
         7         7         Echo                               [35,JBP]
         9         11        Discard                            [32,JBP]
         11        13        Who is on or SYSTAT                   [JBP]
         13        15        Date and Time                         [JBP]
         15        17        Who is up or NETSTAT                  [JBP]
         17        21        Short Text Message                    [JBP]
         19        23        Character generator or TTYTST      [31,JBP]
         21        25        New File Transfer                  [36,JBP]
         23        27        New Telnet                         [41,JBP]
         25        31        SMTP                               [54,JBP]
         27        33        NSW User System w/COMPASS FE       [14,RHT]
         29        35        MSG-3 ICP                          [29,RHT]
         31        37        MSG-3 Authentication               [29,RHT]
         33        41        Unassigned                            [JBP]
         35        43        IO Station Spooler                    [JBP]
         37        45        Time Server                        [22,JBP]
         39        47        Unassigned                            [JBP]
         41        51        Graphics                        [46,17,JBP]
         42        52        Name Server                        [38,JBP]
         43        53        WhoIs                                [JAKE]
         45        55        Message Processing Module          [37,JBP]
         47        57        NI FTP                             [50,CJB]
         49        61        RAND Network Graphics Conference   [30,MO2]
         51        63        Message Generator Control          [52,DFP]
         53        65        AUTODIN II FTP                     [21,EC5]
         55        67        ISI Graphics Language               [3,RB6]
         57        71        MTP                                [45,JBP]
         59        73        New MIT Host Status                   [SWG]
         61-63     75-77     Unassigned                            [JBP]

Port or Socket Numbers

         Host Specific Functions

         Decimal   Octal     Description                      References

         65        101       Unassigned                            [JBP]
         67        103       Datacomputer at CCA                 [8,JZS]
         69        105       Unassigned                            [JBP]
         69        105       Trivial File Transfer              [47,KRS]
         71        107       NETRJS (EBCDIC) at UCLA-CCN      [5,17,RTB]
         73        111       NETRJS (ASCII-68) at UCLA-CCN    [5,17,RTB]
         75        113       NETRJS (ASCII-63) at UCLA-CCN    [5,17,RTB]
         77        115       any private RJE server                [JBP]
         79        117       Name or Finger                  [23,17,KLH]
         81        121       Unassigned                            [JBP]
         83        123       MIT ML Device                        [MOON]
         85        125       MIT ML Device                        [MOON]
         87        127       any terminal link                     [JBP]
         89        131       SU/MIT Telnet Gateway                 [MRC]
         91        133       MIT Dover Spooler                     [EBM]
         93        135       BBN RCC Accounting                     [DT]
         95        137       SUPDUP                             [15,MRC]
         97        141       Datacomputer Status                 [8,JZS]
         99        143       CADC - NIFTP via UCL                  [PLH]
         101       145       NPL - NIFTP via UCL                   [PLH]
         103       147       BNPL - NIFTP via UCL                  [PLH]
         105       151       CAMBRIDGE - NIFTP via UCL             [PLH]
         107       153       HARWELL - NIFTP via UCL               [PLH]
         109       155       SWURCC - NIFTP via UCL                [PLH]
         111       157       ESSEX - NIFTP via UCL                 [PLH]
         113       161       RUTHERFORD - NIFTP via UCL            [PLH]
         115-129   163-201   Unassigned                            [JBP]
         131       203       Datacomputer                        [8,JZS]

         Reserved for Future Use

         Decimal   Octal     Description                      References

         132-223   204-337   Reserved                              [JBP]

         Experimental Functions

         Decimal   Octal     Description                      References

         224-239   340-357   Unassigned                            [JBP]
         241       361       NCP Measurement                     [9,JBP]
         243       363       Survey Measurement                   [2,AV]
         245       365       LINK                               [7,RDB2]
         247       367       TIPSRV                                [RHT]
         249-255   371-377   RSEXEC                             [51,RHT]

                         ASSIGNED LINK NUMBERS

   The word "link" here refers to a field in the original ARPANET
   Host/IMP interface leader.  The link was originally defined as an 8
   bit field.  Some time after the ARPANET Host-to-Host (AHHP) protocol
   was defined and, by now, some time ago the definition of this field
   was changed to "Message-ID" and the length to 12 bits. The name link
   now refers to the high order 8 bits of this 12 bit message-id field.
   The low order 4 bits of the message-id field are to be zero unless
   specifically specified otherwise for the particular protocol used on
   that link.  The Host/IMP interface is defined in BBN report 1822 [1].

   Link Assignments:

      Decimal   Octal     Description                         References

      0         0         AHHP Control Messages              [28,17,JBP]
      1         1         Reserved                                 [JBP]
      2-71      2-107     AHHP Regular Messages              [28,17,JBP]
      72-150    110-226   Reserved                                 [JBP]
      151       227       CHAOS Protocol                          [MOON]
      152       230       PARC Universal Protocol               [4,EAT3]
      153       231       TIP Status Reporting                     [JGH]
      154       232       TIP Accounting                           [JGH]
      155       233       Internet Protocol (regular)           [33,JBP]
      156-158   234-236   Internet Protocol (experimental)      [33,JBP]
      159-191   237-277   Measurements                           [9,VGC]
      192-195   300-303   Unassigned                               [JBP]
      196-255   304-377   Experimental Protocols                   [JBP]
      224-255   340-377   NVP                                 [12,17,DC]
      248-255   370-377   Network Maintenance                      [JGH]
