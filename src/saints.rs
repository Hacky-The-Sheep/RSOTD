use colored::*;
use std::collections::HashMap;
// let outside = "Martyr".truecolor(243, 139, 168).bold().italic();

pub fn months() -> HashMap<String, HashMap<String, String>> {
    let mut months: HashMap<String, HashMap<String, String>> = HashMap::new();

    // JANUARY
    let january: HashMap<String, String> = HashMap::from([
        (
            "2".to_string(),
            "St. Basil \nSt. Gregory Nazianzen \nSt. Marcarius".to_string(),
        ),
        ("3".to_string(), "St. Genevieve".to_string()),
        (
            "4".to_string(),
            "St. Dafrosa \nSt. Elizabeth Seton".to_string(),
        ),
        (
            "5".to_string(),
            "St. John Neumann \nSt. Telesphorus".to_string(),
        ),
        (
            "6".to_string(),
            "Sts. Gaspar, Melchior and Balthasar \nBlessed Andre Bessette".to_string(),
        ),
        ("7".to_string(), "St. Lucian".to_string()),
        ("8".to_string(), "St. Appolinaris".to_string()),
        (
            "9".to_string(),
            "St. Julian of Antioch \nSt. Peter of Sebaste".to_string(),
        ),
        ("10".to_string(), "St. Nicanor \nSt. William".to_string()),
        ("11".to_string(), "St. Hyginus".to_string()),
        (
            "12".to_string(),
            "St. Aelred \nSt. Benedict Biscop \nSt. Marguerite Bourgeoys".to_string(),
        ),
        (
            "14".to_string(),
            "St. Felix \nSt. Hilary of Poitiers \nSt. Malachias".to_string(),
        ),
        (
            "15".to_string(),
            "St. Paul the Hermit \nSt. Maurus, Abbot".to_string(),
        ),
        (
            "16".to_string(),
            "St. Marcellus \nThe Five Franciscan Protomartyrs".to_string(),
        ),
        (
            "17".to_string(),
            "St. Sulpice \nSt. Anthony, Abbot".to_string(),
        ),
        (
            "18".to_string(),
            "St. Prisca \nFeast of the Chair of St. Peter of Rome".to_string(),
        ),
        (
            "19".to_string(),
            format!(
                "{}\n{}",
                "Sts. Marius, Martha and Sons, Martyrs".truecolor(243, 139, 168),
                "\nSt. Canute, King and Martyr".to_string()
            ),
        ),
        (
            "20".to_string(),
            "St. Fabian, Martyr \nSt. Sebastian, Martyr".to_string(),
        ),
        ("21".to_string(), "St. Agnes, Martyr".to_string()),
        (
            "22".to_string(),
            "St. Anastasius, Martyr \nSt. Vincent of Saragozza, Martyr".to_string(),
        ),
        (
            "23".to_string(),
            "St. Raymond of Pennafort \nSt. Emerentiana, Martyr".to_string(),
        ),
        ("24".to_string(), "St. Tomothy, Martyr".to_string()),
        ("25".to_string(), "Coversion of St. Paul".to_string()),
        (
            "26".to_string(),
            "St. Paula \nSt. Polycarp, Martyr".to_string(),
        ),
        ("27".to_string(), "St. John Chrysostom".to_string()),
        (
            "28".to_string(),
            "St. Peter Nolasco \nSt. Thomas Aquinas \nSt. Agnes, Martyr,".to_string(),
        ),
        ("29".to_string(), "St. Francis de Sales".to_string()),
        (
            "30".to_string(),
            "St. Martina, Martyr \nSt. Mutien".to_string(),
        ),
        ("31".to_string(), "St. John Bosco".to_string()),
    ]);
    months.insert("january".to_string(), january);

    // FEBRUARY
    let february: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Brigid of Ireland \nSt. Ignatius of Antioch".to_string()),
        ("2".to_string(), "St. Cornelius the Centurian".to_string()),
        ("3".to_string(), "St. Blaise".to_string()),
        ("4".to_string(), "St. Andrew Corsini".to_string()),
        ("5".to_string(), "The 26 Martyrs of Japan \nSt. Apollonia \nSt. Agatha".to_string()),
        ("6".to_string(), "St. Dorothy \nSt. Titus".to_string()),
        ("7".to_string(), "Bl. Pope Pius IX \nSt. Romauld, Abbot".to_string()),
        ("8".to_string(), "St. Josephine Bakhita \nSt. John of Matha".to_string()),
        ("9".to_string(), "St. Cyril of Alexandria \nSt. Miguel Cordero".to_string()),
        ("10".to_string(), "St. Scholastica".to_string()),
        ("12".to_string(), "Seven Holy Founders of the Servites".to_string()),
        ("13".to_string(), "St. Agabus \nSt. Catherine d'Ricci \nSt. Polyeucte".to_string()),
        ("14".to_string(), "St. Valentine".to_string()),
        ("15".to_string(), "Sts. Faustinus and Jovita".to_string()),
        ("16".to_string(), "St. Onesimus".to_string()),
        ("17".to_string(), "St. Julian".to_string()),
        ("18".to_string(), "St. Bernadette Soubirous \nBl. Fra Angelico \nSt. Simeon".to_string()),
        ("19".to_string(), "St. Gabinus \nSt. Odran".to_string()),
        ("20".to_string(), "St. Tyrannio \nBl. Jacinta and Francisco Marto \nSt. Leo the Wonderworker \nSt. Claude de la Columbiere \nSt. Amata".to_string()),
        ("21".to_string(), "St. Pter Mavimenus \nSt. Robert Southwell".to_string()),
        ("22".to_string(), "St. Peter's Chair".to_string()),
        ("23".to_string(), "St. Peter Damien \nVigil of St. Matthias, Apostle".to_string()),
        ("24".to_string(), "St. Matthias, Apostle".to_string()),
        ("25".to_string(), "St. Walburga \nSts. Victor and Claudian".to_string()),
        ("26".to_string(), "St. Alexander \nSt. Porphyry".to_string()),
        ("26".to_string(), "St. Nestor".to_string()),
        ("27".to_string(), "St. Gabriel of Our Lady of Sorrows".to_string()),
        ("28".to_string(), "Pope St. Hilary \nSts. Rimnus and Lupicinus".to_string()),
    ]);
    months.insert("february".to_string(), february);

    // JUNE
    let june: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Angela Merici".to_string()),
        (
            "2".to_string(),
            "Sts. Peter, Erasmus, and Marcellinus, Martyrs".to_string(),
        ),
        (
            "3".to_string(),
            "St. Kevin \nSt. Clothilde, Queen".to_string(),
        ),
        ("4".to_string(), "St. Francis Caracciolo".to_string()),
        ("5".to_string(), "St. Boniface, Martyr".to_string()),
        (
            "6".to_string(),
            "St. Norbert \nSt. Philip, Deacon".to_string(),
        ),
        ("7".to_string(), "St. Robert of Newminster".to_string()),
        ("8".to_string(), "Sts. Medard and Gildard".to_string()),
        (
            "9".to_string(),
            "St. Columkille \nSts. Primus and Felician, Martyrs".to_string(),
        ),
        ("10".to_string(), "St. Margaret of Scotland".to_string()),
        ("11".to_string(), "St. Barnabas, Apostle".to_string()),
        (
            "12".to_string(),
            "St. John of St. Facundo [us] \nSts. Basilides, Cyprinus, Nabor, and Nazarius, Martyrs"
                .to_string(),
        ),
        ("13".to_string(), "St. Anthony of Padua".to_string()),
        ("14".to_string(), "St. Basil the Great".to_string()),
        (
            "15".to_string(),
            "St. Germaine \nSts. Vitus, Modestus, and Crescentia, Martyrs".to_string(),
        ),
        ("16".to_string(), "St. John Francis Regis".to_string()),
        (
            "17".to_string(),
            "St. Botolph \nSt. Ranier \nSt. Gregory Barbarigo".to_string(),
        ),
        (
            "18".to_string(),
            "St. Ephrem \nSts. Mark and Marcellianus, Martyrs".to_string(),
        ),
        (
            "19".to_string(),
            "St. Juliana Falconieri \nSts. Gervase and Protase".to_string(),
        ),
        ("20 ".to_string(), "Pope St. Silverius, Martyr".to_string()),
        (
            "21".to_string(),
            "St. Terence \nSt. Aloysius Gonzaga".to_string(),
        ),
        ("22".to_string(), "St. Paulinus of Nola".to_string()),
        (
            "23".to_string(),
            "St. Audrey \nVigil of the Birth of St. John the Baptist".to_string(),
        ),
        (
            "24".to_string(),
            "Birth of St. John the Baptist".to_string(),
        ),
        ("25".to_string(), "St. William, Abbot".to_string()),
        ("26".to_string(), "Sts. John and Paul, Martyrs".to_string()),
        (
            "28".to_string(),
            "Vigil of Sts. Peter and Paul \nSt. Irenaeus, Martyr".to_string(),
        ),
        ("29".to_string(), "Sts. Peter and Paul".to_string()),
        (
            "30".to_string(),
            "Commemoration of St. Paul \nSeventeen Irish Martyrs".to_string(),
        ),
    ]);
    months.insert("june".to_string(), june);

    //July
    let july: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "Bl. Junipero Serra".to_string()),
		("2".to_string(), "Sts. Processus and Martinian, Martyrs".to_string()),
		("3".to_string(), "Pope St. Leo & St. Irenaeus".to_string()),
		("4".to_string(), "St. Theodore".to_string()),
		("5".to_string(), "St. Anthony May Zaccaria".to_string()),
		("6".to_string(), "St. John Fisher & St. Thomas More".to_string()),
		("7".to_string(), "Sts. Cyril and Methodius".to_string()),
		("8".to_string(), "St. Elizabeth of Portugal".to_string()),
		("9".to_string(), "St. Maria Goretti".to_string()),
		("10".to_string(), "Holy Seven Brothers with Sts. Rufina and Secunda, Martyrs".to_string()),
		("11".to_string(), "Pope St. Pius I, Martyr \nSt. Oliver Plunket".to_string()),
		("12".to_string(), "St. John Gualbert \nSts. Nabor and Felix, Martyrs \nSt. Veronica of the Veil".to_string()),
		("13".to_string(), "Pope St. Anacletus, Martyr \nSt. Mildred \nSt. Teresa of the Andes".to_string()),
		("14".to_string(), "St. Bonaventure \nSt. Francis Solano \nBl. Kateri Tekakwitha".to_string()),
		("15".to_string(), "St. Henry II, Emperor".to_string()),
		("16".to_string(), "No Official Saint Today!".to_string()),
		("17".to_string(), "St. Alexis the Beggar".to_string()),
		("18".to_string(), "St. Symphorosa and Her Children, Martyrs & St. Camillus de Lellis".to_string()),
		("19".to_string(), "St. Vincent de Paul".to_string()),
		("20".to_string(), "St. Jerome Emilian".to_string()),
		("21".to_string(), "St. Laurence of Brindisi & St. Praxedes".to_string()),
		("22".to_string(), "St. Mary Magdalen".to_string()),
		("23".to_string(), "St. Apollinaris of Ravenna & St. Liborius, Martyrs".to_string()),
		("24".to_string(), "St. Christina".to_string()),
		("25".to_string(), "Vigil of St. James the Greater, Apostle \nSt. James the Greater, Apostle \nSt. Christopher".to_string()),
		("26".to_string(), "St. Anne".to_string()),
		("27".to_string(), "St. Pantaleon, Martyr \nPope St. Celestine I".to_string()),
		("28".to_string(), format!("{}", "Sts. Nazarius and Celsus, Victor I, and Innocent I, Martyrs".truecolor(243, 139, 168))),
		("29".to_string(), "St. Martha & 20: Sts. Felix II, Simplicius, Faustinus, Beatrice".to_string()),
		("30".to_string(), "Sts. Abdon and Sennen \nSt. Ignatius of Loyola".to_string()),
    ]);
    months.insert("july".to_string(), july);

    // AUGUST
    let august: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "The Chains of St. Peter \nSt. Samona and Her Seven Sons [Holy Machabees]".to_string()),
        ("2".to_string(), "St. Alphonsus Liguori \nPope St. Stephen I, Martyr".to_string()),
        ("3".to_string(), "The Finding of the Body of St. Stephen, First Martyr \nSt. Lydia \nSt. Peter Julian Eymard".to_string()),
        ("4".to_string(), "St. Dominic".to_string()),
        ("6".to_string(), "Sts. Xystus, Felicissimus, Agapitus".to_string()),
        ("7".to_string(), "St. Cajetan \nSt. Donatus, Martyr".to_string()),
        ("8".to_string(), "Sts. Cyriacus, Largus and Smaragdus".to_string()),
        ("9".to_string(), "St. John Marie Vianney \nSt. Romanus, Martyr \nVigil of St. Lawrence, Martyr".to_string()),
        ("10".to_string(), "St. Lawrence, Martyr".to_string()),
        ("11".to_string(), "Sts. Tiburtius and Susanna \nSt. Philomena".to_string()),
        ("12".to_string(), "St. Clare".to_string()),
        ("13".to_string(), "St. John Berchmans  \nSts. Hippolytus and Cassian".to_string()),
        ("14".to_string(), "St. Eusebius \nSt. Maxmillian Kolbe".to_string()),
        ("15".to_string(), "St.Tarsicius".to_string()),
        ("16".to_string(), "St. Rocco \nSt. Joachim".to_string()),
        ("17".to_string(), "St. Hyacinth".to_string()),
        ("18".to_string(), "St. Helena \nSt. Agapitus, Martyr".to_string()),
        ("19".to_string(), "St. John Eudes".to_string()),
        ("20".to_string(), "St. Bernard".to_string()),
        ("21".to_string(), "St. Jane Frances de Chantal".to_string()),
        ("22".to_string(), "Sts. Timothy, Hippollytus, Symphorian".to_string()),
        ("23".to_string(), "St. Philip Benizi \nVigil of St. Bartholomew, Apostle".to_string()),
        ("24".to_string(), "St. Bartholomew, Apostle".to_string()),
        ("25".to_string(), "St. Louis, King of France".to_string()),
        ("26".to_string(), "Pope St. Zephyrinus, Martyr".to_string()),
        ("27".to_string(), "St. Joseph Calasanctius".to_string()),
        ("28".to_string(), "St. Augustine \nSt. Hermes, Martyr".to_string()),
        ("29".to_string(), "Beheading of St. John the Baptist \nSt. Sabina, Martyr".to_string()),
        ("30".to_string(), "St. Rose of Lima \nSts. Felix and Adauctus, Martyrs".to_string()),
        ("31".to_string(), "St. Raymond Nonnatus".to_string()),
    ]);
    months.insert("august".to_string(), august);

    // September
    let september: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Aegidius \nSt. Anna, Prophetess \nSt. Giles, Abbot \nThe Holy Twelve Brothers, Martyrs.".to_string()),
        ("2".to_string(), "St. Stephen, King of Hungar".to_string()),
        ("3".to_string(), "Pope St. Pius ".to_string()),
        ("4".to_string(), "St. Rose of Viterbo \nSt. Rosalia".to_string()),
        ("5".to_string(), "St. Lawrence Justinian".to_string()),
        ("6".to_string(), "St. Eleutherius".to_string()),
        ("7".to_string(), "St. Cloud \nSt. Regina".to_string()),
        ("9".to_string(), "St. Peter Claver \nSt. Gorgonius, Martyr".to_string()),
        ("10".to_string(), "St. Pulcheria \nSt. Nicholas of Tolentino".to_string()),
        ("11".to_string(), "St. Paphnutius \nSt. John Gabriel Perboyre \nSts. Protus and Hyacinth, Martyrs".to_string()),
        ("13".to_string(), "St. Eulogius".to_string()),
        ("15".to_string(), "St. Nicomedes, Martyr".to_string()),
        ("16".to_string(), "Pope St. Cornelius, Martyr \n St. Cyprian, Martyr \n Sts. Euphemia, Lucy, and Geminianus, Martyrs".to_string()),
        ("17".to_string(), "St. Hildegarde \n Stgmata of St. Francis of Assisi ".to_string()),
        ("18".to_string(), "St. Joseph Cupertino \n Bl. John Massias".to_string()),
        ("19".to_string(), "St. Januarius and Companions, Martyrs".to_string()),
        ("20".to_string(), "St. Eustace and Companions, Martyrs \n Vigil of St. Matthew".to_string()),
        ("21".to_string(), "St. Matthew, Apostle".to_string()),
        ("22".to_string(), "St. Thomas of Villanova \n St. Maurice and the Theban Legion, Martyrs".to_string()),
        ("23".to_string(), "St. Thecla \n Pope St. Linus, Martyr \n St. Padre Pio".to_string()),
        ("25".to_string(), "St. Cleophas \n Bl. Herman the Cripple".to_string()),
        ("26".to_string(), "Sts. Cyprian and Justina, Martyrs \n Eight North American Martyrs".to_string()),
        ("27".to_string(), "Sts. Cosmas and Damian, Martyrs".to_string()),
        ("28".to_string(), "St. Wenceslaus, Martyr".to_string()),
        ("29".to_string(), "Dedication of St. Michael the Archangel".to_string()),
        ("30".to_string(), "St. Jerome".to_string()),
    ]);
    months.insert("september".to_string(), september);

    // October
    let october: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Remegius".to_string()),
        ("2".to_string(), "Holy Guardian Angels ".to_string()),
        ("3".to_string(), "St. Therese of Lisieux".to_string()),
        ("4".to_string(), "St. Francis of Assisi".to_string()),
        ("5".to_string(), "St. Placidus and Companions, Martyrs \nSt. Faustina Kowalska \nBl. Raymond of Capua".to_string()),
        ("6".to_string(), "St. Bruno".to_string()),
        ("7".to_string(), "Pope St. Mark \nSts. Sergius, Bacchus,Marcellus, and Apuleius, Martyrs".to_string()),
        ("8".to_string(), "St. Bridget of Sweden".to_string()),
        ("9".to_string(), "St. Denis [Dionysius], Rusticus, and Eleutherius, Martyrs \nSt. John Lenard".to_string()),
        ("10".to_string(), "St. Francis Borgia".to_string()),
        ("13".to_string(), "St. Edward King and Confessor \nSt. Gerald".to_string()),
        ("14".to_string(), "Pope St. Callistus, Martyr".to_string()),
        ("15".to_string(), "St. Teresa of Avila".to_string()),
        ("16".to_string(), "St. Hedwig \nSt. Gerard Majella".to_string()),
        ("17".to_string(), "St. Margaret Mary Alacoque".to_string()),
        ("18".to_string(), "St. Luke, Evangelist".to_string()),
        ("19".to_string(), "St. Peter of Alcantara".to_string()),
        ("20".to_string(), "St. John Cantius \nSt. Irene".to_string()),
        ("21".to_string(), "St. Hilarion, Abbot \nSt. Ursula and Companions, Martyrs".to_string()),
        ("22".to_string(), "St. Mary Salome".to_string()),
        ("23".to_string(), "St. Anthony Mary Claret \nSt. Ignatius of Constantinople".to_string()),
        ("24".to_string(), "St. Raphael the Archangel".to_string()),
        ("25".to_string(), "St. Isidore the Farmer \nSts. Crispin and Crispinian, Martyrs \nSts. Chrysanthus and Daria, Martyrs".to_string()),
        ("26".to_string(), "Pope St. Evaristus, Martyr".to_string()),
        ("27".to_string(), "St. Frumentius \nVigil of Sts. Simon and Jude, Apostles".to_string()),
        ("28".to_string(), "Sts. Simon and Jude, Apostles".to_string()),
        ("29".to_string(), "St. Narcissus".to_string()),
        ("30".to_string(), "St. Alphonsus Rodriguez".to_string()),
        ("31".to_string(), "St. Wolfgang \nVigil of All Saints".to_string()),
    ]);
    months.insert("october".to_string(), october);

    // November
    let november: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "All Saints ".to_string()),
        ("2".to_string(), "Holy Souls in Purgatory ".to_string()),
        (
            "3".to_string(),
            "St. Malachy \nSt. Martin de Porres".to_string(),
        ),
        (
            "4".to_string(),
            "St. Charles Borromeo \nSts. Vitalis and Agricola, Martyrs".to_string(),
        ),
        ("5".to_string(), "Sts. Zachary and Elizabeth".to_string()),
        (
            "6".to_string(),
            "St. Leonard of Limoges \nSt. Leonard Reresby".to_string(),
        ),
        ("7".to_string(), "St. Willibrord \nSt. Ernest".to_string()),
        (
            "8".to_string(),
            "St. Geoffrey \nBl. Duns Scotus \nFour Crowned Martyrs".to_string(),
        ),
        ("9".to_string(), "St. Theodore, Martyr".to_string()),
        (
            "10".to_string(),
            "St. Andrew Avellino \nSts. Tryphon, Respicius, and Nympha, Martyrs".to_string(),
        ),
        (
            "11".to_string(),
            "St. Martin of Tours \nSt. Mennas, Martyr".to_string(),
        ),
        ("12".to_string(), "Pope St. Martin, Martyr".to_string()),
        (
            "13".to_string(),
            "St. Didacus \nSt. Stanislaus Kostka \nSt. Frances X. Cabrini".to_string(),
        ),
        (
            "14".to_string(),
            "St. Josaphat, Martyr \nSt. Laurence O'Toole".to_string(),
        ),
        ("15".to_string(), "St. Albert the Great".to_string()),
        ("16".to_string(), "St. Gertrude the Great".to_string()),
        (
            "17".to_string(),
            "St. Hugh of Lincoln \nSt. Gregory the Womderworker \nSt. Rose Philippine Duchesne"
                .to_string(),
        ),
        (
            "18".to_string(),
            "Dedication of the Basilicas of Sts. Peter and Paul \nSt. Romanus, Martyr".to_string(),
        ),
        (
            "19".to_string(),
            "St. Elizabeth, Queen of Hungary \nPope St. Pontianus, Martyr".to_string(),
        ),
        (
            "20".to_string(),
            "St. Felix of Valois \nBl. Angeles and Sixteen Companions, Martyrs".to_string(),
        ),
        ("22".to_string(), "St. Cecilia, Martyr".to_string()),
        (
            "23".to_string(),
            "Pope St. Clement, Martyr \nSt. Miguel Pro \nSt. Felicitas, Martyr".to_string(),
        ),
        (
            "24".to_string(),
            "St. John of the Cross \nSt. Chrysogonus, Martyr \n117 Martyrs of Vietnam".to_string(),
        ),
        (
            "25".to_string(),
            "St. Catherine of Alexandria, Martyr".to_string(),
        ),
        (
            "26".to_string(),
            "St. Leonard of Port Maurice \nSt. Sylvester, Abbot \nSt. Peter of Alexandria, Martyr"
                .to_string(),
        ),
        ("27".to_string(), "Bl. Leonard Kimura".to_string()),
        ("28".to_string(), "St. Joseph Mary Pignatelli".to_string()),
        (
            "29".to_string(),
            "St. Saturninus \nVigil of St. Andrew, Apostle".to_string(),
        ),
        (
            "30".to_string(),
            "St. Andrew, Apostle \nSt. Maura".to_string(),
        ),
    ]);
    months.insert("november".to_string(), november);

    // December
    let december: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Edward Campion, Martyr".to_string()),
        ("2".to_string(), "St. Bibiana [Vivian], Martyr".to_string()),
        ("3".to_string(), "St. Francis Xavier".to_string()),
        (
            "4".to_string(),
            "St. Barbara, Martyr \nSt. Peter Chrysologus".to_string(),
        ),
        ("5".to_string(), "St. Sabbas, Abbot".to_string()),
        ("6".to_string(), "St. Nicholas of Bari".to_string()),
        ("7".to_string(), "St. Ambrose".to_string()),
        ("9".to_string(), "St. Juan Diego \nSt. Leocadia".to_string()),
        ("10".to_string(), "Pope St. Melchiades, Martyr".to_string()),
        ("11".to_string(), "Pope St. Damasus".to_string()),
        ("13".to_string(), "St. Lucy, Martyr".to_string()),
        (
            "14".to_string(),
            "Sts. Nicasius, Eutropia and Companions, Martyrs".to_string(),
        ),
        ("15".to_string(), "St. Christiana".to_string()),
        (
            "16".to_string(),
            "St. Eusebius, Martyr \nSt. Alice \nSts. Ananias, Azarias, and Misael, Martyrs"
                .to_string(),
        ),
        ("17".to_string(), "St. Lazarus".to_string()),
        ("19".to_string(), "Bl. Pope Urban V".to_string()),
        (
            "20".to_string(),
            "St. Dominic of Silos \nVigil of St. Thomas, Apostle".to_string(),
        ),
        ("21".to_string(), "St. Thomas, Apostle".to_string()),
        ("22".to_string(), "St. Zeno \nSt. Flavian".to_string()),
        ("23".to_string(), "St. Yvo of Chartres".to_string()),
        ("24".to_string(), "Sts. Adam and Eve".to_string()),
        ("26".to_string(), "St. Stephen, First Martyr".to_string()),
        (
            "27".to_string(),
            "St. John the Evangelist, Apostle".to_string(),
        ),
        ("28".to_string(), "The Holy Innocents ".to_string()),
        (
            "29".to_string(),
            "St. Thomas Becket, Martyr \nSt. David, King".to_string(),
        ),
        ("30".to_string(), "St. Sabinus".to_string()),
        (
            "31 ".to_string(),
            "Pope St. Sylvester \nSt. Catherine Laboure".to_string(),
        ),
    ]);
    months.insert("december".to_string(), december);

    // Return the months
    months
}
