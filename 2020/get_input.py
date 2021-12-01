from typing import List


def get_input_from_file(file: str) -> List[str]:
    with open(f"input/{file}", "r") as f:
        data = f.readlines()
        data = [d.strip() for d in data]
    return data


def get_input_prob6():
    inp = """
        adgvrhblps
pghsdrbmalv
hrlbpdasgv
bgvsdplahr

lgnpfhrm
hwmng
gunhmo

txkeafsbgjuizd
etmcgdbfajuz

xdtzjioqavmchsbfrkp
bzjkriqmvxedotpcf
azjckxmqovtidbprf
fcxmevrkojzpdibqt
roifztvxmbpwcndkjq

ylcixrdoejn
dyoejlrcxin
dnorleijxyc

u
u

nfjrt
hvgqxculeszok
pmwryfdiab

gh
ebzkr
byqusikr

b
b
alyr
t

jzbafspmynwgqdeuklxc
anbkgduyexjlzmpcfwq
zxdkiujqeglcfynwpamb
qwcubamjpxyngkdlzfe
upadgfkywmnxeqbhjzcl

g
l
g
g
pg

mazi
mnzdai
zmia
izma

pqzl
pzql
zvciplq
szpqrl
plczqi

mquiosghkwtjz
tgnewskmjuozqi
gjwzmqcntosuik
ybtjoqzuwmfgisk

nvwocedxyqiahgt
zbjvipyfeacuwrt

ojkhxmwqs
kqxhmrzjwo
mqkjxhwo

t
t
t
t

dhrxaqejfng
fhdlbxejqra
fqaeldhjrx
seaqjxzdfthr
adjhrqlxef

t
bczrx
ueit
nu

idtmkhnlpbsqfxcevjry
pricynubfvdhjqesmktlx

wt
nw
w

mhqe
ey
e

kwclmuovtihabd
aqheucldfwno
adwnhoclqugez

lbwnohgs
osnbghw
swogbhfn
hnbosgw
gsowhnb

jsxyihpaotdlvckgeznqfbmwr
xkhfnwitojasqyrgcelpmbvd
synvibmdpfxwqltgcjohkare
grxicqelsovtdfhujwkanypmb

druazhicbm
spqizhucrdabmv
imucbayhgzdr

fnx
unqx

zjm
qfwrycva

uktvygcmf
yfktcxmv
ctlkvqysfmb

j
jng
j
j

t
tw
t
t

vogs
ogsv

tzgqxfauw
wgfxqkuaz
dkzawuftgq
gbuwaqzfjo

iphexunlbs
tayqgzmkjfdcrvow

fcdplxkhjbngwzqmiry
khuyrlgvxnpfbdmosqizjc
djzyxikqnmbrlgfhcp
jxycgeidnhlkzqpbmrf
xckrhgbzfnjlpdmiyq

lezco
ezloc
zolec
ocelz
oeczl

emgluf
flebmhg

knafibd
kbrnfd

xktmidfrz
fajlidmrtzy
wftzdm
omeqbfgdzpnut

tndpxaveifhcmzyjrsqlub
oyanuslpqdcevfih

hblawudzknspf
gxnstrmiyqpejo

lesyaikhnvoguwftzrmqdcpx
pxuqoayiwjzvglstrhfkmcend
uvyfepmxokdrhzlcsniagtwq
ynfmkwxqalpdrezhoguvstic

grscfn
azlgdcn
ovcgzn
lwxgctan
mgkbiyjhpnc

ji
ji
iaj

ivtkp
ikwp
ipko
pkiw

p
p

yushxnqa
uokzcaxni
xalvuyn

elw
wle
wlre
wleq
uwel

bvfxe
ebvfx

roaygfv
udazcljhy
pbaryv
aptxy

zdpkslmvnhyfua
czhfkylumrvgoqaspb
twenpsyzkjmihavuflx

zsv
bnzjfcamkw
rvdzq
ltypirz

q
w
q
q
q

uic
uic
uic
iufc

rdve
cvzre
fqls

gxmnabhd
xdbwjngmra
gmxebadny
bgahdxomn

gnoyc
olgy
poyfjg
wsdouiyghqe
jyog

q
e

adszvgclfomph
qavsicdlghozpfm

czfhs
hzfo
zhf

luzrpeivwagyfktjhnob
tlwynevkojgpbfuihr
gklbpjvyoituefrwhn
vhypwujfrbktonedigl
byknejlowfrptghivu

xbu
xbu
xub
bux
xub

vcqtxyf
tqcvfy

vumkixjhaozgfbneydpr
yxeazjmvbnhukpigodfr
pjnomhaxuvdkbiryzfge
oezrxngvpajfkdmuyibh

mkgdruoiawfh
dsumegkpjoq

oucrtxbqienfvsmlzjkdwgpa
akrqisvxbudelwgpmnozcj
xreauzqglpcvmoidbskwjn
mwgpljcusakrbozxvndeqyi
jdrlvcimoewkpxgqasnuzb

dshfqywgbc
eiqflapjh
vrmjfhnq

kojresxi
ubclnpghf

kuitbfh
ofdkqt
fztk

lsawunycojgvi
ornsyijx
yojpdsnbxri
tjysrinoe

ezuqslamgi
alimgeuqz
amgfqzluiew
nmalugivwqze

lrm
mr
vmrgt
mr
mlr

ybcnizhwt
ycbmztih
octgiqsyhxz
hktzcfyilew

mqigxepzwudokytanf
ayztcxbigunordqlmhk

wnfy
wzfp
fjlewksb
pcfwyn

o
c
na
qku

zg
gtl

syr
pbnmqc
j
sk

abenmqvsyotzrcjwglfdx
cnmqrxatjvowbylfgdezs

kfzlgphuwqv
levuhkwqcpfg

iazxrty
xryz

ikxcpebaruwhlm
iqtxadbeyckuvsh

wkfsrvgbyjxpq
rqbyxwfpkjgsv
fgwsyjqkxovrbp
frpyqxbjvwsgk

po
o

arbnupkvxqodcg
znpobhraxugvdckqt
mkaupgobdvscnqrjx
cbqknjavpgwxuord

ftpka
vtifcl
dfwusmbyrhto

y
q

drsleaqghypbtinmfuo
gzinmrwudjbpyloefqahs
fiomngyurlpheqdabs

v
bx
c
v

kh
zk

qkzicvjbgawfdpxmtsloye
ylofxwpvkqemsaigzjtcdb

qf
fq
raf

cbvxjytmsa
tbmxcvyjsa
ybmstxjacv

aekpuyliz
eqpyxls
rwyepchjflomg
oedybpl
evnyclxqp

yxtfnavdhopmcw
qlugbvjcz
xvcnymkrh

yksamve
yksmvxea
mskavye
yamdswevk
eksayvmx

p
k

psrgzxmaif
ifraxgzo

aehoplsvfqr
jfqvkpoas
vsjfxpqcoma
asmfoqpv
uvospqaxf

ercgdyism
msrvebzc
mgrescq
desfrmico

dborguxznkyqciavtfs
kzoymfndrsaihqtcbvgxu
anbcwfrdgjxktlyivouzsq
bgifuzsrekocxydqtnva
dtpsikyavoucxqgbhfznr

jvyhfbpxtkizmlesw
fvmjhkesbpzywixtl
btlezmshjwpxiyfvk

jawdspkghblyvornxmicuq
nrujdhxokcwgylimvqpba
lcrnhktgvipyubwqdomjax

vhiatyrwksodemcxn
hdsamtvuikjqecnwryox

ixthorcaevkp
xiyhrknlboczwjfm

ck
kc
czkmyet
kc

wxjqkpgnshue
qojwkefphzcrgaxn
ektphqjxgnwu

raksq
mdu
a
asb

putzomyciwaqkvbfsldng
absdvyntgikruqpzoflcmw
dkscznaiwulvogqfybmjpt

sxbqjv
jqvxbs
vlqsxjbo
xsbqjv
jvqxbsy

ynbakgvc
yvtnlbsjg
vrqxephwfby

xhdjkrpltfoe
qjplehkotdbfrx
kxfdaotsperlhj

ewgsmi
jafkti
yqkf
copzldurbx

xc
ex
r
rtwce
slk

zimewg
nmzrj

qylbm
byqml
lqmyb
mxyqlbr

egsktr
erskdgt
rkesgt

zkgqireysupwjthdb
swkoyzdcqibpvnr
kyvmwzpirdqsfbclx

jvytcgmkeqalzonhx
kvcjnwztxdmlyoaqg
lznqaxcvjytogkm
vabgfulyxkjoimqztcn

q
q
q
fh
q

xykitfhn
xnhytzilf
tiznfhx
tfxihnre
xhilzytfn

zmxyoakgt
yawiomrgsd
oafgmyqr

lpgqshiyuxjvabz
lzbuhyiqsjagepx
urisxlzjabpyqgh
xsuiqljypzgabh
gkylpamihsotqfdxzcbjwu

jhf
htdejm
jtabh
cjeigmzh
suljqyorxh

hcjkbvmlaoen
faojknvxmehblyc
nhmovlejcbka
vonhckilembja
kojnmvebtclha

clamzrewgonksuqvpb
zusorlkmqpnvwaecgb

uka
ea
aw
a
lxa

lg
lwtg
tgl
glw
rlg

vfsxidctgah
dvhcftsxgia
vdhxgimfsant

tschpyqa

ckn
xckznb
mnsvlfqaitjpdk
cwkzn
kzoyenb

ro
ro
or
or
oer

icumvltoqafdy
tfsxarpnh
atsfgw

btfenmovgusqzijxpcrhdakl
diapqfxrnevlzutsycjgbkom

utvbhgw
vbhgtwu

nkfgajmvbpyowthxr
cwxrtknpmgfazjoy
rwxmfatgouckjypnd

kdlubgiap
podcivgyueafkhm
purgnkixazd
sukdgixap
udkpglia

ajnumbsgr
ibmjsrkagc
bsgozmra

vkbso
ykglhto
gmodikp

sxupqv
xvuqps

x
r
c
x

fjoyuhztseqrawpciglx
igfeplsxtrhaqyzojwuc
rpwgucisoyhflaxtzejq
yqzjphegfuxltawicsor
wzahifloypuxegjscqtr

xeironzdugvqc
quevgsfronxzc

wdjrsknlbihcqzxuv
bzlxjnukvhrsdicwq
cuqiwsxkzvjbrndhl
ubclivhszjdwqrnxk
jldqkihxrzuvsnbwc

dzklrogfvmn
xuwpaqcetij

d
o
m
d

xhgzbj
jvbhg

snbcamefqhuovxkjpz
htckmpybdgvrsaoeuj
pkcbewsgvojmhauidl

i
k
tqj

koyawh
auo
qaoer
zrapo
ioa

broqhaylgsmxzkjtuvf
hatgluorzvkyxsmfjb
whytbkrvsumgzxoalfij
tzabxgfrqnusomvhjykl

hoplnck
ayqxdjofr

dmjspac
dhoazjp
idxfrwepyjo
qlpjgtvndub

riabvstyxphgoez
projxgyhietsavz

jhfqikpmz
kfpiyjvmzsqh
vphkqizjmsf
wpqfmlzkjheid

diap
zkhacy
waog
fojgas
roa

rujzxoygliqnmvkpc
vlonrgpjqzyxmfkic
zgxqkjocirpvlynm

lzgja
jlza
kjazrcl

vteiqrpkjybsmndhfl
lsndjpqertkvhbimyf
lpjmnbyqshvtdfirek
vpibqrehmsnflykjdt

iwqlhotpjsa
slhjyuiotrdk
szjhlocimt
tiajhwsofl

poz
dp

yeqdmto
zgtqyvoli

ahkfsocmdgnwp
cpwhjsmgodakf
clhudwakftvogsepi
dkargcowsphf

p
p
p

eqbcr
jceq
xeocqmi
fqwcve
mjebocq

xkei
ivuaexqk
kxie
ikxe
kxei

jsmzk
cmdpyf
xshm

hdwlcgzyuvke
gjlaqpry
rlgomxy

comszwhfrqlpnvytei
iqpvwolsmnfhray
oulphgswymvfrqnix

lcisfxnzwy
csixywfbnlz
zelcfyiwsxn

lmrsia
gowals

lyknfogpu
zoiw
tesjova
moes
odi

ktfajxloz
ojaxzflk
akzlfoxj
fxkolazj

iwxzmukbhoy
xlzkymbiuwo
bkuzomwxtiqyd
wmokbizxyul

kr
kr
skr
kr
rk

bdpfgnwxtk
gwtkdflnjpzx
kngxpfwdt

xkhmzvylowsgucrfepjai
xriqvhjlozpdkuemcfa

avxoebdgzmqphfytcn
cvfrmneolxtyzdgpqh

wjvslhmufgqxcnodz
fswclgdvxnumo

t
uqi

xopmekybt
pkrxytebm
mtpxbyek

vkxcrizwuhtad
wairuvhdmzcxkt

vrlpifawedksqzhcgnybtjxo
qoplwufekztbhdnvxgarjsciy
svzmteygfbkrlxohjqcpaidwn
rakocwstbpidlhxjgeqzfnuvy

o
o
o
o

ihzarybenqmtpg
zqbiyhanpgetrm
hqizynrpbagetm
yhbgrzeqnipatm
gteahmprbyqinz

jv
uxkasvt
jrgv
qv
ivbpld

sxigewcouvbj
xjwobcsugive
cbjvxsuowneig
uxobwigsjcev
ejogiwbxvcus

enmvubp
pvubm

qrjythvlzubgeikafo
necadbmrtzulvwypx

rbuslgk
lgsbuzkr
lbkugrs
sgkbulr
rublgks

whb
wbh
whb
bwh
whb

khtcw
wkcth
wtkch
ckwth
kwhtc

mkzdbo
dkjz
kbnzlmd
duytksvzgq

brkxvtjpdhqw
ntjqizashkpvwd
dqfrptwkshmvzj
phegdjolwvkqtu
dkhnwqacjpmtvf

bdrioxsqahp
adishrbqoxp
oiqpsbdtyrha
dapobrxsqih

oqsgiptmzlwcxkhvrfedu
rpwhtlgkcuidvqfosazxjem
rmgkeisxzhqwopltcfvud
vfcxiruwlhgoszmekqdpt
hmprfdvoxetulgizsqwkc

luzam
mwuplz
mlurjz

xlpmhzudwriytscobnkfq
uinamwhrtpobfvqceyd
ibflutdsohnqrpcymw
nqprtwuyfdcmiohb
ufhwmrinodkcbtpjyq

ns
ns
sn
sn

dufzbegpjrsaiqcthv
pfzqdjhatbuvescrkig
tbehcpfsvagdqirjuk
tebiravqjpfushwgcdx

gpjafzel
mzgftlead
bleynhgvfcs
egdqfalmz

zbarcdwiukgoqhmtnyjps
sujqytgrbnwzdmhalicok

fyvgdcbqx
fqcgdyin
etfmoqgdsjyc
gqfcdy
fyqcngd

am
bjm
sdm

frvkaupe
qzipevkya
vzabpkesd
bevpymka
pkxeva

t
t

dcveyjgtfkbuo
ebogjfydkvcut
gkoyfeudtcvjb
mvygqckjftuodbe
tujbkeocdgvyf

bkthnlgp
tnghb
gthnb
gnhbt

hpkybmeowacsrgnijfl
jzrsgyuwickpfbmolneh
hcflvrgqpwmdoknsjeyib

qxincam
zdjrlksqhgt
cbafqwo
vqyei

izdkxrpuqgjte
asyngkxrudpvmie
bdkwpgmexruci
opdiknuxger
gecixdrpluok

jymuwvfrixclhga
invyhuglxmcjw
imwghculxvjny
gjmlhiwvyctux
yvjhixugwolnmc

xbcwhv
chdwgxbv

ujycfnsvgqkpwheal
erugwaykpqcnhomlf

gyk
glhrk
gwk
qowygk

vcnju
vuoijbczt
dvcujg

rlsy
ryls
ylsr

tjn
t
n
rixz
t

i
i
i

elq
qle

ksahwvdjbtfle
lfksaebjhtwdv

qfacst
qfaskg
fqasoktb
vmfuilqzprasd

apuxh
uxpah
onahmwupxb
hxpqau
huqapsx

hjbnwua
huwbranj

dj
z
h
hz

hzqo
oqz
uwsozq
zyqxo

ckotpfx
lhrcnpmgfviwobd

wecahmszdjnoix
hmxaijswzydtcgn
njcqtmzixsadlh
cudmxzeasjnh
mxzahjckbnpdvrs

lsqahvixforkw
fxiqskaohw
afjhnxqowsik
xstiqkhmoafpbw

htdcbm
ex
sxq

pfgkrjhub
nptjfixgraubykhld
pmufkbrhjvg
jhrfbgpvuk

sptnugvdfmckehw
wknfhmsucertpvg

zolicbnrvtdwhy
oqdunzywp
zodykuwxgspn
wsanojdzxqy

druisbhpckvwofna
wvcboafsnhipukd
bpsadlcvnkhfziuo
bqfujsvmhcoakitdpne
visaodpfuchkngb

ewniqoulvgstzchkxdmpyajrbf
cbyihmnuradvlktxpjwsogzfeq

zgsdqwj
qdzsg

ombl
yeulqbnimw
hljodbmk

wtzkorqhupxby
yzxhtkwpord
skwzpjtgyrxho

mstgrlwo
rpyowgdts

pidn
ipesrfv
owpdim
pibt

tu
tu

pcdatkohjgbelmqvfxwnu
ukevynsprjtclaxhgmdqfwb
twqungiobxflhckapedjvm
pgqvtowflujnmadbeckxh
damfntwkjlcvhbqepxgu

ilksfovqamrhngcdjyexbt
htkvnbrqaefsloxgciymjd
qfgvrxmbaoeldnkyiswjhtc
ftxpoyhbjkcevdasmigrlqn

oq
qdo
sdlqoa
eopbizq
sndqo

iblpgzoekrnmwhjstay
tsropbkwgyaexilujn
salrwjgbtikpeyon
bsyirktpjonxualewg
teaqljpowyrigbknsd

d
d
d
d

zgqmhnlvekcyutxsbifw
zmkqgwvyjrcdfbu
ckoawvyzgumrfbqd
fdqbypumcgwvzk

krqlncwuxvgbdpsmyjfa
lmuybxdqsvkrjfcpwnga
zkbqnhjspdcumrxgaelyfwv

vafsedqzyrtnjpwug
gvcustypjedaw
owpydujgtaivsmke
tvadlgweiypsju
jebvtoysphugadw

pyixcohfrvtenaugsdmjq
catjqrivdfseogunymx
tvquyscedoxrgifjman
qagtfiocrjmuvdsenyx
gmeufoqaxcrnjdvstyi

p
p
p

jagycukemiqdxw
cmuyadqgkewixjl
fqawimgzjkyuxbdec

mahdekwibs
admswkhei
debamswk
maovdyeskw
mbsdkweaf

wfqimrc
rtqhcwlgdxnkvpo
arwysquc

kroumefnixa
lcn

hvaugnsf
nfxqkugas

wheyqtmdvljucnsbia
hvmyteblnuqacw
ehnultwbyacmqv
mlybuhqtcewvna

pxhafcvymglkzoq
xhwkctlgabz

or
jodr
or
gro
forha

fzmohqld
szqf
iqawckfrp

hkxuwmsbdrpt
ptysjhxkwbmru
tpmbsrkgwh
kphmtrusvwbq
krmhpftsbwzn

r
r
otr
r
r

sxtlkvjacpbngoziewq
jiskzqanlowbxpvtge

r
r
r
r
r

bqwsz
wsfb

zwvyqaichxjre
whvayjrzicxqe
arxcivhewqyzj

mpzasbnregjdwlxuqoft
dnxlrwubzkaosypq

gfaizbqlchy
zibatqykfu
bsufzjqxyiam

fsy
sf
efs
fs
fs

asmugcxhbekl
dkhcluwsmegaxb
kubhcmalegsx
cgluxmbhekas
bajrhelscgumknx

qdpwfec
wjdqfecy
cwdefqj
wqvdclsfe
dcqpfew

r
jcowmyplv
itdru
shbfr

soublfhe
lksfzjdymaw
ywfnrskcl

zsybelirjfwm
sfxiznhmablgtj
jbcifusqlopmdz
znhjerlkfmybis

knvmejaydo
mjoaevkw
jkmhcoave
agomevkyj
elqftajmoskvi

ziajfk
zakifj
kazfij
zfajki
fziajk

ujzyfat
tyuafjz

xihdrjyzvl
axujhtldyr
xdohklyjr
xrlejhyd
bxolyrfkjdh

i
i
i
i

ocnxzlprutqmhsij
xgmloizutqj
xmtlgioujqz
guoxlamzitjq

jhoepxcuwvn
xeuvnfaodjwpc
upjoxwcevn
xncelrvojupwg
qeusjvwynkpotxc

etdilxy
lytdiex
hdeltgryiw
lyixvedt

rge
gr
grp
gr
grp

iquzsnh
qsfunwh
qtsgprvhcn

lipmbftsh
myho
uqzngarkvw

ybrtackvdgqsmonlezuw
ywlzmsgbtocakedrqnv
rbceqauytzgmowkldvns
tyncgkldqserbjoavwmzh
oydsvatnmwzgceqkrlbx

qjb
irgfqjope
jkq
jqb

ncjlbpzedywkshfmqvagxi
jfipaqbmevkgdxnschztl
gbvxnclahmsiqjdekzfp
kedsxbzcvijglphmqfna
szcadkbnpeghjvqifxml

nfteow
ytewforn
estonfuw
erwtnfo

vhawcorfpxydzieqmbg
opyqhzdabfwcegrximv
vbpfqzcierhdgawoxmy
dvprwqzfioagxbycehm
yhdvpxzimqeorcabgwf

bfnrkixsut
jhywbrncziaxkp
iodknxrb
xbrknli
xkfironbu

xwpqbhyakf
rxayi
ogaxy

tnwl
wbnl
lhwn

wjuroesafgizkmvplxc
nxvgfoeucjmprikwlsy
glwrfcumhejisvkpox
kjmuvrwclifaxepogs

kjxygrvsodimabecwuzh
ihmxrogvjsaudbkwcezy
ykwovzjimbgashrudxec
jsewruvymigxdzkohabc
icmleouvhyzxdabjsgkwr

wm
wm
miwkg

slcdjyhbnpi
bxqvejnzwtfag

a
a
a
a
a

jxwonlf
jox

wxhytgp
tpgywx
pgwyuxt
pgxyfwta
dpyzwxrg

uygwfkxvane
hsdicwzkrenfgalqx
jaknwfxge

mxs
msk
smx
msoqz
ms

bxfwc
wcbxf
cxfbw
wkcbxf
cfwxbl

om
om
a
n

ikp
wi
iu
biej
i

aokgqimrnlfbdxzjs
tmqxrbislngykoajd
lsgvboxnkjdqrima
orijsalbqnmdkgx

zfpd
biok
bxcqnk
yikx

sulwztbnqkvioafxchemr
qhmcvlexkazuinfrwbsto
mnfutiblxecdqwvkoszhar
bmxzpqrlkafheouvcsntwi
zmchneutwlqoakxbsifrv

equacsmywgpkixfdv
anxwlcuypskjgfivbqr
iyaksgxmvoufehwpqc

twfzqxsnr
aqdsp

yj
dyoje
yj
yhqj
jy

bhwsczaxunjr
xgcswaflpznruqb
ubzwjxkcsyrnmahd

xyvgkjo
kgjvoyx
vxyokjg
govjkxy
jgyxokv

hneolzrtywq
atbs
mtipgd
vts

flu
lfu
ufcl

ad
da
da
ad
tgqad

xfw
bufw

simdubgowtap
wgbsdomaiutp
bsmodwauigtp

qi
o

n
dthokli
nveq

nvdt
cdglvi

sdxfaveuwlqzgircnhyobtkjp
uiqjrdenyagwkzhvpxscobtfl
otiungzxecdbyksrfwqjhvlap
dtbcyojehfwvsnrpkliuzgaxq

zgovqbryphxmjcasuktfl
bwcpqoutvfjykmashrzlgx
iecjhpmxsqytzorflungvabk
vacztqslhmkgoxyrpufdbj
amgfxjqsptdrhovclkzuby

s
s
fs
s

b
b
bm
b

uzc
zbc
cz

clmeswqpuahi
spahequmlwci

hrclmu
nhlcdurfxzi
orshcluv
uheclr

r
k
r

da
ad
ad

hlfmnga
amhgn
hgamn
geahwnm
malgnuh

bhpgndjwecroq
iwxklmtyzsopf

uqosfv
lmbihrxtedk

gyrtdosqjnkalicmwvh
rmlygiwktqcoshdavnj
wimjknqvaglhdsocrty
wkosyadhtqgnmcrvijl

tcpsjerzykogwmqbxinlad
iyzxcgrtqosnbkjaleudmw

dyzsmnkvuegbwlt
iwrcgs
fpswgqc

nawtjrgmpyxlibcshfudvkzq
jiblrgwyhtmxvucqz

vcruatwneqxi
myhgldscjnfb

qakljbptgwryiuxnmoh
mtquwoagbjknxyrlpih
aktuhpgmqxbwjyonril
wqriogjmubxpknvyltha
mpkuhlxgjtanwoiyrqb

ptjrmc
ucfjpm
pjcsm

jofrkgdves
gsdkrivfe
dgsevqkxrf

fvs
vclsd
s
rehtuyi
vfk

eafupodb
eyrdpgbujfoa
ukadqobefiwxp
pfgudeobay

i
p
i
i
i

udlqpozmayew
dsyalxmupwe
lipydaumwe
delyumpwsa

zthwnjiuyosakfpxbclm
lcpyfmisbztuknaxo
xsnlcyfzbpokmatui
zblkpsficaxmyoutn

n
nt
r

meskuctxrwaflovdpzhj
koxzmhvrcydlbwpsjuae

l
pqd
lh
f
h

otzqusgdeyvc
aetugnycqrxz
fhzwtgjqmkyulip

o
o
o
o
o

frejdvqokushlxtcgwani
dmfesnxhwgluorjiayt
zdtispaxeobgjhunwfrl

retxhvnmzsjqpbodif
txnphyomczqfisedrjb

nmwra
rmvawn
arnwm
mnrwa
wnmra

yatwdlfbuc
flydbtwacuo
upaybixdlfvtcrw
tncdbalyswfu

lenfcbjv
jfecnvlb
jlvefcnbr
lvcgqfnaetbji

bkuycegrnpimoxvfzl
xpmklivbuyrcega
lbktipremguxcyv
pbmirkvglyceux
ykibxregmlupcv

rwjeyxizhgt
ywxetqgajzriol
wrigyezjtx
werygtjizx

az
ja

sumxgwqydjhe
whtdyeqguxsp

pncwmq
ansluwbycmxoe
cwdnmj
wqhkmcn

flucitkw
ilucwjktf
lwukticf
lcwifktu
flcubitwk

qlhnvft
hnvtfl

bwsegp
bmpgsew
gpsewb
bgwqesp
pgwbse

ib
mgjsb
bzk
bpmsgj
xb

bjerpdfuo
wjrbf

dkmxn
nkmdx
ndxmk
ndmxk

rnouk
j

m
m
t

mrzq
zpqj
qiz
vzq
zhqi

tkn
nrkt

yj
jsyqp
awyj
yj
jy

xl
dlx
olx

kqdluj
jlkvdqu
qdukjl
qdklpju
ukqldj

wzy
zy
zy
zy
yz

vea
aewk
berga
waek
lyaen

miaoyj
yjvmoiza
yimajo
jymaoi
yjoiam

azcrbo
bzero

fig
jbsagif
gfi

ed
de
ed
de

naezbtk
ubat
batku
baqvtglmo
treba

gozafjiuerdpyscb
lxuhqvmktwn

ysqdjnvwhagmzitpf
vszdapqhijcyln
pcazqibvhsdjyn
rjuaqhspydikznv

qhgazndtiwb
wtbqdahviou
zbqjhwgtai
qiabwntckh
siawbyfpmxtrheql

ej
je

vtqgbimyo
dpwihgamzovlbtf
tmgvboi
emntbvgcio
btgimvoy

kisynhablrjwempcouxqgdftz
xnzrkiufjpmghsqcadotwebyl
rdqezcmgajpwoylnbusxfhkti
shejiunzgdpwxylqmatrbfkoc

lcqenihaktdgurpvmysjfxzow
sldaifypqrkhcumextwvjongz
piyslcrajgfenhmdwuovtkzqx
esxwujvtarpzkoldmyfncihgq
xqfvgosbyruephwjdaztklincm

gyofwzc
jyluxh
dpimyv
rtsbyqa
myluej

iwcedhs
etdich
cwihaeb
ecihby
licvfeh

nwlfiydcst
wnldtsfy
lsnwdtyf
fdwytsln
tfsywdnl

diphkr
ihf

osagihfnuptwcyxqevz
ynzjfrpqoicklmuvbw
nckfilwvquozjpmy
qzwoivcfpyjun

tbjh
noyjvqt
ejzuligpxm
hnj
jysha

fycqbrk
kcndb
czblkq
blfcky

wpgk
putk
lqamxiv
rd
dcw

wmludkgnozfe
aoezwflgkmdun
uzekmdgflnwo
kgflnuzowedm
goezkndmwufl

zmbsxkiwqrh
grjcyiwptzseqxofd
iubrwqlkxzsvn
zaqswibmxr

soealywdutckrfnbigpxz
dntrgfaopclubeykzwsix
gnswdzialxrckybtfuope
fwodizuepagysxnktlbcr
sbdiefgptynkuwzocxalr

tkbry
fyxr
ory

sio
oi
oi
oi

fipqkjmorwcluenvzx
mvowrcjxsuqpfgikenl
cxrmwvkeulpfqnoij

jhqfvlnetysazio
osjqykft
otqyjfuxs

opfrbnw
dbqnhoftpruz
bgeyxrpnfol
royngfpbw

rvmxfogsbcjkpqltyhandi
xafqvhinsodltpkcwy

pqrkslwfijgdo
pymoxvzsg
vpszyog
sehgop

qnmxjglwavzirceu
advwiomgshuqnclzjer
zjqumrcgaiwlevn
kvugjnewclizasqmr
jilzanmtbgfrequvcw

szkftpg
stkpgzf
ptzkgsf

mefgopznxlusjych
iocqfmjlzgyxkpunhe

zemltjrnbso
stbxojkrzgmneqv
zpsjernombt
stjobrmzne

yblxcozivekmqa
epsrvyigmc

xs
s
s

bnmz
fhqg
dxsin
zsm

brpge
tpm
ptm
p
sp

utpsykci
ituxbykcwr

zvxityphsdnuagbmcjwoqrfl
qiuwkplbtczmnvyrsoxhafjgd
qynizxpujhfmbvdcgwleasrot

y
tx
y
h
y

fsqatkdlmpux
yxinuvasdpfml
tsdpxafblzomu
axtfpulsmdg
lxwfqpsajudmk

joanzmhfldbsgwcipkxuqrey
oclnpxwkyqszdjreagiumfhb
ufvoiyxzbrpwdgkcmjsqlanhe

ciqxenmrlh
ctrfhxiqpnl

uoxfbrglpwjkhzstidmcyq
hzslfyjxbdgecairompwkqut
ygolzckfbxdmqtpivjshrwu
kpoxzdlhurgimcsftbqywj

xhgfmplcvwrqneztju
vqbwxtiprfhl
wxaoytfrvhbsldpq

iop
qvxihy
io

ftlgqpix
bpigfl
ufbgiplr

ek
ekb
kclifmge
key
ersk

ytqglpedjriu
jidgqytrulpe
tgjyudrqplei
jeiytpurgdlq

anuqyij
auczniy
yanklsxivou
byacinu
inazyue

tmryzavsnfdoq
ngmjxatvosz

yurzemitvfqnksxpbdlhgjwa
juwvmadgcpytrkfqnbezishlx
wzudtksjrpevnxagfihlbyqm

a
xa
ya
a

gnukiszbvj
nyzbvpksjgmu
sgujnbzeivk
uzgbnksjv

xoq
qox

xlberjukaztfnoycqsgwip
tscfaexqwgkrpouzbylijn
owaruypenjqtbslizcgxkf
jzcuswiqgkrbnpyoxletfa

pxzmb
rzmxpb
bpxznmrc
umbgpzkyox
tzbmxp

grspmxbyhojdizwlevqfkctu
fixgbtedqomczrjhswupkyl
tlsgyobzpmcqkhrdxewfuij

xrhelazymwntkv
lhvamxeyrznw
vrmwalyhnezx

mdbjcizwxh
jpcbzdmxiwh

xakdgjtimlvzcshw
iskdxfjnlhwrmvtcz

yrli
lriz
rli
irol
ilr

lfamcngvojeqyzrkdspwi
slqcznmwdivkrfjopebayg
cwgodjahrqzyfivepknmsl
kyeuvpjmlnftsizrdogqcaw
kjvrcgypqzinwadmoefls

hfmuzdvyibakcwnsor
cezounhbsydwakmif

vsajkf
jvf
vofj

q
v

h
h
i

frgo
ogfr
grfo
rofg
gfro

saomp
mpdosa
posam
mspao
ompsa

qbxi
xbiq
bixoq
axbqi

oix
xoi
ixo

zqkved
ztvydqe
vdekzq

ubg
vgcub

wdotn
akp
hv
ipj

xfcprnv
dfpcrvu
jpvfirc
frphqvwboc
pcvfxr

prx
rlxp
rxp
ixpr
pxr

uaozgkwlbfxyvmi
zbkgoxalfwuvymi
yimugvxbkfolwaz

hwvlbzcauseydgpxrkq
lamwdrzhsbqefkvytxoicu
zktxwlnhysevqbraducj

vjuprhcl
vjfrlpm
jbplvr
pnvljrbfe

cwygrulzbe
elyzgwbu
uygezlwb
yebuglzw
ygzwuble

wbzecjtdrgiyqls
wlsycjgeizdr
zslyderwigcj
dwzcyjgerisl

qkvegzmnobpuc
pogzvqmkbn

cnepixbwhklmqzodvyautjgrfs
oepszjgmlcxkdiartnybwhfvqu
gvcxaintlrweupbzfsmykdjqho
xalwvjenruhpkmgfqtoysbdizc

omrc
tcer
bguaf
vhtk

rtednpkyxgcs
kxlsredupynt
bartsexnypfmqk

lqrabmznuhgjfi
chligsdnbjrazum
jpghuibarnzlkom
bamhwrngzjfliuye
yuignrzsamhlxbjt

aoslfqnjcghb
cnbflgjqaohs
oqshfljbncaxg

av
av
va
dvqa

lghqidarnczwfxyu
ylfnhqwczaiuxdg
wfdglzixauchnqy

pfgknomr
nfegr
vihnlcfdqgy
fgrns
awogfn

kfdntirsmqap
thjdknbugmr
nxfetkrmd

ydabnisroqeghk
gryenohbaiqsk
sqbingvyoakehr
ngbkyfaicsrhqeo

ghxtulq
ntgqxlsz
lgzqtxsi

qvr
mqljsu
wlqu
gpoqytkbzf
evqw

khybpm
rhboj

rmcqdblnto
qlcnmor
rhmzalcsoq
lgxcrmnqovd

wdfkpmalijbncuvr
qhnmikpzaygxwsovej
        """
    return inp

