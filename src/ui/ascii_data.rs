//! Raw ASCII art constants for distro logos.
//! Neofetch/Fastfetch style; no color variables in strings.
//! Top 100 Linux distros + major families + derivatives + server + BSD.

// --- Major families ---

pub const ARCH: &str = r#"
      /\
     /  \
    /\   \
   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\
"#;

pub const DEBIAN: &str = r#"
       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"        """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   .    $$$
 $$P      d$'     ,    $$P
 $$:      $$.   -    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    `.`"Y$$$$P"'
 `$$b      "-.__
  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `"""
"#;

pub const UBUNTU: &str = r#"
            .-/+oossssoo+-.
        ´:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssssdMMMNysssso.
   /ssssssssssshdmmNNmmyNMMMMhssssss\
  +ssssssssshmydMMMMMMMNddddyssssssss+
 /sssssssshNMMMyhhyyyhmNMMMNhssssssss\
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
.ssssssssdMMMNhsssssssshNMMMdssssssss.
 \sssssssshNMMMyhhyyyhdNMMMNhssssssss/
  +sssssssssdmydMMMMMMMMddddyssssssss+
   \ssssssssssshdmNNNNmyNMMMMhssssss/
    .ossssssssssssssssssdMMMNysssso.
      -+sssssssssssssssssyyyssss+-
        `:+ssssssssssssssssss+:`
            .-\+oossssoo+/-.
"#;

pub const FEDORA: &str = r#"
             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;.:dddl:.;ccccccc;.
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,
:cccccccccccccc;MMM.;cccccccccccccc:
:ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:
cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XM0';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;0MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
cccccccc;.:odl:.;cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
"#;

pub const OPENSUSE: &str = r#"
  ______  _   _ _____ ____ _____
 |  _ \ \| | | / ____/ ____| ____|
 | |_) | | | | | |  | (___ | |__
 |  _ <| | | | | |   \___ \|___ \
 |_| \_\ \____/ \_____|_____/_____|
"#;

pub const GENTOO: &str = r#"
   _____
  /  __ \
 |  /  \/
 |  \__/\
 |  /  \/
  \ \__/\
   \____/
   _/  \_
"#;

pub const SLACKWARE: &str = r#"
   _______________
  |  ___________  |
  | |           | |
  | |  Slack    | |
  | |___________| |
  |_______________|
"#;

pub const RHEL: &str = r#"
   .---.
  /     \
 |  R H  |
 |   E   |
 |    L  |
  \_____/
"#;

// --- Top derivatives ---

pub const MINT: &str = r#"
  \_____/
   \   /
    \ /
   _/ \_
  (     )
   \___/
  (     )
 (       )
"#;

pub const MANJARO: &str = r#"
    ___
   |   |
   | M |
   | A |
   | N |
   |___|
"#;

pub const ENDEAVOUROS: &str = r#"
   ______
  / ____ \
 | |    | |
 | |____| |
  \______/
"#;

pub const POP_OS: &str = r#"
  ______
 |  __ \__
 | |__) \ \
 |  ___/ |
 |_|     \_\
"#;

pub const MX_LINUX: &str = r#"
  __  __ __
 |  \/  |\ \
 | |\/| | > |
 |_|  |_|/_/
"#;

pub const ZORIN: &str = r#"
  ______
 |__  /
   / /
  / /
 /_/
"#;

pub const ELEMENTARY: &str = r#"
   ______
  / ____ \
 | |    | |
 | |____| |
  \______/
"#;

pub const KALI: &str = r#"
  .;dk0KXXK0kd;.
 .x0KXXXXXXXXXK0x.
 .0XXXXXXXXXXXXX0.
 kXKx;.......;xKXk
 KX.  .;oo;.  .XK
 kX  .xXXXXx.  Xk
  Xk  ;xxxx;  kX
   Xk.      .kX
    XKxxxxxKX
"#;

pub const PARROT: &str = r#"
  .''''.
 |  P   |
 |  A   |
 |  R   |
  '.__.'
"#;

pub const GARUDA: &str = r#"
   _____
  /  _  \
 | | | | |
 | |_| | |
  \_____/
"#;

pub const NOBARA: &str = r#"
  _   _
 | \ | |
 |  \| |
 | |\  |
 |_| \_|
"#;

// --- Server / Enterprise ---

pub const ALMALINUX: &str = r#"
    _
   / |
  | |
  | |
  |_|
"#;

pub const ROCKY: &str = r#"
  ____
 |  __ \
 | |__) |
 |  _  /
 |_|
"#;

pub const CENTOS: &str = r#"
  _____
 / ____|
| |     ___
| |    / _ \
| |___| (_) |
 \_____\___/
"#;

pub const ALPINE: &str = r#"
   /\
  /  \
 /    \
/______\
"#;

pub const ORACLE_LINUX: &str = r#"
  ____
 |  _ \
 | | | |
 | |_| |
 |____/
"#;

// --- Others ---

pub const NIXOS: &str = r#"
  _  ___
 | \|_ _|
 | .`| |
 |_|\___|
"#;

pub const VOID: &str = r#"
 __      __
 \ \    / /
  \ \  / /
   \ \/ /
    \__/
"#;

pub const SOLUS: &str = r#"
  _____
 / ___/
 \___ \
 ____/ /
/_____/
"#;

pub const PUPPY: &str = r#"
   __
  /  \
 ( o.o )
  > ^ <
"#;

pub const FREEBSD: &str = r#"
  _____
 |  ___|
 | |_
 |  _|
 |_|
"#;

pub const RASPBIAN: &str = r#"
  ____
 |  __ \
 | |__) |
 |  _  /
 |_|
"#;

// --- Windows / macOS ---

pub const WINDOWS10: &str = r#"
                                ..,
                    ....,,:;+ccllll
      ...,,+:;  cllllllllllllllllll
,cclllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll

llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
`'ccllllllllll  lllllllllllllllllll
       `' \*::  :ccllllllllllllllll
                       ````''*::cll
                                 ``
"#;

pub const WINDOWS11: &str = r#"

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
"#;

pub const MACOS: &str = r#"
                    c.'
                 ,xNMM.
               .OMMMMo
               lMM"
     .;loddo:.  .olloddol;.
   cKMMMMMMMMMMNWMMMMMMMMMM0:
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.
 XMMMMMMMMMMMMMMMMMMMMMMMX.
;MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
.MMMMMMMMMMMMMMMMMMMMMMMMMX.
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.
 'XMMMMMMMMMMMMMMMMMMMMMMMMMMk
  'XMMMMMMMMMMMMMMMMMMMMMMMMK.
    kMMMMMMMMMMMMMMMMMMMMMMd
     ;KMMMMMMMWXXWMMMMMMMk.
       "cooc*"    "*coo'"
"#;

// --- Fallback (Tux) ---

pub const FALLBACK: &str = r#"
   .---.
  /     \
 | .   . |
  \  ~  /
   \_/
  (   )
   ( )
"#;
