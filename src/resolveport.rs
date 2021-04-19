use std::collections::HashMap;

pub fn checkport(port : &u16) -> String{
    let strport = port.to_string();
    
    let mut ports = HashMap::new();

    /*
    *   So ummmmmmmmmmmmmmmmmmmm, I made this... I was too lazy to get getservbyport from libc working so I rewrote it
    *   Of course I didn't manually make all of this, I used some command line magic:
    *   cat /etc/services | grep tcp | cut -d '/' -f1 | tr -s '[:blank:]' ':' | tr : ' ' | awk '{ print "ports.insert(\"" $2 "\".to_string(), \"" $1 "\".to_string());"}'
    */



    ports.insert("1".to_string(), "tcpmux".to_string());
    ports.insert("7".to_string(), "echo".to_string());
    ports.insert("9".to_string(), "discard".to_string());
    ports.insert("11".to_string(), "systat".to_string());
    ports.insert("13".to_string(), "daytime".to_string());
    ports.insert("15".to_string(), "netstat".to_string());
    ports.insert("17".to_string(), "qotd".to_string());
    ports.insert("19".to_string(), "chargen".to_string());
    ports.insert("20".to_string(), "ftp-data".to_string());
    ports.insert("21".to_string(), "ftp".to_string());
    ports.insert("22".to_string(), "ssh".to_string());
    ports.insert("23".to_string(), "telnet".to_string());
    ports.insert("25".to_string(), "smtp".to_string());
    ports.insert("37".to_string(), "time".to_string());
    ports.insert("43".to_string(), "whois".to_string());
    ports.insert("49".to_string(), "tacacs".to_string());
    ports.insert("53".to_string(), "domain".to_string());
    ports.insert("70".to_string(), "gopher".to_string());
    ports.insert("79".to_string(), "finger".to_string());
    ports.insert("80".to_string(), "http".to_string());
    ports.insert("88".to_string(), "kerberos".to_string());
    ports.insert("102".to_string(), "iso-tsap".to_string());
    ports.insert("104".to_string(), "acr-nema".to_string());
    ports.insert("110".to_string(), "pop3".to_string());
    ports.insert("111".to_string(), "sunrpc".to_string());
    ports.insert("113".to_string(), "auth".to_string());
    ports.insert("119".to_string(), "nntp".to_string());
    ports.insert("135".to_string(), "epmap".to_string());
    ports.insert("137".to_string(), "netbios-ns".to_string());
    ports.insert("138".to_string(), "netbios-dgm".to_string());
    ports.insert("139".to_string(), "netbios-ssn".to_string());
    ports.insert("143".to_string(), "imap2".to_string());
    ports.insert("161".to_string(), "snmp".to_string());
    ports.insert("162".to_string(), "snmp-trap".to_string());
    ports.insert("163".to_string(), "cmip-man".to_string());
    ports.insert("164".to_string(), "cmip-agent".to_string());
    ports.insert("174".to_string(), "mailq".to_string());
    ports.insert("179".to_string(), "bgp".to_string());
    ports.insert("199".to_string(), "smux".to_string());
    ports.insert("209".to_string(), "qmtp".to_string());
    ports.insert("210".to_string(), "z3950".to_string());
    ports.insert("345".to_string(), "pawserv".to_string());
    ports.insert("346".to_string(), "zserv".to_string());
    ports.insert("369".to_string(), "rpc2portmap".to_string());
    ports.insert("370".to_string(), "codaauth2".to_string());
    ports.insert("389".to_string(), "ldap".to_string());
    ports.insert("427".to_string(), "svrloc".to_string());
    ports.insert("443".to_string(), "https".to_string());
    ports.insert("444".to_string(), "snpp".to_string());
    ports.insert("445".to_string(), "microsoft-ds".to_string());
    ports.insert("464".to_string(), "kpasswd".to_string());
    ports.insert("465".to_string(), "submissions".to_string());
    ports.insert("487".to_string(), "saft".to_string());
    ports.insert("554".to_string(), "rtsp".to_string());
    ports.insert("607".to_string(), "nqs".to_string());
    ports.insert("628".to_string(), "qmqp".to_string());
    ports.insert("631".to_string(), "ipp".to_string());
    ports.insert("512".to_string(), "exec".to_string());
    ports.insert("513".to_string(), "login".to_string());
    ports.insert("514".to_string(), "shell".to_string());
    ports.insert("515".to_string(), "printer".to_string());
    ports.insert("538".to_string(), "gdomap".to_string());
    ports.insert("540".to_string(), "uucp".to_string());
    ports.insert("543".to_string(), "klogin".to_string());
    ports.insert("544".to_string(), "kshell".to_string());
    ports.insert("548".to_string(), "afpovertcp".to_string());
    ports.insert("563".to_string(), "nntps".to_string());
    ports.insert("587".to_string(), "submission".to_string());
    ports.insert("636".to_string(), "ldaps".to_string());
    ports.insert("655".to_string(), "tinc".to_string());
    ports.insert("706".to_string(), "silc".to_string());
    ports.insert("749".to_string(), "kerberos-adm".to_string());
    ports.insert("853".to_string(), "domain-s".to_string());
    ports.insert("873".to_string(), "rsync".to_string());
    ports.insert("989".to_string(), "ftps-data".to_string());
    ports.insert("990".to_string(), "ftps".to_string());
    ports.insert("992".to_string(), "telnets".to_string());
    ports.insert("993".to_string(), "imaps".to_string());
    ports.insert("995".to_string(), "pop3s".to_string());
    ports.insert("1080".to_string(), "socks".to_string());
    ports.insert("1093".to_string(), "proofd".to_string());
    ports.insert("1094".to_string(), "rootd".to_string());
    ports.insert("1194".to_string(), "openvpn".to_string());
    ports.insert("1099".to_string(), "rmiregistry".to_string());
    ports.insert("1352".to_string(), "lotusnote".to_string());
    ports.insert("1433".to_string(), "ms-sql-s".to_string());
    ports.insert("1434".to_string(), "ms-sql-m".to_string());
    ports.insert("1524".to_string(), "ingreslock".to_string());
    ports.insert("1645".to_string(), "datametrics".to_string());
    ports.insert("1646".to_string(), "sa-msg-port".to_string());
    ports.insert("1649".to_string(), "kermit".to_string());
    ports.insert("1677".to_string(), "groupwise".to_string());
    ports.insert("1812".to_string(), "radius".to_string());
    ports.insert("1813".to_string(), "radius-acct".to_string());
    ports.insert("2000".to_string(), "cisco-sccp".to_string());
    ports.insert("2049".to_string(), "nfs".to_string());
    ports.insert("2086".to_string(), "gnunet".to_string());
    ports.insert("2101".to_string(), "rtcm-sc104".to_string());
    ports.insert("2119".to_string(), "gsigatekeeper".to_string());
    ports.insert("2135".to_string(), "gris".to_string());
    ports.insert("2401".to_string(), "cvspserver".to_string());
    ports.insert("2430".to_string(), "venus".to_string());
    ports.insert("2431".to_string(), "venus-se".to_string());
    ports.insert("2432".to_string(), "codasrv".to_string());
    ports.insert("2433".to_string(), "codasrv-se".to_string());
    ports.insert("2583".to_string(), "mon".to_string());
    ports.insert("2628".to_string(), "dict".to_string());
    ports.insert("2792".to_string(), "f5-globalsite".to_string());
    ports.insert("2811".to_string(), "gsiftp".to_string());
    ports.insert("2947".to_string(), "gpsd".to_string());
    ports.insert("3050".to_string(), "gds-db".to_string());
    ports.insert("3130".to_string(), "icpv2".to_string());
    ports.insert("3205".to_string(), "isns".to_string());
    ports.insert("3260".to_string(), "iscsi-target".to_string());
    ports.insert("3306".to_string(), "mysql".to_string());
    ports.insert("3389".to_string(), "ms-wbt-server".to_string());
    ports.insert("3493".to_string(), "nut".to_string());
    ports.insert("3632".to_string(), "distcc".to_string());
    ports.insert("3689".to_string(), "daap".to_string());
    ports.insert("3690".to_string(), "svn".to_string());
    ports.insert("4031".to_string(), "suucp".to_string());
    ports.insert("4094".to_string(), "sysrqd".to_string());
    ports.insert("4190".to_string(), "sieve".to_string());
    ports.insert("4369".to_string(), "epmd".to_string());
    ports.insert("4373".to_string(), "remctl".to_string());
    ports.insert("4353".to_string(), "f5-iquery".to_string());
    ports.insert("4569".to_string(), "iax".to_string());
    ports.insert("4691".to_string(), "mtn".to_string());
    ports.insert("4899".to_string(), "radmin-port".to_string());
    ports.insert("5060".to_string(), "sip".to_string());
    ports.insert("5061".to_string(), "sip-tls".to_string());
    ports.insert("5222".to_string(), "xmpp-client".to_string());
    ports.insert("5269".to_string(), "xmpp-server".to_string());
    ports.insert("5308".to_string(), "cfengine".to_string());
    ports.insert("5432".to_string(), "postgresql".to_string());
    ports.insert("5556".to_string(), "freeciv".to_string());
    ports.insert("5671".to_string(), "amqps".to_string());
    ports.insert("5672".to_string(), "amqp".to_string());
    ports.insert("6000".to_string(), "x11".to_string());
    ports.insert("6001".to_string(), "x11-1".to_string());
    ports.insert("6002".to_string(), "x11-2".to_string());
    ports.insert("6003".to_string(), "x11-3".to_string());
    ports.insert("6004".to_string(), "x11-4".to_string());
    ports.insert("6005".to_string(), "x11-5".to_string());
    ports.insert("6006".to_string(), "x11-6".to_string());
    ports.insert("6007".to_string(), "x11-7".to_string());
    ports.insert("6346".to_string(), "gnutella-svc".to_string());
    ports.insert("6347".to_string(), "gnutella-rtr".to_string());
    ports.insert("6444".to_string(), "sge-qmaster".to_string());
    ports.insert("6445".to_string(), "sge-execd".to_string());
    ports.insert("6446".to_string(), "mysql-proxy".to_string());
    ports.insert("6697".to_string(), "ircs-u".to_string());
    ports.insert("7000".to_string(), "afs3-fileserver".to_string());
    ports.insert("7001".to_string(), "afs3-callback".to_string());
    ports.insert("7002".to_string(), "afs3-prserver".to_string());
    ports.insert("7003".to_string(), "afs3-vlserver".to_string());
    ports.insert("7004".to_string(), "afs3-kaserver".to_string());
    ports.insert("7005".to_string(), "afs3-volser".to_string());
    ports.insert("7006".to_string(), "afs3-errors".to_string());
    ports.insert("7007".to_string(), "afs3-bos".to_string());
    ports.insert("7008".to_string(), "afs3-update".to_string());
    ports.insert("7009".to_string(), "afs3-rmtsys".to_string());
    ports.insert("7100".to_string(), "font-service".to_string());
    ports.insert("8080".to_string(), "http-alt".to_string());
    ports.insert("8140".to_string(), "puppet".to_string());
    ports.insert("9101".to_string(), "bacula-dir".to_string());
    ports.insert("9102".to_string(), "bacula-fd".to_string());
    ports.insert("9103".to_string(), "bacula-sd".to_string());
    ports.insert("9667".to_string(), "xmms2".to_string());
    ports.insert("10809".to_string(), "nbd".to_string());
    ports.insert("10050".to_string(), "zabbix-agent".to_string());
    ports.insert("10051".to_string(), "zabbix-trapper".to_string());
    ports.insert("10080".to_string(), "amanda".to_string());
    ports.insert("11112".to_string(), "dicom".to_string());
    ports.insert("11371".to_string(), "hkp".to_string());
    ports.insert("17500".to_string(), "db-lsp".to_string());
    ports.insert("22125".to_string(), "dcap".to_string());
    ports.insert("22128".to_string(), "gsidcap".to_string());
    ports.insert("22273".to_string(), "wnn6".to_string());
    ports.insert("750".to_string(), "kerberos4".to_string());
    ports.insert("751".to_string(), "kerberos-master".to_string());
    ports.insert("754".to_string(), "krb-prop".to_string());
    ports.insert("2121".to_string(), "iprop".to_string());
    ports.insert("871".to_string(), "supfilesrv".to_string());
    ports.insert("1127".to_string(), "supfiledbg".to_string());
    ports.insert("106".to_string(), "poppassd".to_string());
    ports.insert("775".to_string(), "moira-db".to_string());
    ports.insert("777".to_string(), "moira-update".to_string());
    ports.insert("783".to_string(), "spamd".to_string());
    ports.insert("1178".to_string(), "skkserv".to_string());
    ports.insert("1236".to_string(), "rmtcfg".to_string());
    ports.insert("1313".to_string(), "xtel".to_string());
    ports.insert("1314".to_string(), "xtelw".to_string());
    ports.insert("1529".to_string(), "support".to_string());
    ports.insert("2003".to_string(), "cfinger".to_string());
    ports.insert("2121".to_string(), "frox".to_string());
    ports.insert("2600".to_string(), "zebrasrv".to_string());
    ports.insert("2601".to_string(), "zebra".to_string());
    ports.insert("2602".to_string(), "ripd".to_string());
    ports.insert("2603".to_string(), "ripngd".to_string());
    ports.insert("2604".to_string(), "ospfd".to_string());
    ports.insert("2605".to_string(), "bgpd".to_string());
    ports.insert("2606".to_string(), "ospf6d".to_string());
    ports.insert("2607".to_string(), "ospfapi".to_string());
    ports.insert("2608".to_string(), "isisd".to_string());
    ports.insert("2988".to_string(), "afbackup".to_string());
    ports.insert("2989".to_string(), "afmbackup".to_string());
    ports.insert("4557".to_string(), "fax".to_string());
    ports.insert("4559".to_string(), "hylafax".to_string());
    ports.insert("4600".to_string(), "distmp3".to_string());
    ports.insert("4949".to_string(), "munin".to_string());
    ports.insert("5051".to_string(), "enbd-cstatd".to_string());
    ports.insert("5052".to_string(), "enbd-sstatd".to_string());
    ports.insert("5151".to_string(), "pcrd".to_string());
    ports.insert("5354".to_string(), "noclog".to_string());
    ports.insert("5355".to_string(), "hostmon".to_string());
    ports.insert("5666".to_string(), "nrpe".to_string());
    ports.insert("5667".to_string(), "nsca".to_string());
    ports.insert("5674".to_string(), "mrtd".to_string());
    ports.insert("5675".to_string(), "bgpsim".to_string());
    ports.insert("5680".to_string(), "canna".to_string());
    ports.insert("6514".to_string(), "syslog-tls".to_string());
    ports.insert("6566".to_string(), "sane-port".to_string());
    ports.insert("6667".to_string(), "ircd".to_string());
    ports.insert("8021".to_string(), "zope-ftp".to_string());
    ports.insert("8081".to_string(), "tproxy".to_string());
    ports.insert("8088".to_string(), "omniorb".to_string());
    ports.insert("8990".to_string(), "clc-build-daemon".to_string());
    ports.insert("9098".to_string(), "xinetd".to_string());
    ports.insert("9418".to_string(), "git".to_string());
    ports.insert("9673".to_string(), "zope".to_string());
    ports.insert("10000".to_string(), "webmin".to_string());
    ports.insert("10081".to_string(), "kamanda".to_string());
    ports.insert("10082".to_string(), "amandaidx".to_string());
    ports.insert("10083".to_string(), "amidxtape".to_string());
    ports.insert("11201".to_string(), "smsqp".to_string());
    ports.insert("15345".to_string(), "xpilot".to_string());
    ports.insert("17004".to_string(), "sgi-cad".to_string());
    ports.insert("20011".to_string(), "isdnlog".to_string());
    ports.insert("20012".to_string(), "vboxd".to_string());
    ports.insert("24554".to_string(), "binkp".to_string());
    ports.insert("27374".to_string(), "asp".to_string());
    ports.insert("30865".to_string(), "csync2".to_string());
    ports.insert("57000".to_string(), "dircproxy".to_string());
    ports.insert("60177".to_string(), "tfido".to_string());
    ports.insert("60179".to_string(), "fido".to_string());

    match ports.get(&strport){
        Some(service) => String::from(service),
        None => String::from("NULL")
    }
}