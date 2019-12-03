use std::collections::HashMap;

enum Move {
    R(i32),
    L(i32),
    U(i32),
    D(i32),
}

fn main() {
    let line1: Vec<Move> = vec![
        Move::R(991),
        Move::U(557),
        Move::R(554),
        Move::U(998),
        Move::L(861),
        Move::D(301),
        Move::L(891),
        Move::U(180),
        Move::L(280),
        Move::D(103),
        Move::R(828),
        Move::D(58),
        Move::R(373),
        Move::D(278),
        Move::L(352),
        Move::D(583),
        Move::L(465),
        Move::D(301),
        Move::R(384),
        Move::D(638),
        Move::L(648),
        Move::D(413),
        Move::L(511),
        Move::U(596),
        Move::L(701),
        Move::U(463),
        Move::L(664),
        Move::U(905),
        Move::L(374),
        Move::D(372),
        Move::L(269),
        Move::U(868),
        Move::R(494),
        Move::U(294),
        Move::R(661),
        Move::U(604),
        Move::L(629),
        Move::U(763),
        Move::R(771),
        Move::U(96),
        Move::R(222),
        Move::U(227),
        Move::L(97),
        Move::D(793),
        Move::L(924),
        Move::U(781),
        Move::L(295),
        Move::D(427),
        Move::R(205),
        Move::D(387),
        Move::L(455),
        Move::D(904),
        Move::R(254),
        Move::D(34),
        Move::R(341),
        Move::U(268),
        Move::L(344),
        Move::D(656),
        Move::L(715),
        Move::U(439),
        Move::R(158),
        Move::U(237),
        Move::R(199),
        Move::U(729),
        Move::L(428),
        Move::D(125),
        Move::R(487),
        Move::D(506),
        Move::R(486),
        Move::D(496),
        Move::R(932),
        Move::D(918),
        Move::R(603),
        Move::U(836),
        Move::R(258),
        Move::U(15),
        Move::L(120),
        Move::U(528),
        Move::L(102),
        Move::D(42),
        Move::R(385),
        Move::U(905),
        Move::L(472),
        Move::D(351),
        Move::R(506),
        Move::U(860),
        Move::L(331),
        Move::D(415),
        Move::R(963),
        Move::D(733),
        Move::R(108),
        Move::D(527),
        Move::L(634),
        Move::U(502),
        Move::L(553),
        Move::D(623),
        Move::R(973),
        Move::U(209),
        Move::L(632),
        Move::D(588),
        Move::R(264),
        Move::U(553),
        Move::L(768),
        Move::D(689),
        Move::L(708),
        Move::D(432),
        Move::R(247),
        Move::U(993),
        Move::L(146),
        Move::U(656),
        Move::R(710),
        Move::U(47),
        Move::R(783),
        Move::U(643),
        Move::R(954),
        Move::U(888),
        Move::L(84),
        Move::U(202),
        Move::R(495),
        Move::U(66),
        Move::R(414),
        Move::U(993),
        Move::R(100),
        Move::D(557),
        Move::L(326),
        Move::D(645),
        Move::R(975),
        Move::U(266),
        Move::R(143),
        Move::U(730),
        Move::L(491),
        Move::D(96),
        Move::L(161),
        Move::U(165),
        Move::R(97),
        Move::D(379),
        Move::R(930),
        Move::D(613),
        Move::R(178),
        Move::D(635),
        Move::R(192),
        Move::U(957),
        Move::L(450),
        Move::U(149),
        Move::R(911),
        Move::U(220),
        Move::L(914),
        Move::U(659),
        Move::L(67),
        Move::D(825),
        Move::L(904),
        Move::U(137),
        Move::L(392),
        Move::U(333),
        Move::L(317),
        Move::U(310),
        Move::R(298),
        Move::D(240),
        Move::R(646),
        Move::U(588),
        Move::R(746),
        Move::U(861),
        Move::L(958),
        Move::D(892),
        Move::L(200),
        Move::U(463),
        Move::R(246),
        Move::D(870),
        Move::R(687),
        Move::U(815),
        Move::R(969),
        Move::U(864),
        Move::L(972),
        Move::U(254),
        Move::L(120),
        Move::D(418),
        Move::L(567),
        Move::D(128),
        Move::R(934),
        Move::D(217),
        Move::R(764),
        Move::U(128),
        Move::R(146),
        Move::U(467),
        Move::R(690),
        Move::U(166),
        Move::R(996),
        Move::D(603),
        Move::R(144),
        Move::D(362),
        Move::R(885),
        Move::D(118),
        Move::L(882),
        Move::U(612),
        Move::R(270),
        Move::U(917),
        Move::L(599),
        Move::D(66),
        Move::L(749),
        Move::D(498),
        Move::L(346),
        Move::D(920),
        Move::L(222),
        Move::U(439),
        Move::R(822),
        Move::U(891),
        Move::R(458),
        Move::U(15),
        Move::R(831),
        Move::U(92),
        Move::L(164),
        Move::D(615),
        Move::L(439),
        Move::U(178),
        Move::R(409),
        Move::D(463),
        Move::L(452),
        Move::U(633),
        Move::L(683),
        Move::U(186),
        Move::R(402),
        Move::D(609),
        Move::L(38),
        Move::D(699),
        Move::L(679),
        Move::D(74),
        Move::R(125),
        Move::D(145),
        Move::R(424),
        Move::U(961),
        Move::L(353),
        Move::U(43),
        Move::R(794),
        Move::D(519),
        Move::L(359),
        Move::D(494),
        Move::R(812),
        Move::D(770),
        Move::L(657),
        Move::U(154),
        Move::L(137),
        Move::U(549),
        Move::L(193),
        Move::D(816),
        Move::R(333),
        Move::U(650),
        Move::R(49),
        Move::D(459),
        Move::R(414),
        Move::U(72),
        Move::R(313),
        Move::U(231),
        Move::R(370),
        Move::U(680),
        Move::L(27),
        Move::D(221),
        Move::L(355),
        Move::U(342),
        Move::L(597),
        Move::U(748),
        Move::R(821),
        Move::D(280),
        Move::L(307),
        Move::U(505),
        Move::L(160),
        Move::U(982),
        Move::L(527),
        Move::D(516),
        Move::L(245),
        Move::U(158),
        Move::R(565),
        Move::D(797),
        Move::R(99),
        Move::D(695),
        Move::L(712),
        Move::U(155),
        Move::L(23),
        Move::U(964),
        Move::L(266),
        Move::U(623),
        Move::L(317),
        Move::U(445),
        Move::R(689),
        Move::U(150),
        Move::L(41),
        Move::U(536),
        Move::R(638),
        Move::D(200),
        Move::R(763),
        Move::D(260),
        Move::L(234),
        Move::U(217),
        Move::L(881),
        Move::D(576),
        Move::L(223),
        Move::U(39),
        Move::L(808),
        Move::D(125),
        Move::R(950),
        Move::U(341),
        Move::L(405),
    ];

    let line2: Vec<Move> = vec![
        Move::L(993),
        Move::D(508),
        Move::R(356),
        Move::U(210),
        Move::R(42),
        Move::D(68),
        Move::R(827),
        Move::D(513),
        Move::L(564),
        Move::D(407),
        Move::L(945),
        Move::U(757),
        Move::L(517),
        Move::D(253),
        Move::R(614),
        Move::U(824),
        Move::R(174),
        Move::D(536),
        Move::R(906),
        Move::D(291),
        Move::R(70),
        Move::D(295),
        Move::R(916),
        Move::D(754),
        Move::L(892),
        Move::D(736),
        Move::L(528),
        Move::D(399),
        Move::R(76),
        Move::D(588),
        Move::R(12),
        Move::U(617),
        Move::R(173),
        Move::D(625),
        Move::L(533),
        Move::D(355),
        Move::R(178),
        Move::D(706),
        Move::R(139),
        Move::D(419),
        Move::R(460),
        Move::U(976),
        Move::L(781),
        Move::U(973),
        Move::L(931),
        Move::D(254),
        Move::R(195),
        Move::U(42),
        Move::R(555),
        Move::D(151),
        Move::R(226),
        Move::U(713),
        Move::L(755),
        Move::U(398),
        Move::L(933),
        Move::U(264),
        Move::R(352),
        Move::U(461),
        Move::L(472),
        Move::D(810),
        Move::L(257),
        Move::U(901),
        Move::R(429),
        Move::U(848),
        Move::L(181),
        Move::D(362),
        Move::R(404),
        Move::D(234),
        Move::L(985),
        Move::D(392),
        Move::R(341),
        Move::U(608),
        Move::L(518),
        Move::D(59),
        Move::L(804),
        Move::D(219),
        Move::L(366),
        Move::D(28),
        Move::L(238),
        Move::D(491),
        Move::R(265),
        Move::U(131),
        Move::L(727),
        Move::D(504),
        Move::R(122),
        Move::U(461),
        Move::R(732),
        Move::D(411),
        Move::L(910),
        Move::D(884),
        Move::R(954),
        Move::U(341),
        Move::L(619),
        Move::D(949),
        Move::L(570),
        Move::D(823),
        Move::R(646),
        Move::D(226),
        Move::R(197),
        Move::U(892),
        Move::L(691),
        Move::D(294),
        Move::L(955),
        Move::D(303),
        Move::R(490),
        Move::D(469),
        Move::L(503),
        Move::D(482),
        Move::R(390),
        Move::D(741),
        Move::L(715),
        Move::D(187),
        Move::R(378),
        Move::U(853),
        Move::L(70),
        Move::D(903),
        Move::L(589),
        Move::D(481),
        Move::L(589),
        Move::U(911),
        Move::R(45),
        Move::U(348),
        Move::R(214),
        Move::D(10),
        Move::R(737),
        Move::D(305),
        Move::R(458),
        Move::D(291),
        Move::R(637),
        Move::D(721),
        Move::R(440),
        Move::U(573),
        Move::R(442),
        Move::D(407),
        Move::L(63),
        Move::U(569),
        Move::L(903),
        Move::D(936),
        Move::R(518),
        Move::U(859),
        Move::L(370),
        Move::D(888),
        Move::R(498),
        Move::D(759),
        Move::R(283),
        Move::U(469),
        Move::R(548),
        Move::D(185),
        Move::R(808),
        Move::D(81),
        Move::L(629),
        Move::D(761),
        Move::R(807),
        Move::D(878),
        Move::R(712),
        Move::D(183),
        Move::R(382),
        Move::D(484),
        Move::L(791),
        Move::D(371),
        Move::L(188),
        Move::D(397),
        Move::R(645),
        Move::U(679),
        Move::R(415),
        Move::D(446),
        Move::L(695),
        Move::U(174),
        Move::R(707),
        Move::D(36),
        Move::R(483),
        Move::U(877),
        Move::L(819),
        Move::D(538),
        Move::L(277),
        Move::D(2),
        Move::R(200),
        Move::D(838),
        Move::R(837),
        Move::U(347),
        Move::L(865),
        Move::D(945),
        Move::R(958),
        Move::U(575),
        Move::L(924),
        Move::D(351),
        Move::L(881),
        Move::U(961),
        Move::R(899),
        Move::U(845),
        Move::R(816),
        Move::U(866),
        Move::R(203),
        Move::D(380),
        Move::R(766),
        Move::D(97),
        Move::R(38),
        Move::U(148),
        Move::L(999),
        Move::D(332),
        Move::R(543),
        Move::U(10),
        Move::R(351),
        Move::U(281),
        Move::L(460),
        Move::U(309),
        Move::L(543),
        Move::U(795),
        Move::L(639),
        Move::D(556),
        Move::L(882),
        Move::D(513),
        Move::R(722),
        Move::U(314),
        Move::R(531),
        Move::D(604),
        Move::L(418),
        Move::U(840),
        Move::R(864),
        Move::D(694),
        Move::L(530),
        Move::U(862),
        Move::R(559),
        Move::D(639),
        Move::R(689),
        Move::D(201),
        Move::L(439),
        Move::D(697),
        Move::R(441),
        Move::U(175),
        Move::R(558),
        Move::D(585),
        Move::R(92),
        Move::D(191),
        Move::L(533),
        Move::D(788),
        Move::R(154),
        Move::D(528),
        Move::R(341),
        Move::D(908),
        Move::R(811),
        Move::U(750),
        Move::R(172),
        Move::D(742),
        Move::R(113),
        Move::U(56),
        Move::L(517),
        Move::D(826),
        Move::L(250),
        Move::D(269),
        Move::L(278),
        Move::U(74),
        Move::R(285),
        Move::U(904),
        Move::L(221),
        Move::U(270),
        Move::R(296),
        Move::U(671),
        Move::L(535),
        Move::U(340),
        Move::L(206),
        Move::U(603),
        Move::L(852),
        Move::D(60),
        Move::R(648),
        Move::D(313),
        Move::L(282),
        Move::D(685),
        Move::R(482),
        Move::U(10),
        Move::R(829),
        Move::U(14),
        Move::L(12),
        Move::U(365),
        Move::R(996),
        Move::D(10),
        Move::R(104),
        Move::U(654),
        Move::R(346),
        Move::D(458),
        Move::R(219),
        Move::U(247),
        Move::L(841),
        Move::D(731),
        Move::R(115),
        Move::U(400),
        Move::L(731),
        Move::D(904),
        Move::L(487),
        Move::U(430),
        Move::R(612),
        Move::U(437),
        Move::L(865),
        Move::D(618),
        Move::R(747),
        Move::U(522),
        Move::R(309),
        Move::U(302),
        Move::R(9),
        Move::U(609),
        Move::L(201),
    ];

    let mut traversed = HashMap::<(i32, i32), i32>::new();
    let mut intersections = Vec::<((i32, i32), i32)>::new();

    let mut current_x = 0;
    let mut current_y = 0;
    let mut steps = 0;

    traversed.insert((current_x, current_y), steps);

    for m in line1 {
        match m {
            Move::U(y) => {
                for _j in 0..y {
                    traversed.insert((current_x, current_y), steps);
                    current_y -= 1;
                    steps += 1;
                }
            },
            Move::D(y) => {
                for _j in 0..y {
                    traversed.insert((current_x, current_y), steps);
                    current_y += 1;
                    steps += 1;
                }
            },
            Move::L(x) => {
                for _i in 0..x {
                    traversed.insert((current_x, current_y), steps);
                    current_x -= 1;
                    steps += 1;
                }
            },
            Move::R(x) => {
                for _i in 0..x {
                    traversed.insert((current_x, current_y), steps);
                    current_x += 1;
                    steps += 1;
                }
            },
        };
    }

    current_x = 0;
    current_y = 0;
    steps = 0;

    for m in line2 {
        match m {
            Move::U(y) => {
                for j in 0..y {
                    current_y -= 1;
                    steps += 1;
                    if traversed.contains_key(&(current_x, current_y)) {
                        let other_steps = traversed.get(&(current_x, current_y)).unwrap();
                        intersections.push(((current_x, current_y), steps+other_steps));
                    }
                }
            },
            Move::D(y) => {
                for j in 0..y {
                    current_y += 1;
                    steps += 1;
                    if traversed.contains_key(&(current_x, current_y)) {
                        let other_steps = traversed.get(&(current_x, current_y)).unwrap();
                        intersections.push(((current_x, current_y), steps+other_steps));
                    }
                }
            },
            Move::L(x) => {
                for i in 0..x {
                    current_x -= 1;
                    steps += 1;
                    if traversed.contains_key(&(current_x, current_y)) {
                        let other_steps = traversed.get(&(current_x, current_y)).unwrap();
                        intersections.push(((current_x, current_y), steps+other_steps));
                    }
                }
            },
            Move::R(x) => {
                for i in 0..x {
                    current_x += 1;
                    steps += 1;
                    if traversed.contains_key(&(current_x, current_y)) {
                        let other_steps = traversed.get(&(current_x, current_y)).unwrap();
                        intersections.push(((current_x, current_y), steps+other_steps));
                    }
                }
            },
        };

    }

    println!("current_x: {}, current_y: {}, steps: {}", current_x, current_y, steps);
    println!("traversed.len(): {:?}", traversed.len());
    println!("intersections.len(): {:?}", intersections.len());
    println!("intersections: {:?}", intersections);
}
