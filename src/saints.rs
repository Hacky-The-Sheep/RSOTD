use std::collections::HashMap;

pub fn months() -> HashMap<String, HashMap<String, String>> {
    let mut months: HashMap<String, HashMap<String, String>> = HashMap::new();

    // JANUARY
    let january: HashMap<String, String> = HashMap::from([
        (
            "2".to_string(),
            "St. Basil | St. Gregory Nazianzen | St. Marcarius".to_string(),
        ),
        ("3".to_string(), "St. Genevieve".to_string()),
        (
            "4".to_string(),
            "St. Dafrosa | St. Elizabeth Seton".to_string(),
        ),
        (
            "5".to_string(),
            "St. John Neumann | St. Telesphorus".to_string(),
        ),
        (
            "6".to_string(),
            "Sts. Gaspar, Melchior and Balthasar | Blessed Andre Bessette".to_string(),
        ),
        ("7".to_string(), "St. Lucian".to_string()),
        ("8".to_string(), "St. Appolinaris".to_string()),
        (
            "9".to_string(),
            "St. Julian of Antioch | St. Peter of Sebaste".to_string(),
        ),
        ("10".to_string(), "St. Nicanor | St. William".to_string()),
        ("11".to_string(), "St. Hyginus".to_string()),
        (
            "12".to_string(),
            "St. Aelred | St. Benedict Biscop | St. Marguerite Bourgeoys".to_string(),
        ),
        (
            "14".to_string(),
            "St. Felix | St. Hilary of Poitiers | St. Malachias".to_string(),
        ),
        (
            "15".to_string(),
            "St. Paul the Hermit | St. Maurus, Abbot".to_string(),
        ),
        (
            "16".to_string(),
            "St. Marcellus | The Five Franciscan Protomartyrs".to_string(),
        ),
        (
            "17".to_string(),
            "St. Sulpice | St. Anthony, Abbot".to_string(),
        ),
        (
            "18".to_string(),
            "St. Prisca | Feast of the Chair of St. Peter of Rome".to_string(),
        ),
        (
            "19".to_string(),
            "Sts. Marius, Martha and Sons, Martyrs | St. Canute, King and Martyr".to_string(),
        ),
        (
            "20".to_string(),
            "St. Fabian, Martyr | St. Sebastian, Martyr".to_string(),
        ),
        ("21".to_string(), "St. Agnes, Martyr".to_string()),
        (
            "22".to_string(),
            "St. Anastasius, Martyr | St. Vincent of Saragozza, Martyr".to_string(),
        ),
        (
            "23".to_string(),
            "St. Raymond of Pennafort | St. Emerentiana, Martyr".to_string(),
        ),
        ("24".to_string(), "St. Tomothy, Martyr".to_string()),
        ("25".to_string(), "Coversion of St. Paul".to_string()),
        (
            "26".to_string(),
            "St. Paula | St. Polycarp, Martyr".to_string(),
        ),
        ("27".to_string(), "St. John Chrysostom".to_string()),
        (
            "28".to_string(),
            "St. Peter Nolasco | St. Thomas Aquinas | St. Agnes, Martyr,".to_string(),
        ),
        ("29".to_string(), "St. Francis de Sales".to_string()),
        (
            "30".to_string(),
            "St. Martina, Martyr | St. Mutien".to_string(),
        ),
        ("31".to_string(), "St. John Bosco".to_string()),
    ]);
    months.insert("january".to_string(), january);

    // FEBRUARY
    let february: HashMap<String, String> = HashMap::from([
        ("1".to_string(), "St. Brigid of Ireland | St. Ignatius of Antioch".to_string()),
        ("2".to_string(), "St. Cornelius the Centurian".to_string()),
        ("3".to_string(), "St. Blaise".to_string()),
        ("4".to_string(), "St. Andrew Corsini".to_string()),
        ("5".to_string(), "The 26 Martyrs of Japan | St. Apollonia | St. Agatha".to_string()),
        ("6".to_string(), "St. Dorothy | St. Titus".to_string()),
        ("7".to_string(), "Bl. Pope Pius IX | St. Romauld, Abbot".to_string()),
        ("8".to_string(), "St. Josephine Bakhita | St. John of Matha".to_string()),
        ("9".to_string(), "St. Cyril of Alexandria | St. Miguel Cordero".to_string()),
        ("10".to_string(), "St. Scholastica".to_string()),
        ("12".to_string(), "Seven Holy Founders of the Servites".to_string()),
        ("13".to_string(), "St. Agabus | St. Catherine d'Ricci | St. Polyeucte".to_string()),
        ("14".to_string(), "St. Valentine".to_string()),
        ("15".to_string(), "Sts. Faustinus and Jovita".to_string()),
        ("16".to_string(), "St. Onesimus".to_string()),
        ("17".to_string(), "St. Julian".to_string()),
        ("18".to_string(), "St. Bernadette Soubirous | Bl. Fra Angelico | St. Simeon".to_string()),
        ("19".to_string(), "St. Gabinus | St. Odran".to_string()),
        ("20".to_string(), "St. Tyrannio | Bl. Jacinta and Francisco Marto | St. Leo the Wonderworker | St. Claude de la Columbiere | St. Amata".to_string()),
        ("21".to_string(), "St. Pter Mavimenus | St. Robert Southwell".to_string()),
        ("22".to_string(), "St. Peter's Chair".to_string()),
        ("23".to_string(), "St. Peter Damien | Vigil of St. Matthias, Apostle".to_string()),
        ("24".to_string(), "St. Matthias, Apostle".to_string()),
        ("25".to_string(), "St. Walburga | Sts. Victor and Claudian".to_string()),
        ("26".to_string(), "St. Alexander | St. Porphyry".to_string()),
        ("26".to_string(), "St. Nestor".to_string()),
        ("27".to_string(), "St. Gabriel of Our Lady of Sorrows".to_string()),
        ("28".to_string(), "Pope St. Hilary | Sts. Rimnus and Lupicinus".to_string()),
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
            "St. Kevin | St. Clothilde, Queen".to_string(),
        ),
        ("4".to_string(), "St. Francis Caracciolo".to_string()),
        ("5".to_string(), "St. Boniface, Martyr".to_string()),
        (
            "6".to_string(),
            "St. Norbert | St. Philip, Deacon".to_string(),
        ),
        ("7".to_string(), "St. Robert of Newminster".to_string()),
        ("8".to_string(), "Sts. Medard and Gildard".to_string()),
        (
            "9".to_string(),
            "St. Columkille | Sts. Primus and Felician, Martyrs".to_string(),
        ),
        ("10".to_string(), "St. Margaret of Scotland".to_string()),
        ("11".to_string(), "St. Barnabas, Apostle".to_string()),
        (
            "12".to_string(),
            "St. John of St. Facundo [us] | Sts. Basilides, Cyprinus, Nabor, and Nazarius, Martyrs"
                .to_string(),
        ),
        ("13".to_string(), "St. Anthony of Padua".to_string()),
        ("14".to_string(), "St. Basil the Great".to_string()),
        (
            "15".to_string(),
            "St. Germaine | Sts. Vitus, Modestus, and Crescentia, Martyrs".to_string(),
        ),
        ("16".to_string(), "St. John Francis Regis".to_string()),
        (
            "17".to_string(),
            "St. Botolph | St. Ranier | St. Gregory Barbarigo".to_string(),
        ),
        (
            "18".to_string(),
            "St. Ephrem | Sts. Mark and Marcellianus, Martyrs".to_string(),
        ),
        (
            "19".to_string(),
            "St. Juliana Falconieri | Sts. Gervase and Protase".to_string(),
        ),
        ("20 ".to_string(), "Pope St. Silverius, Martyr".to_string()),
        (
            "21".to_string(),
            "St. Terence | St. Aloysius Gonzaga".to_string(),
        ),
        ("22".to_string(), "St. Paulinus of Nola".to_string()),
        (
            "23".to_string(),
            "St. Audrey | Vigil of the Birth of St. John the Baptist".to_string(),
        ),
        (
            "24".to_string(),
            "Birth of St. John the Baptist".to_string(),
        ),
        ("25".to_string(), "St. William, Abbot".to_string()),
        ("26".to_string(), "Sts. John and Paul, Martyrs".to_string()),
        (
            "28".to_string(),
            "Vigil of Sts. Peter and Paul | St. Irenaeus, Martyr".to_string(),
        ),
        ("29".to_string(), "Sts. Peter and Paul".to_string()),
        (
            "30".to_string(),
            "Commemoration of St. Paul | Seventeen Irish Martyrs".to_string(),
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
		("11".to_string(), "Pope St. Pius I, Martyr | St. Oliver Plunket".to_string()),
		("12".to_string(), "St. John Gualbert | Sts. Nabor and Felix, Martyrs | St. Veronica of the Veil".to_string()),
		("13".to_string(), "Pope St. Anacletus, Martyr | St. Mildred | St. Teresa of the Andes".to_string()),
		("14".to_string(), "St. Bonaventure | St. Francis Solano | Bl. Kateri Tekakwitha".to_string()),
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
		("25".to_string(), "Vigil of St. James the Greater, Apostle | St. James the Greater, Apostle | St. Christopher".to_string()),
		("26".to_string(), "St. Anne".to_string()),
		("27".to_string(), "St. Pantaleon, Martyr | Pope St. Celestine I".to_string()),
		("28".to_string(), "Sts. Nazarius and Celsus, Victor I, and Innocent I, Martyrs".to_string()),
		("29".to_string(), "St. Martha & 20: Sts. Felix II, Simplicius, Faustinus, Beatrice".to_string()),
		("30".to_string(), "Sts. Abdon and Sennen | St. Ignatius of Loyola".to_string()),
    ]);
    months.insert("july".to_string(), july);
    months
}
