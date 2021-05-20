pub fn run(){
    let quot = '"';
    let mut string1 = String::from("");
    let mut string2 = String::from("");
    let string = "andas	andas
    andras	andRas
    ange	anje:
    anger	anje:R
    anges	anje:s
    anne	ann
    anse	anse:
    anser	anse:R
    anses	anse:s
    arkiv    'aRki:v
    av      A:v
    barock  baR'Ok
    bäste     b'EstE
    bet	be:t
    betalt    b'Et'A:lt
    bort	bORt
    bott     b'Ut
    botten     b'OtEn
    bra	bRA:
    dags	daks
    dan	dA:n
    dem	dEm
    dig	dEj
    dra	dRA:
    drar	dRA:R
    dras	dRA:s
    duger	du-gER
    därför	d'ERfY:R
    egen	e:gEn
    egna	e:gna
    emot    Em'u:t
    etc	e:te:s'e:
    ett	'Et
    far         fA:R
    fem         fEm
    fram     fR'am
    framme     fR'amE
    framåt	   fRamOt
    gemensam	 jEm'e:nsam
    gemensamt	 jEm'e:nsamt
    grep	gRe:p
    greps	gRe:ps
    gripna	gRi:pna
    ha	hA:
    har	hA:R $u
    hej          hEj
    helt	he:lt
    heta	he:ta
    honom	hOnOm
    idag	Id,A:g
    in        In
    inom        InOm
    ja	jA:
    jan	jA:n
    klar	klA:R
    klas	klA:s
    kvar	kvA:R
    log	 l'u:g
    mat	mA:t
    men	mEn
    mig	mEj
    måste	m'Ost@
    måsten	m'Ost@n
    monetär  m,OnEt'E:R
    nvda	Enve:de:A:
    ont	Unt
    ost	Ust
    par         pA:R
    per	pE:R
    program     pRUgR'am
    rad	RA:d
    raden	RA:d%En
    rader	RA:d%ER
    sade	 sA:dE
    sades	 sA:dEs
    sent	se:nt
    sex      s'Eks
    sig	sEj
    skolan	 sk'u:lan
    son	so:n
    sonen	so:nEn
    stad	stA:d
    staden	stA:dEn
    stadens	stA:dEns
    stan	stA:n
    stor	stu:R
    stort	stu:Rt
    susanna	s8s'ana
    susanne	s8s'an
    svar	svA:R
    ta      tA:
    tar	tA:R
    tas      tA:s
    togs	t'u:gs
    tomt	t'Umt
    tom	t'Um
    torsdag	 t'u:SdA:g
    torsdags	 t'u:Sdags
    torsdagen	 t'u:SdA:gEn
    torsdagens	 t'u:SdA:gEns
    tredje	 tR'e:djE-
    tyvärr	 t%yv'E:R
    usa	 u-Es'A:
    usas	 u-Es'A:s
    usa:s	 u-Es'A:s
    vad	vA:d $u
    valt	vA:lt
    var	vA:R $u
    vare	vA:RE $u
    vem	vEm ";
    for word in string.split_terminator('\n'){
        let v: Vec<&str> = word.split_whitespace().collect();
        println!("ord_med_fonem.insert({0}{1}{0},{0}{2}{0});",quot,v[0],v[1]);
    }
}