def get_input_prob7():
    return """
vibrant bronze bags contain 3 dim olive bags.
shiny teal bags contain 1 posh green bag, 5 pale indigo bags, 1 mirrored purple bag.
striped aqua bags contain 5 bright orange bags.
clear chartreuse bags contain 3 dotted black bags, 2 wavy olive bags.
light lime bags contain 1 posh silver bag, 5 clear orange bags, 2 light olive bags, 3 dull maroon bags.
light olive bags contain 4 striped turquoise bags.
shiny purple bags contain 2 posh silver bags, 3 striped silver bags, 5 shiny beige bags, 2 plaid chartreuse bags.
mirrored crimson bags contain 2 faded cyan bags.
shiny turquoise bags contain 5 dull purple bags.
dim red bags contain 2 dim salmon bags, 2 faded orange bags, 5 muted aqua bags.
vibrant yellow bags contain 5 mirrored white bags, 5 vibrant blue bags, 3 mirrored lavender bags, 1 wavy cyan bag.
posh salmon bags contain 1 dull black bag, 1 striped indigo bag, 1 muted silver bag, 2 vibrant crimson bags.
pale black bags contain 1 plaid cyan bag.
dotted salmon bags contain 3 wavy brown bags, 3 pale coral bags, 1 light maroon bag.
posh orange bags contain 5 muted green bags, 3 striped violet bags.
dull maroon bags contain 2 clear brown bags, 5 posh silver bags, 5 mirrored coral bags, 2 dim lavender bags.
bright lavender bags contain 3 dark chartreuse bags, 1 mirrored chartreuse bag, 2 striped orange bags, 4 striped bronze bags.
plaid white bags contain 2 pale aqua bags.
posh teal bags contain 1 muted crimson bag, 2 dark fuchsia bags, 2 dim black bags, 4 plaid cyan bags.
wavy maroon bags contain 2 dull magenta bags, 3 dark red bags, 5 dull green bags, 4 bright turquoise bags.
plaid teal bags contain 5 plaid plum bags, 3 light magenta bags.
plaid plum bags contain 3 striped lime bags, 5 clear maroon bags, 3 muted plum bags.
muted purple bags contain 5 muted fuchsia bags, 4 pale tomato bags.
dark gold bags contain 5 dim lime bags, 3 clear orange bags, 4 drab crimson bags, 1 faded cyan bag.
striped coral bags contain 4 pale aqua bags, 5 clear silver bags.
shiny chartreuse bags contain 1 muted plum bag, 3 vibrant tomato bags.
bright salmon bags contain 5 pale gold bags, 1 muted gold bag, 5 dark gray bags, 4 dull cyan bags.
dark crimson bags contain 1 plaid turquoise bag.
light coral bags contain 1 muted brown bag, 2 striped black bags, 5 dark gray bags.
dotted lavender bags contain 1 bright turquoise bag.
posh red bags contain 2 muted green bags.
dim turquoise bags contain 4 dull chartreuse bags.
posh lime bags contain 5 mirrored yellow bags, 1 striped silver bag.
wavy black bags contain 5 striped cyan bags, 4 wavy red bags, 2 dotted coral bags.
dotted brown bags contain 1 dim gray bag, 1 plaid tomato bag.
mirrored red bags contain 4 posh aqua bags, 4 dark gray bags, 5 dark turquoise bags.
plaid tan bags contain 4 plaid black bags, 4 dull fuchsia bags, 1 plaid plum bag, 3 dark chartreuse bags.
plaid bronze bags contain 2 muted lavender bags, 3 faded cyan bags, 3 mirrored chartreuse bags, 1 dull coral bag.
mirrored silver bags contain 3 dull bronze bags, 3 dim tomato bags.
shiny fuchsia bags contain 3 bright maroon bags, 1 vibrant tomato bag, 4 posh bronze bags, 1 striped bronze bag.
dotted plum bags contain 5 wavy fuchsia bags.
dim bronze bags contain 3 shiny red bags, 5 dotted chartreuse bags.
faded crimson bags contain 3 bright olive bags, 1 dark bronze bag, 5 drab crimson bags.
striped blue bags contain 4 drab blue bags.
posh purple bags contain 1 bright blue bag, 4 light black bags, 1 dotted violet bag.
shiny lavender bags contain 4 mirrored bronze bags.
dull beige bags contain 2 wavy chartreuse bags.
dim blue bags contain 1 bright magenta bag, 5 muted red bags.
pale yellow bags contain 1 dotted white bag.
mirrored blue bags contain 3 striped tan bags.
mirrored turquoise bags contain 2 plaid red bags, 5 muted red bags, 2 muted green bags.
faded fuchsia bags contain 3 wavy tomato bags, 1 vibrant red bag, 1 dotted green bag, 2 posh plum bags.
drab silver bags contain 2 muted fuchsia bags, 4 dotted gray bags, 4 dotted aqua bags.
clear teal bags contain 5 shiny maroon bags, 1 clear green bag.
dim brown bags contain 4 faded lavender bags, 5 striped lime bags, 1 dark aqua bag, 1 dark fuchsia bag.
muted beige bags contain 1 dim aqua bag, 4 plaid plum bags, 3 light white bags, 4 muted cyan bags.
muted blue bags contain 2 bright blue bags.
vibrant turquoise bags contain 3 muted crimson bags.
mirrored indigo bags contain 2 wavy lime bags, 5 bright olive bags, 5 bright black bags, 5 vibrant violet bags.
posh tomato bags contain 4 muted orange bags, 3 plaid white bags, 3 shiny tomato bags, 3 light beige bags.
bright gray bags contain 5 pale aqua bags, 3 shiny gold bags, 1 clear olive bag, 1 dull fuchsia bag.
pale green bags contain 4 light black bags, 3 posh purple bags, 2 clear chartreuse bags, 2 drab lime bags.
light orange bags contain 5 shiny chartreuse bags, 2 wavy blue bags, 2 wavy yellow bags.
light green bags contain 5 dark bronze bags, 4 light tan bags, 4 dim chartreuse bags.
shiny indigo bags contain 3 faded cyan bags.
dotted orange bags contain 2 wavy crimson bags, 3 dull green bags, 5 dark indigo bags.
dotted black bags contain 2 vibrant white bags.
plaid gold bags contain 3 mirrored bronze bags, 5 striped tan bags.
muted salmon bags contain 5 dull maroon bags, 1 vibrant tan bag, 1 dim purple bag, 4 dull chartreuse bags.
plaid salmon bags contain 5 dotted purple bags, 5 dim orange bags.
bright crimson bags contain 3 plaid maroon bags, 2 dim aqua bags, 5 dull magenta bags, 5 pale tomato bags.
dotted fuchsia bags contain 1 dark cyan bag, 1 striped magenta bag, 3 clear coral bags, 4 light purple bags.
dull blue bags contain 5 dim magenta bags, 1 mirrored maroon bag, 5 dark indigo bags.
dull orange bags contain 3 drab blue bags, 1 shiny beige bag.
muted red bags contain 5 clear brown bags, 5 striped turquoise bags, 3 dull fuchsia bags.
shiny coral bags contain 5 dark olive bags, 5 light blue bags.
drab violet bags contain 5 clear chartreuse bags, 2 posh orange bags, 3 pale purple bags.
clear blue bags contain 4 dull fuchsia bags, 4 faded purple bags, 3 mirrored plum bags.
vibrant beige bags contain 2 posh silver bags.
bright bronze bags contain 1 shiny yellow bag, 5 muted green bags, 3 dark gray bags.
bright orange bags contain no other bags.
light teal bags contain 3 mirrored magenta bags, 5 faded gray bags.
dark green bags contain 4 drab white bags, 2 drab green bags, 5 dotted coral bags, 1 mirrored black bag.
plaid silver bags contain 2 dotted bronze bags.
drab turquoise bags contain 3 bright white bags, 3 drab maroon bags.
dim magenta bags contain 5 dark fuchsia bags, 2 drab teal bags, 2 drab crimson bags, 2 dull fuchsia bags.
pale coral bags contain no other bags.
dull indigo bags contain 2 bright black bags, 1 drab lime bag, 5 light magenta bags, 1 faded orange bag.
dim indigo bags contain 5 dark maroon bags.
shiny gold bags contain 5 light black bags, 3 mirrored yellow bags, 5 muted plum bags.
faded lime bags contain 5 dark crimson bags, 3 shiny orange bags, 5 plaid tomato bags, 4 mirrored cyan bags.
faded cyan bags contain 3 shiny gold bags.
striped olive bags contain 4 vibrant red bags.
wavy white bags contain 5 posh silver bags, 5 mirrored gold bags, 5 pale black bags.
dim plum bags contain 5 dotted plum bags, 2 clear silver bags, 2 wavy bronze bags.
drab cyan bags contain 2 muted plum bags, 2 dotted bronze bags, 3 posh violet bags.
clear bronze bags contain 1 clear plum bag, 5 striped plum bags.
posh gray bags contain 5 clear chartreuse bags.
striped brown bags contain 3 dim olive bags, 1 light black bag, 4 vibrant crimson bags, 2 striped fuchsia bags.
wavy tomato bags contain 2 mirrored chartreuse bags.
dull white bags contain 1 mirrored brown bag, 2 dull green bags.
vibrant green bags contain 4 drab fuchsia bags.
dark tomato bags contain 3 light gray bags, 2 dull cyan bags, 4 striped silver bags, 5 dark fuchsia bags.
dim tomato bags contain 2 dark turquoise bags, 1 mirrored black bag, 3 posh maroon bags.
bright gold bags contain 5 pale aqua bags, 3 clear tomato bags, 1 dark yellow bag, 1 drab green bag.
dotted indigo bags contain 5 light purple bags, 2 plaid coral bags, 5 pale green bags.
plaid magenta bags contain 1 dotted bronze bag, 3 drab turquoise bags, 1 dark lime bag.
drab gray bags contain 3 dark gray bags, 5 clear turquoise bags.
dim gold bags contain 5 dark maroon bags.
dark black bags contain 5 dark violet bags, 2 dotted cyan bags.
posh chartreuse bags contain 2 posh magenta bags, 2 striped lime bags.
wavy beige bags contain 3 drab olive bags, 2 shiny beige bags, 1 faded purple bag.
plaid green bags contain 5 pale olive bags, 5 posh bronze bags, 4 bright tomato bags.
dotted blue bags contain 3 posh silver bags.
shiny cyan bags contain 1 mirrored turquoise bag, 1 striped beige bag, 5 bright silver bags, 3 light olive bags.
dark aqua bags contain 1 plaid black bag, 3 posh coral bags, 4 striped magenta bags, 2 mirrored turquoise bags.
drab salmon bags contain 2 striped bronze bags.
posh brown bags contain 3 bright white bags.
mirrored chartreuse bags contain 5 posh lime bags, 4 bright blue bags, 4 clear brown bags, 3 bright orange bags.
clear red bags contain 1 plaid beige bag, 2 posh brown bags, 2 shiny aqua bags.
wavy olive bags contain 4 clear maroon bags, 1 striped silver bag.
faded green bags contain 4 plaid red bags, 3 dim olive bags.
clear purple bags contain 1 plaid olive bag, 3 light chartreuse bags.
dull lime bags contain 4 dark tan bags, 1 light chartreuse bag, 5 vibrant silver bags.
dark red bags contain 5 faded orange bags.
wavy plum bags contain 5 dull teal bags, 3 clear maroon bags, 3 shiny tan bags.
dark white bags contain 3 muted red bags.
light turquoise bags contain 3 light black bags.
pale fuchsia bags contain 3 dim brown bags, 5 clear purple bags.
light maroon bags contain 3 mirrored turquoise bags.
striped fuchsia bags contain 1 dotted aqua bag.
mirrored magenta bags contain 5 striped white bags, 4 striped violet bags, 4 dull maroon bags, 5 striped indigo bags.
dark violet bags contain 2 mirrored black bags, 5 dotted fuchsia bags, 3 muted fuchsia bags.
muted cyan bags contain 3 muted salmon bags, 4 drab black bags, 2 posh green bags.
muted tan bags contain 4 posh coral bags, 2 bright fuchsia bags.
faded aqua bags contain 5 striped magenta bags, 1 dim aqua bag.
bright tomato bags contain 2 muted green bags, 1 light olive bag.
clear silver bags contain 4 wavy cyan bags, 3 bright orange bags, 5 mirrored coral bags, 3 light olive bags.
vibrant chartreuse bags contain 4 light gray bags.
dull olive bags contain 1 mirrored lavender bag, 4 dotted coral bags, 4 pale chartreuse bags, 1 dull coral bag.
pale gray bags contain 3 plaid orange bags.
wavy lime bags contain 1 bright orange bag, 2 wavy yellow bags, 2 light purple bags, 4 wavy indigo bags.
faded white bags contain 1 dotted violet bag, 1 dark maroon bag, 3 posh coral bags.
muted white bags contain 3 faded magenta bags.
wavy brown bags contain 4 vibrant yellow bags, 4 dull lavender bags.
clear violet bags contain 5 shiny tan bags.
clear olive bags contain 3 bright fuchsia bags, 5 dark maroon bags, 4 mirrored white bags, 5 shiny beige bags.
light beige bags contain 4 mirrored gray bags, 2 wavy brown bags, 3 pale blue bags, 4 striped silver bags.
plaid turquoise bags contain 1 vibrant aqua bag, 4 bright fuchsia bags.
posh coral bags contain 3 dark tomato bags.
wavy red bags contain 5 posh gray bags, 3 dim lime bags, 2 light tan bags, 3 bright blue bags.
wavy fuchsia bags contain 5 vibrant aqua bags.
faded turquoise bags contain 2 dark maroon bags, 1 pale indigo bag, 4 faded white bags.
clear indigo bags contain 4 pale purple bags, 5 dull green bags, 1 bright olive bag.
vibrant lime bags contain 1 light purple bag, 5 posh bronze bags, 5 drab blue bags, 1 bright black bag.
shiny bronze bags contain 1 plaid red bag.
vibrant plum bags contain 1 wavy black bag, 4 drab aqua bags, 5 dark cyan bags.
mirrored fuchsia bags contain 5 bright olive bags, 4 mirrored crimson bags, 1 dim salmon bag.
bright fuchsia bags contain 4 light gray bags.
bright silver bags contain 5 striped tan bags.
dotted gray bags contain 1 striped silver bag, 5 bright black bags, 2 mirrored yellow bags.
dark olive bags contain 5 striped lime bags, 1 bright black bag.
light lavender bags contain 1 dark bronze bag, 2 faded gold bags, 3 light orange bags.
mirrored green bags contain 2 faded orange bags.
faded blue bags contain 2 drab coral bags, 3 posh salmon bags.
vibrant red bags contain 3 bright cyan bags, 4 light aqua bags, 4 posh gray bags, 5 wavy purple bags.
dotted yellow bags contain 3 mirrored tan bags, 1 clear crimson bag, 3 light turquoise bags.
clear orange bags contain 4 pale coral bags, 3 posh silver bags, 2 dull fuchsia bags.
pale violet bags contain 5 light crimson bags.
mirrored beige bags contain 5 bright coral bags.
shiny tomato bags contain 4 dotted red bags, 2 plaid lavender bags, 5 dim orange bags.
muted bronze bags contain 3 striped tan bags, 3 faded orange bags, 2 faded maroon bags, 3 clear tomato bags.
muted fuchsia bags contain 1 dark maroon bag, 2 dotted bronze bags, 4 mirrored bronze bags, 1 faded cyan bag.
mirrored orange bags contain 2 plaid cyan bags, 5 wavy orange bags, 5 shiny aqua bags, 5 wavy tan bags.
light blue bags contain 1 mirrored chartreuse bag, 3 dim crimson bags.
bright turquoise bags contain 2 clear orange bags.
dark plum bags contain 3 wavy lime bags, 1 light tan bag, 3 light silver bags, 1 light lime bag.
wavy cyan bags contain 4 dull coral bags, 4 light olive bags.
striped cyan bags contain 4 dull lavender bags.
drab purple bags contain 4 shiny tomato bags, 4 bright orange bags, 4 mirrored gold bags.
faded indigo bags contain 5 mirrored indigo bags, 2 muted silver bags, 5 faded lime bags, 4 dim salmon bags.
faded orange bags contain 5 plaid chartreuse bags, 4 bright black bags, 5 light magenta bags, 4 wavy bronze bags.
wavy gold bags contain 1 shiny orange bag, 3 clear salmon bags, 3 plaid orange bags, 4 vibrant tan bags.
wavy blue bags contain 3 clear brown bags, 1 faded tomato bag, 5 drab green bags.
plaid violet bags contain 1 light blue bag, 5 drab purple bags.
wavy tan bags contain 4 dotted blue bags.
drab plum bags contain 2 muted silver bags, 5 shiny maroon bags.
drab fuchsia bags contain 2 muted maroon bags, 2 mirrored turquoise bags, 5 clear green bags, 3 light olive bags.
light violet bags contain 4 clear turquoise bags, 4 mirrored gold bags, 2 wavy chartreuse bags, 2 mirrored tan bags.
shiny maroon bags contain 1 plaid salmon bag, 4 pale brown bags, 1 dim orange bag, 1 wavy tomato bag.
drab crimson bags contain 2 dim gray bags, 5 dull fuchsia bags.
faded plum bags contain 2 striped turquoise bags, 5 light gray bags.
clear beige bags contain 4 faded orange bags, 2 mirrored black bags, 1 shiny red bag, 1 dark teal bag.
faded black bags contain 5 mirrored plum bags, 5 muted plum bags.
pale olive bags contain 4 muted yellow bags, 5 mirrored maroon bags.
plaid black bags contain 3 dark gray bags.
plaid fuchsia bags contain 1 wavy beige bag.
shiny white bags contain 3 posh orange bags, 5 posh blue bags, 4 faded white bags, 1 wavy crimson bag.
shiny lime bags contain 5 posh black bags, 2 mirrored bronze bags, 5 muted bronze bags, 2 posh violet bags.
shiny magenta bags contain 5 dark lime bags.
plaid tomato bags contain 5 wavy cyan bags, 3 clear brown bags, 3 dark olive bags, 4 vibrant white bags.
dark blue bags contain 3 mirrored black bags, 3 pale indigo bags, 3 dim cyan bags, 3 light olive bags.
muted lavender bags contain 1 dotted purple bag, 1 drab blue bag, 5 mirrored bronze bags, 3 striped violet bags.
faded silver bags contain 2 dim orange bags, 4 shiny chartreuse bags, 2 drab blue bags, 1 wavy violet bag.
shiny beige bags contain 4 muted green bags, 5 striped aqua bags, 2 dim black bags, 3 dull fuchsia bags.
vibrant indigo bags contain 3 pale red bags, 3 clear lime bags, 4 vibrant cyan bags, 2 pale tomato bags.
bright red bags contain 2 mirrored lime bags, 1 dim indigo bag, 5 bright black bags, 2 drab crimson bags.
light gray bags contain no other bags.
dim orange bags contain 4 clear chartreuse bags, 4 striped tan bags.
dull plum bags contain 4 faded aqua bags, 3 pale salmon bags, 1 posh gray bag.
dull crimson bags contain 5 posh red bags, 4 mirrored plum bags, 1 dull fuchsia bag.
pale blue bags contain 1 dotted red bag, 5 muted chartreuse bags, 3 clear green bags, 1 wavy beige bag.
dotted cyan bags contain 4 faded red bags, 2 bright gold bags.
mirrored lavender bags contain 1 vibrant white bag, 1 mirrored plum bag, 5 dotted black bags, 5 bright orange bags.
mirrored tomato bags contain 2 shiny chartreuse bags, 2 shiny bronze bags, 4 bright turquoise bags.
bright indigo bags contain 3 striped orange bags, 1 dotted lime bag, 1 shiny magenta bag, 2 light fuchsia bags.
drab brown bags contain 5 plaid magenta bags, 5 dim aqua bags, 4 vibrant aqua bags.
posh turquoise bags contain 3 bright tomato bags, 4 striped tomato bags, 5 dim turquoise bags.
shiny violet bags contain 1 drab gold bag, 5 plaid silver bags, 3 vibrant magenta bags.
bright green bags contain 5 dull aqua bags, 2 pale tomato bags, 1 posh lavender bag, 1 dim tomato bag.
striped tomato bags contain 4 bright salmon bags.
shiny black bags contain 3 drab aqua bags, 4 drab salmon bags, 1 dim turquoise bag.
dotted silver bags contain 4 plaid orange bags, 3 mirrored tan bags.
shiny red bags contain 4 dim lime bags, 3 posh bronze bags, 3 striped tomato bags, 2 vibrant aqua bags.
dim green bags contain 3 dotted blue bags, 4 faded cyan bags, 4 drab silver bags, 5 clear blue bags.
dull teal bags contain 4 striped orange bags, 5 bright coral bags, 4 bright gold bags.
posh beige bags contain 4 dark indigo bags.
clear green bags contain 3 drab blue bags, 2 dark maroon bags.
faded teal bags contain 4 mirrored maroon bags, 3 clear cyan bags, 4 plaid silver bags.
plaid maroon bags contain 2 plaid brown bags.
light yellow bags contain 3 dotted chartreuse bags.
dotted green bags contain 1 clear olive bag, 2 bright blue bags, 3 striped indigo bags, 3 dull indigo bags.
vibrant lavender bags contain 4 clear tomato bags, 1 posh tomato bag, 4 drab bronze bags.
dull red bags contain 1 dark red bag, 4 bright black bags.
dark chartreuse bags contain 2 clear turquoise bags, 2 clear coral bags, 2 vibrant magenta bags.
clear aqua bags contain 3 mirrored lime bags.
posh white bags contain 4 mirrored chartreuse bags, 1 light purple bag, 3 muted maroon bags, 2 pale olive bags.
pale cyan bags contain 3 plaid lime bags, 1 drab salmon bag.
mirrored yellow bags contain 4 light olive bags.
faded violet bags contain 2 muted red bags, 1 striped coral bag, 1 dark chartreuse bag, 3 vibrant aqua bags.
bright lime bags contain 1 muted chartreuse bag.
dotted teal bags contain 3 dark orange bags.
plaid beige bags contain 4 drab chartreuse bags, 5 clear orange bags, 1 dim orange bag, 4 dotted bronze bags.
muted violet bags contain 5 striped crimson bags, 3 dark gold bags, 4 muted magenta bags, 5 vibrant olive bags.
dotted turquoise bags contain 1 drab olive bag, 1 plaid turquoise bag.
dim yellow bags contain 4 dotted blue bags, 4 wavy teal bags.
light purple bags contain 1 mirrored yellow bag.
wavy turquoise bags contain 4 muted gold bags, 3 wavy orange bags, 3 clear tomato bags, 1 light tan bag.
vibrant silver bags contain 1 plaid red bag, 2 clear turquoise bags.
faded brown bags contain 5 faded gray bags, 3 drab maroon bags, 5 striped aqua bags.
posh indigo bags contain 1 wavy green bag, 5 dotted blue bags.
drab lavender bags contain 3 vibrant indigo bags, 2 faded black bags, 4 dull coral bags, 2 wavy lime bags.
vibrant blue bags contain 5 posh purple bags, 4 dark gold bags, 2 mirrored lavender bags.
posh blue bags contain 4 wavy bronze bags, 5 dull chartreuse bags, 1 muted teal bag, 3 bright black bags.
posh aqua bags contain 4 pale gold bags, 2 faded white bags.
bright yellow bags contain 1 wavy coral bag, 2 drab turquoise bags.
wavy lavender bags contain 2 faded red bags, 4 faded cyan bags.
dotted aqua bags contain 1 muted cyan bag, 2 muted black bags, 3 wavy chartreuse bags, 1 shiny magenta bag.
vibrant white bags contain 3 muted green bags, 2 bright tomato bags.
posh yellow bags contain 3 mirrored lime bags, 5 dark fuchsia bags, 1 posh red bag, 5 plaid cyan bags.
clear gray bags contain 4 wavy magenta bags, 3 shiny orange bags.
clear fuchsia bags contain 2 dull beige bags, 5 striped turquoise bags, 2 posh silver bags.
shiny tan bags contain 1 drab orange bag, 4 faded cyan bags, 5 dark teal bags.
faded bronze bags contain 2 shiny aqua bags.
pale indigo bags contain 1 plaid turquoise bag.
faded tomato bags contain 3 pale brown bags.
plaid indigo bags contain 1 dull beige bag.
dark salmon bags contain 3 wavy purple bags, 3 dull indigo bags, 4 dim blue bags, 3 dull green bags.
bright tan bags contain 2 posh bronze bags.
vibrant fuchsia bags contain 4 striped olive bags, 5 clear yellow bags, 5 muted fuchsia bags, 3 shiny plum bags.
dark gray bags contain 3 light gray bags.
posh magenta bags contain 5 bright blue bags.
mirrored bronze bags contain 5 bright olive bags, 4 light magenta bags.
posh cyan bags contain 4 light indigo bags, 2 dark aqua bags, 5 mirrored lime bags, 2 faded magenta bags.
light chartreuse bags contain 4 dark indigo bags, 2 wavy magenta bags, 5 dim white bags, 1 plaid bronze bag.
bright plum bags contain 4 dotted lime bags, 2 mirrored red bags, 1 plaid plum bag, 1 mirrored gold bag.
dark beige bags contain 4 mirrored white bags, 2 muted plum bags, 5 mirrored lime bags, 2 plaid teal bags.
light bronze bags contain 1 muted black bag.
bright magenta bags contain 3 striped lime bags.
dull green bags contain 2 light purple bags, 1 dull maroon bag, 2 dotted violet bags, 4 clear blue bags.
drab maroon bags contain 3 bright tomato bags.
muted gray bags contain 1 shiny plum bag, 2 posh fuchsia bags, 1 plaid black bag, 2 dim black bags.
pale aqua bags contain 5 mirrored plum bags, 1 dark fuchsia bag, 3 faded tomato bags, 1 striped aqua bag.
plaid yellow bags contain 2 plaid plum bags.
vibrant tan bags contain 3 light gray bags, 5 bright salmon bags, 3 pale green bags, 5 posh gray bags.
faded lavender bags contain 2 shiny yellow bags, 3 dotted black bags, 4 dotted purple bags.
muted aqua bags contain 3 mirrored blue bags, 2 plaid salmon bags.
wavy silver bags contain 3 posh lime bags, 2 striped tan bags.
pale lime bags contain 5 striped orange bags, 4 plaid turquoise bags, 1 dark lime bag, 5 muted cyan bags.
dark bronze bags contain 3 clear silver bags, 5 faded tomato bags, 5 light olive bags, 4 bright fuchsia bags.
dull gray bags contain 1 clear silver bag, 1 light purple bag.
clear tomato bags contain 3 wavy coral bags, 2 dim orange bags, 2 dim magenta bags.
dull chartreuse bags contain 2 plaid teal bags, 4 dotted purple bags, 1 faded tomato bag.
vibrant gold bags contain 2 striped aqua bags, 5 vibrant cyan bags, 2 dotted olive bags, 2 clear olive bags.
wavy green bags contain 5 dim lavender bags.
posh olive bags contain 3 striped blue bags, 4 striped beige bags, 4 dim violet bags, 4 muted blue bags.
vibrant brown bags contain 5 light purple bags, 1 bright orange bag.
faded yellow bags contain 2 dark salmon bags.
vibrant teal bags contain 5 vibrant brown bags, 5 shiny indigo bags.
drab black bags contain 4 bright magenta bags, 1 shiny green bag.
mirrored black bags contain 3 posh silver bags.
muted yellow bags contain 1 clear orange bag, 2 shiny gold bags, 4 wavy purple bags.
posh silver bags contain no other bags.
plaid coral bags contain 2 mirrored lavender bags, 5 drab lime bags, 4 pale brown bags, 4 dark maroon bags.
muted plum bags contain no other bags.
pale silver bags contain 2 dim brown bags, 1 light aqua bag, 4 shiny lavender bags.
dotted coral bags contain 2 dotted bronze bags, 1 clear violet bag, 1 vibrant magenta bag.
drab indigo bags contain 1 dotted crimson bag.
dim salmon bags contain 2 clear green bags, 4 muted chartreuse bags.
vibrant cyan bags contain 4 posh magenta bags, 4 clear violet bags.
muted olive bags contain 5 bright salmon bags, 2 dark silver bags.
drab blue bags contain 2 shiny yellow bags, 5 clear olive bags.
dark brown bags contain 3 muted cyan bags, 5 posh fuchsia bags.
dotted red bags contain 3 posh gray bags, 5 clear maroon bags, 3 posh fuchsia bags, 1 dark white bag.
light crimson bags contain 2 dark chartreuse bags.
wavy chartreuse bags contain 1 muted red bag, 5 dull chartreuse bags, 2 wavy bronze bags, 1 posh bronze bag.
plaid lime bags contain 3 pale white bags, 2 dull chartreuse bags, 3 plaid olive bags, 1 vibrant cyan bag.
striped gray bags contain 5 mirrored blue bags, 3 dark turquoise bags, 2 clear aqua bags, 5 drab cyan bags.
dull cyan bags contain no other bags.
dotted beige bags contain 5 dull cyan bags, 2 dull purple bags, 4 mirrored white bags, 3 vibrant olive bags.
pale turquoise bags contain 5 pale beige bags, 2 pale olive bags, 2 wavy coral bags, 5 light fuchsia bags.
muted tomato bags contain 1 vibrant olive bag, 1 bright purple bag, 3 pale turquoise bags, 3 striped coral bags.
pale white bags contain 4 dull gold bags, 5 wavy olive bags, 4 faded red bags, 2 plaid teal bags.
clear magenta bags contain 4 dark violet bags, 5 plaid chartreuse bags, 3 vibrant yellow bags.
vibrant tomato bags contain 2 posh lime bags, 4 drab orange bags, 1 striped turquoise bag.
striped green bags contain 1 dim purple bag, 3 dotted bronze bags, 4 bright bronze bags.
wavy salmon bags contain 1 shiny lime bag.
plaid purple bags contain 1 muted tomato bag, 2 shiny lavender bags, 5 light olive bags.
clear tan bags contain 2 striped plum bags, 1 striped fuchsia bag.
dull coral bags contain 5 bright orange bags, 5 faded purple bags, 5 plaid chartreuse bags, 3 muted green bags.
light tomato bags contain 4 faded tomato bags, 1 clear chartreuse bag, 2 plaid black bags, 2 posh plum bags.
dotted lime bags contain 3 bright lavender bags.
plaid blue bags contain 4 plaid black bags.
dull silver bags contain 1 wavy magenta bag, 2 mirrored fuchsia bags, 4 striped salmon bags.
dotted maroon bags contain 3 dull cyan bags, 5 plaid lavender bags, 3 bright gray bags.
bright black bags contain 4 mirrored plum bags, 2 drab blue bags, 3 light gray bags, 1 posh coral bag.
pale salmon bags contain 5 bright gray bags.
muted chartreuse bags contain 1 mirrored lavender bag.
pale lavender bags contain 4 dim black bags.
dull fuchsia bags contain 1 bright olive bag, 3 dull cyan bags, 3 bright tomato bags.
posh fuchsia bags contain 4 striped cyan bags, 1 shiny purple bag, 5 muted lavender bags.
dull tan bags contain 2 light magenta bags.
mirrored olive bags contain 5 clear maroon bags, 3 bright cyan bags, 2 vibrant plum bags.
plaid chartreuse bags contain 2 pale coral bags, 1 posh lime bag, 5 light olive bags, 2 bright orange bags.
muted teal bags contain 3 plaid teal bags.
dim violet bags contain 3 striped tomato bags, 1 dotted fuchsia bag.
striped yellow bags contain 2 mirrored brown bags, 3 faded cyan bags, 1 clear silver bag, 5 wavy orange bags.
faded salmon bags contain 5 striped coral bags.
striped turquoise bags contain no other bags.
dim white bags contain 5 clear coral bags.
dull violet bags contain 4 striped violet bags, 5 dotted olive bags, 4 pale gold bags, 2 vibrant olive bags.
posh plum bags contain 2 bright orange bags, 5 faded tomato bags, 3 pale brown bags, 1 posh silver bag.
wavy orange bags contain 3 dull maroon bags, 1 drab orange bag, 4 posh plum bags.
dotted purple bags contain 2 mirrored white bags.
dark indigo bags contain 3 muted green bags, 5 dark white bags, 4 drab olive bags, 5 vibrant tomato bags.
shiny silver bags contain 2 pale green bags.
shiny crimson bags contain 3 wavy chartreuse bags, 2 wavy olive bags.
dull salmon bags contain 5 plaid plum bags.
bright brown bags contain 4 clear tan bags.
wavy aqua bags contain 1 dotted tan bag, 4 bright turquoise bags, 1 wavy maroon bag, 4 shiny cyan bags.
mirrored cyan bags contain 1 pale green bag, 5 plaid chartreuse bags, 5 muted chartreuse bags, 1 faded purple bag.
dark yellow bags contain 5 mirrored crimson bags, 2 shiny beige bags, 5 mirrored brown bags, 4 muted aqua bags.
faded gray bags contain 5 wavy cyan bags, 2 dim olive bags, 5 wavy gray bags.
drab aqua bags contain 3 shiny purple bags, 2 dim gray bags, 3 wavy cyan bags.
vibrant salmon bags contain 2 light indigo bags, 4 pale maroon bags.
drab yellow bags contain 1 light purple bag, 5 muted fuchsia bags, 2 drab blue bags, 4 muted green bags.
dark maroon bags contain 4 posh coral bags.
drab orange bags contain 4 bright tomato bags, 4 faded purple bags, 5 pale brown bags.
dim teal bags contain 4 shiny gray bags.
dotted bronze bags contain 2 drab blue bags, 1 light magenta bag.
faded maroon bags contain 5 wavy cyan bags, 1 pale gold bag.
vibrant gray bags contain 3 dull coral bags, 4 faded lime bags, 3 mirrored turquoise bags.
wavy yellow bags contain 3 striped bronze bags.
mirrored white bags contain no other bags.
pale chartreuse bags contain 5 drab blue bags, 3 bright black bags, 1 mirrored lavender bag, 4 dotted magenta bags.
posh crimson bags contain 1 mirrored lavender bag, 1 clear cyan bag.
dim coral bags contain 2 posh brown bags.
striped crimson bags contain 3 dim gray bags, 1 light turquoise bag, 3 wavy bronze bags, 4 faded orange bags.
posh violet bags contain 1 dark teal bag, 4 posh red bags, 3 vibrant lime bags.
light salmon bags contain 3 plaid salmon bags.
plaid red bags contain 2 dull lavender bags, 1 posh plum bag, 4 faded cyan bags, 1 plaid turquoise bag.
pale maroon bags contain 1 plaid turquoise bag, 4 faded maroon bags, 4 shiny yellow bags, 1 pale purple bag.
striped magenta bags contain 2 posh turquoise bags, 5 wavy indigo bags, 4 plaid tomato bags, 3 dim lavender bags.
striped silver bags contain no other bags.
striped gold bags contain 5 bright brown bags, 1 dotted crimson bag, 2 bright olive bags.
clear turquoise bags contain 3 wavy teal bags, 2 muted red bags.
mirrored tan bags contain 5 dark yellow bags, 3 posh coral bags.
shiny blue bags contain 2 dull olive bags, 2 muted brown bags.
clear black bags contain 2 wavy teal bags, 5 plaid chartreuse bags, 4 dull coral bags, 5 dark yellow bags.
faded gold bags contain 5 muted teal bags, 3 bright white bags, 4 striped tan bags.
drab tan bags contain 1 faded black bag, 2 clear olive bags.
dark cyan bags contain 1 plaid black bag, 1 muted aqua bag, 5 bright fuchsia bags.
muted silver bags contain 1 clear olive bag, 5 striped indigo bags.
dim beige bags contain 1 muted tomato bag, 5 clear fuchsia bags, 1 faded coral bag.
striped red bags contain 3 plaid brown bags, 4 posh black bags, 2 dotted gray bags.
striped purple bags contain 5 light lavender bags, 2 dotted brown bags, 1 dull olive bag, 2 shiny aqua bags.
shiny orange bags contain 3 mirrored brown bags, 1 wavy bronze bag, 5 vibrant aqua bags.
striped salmon bags contain 4 bright silver bags.
shiny olive bags contain 4 pale gold bags, 5 drab indigo bags, 3 mirrored salmon bags, 2 muted gray bags.
pale bronze bags contain 3 clear cyan bags, 3 drab blue bags, 5 drab bronze bags, 4 shiny gray bags.
plaid aqua bags contain 5 pale yellow bags, 4 pale black bags, 3 muted red bags.
faded coral bags contain 3 dark bronze bags, 5 striped silver bags, 5 clear olive bags, 2 wavy gray bags.
striped bronze bags contain 4 posh orange bags.
bright violet bags contain 5 light white bags, 1 dull olive bag, 5 drab fuchsia bags, 3 dim chartreuse bags.
wavy bronze bags contain 2 dark tomato bags, 2 muted red bags, 1 drab orange bag.
pale beige bags contain 5 muted lavender bags, 1 vibrant aqua bag, 4 drab lime bags.
dim crimson bags contain 4 plaid plum bags.
light silver bags contain 3 shiny yellow bags, 4 dull fuchsia bags, 4 dark chartreuse bags, 1 bright orange bag.
dark coral bags contain 1 clear indigo bag, 1 muted gold bag, 5 pale lime bags.
striped plum bags contain 5 plaid white bags, 3 pale gold bags, 3 pale yellow bags, 2 dim orange bags.
light tan bags contain 5 dark crimson bags, 1 clear silver bag, 2 striped tomato bags, 1 vibrant magenta bag.
shiny brown bags contain 4 bright bronze bags.
faded red bags contain 2 dotted bronze bags.
dim gray bags contain 1 pale gold bag, 5 shiny orange bags.
mirrored plum bags contain 2 muted plum bags, 1 posh silver bag.
shiny yellow bags contain 4 faded black bags, 4 light olive bags.
posh bronze bags contain 3 posh orange bags.
clear lime bags contain 5 mirrored lavender bags, 1 dark tomato bag, 4 dim aqua bags, 1 pale purple bag.
drab red bags contain 3 dark tan bags, 2 shiny maroon bags, 2 mirrored purple bags, 5 dotted orange bags.
dull purple bags contain 1 striped yellow bag, 3 faded cyan bags, 5 pale red bags, 4 plaid green bags.
muted magenta bags contain 1 drab yellow bag, 1 dark lavender bag.
dim fuchsia bags contain 4 pale aqua bags, 3 mirrored indigo bags, 2 wavy lime bags.
muted black bags contain 3 bright turquoise bags, 3 plaid cyan bags, 5 dim cyan bags.
dark lime bags contain 1 posh orange bag.
drab bronze bags contain 1 pale tomato bag, 4 light purple bags, 1 light olive bag, 4 posh silver bags.
posh tan bags contain 5 bright gold bags.
dim lavender bags contain 1 mirrored white bag, 4 posh lime bags, 3 dark fuchsia bags.
dark fuchsia bags contain no other bags.
muted maroon bags contain 2 bright white bags, 4 dark salmon bags, 4 posh gray bags, 4 posh plum bags.
dotted crimson bags contain 2 plaid salmon bags.
drab olive bags contain 2 dull lavender bags, 3 dark tomato bags.
wavy crimson bags contain 3 clear orange bags, 5 dull maroon bags.
dark silver bags contain 1 muted red bag, 5 dim bronze bags.
dull magenta bags contain 2 mirrored cyan bags, 2 mirrored plum bags, 1 drab olive bag.
pale magenta bags contain 4 dark olive bags, 1 wavy teal bag.
plaid crimson bags contain 1 dim purple bag, 1 shiny gold bag, 5 shiny tan bags, 2 striped silver bags.
vibrant magenta bags contain 3 striped turquoise bags.
light brown bags contain 2 clear magenta bags, 2 light lime bags.
drab lime bags contain 2 striped aqua bags.
light aqua bags contain 2 dim orange bags, 5 mirrored brown bags, 4 vibrant tomato bags.
dotted white bags contain 3 faded plum bags, 1 striped lime bag.
muted orange bags contain 4 clear purple bags, 5 light indigo bags, 1 plaid bronze bag.
dark turquoise bags contain 2 clear cyan bags.
striped beige bags contain 3 wavy yellow bags, 2 clear brown bags, 1 faded plum bag, 2 dotted bronze bags.
bright chartreuse bags contain 2 dim cyan bags, 2 faded lavender bags, 3 muted yellow bags, 1 dotted turquoise bag.
bright beige bags contain 3 vibrant silver bags, 3 faded bronze bags, 4 bright lime bags, 5 plaid lavender bags.
pale orange bags contain 1 striped tomato bag, 3 pale brown bags, 5 plaid bronze bags, 4 dark salmon bags.
dim black bags contain 4 striped turquoise bags, 2 plaid chartreuse bags, 5 posh red bags, 1 bright tomato bag.
plaid olive bags contain 5 dark indigo bags, 1 dark teal bag.
plaid orange bags contain 5 muted plum bags, 4 dark tomato bags, 5 dull crimson bags.
dark purple bags contain 3 pale beige bags, 4 pale gold bags, 1 vibrant blue bag.
mirrored salmon bags contain 3 plaid red bags, 3 dark plum bags.
vibrant aqua bags contain 3 clear maroon bags, 1 striped silver bag, 5 shiny gold bags, 3 faded tomato bags.
dull lavender bags contain 3 faded white bags, 1 dim lavender bag, 2 dull fuchsia bags.
posh green bags contain 5 dim orange bags.
clear salmon bags contain 4 bright black bags, 5 dotted plum bags, 2 striped tomato bags.
mirrored violet bags contain 2 wavy black bags, 5 dotted gold bags, 3 posh brown bags.
faded magenta bags contain 2 vibrant crimson bags, 5 drab orange bags, 1 dark gray bag, 4 striped coral bags.
light plum bags contain 4 muted plum bags.
dotted tan bags contain 1 wavy gray bag.
dim tan bags contain 5 mirrored bronze bags, 3 drab olive bags, 2 wavy olive bags, 3 dark tan bags.
striped indigo bags contain 2 drab green bags, 1 light olive bag, 5 bright orange bags.
vibrant orange bags contain 4 dull fuchsia bags, 1 shiny violet bag.
dim lime bags contain 5 striped silver bags.
plaid gray bags contain 4 dark tan bags, 3 dark magenta bags, 2 drab black bags, 3 faded bronze bags.
faded chartreuse bags contain 2 clear beige bags, 2 light beige bags.
striped orange bags contain 5 mirrored coral bags, 4 light gray bags, 2 mirrored white bags.
dim silver bags contain 3 posh coral bags, 4 dotted bronze bags, 4 muted yellow bags, 1 faded gray bag.
dotted gold bags contain 1 posh gray bag.
dotted olive bags contain 5 faded aqua bags, 4 faded brown bags, 2 dim salmon bags.
posh maroon bags contain 4 striped aqua bags, 1 wavy yellow bag, 1 mirrored crimson bag.
dark orange bags contain 3 mirrored fuchsia bags, 1 light magenta bag, 2 muted bronze bags, 2 wavy blue bags.
pale purple bags contain 4 dotted blue bags.
muted indigo bags contain 1 dark fuchsia bag, 2 posh fuchsia bags, 5 plaid turquoise bags, 2 mirrored cyan bags.
faded tan bags contain 1 drab fuchsia bag, 3 dark beige bags.
bright cyan bags contain 3 dotted blue bags, 5 muted plum bags, 2 vibrant crimson bags, 3 dark gold bags.
mirrored gray bags contain 3 pale lavender bags, 2 shiny blue bags, 4 dark salmon bags.
dotted violet bags contain 4 posh red bags, 4 bright tomato bags, 3 muted plum bags.
pale tan bags contain 2 clear crimson bags, 4 drab lime bags.
dull tomato bags contain 2 drab green bags, 5 dark crimson bags.
wavy coral bags contain 3 light aqua bags, 3 bright blue bags, 1 posh gray bag.
dull aqua bags contain 4 drab brown bags.
clear white bags contain 5 dark indigo bags, 5 dark maroon bags, 1 striped orange bag, 2 shiny gold bags.
dotted magenta bags contain 3 light olive bags, 4 dark indigo bags, 3 dotted blue bags, 3 striped lime bags.
plaid cyan bags contain 3 wavy olive bags, 5 dim black bags, 4 dotted bronze bags, 2 striped tomato bags.
muted lime bags contain 3 pale turquoise bags, 1 posh white bag.
wavy teal bags contain 2 dim orange bags.
muted gold bags contain 1 pale aqua bag, 4 shiny beige bags, 2 light olive bags.
wavy magenta bags contain 2 wavy purple bags, 5 wavy tomato bags.
dim maroon bags contain 5 plaid red bags.
drab green bags contain 5 light gray bags, 4 clear maroon bags, 2 dark indigo bags.
plaid lavender bags contain 5 drab blue bags.
light white bags contain 4 light tan bags.
wavy gray bags contain 1 posh coral bag.
mirrored teal bags contain 5 clear brown bags, 4 bright magenta bags, 1 drab brown bag, 2 dull gold bags.
light black bags contain 1 posh coral bag, 4 dotted black bags, 4 posh lime bags, 4 bright blue bags.
shiny gray bags contain 2 dotted blue bags, 5 striped turquoise bags, 4 pale aqua bags, 1 dim black bag.
posh lavender bags contain 1 wavy yellow bag, 2 dotted tan bags, 3 dull lavender bags.
faded beige bags contain 5 posh brown bags, 1 vibrant indigo bag, 2 light cyan bags, 1 clear aqua bag.
clear lavender bags contain 1 dull salmon bag.
mirrored coral bags contain 2 light olive bags, 5 clear olive bags, 2 pale tomato bags.
bright white bags contain 5 wavy yellow bags, 5 wavy bronze bags, 1 wavy olive bag, 5 muted red bags.
dark lavender bags contain 5 pale tomato bags, 4 faded white bags.
light red bags contain 3 posh turquoise bags, 3 dull indigo bags, 3 wavy silver bags, 2 drab salmon bags.
vibrant violet bags contain 1 shiny orange bag.
clear gold bags contain 1 drab magenta bag, 4 plaid tan bags, 2 vibrant yellow bags.
striped white bags contain 5 light salmon bags, 1 mirrored purple bag.
vibrant maroon bags contain 4 striped coral bags.
bright olive bags contain 2 light gray bags, 1 posh silver bag, 2 bright orange bags, 1 dark fuchsia bag.
mirrored gold bags contain 1 drab black bag.
shiny green bags contain 4 posh purple bags.
dim aqua bags contain 4 pale gold bags, 1 dull coral bag, 3 faded teal bags, 2 pale yellow bags.
dotted tomato bags contain 4 dull salmon bags.
faded olive bags contain 5 muted green bags, 1 drab crimson bag.
striped teal bags contain 4 dotted green bags, 5 muted aqua bags.
pale crimson bags contain 1 striped lime bag, 4 mirrored tan bags, 2 clear lavender bags.
shiny salmon bags contain 2 muted salmon bags, 5 shiny cyan bags, 4 faded red bags, 5 light tan bags.
plaid brown bags contain 5 pale orange bags.
posh gold bags contain 3 shiny crimson bags.
wavy violet bags contain 5 muted olive bags.
drab white bags contain 3 posh brown bags, 2 striped cyan bags, 1 clear coral bag.
light magenta bags contain 2 clear maroon bags, 3 light gray bags, 2 dotted black bags, 4 bright fuchsia bags.
bright aqua bags contain 4 plaid cyan bags, 2 clear black bags.
muted green bags contain 5 striped silver bags, 5 bright orange bags.
dim cyan bags contain 5 plaid brown bags, 3 striped tan bags.
vibrant olive bags contain 5 dark yellow bags.
dark tan bags contain 1 muted lavender bag, 2 mirrored turquoise bags, 1 dim lime bag, 1 dull olive bag.
mirrored brown bags contain 3 pale brown bags.
mirrored aqua bags contain 1 mirrored plum bag, 3 dark maroon bags.
clear cyan bags contain 1 dull cyan bag, 2 dark tomato bags, 4 pale brown bags.
dull yellow bags contain 4 wavy coral bags, 4 striped tan bags, 3 muted chartreuse bags.
mirrored maroon bags contain 3 plaid tomato bags, 4 shiny purple bags, 1 plaid lavender bag, 5 light gray bags.
drab coral bags contain 2 dark turquoise bags, 2 clear crimson bags, 4 drab lime bags, 5 dull crimson bags.
muted turquoise bags contain 5 posh purple bags, 2 dim magenta bags.
dull turquoise bags contain 4 striped aqua bags, 2 light fuchsia bags, 3 pale turquoise bags, 2 faded maroon bags.
bright maroon bags contain 1 muted red bag, 3 faded black bags.
wavy indigo bags contain 3 shiny beige bags, 5 dim lavender bags, 2 striped tan bags.
pale red bags contain 1 vibrant brown bag, 3 faded black bags, 4 posh turquoise bags, 5 plaid brown bags.
drab teal bags contain 1 faded purple bag, 2 mirrored blue bags, 2 vibrant tomato bags, 1 pale purple bag.
faded purple bags contain 1 vibrant white bag.
striped maroon bags contain 1 mirrored brown bag, 1 shiny black bag, 5 dotted lime bags.
dotted chartreuse bags contain 3 dim blue bags.
posh black bags contain 3 light aqua bags, 5 bright orange bags, 1 plaid plum bag, 5 plaid fuchsia bags.
pale gold bags contain 1 faded tomato bag, 2 dark tomato bags, 3 dotted blue bags.
muted brown bags contain 1 plaid brown bag.
vibrant purple bags contain 1 shiny gray bag, 5 dull green bags.
mirrored purple bags contain 1 bright gray bag, 2 plaid plum bags, 5 dotted chartreuse bags, 2 posh lime bags.
bright coral bags contain 3 dim blue bags.
dark magenta bags contain 4 muted maroon bags, 2 dark olive bags, 1 dull olive bag.
pale brown bags contain 2 faded purple bags, 1 muted green bag, 3 dark fuchsia bags.
drab gold bags contain 3 posh bronze bags, 2 plaid tomato bags.
pale teal bags contain 1 clear turquoise bag, 4 muted yellow bags, 1 posh tomato bag, 3 vibrant orange bags.
vibrant black bags contain 1 faded tomato bag, 5 dim white bags, 2 drab aqua bags, 1 vibrant silver bag.
dull black bags contain 3 pale yellow bags, 4 clear aqua bags, 1 shiny yellow bag, 3 faded maroon bags.
drab tomato bags contain 2 dim purple bags, 5 plaid fuchsia bags.
bright blue bags contain 5 posh silver bags, 4 dull cyan bags, 4 light olive bags, 1 mirrored white bag.
muted crimson bags contain 3 bright orange bags, 1 pale tomato bag, 3 posh yellow bags, 4 shiny purple bags.
shiny plum bags contain 1 dim turquoise bag.
clear maroon bags contain 1 posh red bag, 2 light olive bags, 1 dotted purple bag.
drab chartreuse bags contain 4 dotted green bags, 1 dark red bag.
dull brown bags contain 5 dim lavender bags, 4 bright tomato bags, 5 drab crimson bags, 1 vibrant tomato bag.
wavy purple bags contain 4 dim lavender bags, 2 plaid plum bags, 2 dim lime bags, 2 striped cyan bags.
vibrant coral bags contain 1 striped crimson bag, 3 drab cyan bags, 5 vibrant yellow bags, 2 dotted teal bags.
clear crimson bags contain 4 dotted purple bags, 5 faded cyan bags.
clear coral bags contain 4 clear chartreuse bags, 2 bright olive bags.
light cyan bags contain 3 shiny violet bags, 3 dotted blue bags, 3 drab beige bags.
light fuchsia bags contain 2 plaid lavender bags, 4 dull green bags, 2 plaid salmon bags.
dull bronze bags contain 3 bright magenta bags, 1 bright black bag, 2 wavy lime bags.
light gold bags contain 5 bright indigo bags.
drab magenta bags contain 5 faded fuchsia bags, 2 dim black bags, 5 dim crimson bags, 5 dotted fuchsia bags.
mirrored lime bags contain 2 mirrored white bags, 1 shiny gray bag.
dim olive bags contain 4 bright blue bags, 1 faded tomato bag.
striped tan bags contain 4 dark tomato bags, 4 mirrored coral bags, 2 mirrored lavender bags.
muted coral bags contain 4 pale salmon bags, 3 faded purple bags.
striped violet bags contain 4 dark gray bags, 3 posh coral bags, 2 striped turquoise bags, 4 bright fuchsia bags.
clear plum bags contain 3 bright cyan bags, 1 drab salmon bag, 2 vibrant brown bags.
clear brown bags contain 1 mirrored plum bag.
light indigo bags contain 4 drab blue bags, 5 mirrored chartreuse bags, 2 muted red bags, 2 dark beige bags.
dim purple bags contain 4 wavy teal bags.
clear yellow bags contain 4 clear maroon bags, 3 bright olive bags, 5 shiny gray bags, 2 bright orange bags.
pale tomato bags contain 2 dotted violet bags, 2 dark teal bags.
bright teal bags contain 2 shiny maroon bags, 2 muted indigo bags.
pale plum bags contain 5 posh gray bags, 3 shiny indigo bags, 3 wavy olive bags, 1 pale white bag.
dark teal bags contain 3 muted plum bags, 4 faded plum bags, 1 wavy bronze bag.
vibrant crimson bags contain 5 dark tomato bags, 2 dark white bags, 5 posh red bags.
shiny aqua bags contain 1 pale coral bag.
dim chartreuse bags contain 2 wavy teal bags, 5 mirrored black bags, 5 mirrored bronze bags, 4 muted lavender bags.
drab beige bags contain 2 faded coral bags, 5 muted tan bags, 5 plaid cyan bags.
bright purple bags contain 4 muted red bags, 5 wavy beige bags, 4 clear coral bags.
striped black bags contain 3 light red bags, 2 plaid chartreuse bags.
striped chartreuse bags contain 2 dotted gray bags, 2 wavy olive bags, 1 muted lavender bag.
dull gold bags contain 5 wavy olive bags, 2 posh plum bags, 4 shiny gold bags.
striped lavender bags contain 5 dim teal bags, 3 light blue bags.
striped lime bags contain 4 light black bags, 5 striped turquoise bags, 5 wavy cyan bags.
    """


def get_input_prob8():
    return """
nop +283
acc +26
acc +37
acc +6
jmp +109
acc +10
jmp +18
acc +5
jmp +327
acc -4
jmp +269
acc -7
acc +27
nop +7
acc +0
jmp +81
acc +42
nop +338
acc -5
jmp +391
nop +276
jmp +354
acc +22
jmp +528
acc +0
acc +20
acc +15
acc -17
jmp +537
acc -15
acc +12
acc -17
acc +17
jmp +34
acc -19
jmp +88
acc +19
acc +35
acc +17
acc +7
jmp +443
acc +22
jmp +584
jmp -2
jmp +408
acc +46
acc +43
acc +4
jmp +532
acc -19
acc -19
acc +38
acc -10
jmp +476
acc +1
acc +3
acc +19
acc +28
jmp +480
jmp +1
acc +32
acc -2
jmp +518
acc +5
acc -19
acc +19
jmp +344
jmp +99
acc +0
acc +30
acc -13
acc -19
jmp +385
acc -18
jmp +157
acc +15
acc +4
jmp +503
acc -6
acc +42
jmp +461
acc -6
jmp +328
acc -9
nop +199
acc +15
jmp +206
jmp +182
acc +35
nop +275
acc +3
jmp +1
jmp -25
nop -20
nop -6
jmp -7
nop +145
acc +4
acc +28
jmp +315
nop -76
nop +12
nop +170
jmp +291
acc -16
acc +5
nop -10
jmp +235
acc +6
acc -1
nop +492
acc +44
jmp +119
jmp +128
jmp +1
jmp +328
acc -7
jmp +126
nop +351
acc +9
acc +4
acc -1
jmp +276
acc +0
nop +133
acc +36
acc +32
jmp +173
acc +41
nop -95
jmp +153
acc +7
acc +13
acc -10
jmp +223
jmp +186
acc +4
jmp +90
acc -7
acc +15
jmp +366
acc +9
acc +27
acc +1
jmp +417
acc -19
jmp +268
acc +38
acc +1
acc +27
jmp +1
jmp +420
acc +13
acc +9
acc +1
jmp +370
acc +25
acc +3
acc -1
jmp +324
nop +352
acc +39
jmp +121
acc +15
jmp +348
jmp +11
acc -12
acc +23
jmp +407
jmp -6
acc +43
jmp -8
acc +48
nop +316
acc +5
jmp +323
acc +3
jmp +1
acc +34
jmp +191
jmp -160
acc -18
acc +33
jmp -79
acc +9
acc +50
acc -15
acc -1
jmp -100
acc -18
acc +49
nop -184
acc +20
jmp +404
nop +280
jmp +294
acc -12
jmp +1
acc +8
jmp +320
nop +387
acc +15
nop +359
acc -7
jmp +182
nop +1
nop -40
acc +3
jmp -38
acc +44
acc -11
nop +297
jmp +174
jmp -140
acc +32
acc +28
acc +8
acc +9
jmp -194
acc -9
acc +32
jmp +291
acc +43
nop +220
acc +9
acc +15
jmp -167
jmp -8
acc -3
acc +12
jmp +195
acc +48
acc +16
nop +137
acc +29
jmp +48
acc +11
acc +46
acc +22
acc -2
jmp -167
jmp +123
jmp +128
acc +24
acc +50
acc -10
jmp -202
acc -17
acc -13
jmp +1
jmp +89
acc -4
acc +41
jmp +111
acc +50
acc +41
jmp +83
acc -2
nop +194
jmp +239
acc +33
acc +25
jmp +347
nop +6
acc +0
acc -16
jmp +73
acc -12
jmp -5
jmp +188
jmp +1
jmp -264
acc +44
acc +6
acc +35
jmp +312
acc +28
acc +8
jmp -15
acc +48
jmp +215
acc -1
jmp -55
acc +22
acc -18
acc +47
jmp -266
jmp +1
acc +18
acc +0
acc -11
jmp +221
acc -10
nop -189
jmp -216
jmp -3
acc -8
acc +22
jmp +253
jmp -168
acc -7
acc +14
nop +315
acc +11
jmp -47
nop -36
acc +40
jmp +95
jmp +13
acc -14
acc -5
acc +48
jmp -85
acc -17
acc +20
acc -5
acc +6
jmp +221
acc +32
acc +7
jmp +12
nop +266
acc -11
acc -8
nop +182
jmp -184
nop -137
acc +48
jmp +155
jmp -124
acc +44
jmp +24
acc +12
jmp -292
jmp +195
jmp -301
acc +45
acc -14
jmp -66
jmp +86
acc +33
jmp -136
jmp -146
acc -3
acc -13
acc +16
jmp -183
acc +4
acc -8
acc +14
jmp -169
acc +35
acc +18
nop -24
jmp -127
jmp -219
jmp +190
acc -4
acc +1
jmp +62
nop +220
acc +18
acc +36
jmp +58
acc +25
jmp +21
nop -24
acc +2
acc +49
jmp -325
acc +24
acc +23
acc +13
jmp +143
jmp -45
nop +212
jmp -29
acc -12
acc -12
jmp -107
nop +126
acc +32
jmp -113
jmp +1
acc -6
jmp -102
nop +57
acc -16
acc +25
jmp -213
acc +19
acc +29
acc +0
jmp -320
acc +42
jmp +94
acc +6
jmp -363
acc -18
jmp -365
acc +39
jmp +13
acc +47
acc +24
acc +9
acc +25
jmp +151
acc +17
jmp +1
jmp -77
jmp +24
acc -13
acc -13
jmp -141
acc +22
acc +9
nop +92
jmp -334
acc +30
acc +11
jmp -304
acc +8
jmp -275
acc +35
jmp -95
jmp +1
acc -18
nop -407
nop -18
jmp +146
acc +37
acc -4
acc +19
jmp -409
acc +28
acc -10
nop +151
acc +17
jmp -418
nop +56
acc +40
acc -13
jmp -301
acc +28
acc -7
acc -6
jmp +62
acc +0
acc +6
acc +25
acc +26
jmp +18
acc -14
jmp +93
acc +43
acc +19
jmp -109
acc +24
acc +0
jmp -328
acc +42
jmp -165
acc -3
acc +18
jmp +153
jmp +1
acc -10
acc -7
jmp -199
acc +30
nop -403
acc -12
jmp -209
jmp -242
acc +38
nop +33
acc -10
acc +22
jmp -419
acc -18
acc +27
acc +22
jmp -57
nop -313
acc +20
acc -7
acc -10
jmp -371
jmp -159
jmp -478
acc +9
acc +7
acc +15
nop +72
jmp -358
jmp -138
acc -17
jmp +9
acc +47
acc -2
jmp -221
nop -331
nop -297
acc +12
acc -13
jmp +3
jmp -198
jmp -150
acc +17
jmp -313
nop -314
jmp +69
acc +0
nop -397
jmp -104
jmp -223
acc -14
jmp +44
jmp -61
acc -7
acc -18
jmp -270
acc -14
acc +32
jmp -177
jmp +84
acc +6
nop +14
jmp +47
acc +37
acc -19
acc -9
jmp -200
acc +11
acc -5
acc +2
acc +37
jmp -488
nop +19
jmp -490
jmp -491
acc +24
acc +30
acc +14
jmp -19
jmp -37
acc +19
jmp -540
acc +48
acc +22
jmp -434
jmp -196
acc +12
acc -9
acc +48
acc -5
jmp -433
acc +23
jmp -245
acc +43
jmp -228
acc +44
jmp -168
nop -221
jmp -102
jmp +1
acc +39
nop -153
jmp -455
acc +48
jmp -75
jmp +31
nop -383
acc -12
jmp -245
acc -2
acc +3
jmp -421
acc +38
jmp -158
acc +39
acc -4
acc -1
acc +0
jmp -186
acc +28
jmp -247
jmp +1
acc -19
acc +31
acc +34
jmp -148
acc +5
nop -417
nop -230
acc +11
jmp -162
jmp +1
acc +32
jmp -303
nop -214
jmp -332
acc -10
acc +33
jmp -142
acc +19
acc +41
acc +12
jmp -495
acc +42
nop -318
acc +36
jmp -524
jmp +1
acc +46
acc -6
jmp -582
acc +28
acc +38
acc -17
acc +2
jmp -432
acc +35
nop -550
acc -6
jmp -394
acc +38
acc +49
nop -99
acc +50
jmp +1
    """
