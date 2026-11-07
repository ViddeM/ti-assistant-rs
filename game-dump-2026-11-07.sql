--
-- PostgreSQL database dump
--

\restrict qMi5VtZWDZ3cwM8mCPVjeijuoQLbiccgfcBY8hwRUO1eCVokVUHfV0UiFmmfdZS

-- Dumped from database version 13.2 (Debian 13.2-1.pgdg100+1)
-- Dumped by pg_dump version 16.15 (Ubuntu 16.15-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: -
--

-- *not* creating schema, since initdb creates it


--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: next_game_event_seq(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.next_game_event_seq() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
	DECLARE new_seq INTEGER;
	BEGIN
	SELECT COUNT(seq) into new_seq FROM game_event WHERE game_id = new.game_id;
		RETURN (new.id, new.game_id, new_seq, new.timestamp, new.event);
	END
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- Name: game; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.game (
    id character(8) NOT NULL,
    name character varying(128) NOT NULL
);


--
-- Name: game_event; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.game_event (
    id integer NOT NULL,
    game_id character(8) NOT NULL,
    seq integer DEFAULT '-1'::integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    event jsonb NOT NULL
);


--
-- Name: game_event_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.game_event_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: game_event_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.game_event_id_seq OWNED BY public.game_event.id;


--
-- Name: game_event id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_event ALTER COLUMN id SET DEFAULT nextval('public.game_event_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2026-10-26 12:16:09.523933
20231102214911	2026-10-26 12:16:09.534318
20261106162838	2026-11-06 16:24:31.594612
20261106163253	2026-11-06 16:24:31.616445
\.


--
-- Data for Name: game; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.game (id, name) FROM stdin;
04831ded	Funny Game
7553b30d	Funny Game
a9d66065	Funny Game
05b6ffbb	Funny Game
5aa9066f	Funny Game
b3085ac4	Funny Game
20a12af2	Funny Game
e15c9638	setup_base_three_players
f4445ee6	setup_special_factions
5c596a6c	setup_milty_import
69d97f21	strategy_three_players_pick_two_each
36242911	strategy_naalu_telepathy
cdd60863	strategy_gift_of_prescience
dd2da702	action_turn_order_and_passing
c828f2f2	action_tactical_planets_and_attachments
bcebac25	action_strategic_technology
e3857a9f	action_strategic_technology_nekro
a68a7f72	action_strategic_politics_and_imperial
2ec3d2d7	action_cards
71b34856	action_leaders
eaf9b225	action_frontier_cards
459c2e20	action_relics
cc8bb7bd	status_scoring_and_reveal
f19ebc78	status_stage_two_unlock
3b29b712	status_relics_phase_maw_and_crown
1ef78e6b	agenda_laws_enact_and_repeal
ae72c5d9	agenda_veto_against_and_discard
27d0c2d4	agenda_mutiny_and_seed_for
9a5c9ddc	agenda_mutiny_and_seed_against
49897d7d	agenda_elect_law_and_secret_objective
d624536b	agenda_override_outside_agenda_phase
d0a685ae	agenda_elect_planet_with_trait
9b227a4f	score_manual_adjustments
69a4c6a8	score_manual_planets_and_techs
006fa03a	thunders_edge_commit_and_complete
38720452	thunders_edge_passing
a19cc3eb	thunders_edge_admin_flow
b436b5db	firmament_becomes_obsidian
11ed8fa8	thunders_edge_take_planets_without_tile
728830d9	thunders_edge_styx_victory_point
d92655e1	thunders_edge_action_cards
65cba6c2	thunders_edge_overrule_and_strategize
980076ac	thunders_edge_crisis_and_puppets_on_a_string
6cd7ff7b	codex_iv_book_of_latvinia
8125c3f2	codex_iv_neuraloop
e3f1e7db	time_tracking_and_pause
d68a3307	full_game_base_three_players
1e7850a0	full_game_pok_six_players
dbe8cd53	Funny Game
01bde7ff	Funny Game
d71a8543	thunders_edge_breakthrough_planets
3c42917a	Funny Game
2ac89bef	thunders_edge_quantumcore
a44dae41	thunders_edge_map_tiles
6762a561	codex_iv_book_of_latvinia_research
f61dbf9d	Funny Game
ab4e4e80	thunders_edge_the_fracture_enters_play
2f2807a6	agenda_minister_of_antiques
0f0e0005	score_manual_relics
450c18d7	Funny Game
db9f6e36	Funny Game
23e3bcec	Funny Game
ad779ebc	Funny Game
53e02a07	Funny Game
3fc75ce7	Funny Game
d9741780	Funny Game
ea4edebf	Funny Game
54757682	Funny Game
60fff71a	Funny Game
434a0278	Funny Game
31c1512f	Funny Game
257ca62a	Funny Game
ba1a832b	Funny Game
aeffde45	Funny Game
e0a811de	Funny Game
cf738caf	Funny Game
4b5546f9	Funny Game
4a9e1d07	hidden_agenda_ui_check
11751527	civilized_society_setup_ui_check
5554afee	civilized_society_status_ui_check
33ae89db	breakthrough_actions_ui_check
8abd8371	cultural_exchange_setup_ui_check
f3b35c18	cultural_exchange_leaders_ui_check
2cf183d9	monuments_tactical_ui_check
b645aaf4	monuments_score_ui_check
0e9e7177	Funny Game
b6e8aaef	galactic_events_age_of_exploration
7a9d0b11	rapid_mobilization_ui_check
66a56181	Funny Game
55f31401	Funny Game
d4bd829a	Funny Game
0806fe00	Funny Game
100a9fd2	GE: Mercenaries for Hire (action)
20e0b4a1	GE: Stellar Atomics (action)
942cbdd8	GE: Stellar Atomics (agenda)
ccc97b0b	GE: Conventions of War Abandoned (action)
dc46c53a	GE: Age of Exploration (action)
03d042d0	GE: Wild, Wild Galaxy (Stellar Converter)
d4406362	GE: Dangerous Wilds (tactical action)
63b0393b	GE: Monuments to the Ages (build)
d8bc49b5	GE: Zealous Orthodoxy (second secret)
0bc7835e	GE: Hidden Agenda (agenda)
185090c7	GE: Civilized Society (setup)
1aac2dae	GE: Civilized Society (status)
7d8a64d9	GE: Cultural Exchange Program (setup)
260cb545	GE: Cultural Exchange Program (leaders)
d64c88d1	GE: Rapid Mobilization (start)
551dc1fe	GE: Age of Fighters (setup)
adf05cd3	GE: Advent of the War Sun (setup)
099a8a18	GE: Total War (action)
09d3557e	GE: Minor Factions (alliance)
1ac9a493	GE: Minor Factions (setup)
f1a70001	GE: Minor Factions (milty map)
\.


--
-- Data for Name: game_event; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.game_event (id, game_id, seq, "timestamp", event) FROM stdin;
1	04831ded	0	2026-10-26 12:23:55.272285+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "104", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "25", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "61", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "19", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "59", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "33", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "44", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "109", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "39", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "72", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "79", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "26", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "45", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "40", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "34", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "114", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "28", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "92", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "78", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "97", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 0}}}, {"system": "47", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "70", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 3, "faction": "LastBastion"}, "Gurr": {"name": "Gurr", "order": 4, "faction": "FirmamentObsidian"}, "Vidde": {"name": "Vidde", "order": 2, "faction": "CrimsonRebellion"}, "Portals": {"name": "Portals", "order": 0, "faction": "RalNelConsortium"}, "Sponken": {"name": "Sponken", "order": 1, "faction": "DeepwroughtScholarate"}}, "game_name": "Helluw", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
2	04831ded	1	2026-10-26 12:23:59.435336+00	"CreationDone"
3	04831ded	2	2026-10-26 12:26:34.835712+00	{"SetupPlayerTechs": {"player": "Portals", "technologies": ["AiDevelopmentAlgorithm"]}}
4	04831ded	3	2026-10-26 12:26:44.729306+00	{"SetupPlayerTechs": {"player": "Sponken", "technologies": ["CruiserII", "PredictiveIntelligence"]}}
5	04831ded	4	2026-10-26 12:26:47.367822+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["AntimassDeflectors"]}}
6	04831ded	5	2026-10-26 12:26:50.332192+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["SarweenTools"]}}
7	04831ded	6	2026-10-26 12:26:52.633523+00	{"SetupPlayerTechs": {"player": "Gurr", "technologies": ["ScanlinkDroneNetwork"]}}
8	04831ded	7	2026-10-26 12:26:58.904956+00	{"RevealInitialObjectives": {"firstObjective": "FoundResearchOutposts", "secondObjective": "LeadFromTheFront"}}
9	04831ded	8	2026-10-26 12:26:59.833711+00	"StartGame"
10	04831ded	9	2026-10-26 12:27:02.230802+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Portals"}}
11	04831ded	10	2026-10-26 12:27:04.032662+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Sponken"}}
12	04831ded	11	2026-10-26 12:27:05.695755+00	{"TakeStrategyCard": {"card": "Politics", "player": "Vidde"}}
13	04831ded	12	2026-10-26 13:43:50.893549+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
14	04831ded	13	2026-10-26 13:43:51.833391+00	{"TakeStrategyCard": {"card": "Trade", "player": "Gurr"}}
15	04831ded	14	2026-10-26 13:43:53.404077+00	"CompleteStrategyPhase"
17	04831ded	15	2026-10-26 14:13:00.707783+00	{"ActionCardActionBegin": {"card": "Brilliance", "player": "Portals"}}
18	04831ded	16	2026-10-26 14:13:40.5091+00	{"ActionCardActionCommit": {"data": {"Brilliance": {"player": "Sponken"}}, "player": "Portals"}}
19	04831ded	17	2026-10-26 14:13:50.510021+00	{"EndTurn": {"player": "Portals"}}
20	7553b30d	0	2026-10-26 19:26:55.840111+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
21	7553b30d	1	2026-10-26 19:26:57.481816+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
22	7553b30d	2	2026-10-26 19:26:58.137927+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
23	7553b30d	3	2026-10-26 19:26:58.771481+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
24	7553b30d	4	2026-10-26 19:26:59.336499+00	"CreationDone"
25	7553b30d	5	2026-10-26 19:27:00.370168+00	{"SetupSpeaker": {"player": "Alice"}}
26	7553b30d	6	2026-10-26 19:27:00.751688+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27	7553b30d	7	2026-10-26 19:27:01.082418+00	"StartGame"
28	7553b30d	8	2026-10-26 19:27:02.117046+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29	7553b30d	9	2026-10-26 19:27:02.549127+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30	7553b30d	10	2026-10-26 19:27:02.982477+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
31	7553b30d	11	2026-10-26 19:27:03.416894+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
32	7553b30d	12	2026-10-26 19:27:03.850013+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
33	7553b30d	13	2026-10-26 19:27:04.283033+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
34	7553b30d	14	2026-10-26 19:27:04.716814+00	"CompleteStrategyPhase"
35	7553b30d	15	2026-10-26 19:27:11.089911+00	{"TacticalActionBegin": {"player": "Alice"}}
36	7553b30d	16	2026-10-26 19:27:11.625757+00	{"TacticalActionCommit": {"player": "Alice"}}
37	7553b30d	17	2026-10-26 19:27:35.870537+00	{"PlayCrisis": {"player": "Carol"}}
38	7553b30d	18	2026-10-26 19:27:58.547016+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
39	7553b30d	19	2026-10-26 19:27:59.183959+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
40	7553b30d	20	2026-10-26 19:27:59.622134+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
41	7553b30d	21	2026-10-26 19:28:00.065689+00	"StrategicActionCommit"
42	7553b30d	22	2026-10-26 19:28:00.597628+00	{"EndTurn": {"player": "Carol"}}
43	7553b30d	23	2026-10-26 19:28:01.132245+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
44	7553b30d	24	2026-10-26 19:28:01.765464+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
45	7553b30d	25	2026-10-26 19:28:02.206312+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
46	7553b30d	26	2026-10-26 19:28:02.637444+00	"StrategicActionCommit"
47	7553b30d	27	2026-10-26 19:28:03.176247+00	{"EndTurn": {"player": "Alice"}}
48	7553b30d	28	2026-10-26 19:28:03.708564+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
49	7553b30d	29	2026-10-26 19:28:04.341346+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
50	7553b30d	30	2026-10-26 19:28:04.775986+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
51	7553b30d	31	2026-10-26 19:28:05.208349+00	"StrategicActionCommit"
52	7553b30d	32	2026-10-26 19:28:05.743307+00	{"EndTurn": {"player": "Bob"}}
53	7553b30d	33	2026-10-26 19:28:06.27569+00	{"TacticalActionBegin": {"player": "Carol"}}
54	7553b30d	34	2026-10-26 19:28:06.810734+00	{"TacticalActionCommit": {"player": "Carol"}}
55	7553b30d	35	2026-10-26 19:28:07.360529+00	{"EndTurn": {"player": "Carol"}}
56	7553b30d	36	2026-10-26 19:28:07.893382+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
57	7553b30d	37	2026-10-26 19:28:08.525426+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
58	7553b30d	38	2026-10-26 19:28:08.956121+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
59	7553b30d	39	2026-10-26 19:28:09.394608+00	"StrategicActionCommit"
60	7553b30d	40	2026-10-26 19:28:09.932635+00	{"EndTurn": {"player": "Alice"}}
61	7553b30d	41	2026-10-26 19:28:10.464226+00	{"TacticalActionBegin": {"player": "Bob"}}
62	7553b30d	42	2026-10-26 19:28:10.999646+00	{"TacticalActionCommit": {"player": "Bob"}}
63	7553b30d	43	2026-10-26 19:28:11.531982+00	{"EndTurn": {"player": "Bob"}}
64	7553b30d	44	2026-10-26 19:28:12.065432+00	{"TacticalActionBegin": {"player": "Carol"}}
65	7553b30d	45	2026-10-26 19:28:12.597842+00	{"TacticalActionCommit": {"player": "Carol"}}
66	7553b30d	46	2026-10-26 19:28:13.133325+00	{"EndTurn": {"player": "Carol"}}
67	7553b30d	47	2026-10-26 19:28:13.665107+00	{"PassAction": {"player": "Alice"}}
68	7553b30d	48	2026-10-26 19:28:14.2995+00	{"EndTurn": {"player": "Alice"}}
69	7553b30d	49	2026-10-26 19:28:14.932488+00	{"TacticalActionBegin": {"player": "Bob"}}
70	7553b30d	50	2026-10-26 19:28:15.462695+00	{"TacticalActionCommit": {"player": "Bob"}}
71	7553b30d	51	2026-10-26 19:28:24.230573+00	{"PlayPuppetsOnAString": {"player": "Alice"}}
72	7553b30d	52	2026-10-26 19:28:25.070965+00	{"TacticalActionBegin": {"player": "Alice"}}
73	7553b30d	53	2026-10-26 19:28:25.611014+00	{"TacticalActionCommit": {"player": "Alice"}}
74	7553b30d	54	2026-10-26 19:28:26.241923+00	{"EndTurn": {"player": "Alice"}}
75	7553b30d	55	2026-10-26 19:29:48.072203+00	{"TacticalActionBegin": {"player": "Carol"}}
76	7553b30d	56	2026-10-26 19:29:48.613521+00	{"TacticalActionCommit": {"player": "Carol"}}
77	a9d66065	0	2026-10-26 19:57:10.664878+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
78	a9d66065	1	2026-10-26 19:57:12.268434+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
79	a9d66065	2	2026-10-26 19:57:12.86928+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
80	a9d66065	3	2026-10-26 19:57:13.489828+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
81	a9d66065	4	2026-10-26 19:57:14.038553+00	"CreationDone"
82	a9d66065	5	2026-10-26 19:57:15.078106+00	{"SetupSpeaker": {"player": "Alice"}}
83	a9d66065	6	2026-10-26 19:57:15.461864+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
84	a9d66065	7	2026-10-26 19:57:15.792639+00	"StartGame"
85	a9d66065	8	2026-10-26 19:57:16.826551+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
86	a9d66065	9	2026-10-26 19:57:17.260513+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
87	a9d66065	10	2026-10-26 19:57:17.693246+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
88	a9d66065	11	2026-10-26 19:57:18.125512+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
89	a9d66065	12	2026-10-26 19:57:18.558075+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
90	a9d66065	13	2026-10-26 19:57:18.982752+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
91	a9d66065	14	2026-10-26 19:57:19.416506+00	"CompleteStrategyPhase"
92	a9d66065	15	2026-10-26 19:57:20.455954+00	{"TacticalActionBegin": {"player": "Alice"}}
93	a9d66065	16	2026-10-26 19:57:20.986639+00	{"TacticalActionCommit": {"player": "Alice"}}
94	a9d66065	17	2026-10-26 19:57:37.12049+00	{"EndTurn": {"player": "Alice"}}
95	a9d66065	18	2026-10-26 19:58:39.200568+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Resources", "player": "Alice"}}
96	a9d66065	19	2026-10-26 19:58:39.836409+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "ActionCards", "player": "Alice"}}
97	a9d66065	20	2026-10-26 19:58:40.471546+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Influence", "player": "Alice"}}
98	a9d66065	21	2026-10-26 19:58:41.10589+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "SecretObjective", "player": "Alice"}}
99	a9d66065	22	2026-10-26 19:58:41.739914+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TechSpecialityPlanet", "player": "Alice"}}
100	a9d66065	23	2026-10-26 19:58:48.760666+00	{"TacticalActionBegin": {"player": "Bob"}}
101	a9d66065	24	2026-10-26 19:58:49.288739+00	{"TacticalActionCommit": {"player": "Bob"}}
102	a9d66065	25	2026-10-26 19:58:55.410098+00	{"CommitToThundersEdgeExpedition": {"slot": "TradeGoods", "player": "Bob"}}
103	04831ded	18	2026-10-27 08:33:00.198355+00	{"TacticalActionBegin": {"player": "Sponken"}}
104	04831ded	19	2026-10-27 08:33:01.219248+00	{"TacticalActionCommit": {"player": "Sponken"}}
105	05b6ffbb	0	2026-10-27 08:39:25.425876+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
106	05b6ffbb	1	2026-10-27 08:39:27.011629+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
107	05b6ffbb	2	2026-10-27 08:39:27.570139+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
108	05b6ffbb	3	2026-10-27 08:39:28.128406+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
109	05b6ffbb	4	2026-10-27 08:39:28.650349+00	"CreationDone"
110	05b6ffbb	5	2026-10-27 08:39:29.677237+00	{"SetupSpeaker": {"player": "Alice"}}
111	05b6ffbb	6	2026-10-27 08:39:30.033627+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
112	05b6ffbb	7	2026-10-27 08:39:30.352174+00	"StartGame"
113	05b6ffbb	8	2026-10-27 08:39:31.381675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
114	05b6ffbb	9	2026-10-27 08:39:31.815383+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
115	05b6ffbb	10	2026-10-27 08:39:32.247387+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
116	05b6ffbb	11	2026-10-27 08:39:32.663725+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
117	05b6ffbb	12	2026-10-27 08:39:33.080229+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
118	05b6ffbb	13	2026-10-27 08:39:33.49659+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
119	05b6ffbb	14	2026-10-27 08:39:33.913716+00	"CompleteStrategyPhase"
120	05b6ffbb	15	2026-10-27 08:39:34.932997+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
121	05b6ffbb	16	2026-10-27 08:39:35.553579+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
122	05b6ffbb	17	2026-10-27 08:39:35.971849+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
123	05b6ffbb	18	2026-10-27 08:39:36.387696+00	"StrategicActionCommit"
124	05b6ffbb	19	2026-10-27 08:39:36.903785+00	{"EndTurn": {"player": "Alice"}}
125	05b6ffbb	20	2026-10-27 08:39:37.420269+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
126	05b6ffbb	21	2026-10-27 08:39:38.039132+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
127	05b6ffbb	22	2026-10-27 08:39:38.454142+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
128	05b6ffbb	23	2026-10-27 08:39:38.883827+00	"StrategicActionCommit"
129	05b6ffbb	24	2026-10-27 08:39:39.39872+00	{"EndTurn": {"player": "Bob"}}
130	05b6ffbb	25	2026-10-27 08:39:39.916139+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
131	05b6ffbb	26	2026-10-27 08:39:40.552663+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
133	05b6ffbb	27	2026-10-27 08:41:38.825589+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Bob"}}, "player": "Carol"}}
134	05b6ffbb	28	2026-10-27 08:41:39.357827+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
135	05b6ffbb	29	2026-10-27 08:41:39.772701+00	"StrategicActionCommit"
136	05b6ffbb	30	2026-10-27 08:41:40.292193+00	{"EndTurn": {"player": "Carol"}}
137	05b6ffbb	31	2026-10-27 08:41:40.909012+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
138	05b6ffbb	32	2026-10-27 08:41:41.525111+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
139	05b6ffbb	33	2026-10-27 08:41:41.94286+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
140	05b6ffbb	34	2026-10-27 08:41:42.364312+00	"StrategicActionCommit"
141	05b6ffbb	35	2026-10-27 08:41:42.887877+00	{"EndTurn": {"player": "Alice"}}
142	05b6ffbb	36	2026-10-27 08:41:43.411376+00	{"TacticalActionBegin": {"player": "Bob"}}
143	05b6ffbb	37	2026-10-27 08:41:43.925964+00	{"TacticalActionCommit": {"player": "Bob"}}
144	05b6ffbb	38	2026-10-27 08:41:44.454122+00	{"EndTurn": {"player": "Bob"}}
145	05b6ffbb	39	2026-10-27 08:41:44.976709+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
146	05b6ffbb	40	2026-10-27 08:41:45.610164+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
147	05b6ffbb	41	2026-10-27 08:41:46.026212+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
148	05b6ffbb	42	2026-10-27 08:41:46.446631+00	"StrategicActionCommit"
149	05b6ffbb	43	2026-10-27 08:41:46.967187+00	{"EndTurn": {"player": "Carol"}}
150	05b6ffbb	44	2026-10-27 08:41:47.486833+00	{"PassAction": {"player": "Alice"}}
151	05b6ffbb	45	2026-10-27 08:41:48.103356+00	{"EndTurn": {"player": "Alice"}}
152	05b6ffbb	46	2026-10-27 08:41:48.728063+00	{"TacticalActionBegin": {"player": "Bob"}}
153	05b6ffbb	47	2026-10-27 08:41:49.244522+00	{"TacticalActionCommit": {"player": "Bob"}}
154	04831ded	20	2026-10-27 12:20:21.756477+00	{"PlayCrisis": {"player": "Tux"}}
155	04831ded	21	2026-10-27 12:20:24.978884+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
156	04831ded	22	2026-10-27 12:20:26.586864+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
157	04831ded	23	2026-10-27 12:20:27.7532+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Portals"}}
158	04831ded	24	2026-10-27 12:20:28.602027+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Sponken"}}
159	04831ded	25	2026-10-27 12:20:29.875012+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Vidde"}}
160	04831ded	26	2026-10-27 12:20:30.51674+00	"StrategicActionCommit"
161	04831ded	27	2026-10-27 12:20:34.129862+00	{"EndTurn": {"player": "Tux"}}
162	04831ded	28	2026-10-27 12:20:36.411008+00	{"StrategicActionBegin": {"card": "Trade", "player": "Gurr"}}
163	04831ded	29	2026-10-27 12:20:37.657026+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Portals"}}
164	04831ded	30	2026-10-27 12:20:38.385286+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Sponken"}}
165	04831ded	31	2026-10-27 12:20:39.292343+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Tux"}}
166	04831ded	32	2026-10-27 12:20:40.557971+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
167	04831ded	33	2026-10-27 12:20:41.730104+00	"StrategicActionCommit"
168	04831ded	34	2026-10-27 12:28:01.250869+00	{"EndTurn": {"player": "Gurr"}}
169	04831ded	35	2026-10-27 12:28:02.818521+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Portals"}}
170	04831ded	36	2026-10-27 12:28:04.33602+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
171	04831ded	37	2026-10-27 12:28:05.385954+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Sponken"}}
172	04831ded	38	2026-10-27 12:28:06.054807+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
173	04831ded	39	2026-10-27 12:28:06.902207+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Vidde"}}
174	04831ded	40	2026-10-27 12:28:07.415849+00	"StrategicActionCommit"
175	04831ded	41	2026-10-27 12:28:09.150606+00	{"EndTurn": {"player": "Portals"}}
176	04831ded	42	2026-10-27 12:28:11.338343+00	{"TacticalActionBegin": {"player": "Sponken"}}
177	04831ded	43	2026-10-27 12:28:12.96815+00	{"TacticalActionCommit": {"player": "Sponken"}}
178	04831ded	44	2026-10-27 12:28:15.468097+00	{"EndTurn": {"player": "Sponken"}}
179	04831ded	45	2026-10-27 12:28:17.586277+00	{"TacticalActionBegin": {"player": "Vidde"}}
180	04831ded	46	2026-10-27 12:28:18.576027+00	{"TacticalActionCommit": {"player": "Vidde"}}
181	04831ded	47	2026-10-27 12:28:19.755194+00	{"EndTurn": {"player": "Vidde"}}
182	04831ded	48	2026-10-27 12:28:20.90385+00	{"PassAction": {"player": "Tux"}}
183	04831ded	49	2026-10-27 12:28:27.930027+00	{"PlayPuppetsOnAString": {"player": "Tux"}}
184	04831ded	50	2026-10-27 14:01:05.637956+00	{"TacticalActionBegin": {"player": "Tux"}}
185	04831ded	51	2026-10-27 14:01:08.2425+00	{"TacticalActionTakePlanet": {"planet": "Styx", "player": "Tux"}}
186	04831ded	52	2026-10-27 14:01:10.501508+00	{"TacticalActionCommit": {"player": "Tux"}}
187	5aa9066f	0	2026-10-27 14:11:18.317151+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
188	b3085ac4	0	2026-10-27 14:31:40.019678+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
189	b3085ac4	1	2026-10-27 14:32:41.503153+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "Arborec"}}}
190	b3085ac4	2	2026-10-27 14:32:51.88362+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "BaronyOfLetnev"}}}
191	b3085ac4	3	2026-10-27 14:32:58.581239+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "ClanOfSaar"}}}
192	b3085ac4	4	2026-10-27 14:33:03.072617+00	"CreationDone"
193	b3085ac4	5	2026-10-27 14:33:13.772153+00	{"SetBookOfLatviniaOwner": {"player": "Bob"}}
194	b3085ac4	6	2026-10-27 14:33:17.809083+00	{"SetBookOfLatviniaOwner": {"player": null}}
195	b3085ac4	7	2026-10-27 14:33:58.708148+00	{"SetBookOfLatviniaOwner": {"player": "Alice"}}
196	04831ded	53	2026-10-27 14:35:16.71291+00	{"SetBookOfLatviniaOwner": {"player": "Gurr"}}
197	04831ded	54	2026-10-27 14:35:19.28459+00	{"SetBookOfLatviniaOwner": {"player": "Sponken"}}
198	04831ded	55	2026-10-27 14:35:20.495073+00	{"SetBookOfLatviniaOwner": {"player": "Vidde"}}
199	04831ded	56	2026-10-27 14:35:21.720981+00	{"SetBookOfLatviniaOwner": {"player": "Tux"}}
200	20a12af2	0	2026-10-27 14:59:00.039254+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
201	20a12af2	1	2026-10-27 14:59:17.374591+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "Arborec"}}}
202	20a12af2	2	2026-10-27 14:59:19.187249+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "BaronyOfLetnev"}}}
203	20a12af2	3	2026-10-27 14:59:20.985648+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "ClanOfSaar"}}}
204	20a12af2	4	2026-10-27 14:59:22.056482+00	"CreationDone"
205	20a12af2	5	2026-10-27 14:59:30.689919+00	{"SetupSpeaker": {"player": "Alice"}}
206	20a12af2	6	2026-10-27 14:59:34.335872+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
207	20a12af2	7	2026-10-27 14:59:34.900672+00	"StartGame"
23095	f61dbf9d	67	2026-10-30 11:24:23.156092+00	{"SetFractureInPlay": {"in_play": true}}
23096	f61dbf9d	68	2026-10-30 11:24:31.89135+00	{"SetFractureInPlay": {"in_play": false}}
23097	f61dbf9d	69	2026-10-30 11:24:42.005183+00	{"TacticalActionBegin": {"player": "Gurr"}}
23098	f61dbf9d	70	2026-10-30 11:24:59.084556+00	{"SetFractureInPlay": {"in_play": true}}
23099	f61dbf9d	71	2026-10-30 11:25:05.650068+00	{"TacticalActionTakePlanet": {"planet": "Cocytus", "player": "Gurr"}}
23100	f61dbf9d	72	2026-10-30 11:25:06.245681+00	{"TacticalActionCommit": {"player": "Gurr"}}
23101	f61dbf9d	73	2026-10-30 11:25:25.408056+00	{"EndTurn": {"player": "Gurr"}}
23102	f61dbf9d	74	2026-10-30 11:25:37.680361+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Resources", "player": null}}
23103	f61dbf9d	75	2026-10-30 11:25:38.351732+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "ActionCards", "player": null}}
23104	f61dbf9d	76	2026-10-30 11:25:38.781943+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Influence", "player": null}}
23105	f61dbf9d	77	2026-10-30 11:25:39.189367+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "SecretObjective", "player": null}}
23106	f61dbf9d	78	2026-10-30 11:25:39.599545+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TechSpecialityPlanet", "player": null}}
23107	f61dbf9d	79	2026-10-30 11:25:39.995654+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TradeGoods", "player": null}}
23108	f61dbf9d	80	2026-10-30 11:25:41.108483+00	{"SetThundersEdgeCompletion": {"new_completion": null}}
23109	f61dbf9d	81	2026-10-30 11:25:42.829153+00	{"SetFractureInPlay": {"in_play": false}}
23110	f61dbf9d	82	2026-10-30 11:25:46.558025+00	{"StrategicActionBegin": {"card": "Technology", "player": "Portals"}}
23111	f61dbf9d	83	2026-10-30 11:25:48.888397+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": null, "second_tech": null}}, "player": "Portals"}}
23112	f61dbf9d	84	2026-10-30 11:25:50.392714+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
23113	f61dbf9d	85	2026-10-30 11:25:53.221899+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
23114	f61dbf9d	86	2026-10-30 11:25:54.700355+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
23115	f61dbf9d	87	2026-10-30 11:26:05.973414+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Sponken"}}
23116	f61dbf9d	88	2026-10-30 11:26:06.724409+00	"StrategicActionCommit"
29772	f61dbf9d	97	2026-10-30 14:11:54.499437+00	{"PassAction": {"player": "Sponken"}}
23119	f61dbf9d	89	2026-10-30 11:26:26.298797+00	{"SetBreakthrough": {"player": "Gurr", "breakthrough": null}}
23120	f61dbf9d	90	2026-10-30 11:26:27.308093+00	{"SetBreakthrough": {"player": "Portals", "breakthrough": null}}
23121	f61dbf9d	91	2026-10-30 11:26:28.001973+00	{"SetBreakthrough": {"player": "Sponken", "breakthrough": null}}
23122	f61dbf9d	92	2026-10-30 11:26:28.523614+00	{"SetBreakthrough": {"player": "Tux", "breakthrough": null}}
23123	f61dbf9d	93	2026-10-30 11:26:28.846056+00	{"SetBreakthrough": {"player": "Vidde", "breakthrough": null}}
23124	f61dbf9d	94	2026-10-30 11:27:20.403953+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Portals"}}
23125	f61dbf9d	95	2026-10-30 11:27:27.879249+00	{"RollForTheFracture": {"player": "Portals", "entered_play": true}}
23126	f61dbf9d	96	2026-10-30 11:27:36.001729+00	{"EndTurn": {"player": "Portals"}}
29773	f61dbf9d	98	2026-10-30 14:11:56.091686+00	{"EndTurn": {"player": "Sponken"}}
29774	f61dbf9d	99	2026-10-30 14:11:57.363614+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}
29775	f61dbf9d	100	2026-10-30 14:12:02.359627+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
29776	f61dbf9d	101	2026-10-30 14:12:06.292167+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Portals"}}
29777	f61dbf9d	102	2026-10-30 14:12:07.531669+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Sponken"}}
29778	f61dbf9d	103	2026-10-30 14:12:08.199739+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
29779	f61dbf9d	104	2026-10-30 14:12:09.193319+00	"StrategicActionCommit"
29780	f61dbf9d	105	2026-10-30 14:12:11.787319+00	{"EndTurn": {"player": "Vidde"}}
29781	f61dbf9d	106	2026-10-30 14:12:14.653115+00	{"TacticalActionBegin": {"player": "Tux"}}
29782	f61dbf9d	107	2026-10-30 14:12:18.672791+00	{"TacticalActionTakePlanet": {"planet": "MecatolRexOmega", "player": "Tux"}}
29783	f61dbf9d	108	2026-10-30 14:12:19.324797+00	{"TacticalActionCommit": {"player": "Tux"}}
29784	f61dbf9d	109	2026-10-30 14:12:21.295531+00	{"EndTurn": {"player": "Tux"}}
27533	f4445ee6	2	2026-10-30 13:19:19.838616+00	{"AddPlayer": {"player": {"name": "Argus", "color": "Orange", "faction": "ArgentFlight"}}}
27534	f4445ee6	3	2026-10-30 13:20:19.838616+00	{"AddPlayer": {"player": {"name": "Kel", "color": "Green", "faction": "CouncilKeleres"}}}
27535	f4445ee6	4	2026-10-30 13:21:19.838616+00	"CreationDone"
27536	f4445ee6	5	2026-10-30 13:22:19.838616+00	{"SetupPlayerTechs": {"player": "Wendy", "technologies": ["NeuralMotivator"]}}
27537	f4445ee6	6	2026-10-30 13:23:19.838616+00	{"SetupPlayerTechs": {"player": "Argus", "technologies": ["SarweenTools", "PlasmaScoring"]}}
27538	f4445ee6	7	2026-10-30 13:24:19.838616+00	{"SetupSpeaker": {"player": "Kel"}}
27539	f4445ee6	8	2026-10-30 13:25:19.838616+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "BuildDefenses"}}
27540	f4445ee6	9	2026-10-30 13:26:19.838616+00	{"SetupTheTribunii": {"player": "Kel", "faction": "XxchaKingdom"}}
27541	f4445ee6	10	2026-10-30 13:27:19.838616+00	{"SetupPlayerTechs": {"player": "Kel", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
27542	f4445ee6	11	2026-10-30 13:28:19.838616+00	"StartGame"
27543	5c596a6c	0	2026-10-30 13:13:19.910638+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "74", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "113", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "100", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "39", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "102", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "36", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "78", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "59", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "34", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "32", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "68", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "47", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "49", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "50", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 0}}}, {"system": "64", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "24", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "35", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "80", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 0}}}, {"system": "107", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Ann": {"name": "Ann", "order": 2, "faction": "Arborec"}, "Ben": {"name": "Ben", "order": 0, "faction": "NaazRokhaAlliance"}, "Cid": {"name": "Cid", "order": 1, "faction": "RalNelConsortium"}, "Dee": {"name": "Dee", "order": 3, "faction": "DeepwroughtScholarate"}, "Eve": {"name": "Eve", "order": 4, "faction": "FirmamentObsidian"}}, "game_name": "Scenario", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
27544	5c596a6c	1	2026-10-30 13:14:19.910638+00	"CreationDone"
27545	5c596a6c	2	2026-10-30 13:15:19.910638+00	{"SetupPlayerTechs": {"player": "Cid", "technologies": ["NeuralMotivator"]}}
27546	5c596a6c	3	2026-10-30 13:16:19.910638+00	{"SetupPlayerTechs": {"player": "Dee", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
27547	5c596a6c	4	2026-10-30 13:17:19.910638+00	{"SetupPlayerTechs": {"player": "Eve", "technologies": ["NeuralMotivator"]}}
27548	5c596a6c	5	2026-10-30 13:18:19.910638+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27549	5c596a6c	6	2026-10-30 13:19:19.910638+00	"StartGame"
27550	5c596a6c	7	2026-10-30 13:20:19.910638+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Ben"}}
27551	5c596a6c	8	2026-10-30 13:21:19.910638+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Cid"}}
27552	5c596a6c	9	2026-10-30 13:22:19.910638+00	{"TakeStrategyCard": {"card": "Politics", "player": "Ann"}}
27553	5c596a6c	10	2026-10-30 13:23:19.910638+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dee"}}
27554	5c596a6c	11	2026-10-30 13:24:19.910638+00	{"TakeStrategyCard": {"card": "Trade", "player": "Eve"}}
27555	5c596a6c	12	2026-10-30 13:25:19.910638+00	"CompleteStrategyPhase"
27556	5c596a6c	13	2026-10-30 13:26:19.910638+00	{"FrontierCardActionBegin": {"card": "Mirage", "player": "Ben"}}
27557	5c596a6c	14	2026-10-30 13:27:19.910638+00	{"FrontierCardActionCommit": {"data": {"Mirage": {"system": "65", "attachment": "ParadiseWorld"}}, "player": "Ben"}}
27558	5c596a6c	15	2026-10-30 13:28:19.910638+00	{"EndTurn": {"player": "Ben"}}
27559	69d97f21	0	2026-10-30 13:14:19.99373+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27560	69d97f21	1	2026-10-30 13:15:19.99373+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27561	69d97f21	2	2026-10-30 13:16:19.99373+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27562	69d97f21	3	2026-10-30 13:17:19.99373+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27563	69d97f21	4	2026-10-30 13:18:19.99373+00	"CreationDone"
27564	69d97f21	5	2026-10-30 13:19:19.99373+00	{"SetupSpeaker": {"player": "Alice"}}
27565	69d97f21	6	2026-10-30 13:20:19.99373+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27566	69d97f21	7	2026-10-30 13:21:19.99373+00	"StartGame"
27567	69d97f21	8	2026-10-30 13:22:19.99373+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Alice"}}
27568	69d97f21	9	2026-10-30 13:23:19.99373+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Bob"}}
29785	f61dbf9d	110	2026-10-30 14:12:22.528509+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Gurr"}}
29786	f61dbf9d	111	2026-10-30 14:12:23.805898+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Portals"}}
29787	f61dbf9d	112	2026-10-30 14:12:25.423953+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Sponken"}}
29788	f61dbf9d	113	2026-10-30 14:12:26.505549+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Tux"}}
29789	f61dbf9d	114	2026-10-30 14:12:27.828348+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
18801	f61dbf9d	66	2026-10-29 12:55:15.496962+00	{"EndTurn": {"player": "Tux"}}
12397	f61dbf9d	62	2026-10-29 11:44:46.488598+00	{"EndTurn": {"player": "Vidde"}}
12398	f61dbf9d	63	2026-10-29 11:44:47.488361+00	{"TacticalActionBegin": {"player": "Tux"}}
12399	f61dbf9d	64	2026-10-29 11:44:50.441131+00	{"TacticalActionTakePlanet": {"planet": "Cocytus", "player": "Tux"}}
12400	f61dbf9d	65	2026-10-29 11:44:52.160032+00	{"TacticalActionCommit": {"player": "Tux"}}
20945	f61dbf9d	10	2026-10-28 23:24:32.017914+00	{"RollForTheFracture": {"player": "Vidde", "entered_play": false}}
20946	f61dbf9d	24	2026-10-28 23:26:07.897791+00	{"RollForTheFracture": {"player": "Sponken", "entered_play": false}}
20947	f61dbf9d	38	2026-10-28 23:26:52.935118+00	{"RollForTheFracture": {"player": "Tux", "entered_play": false}}
20948	f61dbf9d	43	2026-10-28 23:27:02.456495+00	{"RollForTheFracture": {"player": "Gurr", "entered_play": false}}
20949	f61dbf9d	48	2026-10-28 23:27:10.118533+00	{"RollForTheFracture": {"player": "Portals", "entered_play": false}}
10249	f61dbf9d	4	2026-10-28 23:24:16.020567+00	{"SetupPlayerTechs": {"player": "Sponken", "technologies": ["HyperMetabolism", "SarweenTools"]}}
10250	f61dbf9d	5	2026-10-28 23:24:19.050552+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["AntimassDeflectors"]}}
10251	f61dbf9d	6	2026-10-28 23:24:21.967393+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["DarkEnergyTap"]}}
10252	f61dbf9d	7	2026-10-28 23:24:25.002978+00	{"SetupPlayerTechs": {"player": "Gurr", "technologies": ["SarweenTools"]}}
10253	f61dbf9d	8	2026-10-28 23:24:30.819851+00	{"RevealInitialObjectives": {"firstObjective": "ErectAMonument", "secondObjective": "ExploreDeepSpace"}}
10254	f61dbf9d	9	2026-10-28 23:24:32.016914+00	"StartGame"
10255	f61dbf9d	11	2026-10-28 23:24:42.77362+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Sponken"}}
10256	f61dbf9d	12	2026-10-28 23:24:45.060727+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
10257	f61dbf9d	13	2026-10-28 23:24:48.148385+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
10258	f61dbf9d	14	2026-10-28 23:24:49.992499+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Gurr"}}
10259	f61dbf9d	15	2026-10-28 23:24:53.31859+00	{"TakeStrategyCard": {"card": "Technology", "player": "Portals"}}
10260	f61dbf9d	16	2026-10-28 23:24:54.991746+00	"CompleteStrategyPhase"
10261	f61dbf9d	17	2026-10-28 23:26:01.083909+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Sponken"}}
10262	f61dbf9d	18	2026-10-28 23:26:02.084907+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
10263	f61dbf9d	19	2026-10-28 23:26:03.310024+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Portals"}}
10264	f61dbf9d	20	2026-10-28 23:26:03.983174+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
10265	f61dbf9d	21	2026-10-28 23:26:04.528289+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Vidde"}}
10266	f61dbf9d	22	2026-10-28 23:26:05.387844+00	"StrategicActionCommit"
10267	f61dbf9d	23	2026-10-28 23:26:07.896791+00	{"CommitToThundersEdgeExpedition": {"slot": "ActionCards", "player": "Sponken"}}
10268	f61dbf9d	25	2026-10-28 23:26:09.266586+00	{"EndTurn": {"player": "Sponken"}}
10269	f61dbf9d	26	2026-10-28 23:26:10.745692+00	{"TacticalActionBegin": {"player": "Vidde"}}
10270	f61dbf9d	27	2026-10-28 23:26:13.753155+00	{"TacticalActionTakePlanet": {"planet": "TsionStation", "player": "Vidde"}}
10271	f61dbf9d	28	2026-10-28 23:26:14.603175+00	{"TacticalActionTakePlanet": {"planet": "Bellatrix", "player": "Vidde"}}
10272	f61dbf9d	29	2026-10-28 23:26:15.30436+00	{"TacticalActionCommit": {"player": "Vidde"}}
10273	f61dbf9d	30	2026-10-28 23:26:17.4365+00	{"EndTurn": {"player": "Vidde"}}
10274	f61dbf9d	31	2026-10-28 23:26:41.06279+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
10275	f61dbf9d	32	2026-10-28 23:26:46.254052+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}
10276	f61dbf9d	33	2026-10-28 23:26:47.035378+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Portals"}}
10277	f61dbf9d	34	2026-10-28 23:26:47.773656+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Sponken"}}
10278	f61dbf9d	35	2026-10-28 23:26:48.43481+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
10279	f61dbf9d	36	2026-10-28 23:26:48.999616+00	"StrategicActionCommit"
10280	f61dbf9d	37	2026-10-28 23:26:52.934118+00	{"CommitToThundersEdgeExpedition": {"slot": "Influence", "player": "Tux"}}
10281	f61dbf9d	39	2026-10-28 23:26:55.383552+00	{"EndTurn": {"player": "Tux"}}
10282	f61dbf9d	40	2026-10-28 23:26:57.529257+00	{"TacticalActionBegin": {"player": "Gurr"}}
10283	f61dbf9d	41	2026-10-28 23:26:58.845105+00	{"TacticalActionCommit": {"player": "Gurr"}}
10284	f61dbf9d	42	2026-10-28 23:27:02.455495+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Gurr"}}
10285	f61dbf9d	44	2026-10-28 23:27:04.000281+00	{"EndTurn": {"player": "Gurr"}}
10294	f61dbf9d	54	2026-10-28 23:27:24.015255+00	{"TacticalActionAttachPlanetAttachment": {"planet": "LirtaIV", "player": "Sponken", "attachment": "WarfareResearchFacility"}}
10296	f61dbf9d	56	2026-10-28 23:27:28.542846+00	{"CommitToThundersEdgeExpedition": {"slot": "SecretObjective", "player": "Sponken"}}
10297	f61dbf9d	57	2026-10-28 23:27:31.74419+00	{"EndTurn": {"player": "Sponken"}}
10298	f61dbf9d	58	2026-10-28 23:27:33.603734+00	{"TacticalActionBegin": {"player": "Vidde"}}
10299	f61dbf9d	59	2026-10-28 23:27:34.785919+00	{"TacticalActionCommit": {"player": "Vidde"}}
10300	f61dbf9d	60	2026-10-28 23:27:38.368882+00	{"CommitToThundersEdgeExpedition": {"slot": "TechSpecialityPlanet", "player": "Vidde"}}
10303	f61dbf9d	61	2026-10-28 23:29:28.075056+00	{"CompleteThundersEdgeExpedition": {"player": "Vidde", "won_by": "Sponken", "placed_in": "47"}}
29790	f61dbf9d	115	2026-10-30 14:12:29.259746+00	"StrategicActionCommit"
29791	f61dbf9d	116	2026-10-30 14:12:31.533889+00	{"EndTurn": {"player": "Gurr"}}
29792	f61dbf9d	117	2026-10-30 14:12:33.250944+00	{"PassAction": {"player": "Portals"}}
29793	f61dbf9d	118	2026-10-30 14:12:35.673355+00	{"EndTurn": {"player": "Portals"}}
29794	f61dbf9d	119	2026-10-30 14:12:37.81381+00	{"PassAction": {"player": "Vidde"}}
29795	f61dbf9d	120	2026-10-30 14:12:39.659226+00	{"EndTurn": {"player": "Vidde"}}
29796	f61dbf9d	121	2026-10-30 14:12:41.129124+00	{"PassAction": {"player": "Tux"}}
29797	f61dbf9d	122	2026-10-30 14:12:43.499348+00	{"EndTurn": {"player": "Tux"}}
29798	f61dbf9d	123	2026-10-30 14:12:44.982746+00	{"PassAction": {"player": "Gurr"}}
29799	f61dbf9d	124	2026-10-30 14:12:48.577267+00	{"EndTurn": {"player": "Gurr"}}
29800	f61dbf9d	125	2026-10-30 14:12:53.20683+00	{"ScorePublicObjective": {"player": "Sponken", "objective": null}}
29801	f61dbf9d	126	2026-10-30 14:12:55.321562+00	{"ScoreSecretObjective": {"player": "Sponken", "objective": null}}
29802	f61dbf9d	127	2026-10-30 14:12:56.2832+00	{"ScorePublicObjective": {"player": "Vidde", "objective": null}}
29803	f61dbf9d	128	2026-10-30 14:12:58.710288+00	{"ScoreSecretObjective": {"player": "Vidde", "objective": null}}
29804	f61dbf9d	129	2026-10-30 14:12:59.738427+00	{"ScorePublicObjective": {"player": "Tux", "objective": null}}
29805	f61dbf9d	130	2026-10-30 14:13:00.953293+00	{"ScoreSecretObjective": {"player": "Tux", "objective": null}}
29806	f61dbf9d	131	2026-10-30 14:13:02.206535+00	{"ScorePublicObjective": {"player": "Gurr", "objective": null}}
29807	f61dbf9d	132	2026-10-30 14:13:04.386738+00	{"ScoreSecretObjective": {"player": "Gurr", "objective": null}}
29808	f61dbf9d	133	2026-10-30 14:13:05.459583+00	{"ScorePublicObjective": {"player": "Portals", "objective": null}}
29809	f61dbf9d	134	2026-10-30 14:13:06.943396+00	{"ScoreSecretObjective": {"player": "Portals", "objective": null}}
29810	f61dbf9d	135	2026-10-30 14:13:09.727245+00	{"RevealPublicObjective": {"objective": "ImproveInfrastructure"}}
29811	f61dbf9d	136	2026-10-30 14:13:11.230516+00	"CompleteStatusPhase"
29812	f61dbf9d	137	2026-10-30 14:13:41.415625+00	{"RevealAgenda": {"agenda": "MinisterOfAntiques"}}
29813	f61dbf9d	138	2026-10-30 14:13:46.143044+00	{"CastAgendaVote": {"votes": 45, "player": "Vidde", "outcome": {"value": "Portals", "electKind": "Player"}}}
29814	f61dbf9d	139	2026-10-30 14:13:47.497818+00	{"CastAgendaVote": {"votes": 0, "player": "Tux", "outcome": null}}
29815	f61dbf9d	140	2026-10-30 14:13:48.600528+00	{"CastAgendaVote": {"votes": 0, "player": "Gurr", "outcome": null}}
29816	f61dbf9d	141	2026-10-30 14:13:49.637492+00	{"CastAgendaVote": {"votes": 0, "player": "Portals", "outcome": null}}
29817	f61dbf9d	142	2026-10-30 14:13:50.64297+00	{"CastAgendaVote": {"votes": 0, "player": "Sponken", "outcome": null}}
29818	f61dbf9d	143	2026-10-30 14:13:55.47381+00	{"ResolveAgenda": {"outcome": {"value": "Portals", "electKind": "Player"}}}
29819	f61dbf9d	144	2026-10-30 14:13:59.747337+00	{"GainMinisterOfAntiquesRelic": {"relic": "MetaliVoidArmaments", "player": "Portals"}}
31156	942cbdd8	34	2026-11-05 23:11:02.322336+00	"StrategicActionCommit"
31157	942cbdd8	35	2026-11-05 23:11:02.329222+00	{"EndTurn": {"player": "Tux"}}
31158	942cbdd8	36	2026-11-05 23:11:02.335783+00	{"StrategicActionBegin": {"card": "Trade", "player": "Vidde"}}
31159	942cbdd8	37	2026-11-05 23:11:02.343283+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Tux"}}
31160	942cbdd8	38	2026-11-05 23:11:02.349716+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Yoda"}}
10245	f61dbf9d	0	2026-10-28 23:23:56.725694+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "104", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "25", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "61", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "19", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "59", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "33", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "44", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "109", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "39", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "72", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "79", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "26", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "45", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "40", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "34", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "114", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "28", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "92", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "78", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "97", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 0}}}, {"system": "47", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "70", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 3, "faction": "LastBastion"}, "Gurr": {"name": "Gurr", "order": 4, "faction": "FirmamentObsidian"}, "Vidde": {"name": "Vidde", "order": 2, "faction": "CrimsonRebellion"}, "Portals": {"name": "Portals", "order": 0, "faction": "RalNelConsortium"}, "Sponken": {"name": "Sponken", "order": 1, "faction": "DeepwroughtScholarate"}}, "game_name": "Helluw", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
10246	f61dbf9d	1	2026-10-28 23:24:00.286255+00	"CreationDone"
10247	f61dbf9d	2	2026-10-28 23:24:04.153519+00	{"SetupPlayerTechs": {"player": "Portals", "technologies": ["Psychoarchaeology"]}}
10248	f61dbf9d	3	2026-10-28 23:24:05.636042+00	{"SetupSpeaker": {"player": "Sponken"}}
10286	f61dbf9d	45	2026-10-28 23:27:06.484239+00	{"TacticalActionBegin": {"player": "Portals"}}
10287	f61dbf9d	46	2026-10-28 23:27:07.458649+00	{"TacticalActionCommit": {"player": "Portals"}}
10288	f61dbf9d	47	2026-10-28 23:27:10.117533+00	{"CommitToThundersEdgeExpedition": {"slot": "TradeGoods", "player": "Portals"}}
10289	f61dbf9d	49	2026-10-28 23:27:11.328702+00	{"EndTurn": {"player": "Portals"}}
10290	f61dbf9d	50	2026-10-28 23:27:13.241054+00	{"TacticalActionBegin": {"player": "Sponken"}}
10291	f61dbf9d	51	2026-10-28 23:27:16.949209+00	{"TacticalActionTakePlanet": {"planet": "Bereg", "player": "Sponken"}}
10292	f61dbf9d	52	2026-10-28 23:27:19.677592+00	{"TacticalActionAttachPlanetAttachment": {"planet": "Bereg", "player": "Sponken", "attachment": "WarfareResearchFacility"}}
10293	f61dbf9d	53	2026-10-28 23:27:20.565902+00	{"TacticalActionTakePlanet": {"planet": "LirtaIV", "player": "Sponken"}}
10295	f61dbf9d	55	2026-10-28 23:27:24.915282+00	{"TacticalActionCommit": {"player": "Sponken"}}
29820	450c18d7	0	2026-10-30 17:03:50.316449+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
29821	450c18d7	1	2026-10-30 17:04:39.939397+00	{"AddPlayer": {"player": {"name": "Alexandria Vandersloot", "color": "Black", "faction": "ArgentFlight"}}}
29822	450c18d7	2	2026-10-30 17:04:40.486384+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "CouncilKeleres"}}}
29823	450c18d7	3	2026-10-30 17:04:41.020424+00	{"AddPlayer": {"player": {"name": "Charlie", "color": "Green", "faction": "DeepwroughtScholarate"}}}
29824	450c18d7	4	2026-10-30 17:04:41.532094+00	{"AddPlayer": {"player": {"name": "Dee", "color": "Orange", "faction": "Winnu"}}}
29825	450c18d7	5	2026-10-30 17:04:42.068013+00	{"AddPlayer": {"player": {"name": "Eve", "color": "Red", "faction": "FederationOfSol"}}}
29826	450c18d7	6	2026-10-30 17:04:42.620529+00	{"AddPlayer": {"player": {"name": "Frank", "color": "Yellow", "faction": "FirmamentObsidian"}}}
29827	450c18d7	7	2026-10-30 17:04:54.135512+00	"CreationDone"
29828	450c18d7	8	2026-10-30 17:05:07.08859+00	{"SetupPlayerTechs": {"player": "Alexandria Vandersloot", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
29829	450c18d7	9	2026-10-30 17:05:07.478739+00	{"SetupPlayerTechs": {"player": "Dee", "technologies": ["AiDevelopmentAlgorithm"]}}
29830	450c18d7	10	2026-10-30 17:05:07.846429+00	{"SetupPlayerTechs": {"player": "Frank", "technologies": ["NeuralMotivator"]}}
29831	450c18d7	11	2026-10-30 17:05:16.721522+00	{"SetupTheTribunii": {"player": "Bob", "faction": "MentakCoalition"}}
29832	450c18d7	12	2026-10-30 17:05:33.30494+00	{"SetupPlayerTechs": {"player": "Charlie", "technologies": ["BioStims", "AntimassDeflectors"]}}
31161	942cbdd8	39	2026-11-05 23:11:02.357993+00	"StrategicActionCommit"
31162	942cbdd8	40	2026-11-05 23:11:02.364014+00	{"EndTurn": {"player": "Vidde"}}
31163	942cbdd8	41	2026-11-05 23:11:02.370523+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Yoda"}}
31164	942cbdd8	42	2026-11-05 23:11:02.376811+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Tux"}}
31165	942cbdd8	43	2026-11-05 23:11:02.38363+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Vidde"}}
31166	942cbdd8	44	2026-11-05 23:11:02.389856+00	"StrategicActionCommit"
31167	942cbdd8	45	2026-11-05 23:11:02.396013+00	{"EndTurn": {"player": "Yoda"}}
31168	942cbdd8	46	2026-11-05 23:11:02.402293+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
31169	942cbdd8	47	2026-11-05 23:11:02.408507+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31170	942cbdd8	48	2026-11-05 23:11:02.415204+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Yoda"}}
31171	942cbdd8	49	2026-11-05 23:11:02.421654+00	"StrategicActionCommit"
31172	942cbdd8	50	2026-11-05 23:11:02.428185+00	{"EndTurn": {"player": "Tux"}}
31173	942cbdd8	51	2026-11-05 23:11:02.435298+00	{"PassAction": {"player": "Vidde"}}
31174	942cbdd8	52	2026-11-05 23:11:02.441459+00	{"EndTurn": {"player": "Vidde"}}
31175	942cbdd8	53	2026-11-05 23:11:02.448148+00	{"PassAction": {"player": "Yoda"}}
31176	942cbdd8	54	2026-11-05 23:11:02.45409+00	{"EndTurn": {"player": "Yoda"}}
31177	942cbdd8	55	2026-11-05 23:11:02.460231+00	{"PassAction": {"player": "Tux"}}
31178	942cbdd8	56	2026-11-05 23:11:02.466593+00	{"EndTurn": {"player": "Tux"}}
31179	942cbdd8	57	2026-11-05 23:11:02.472936+00	{"ScorePublicObjective": {"player": "Tux", "objective": null}}
31180	942cbdd8	58	2026-11-05 23:11:02.479082+00	{"ScorePublicObjective": {"player": "Vidde", "objective": null}}
31181	942cbdd8	59	2026-11-05 23:11:02.484903+00	{"ScorePublicObjective": {"player": "Yoda", "objective": null}}
31182	942cbdd8	60	2026-11-05 23:11:02.490815+00	{"ScoreSecretObjective": {"player": "Tux", "objective": null}}
31183	942cbdd8	61	2026-11-05 23:11:02.49737+00	{"ScoreSecretObjective": {"player": "Vidde", "objective": null}}
31184	942cbdd8	62	2026-11-05 23:11:02.503658+00	{"ScoreSecretObjective": {"player": "Yoda", "objective": null}}
29833	450c18d7	13	2026-10-30 17:06:52.310842+00	{"SetupPlayerTechs": {"player": "Bob", "technologies": ["AiDevelopmentAlgorithm", "AntimassDeflectors"]}}
29834	450c18d7	14	2026-10-30 18:22:48.037118+00	{"SetupSpeaker": {"player": "Alexandria Vandersloot"}}
29835	450c18d7	15	2026-10-30 18:22:48.412071+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
29836	450c18d7	16	2026-10-30 18:22:48.844022+00	"StartGame"
29837	450c18d7	17	2026-10-30 18:37:57.505471+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alexandria Vandersloot"}}
29838	450c18d7	18	2026-10-30 18:37:57.937959+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29839	450c18d7	19	2026-10-30 18:37:58.363635+00	{"TakeStrategyCard": {"card": "Politics", "player": "Charlie"}}
29840	450c18d7	20	2026-10-30 18:37:58.79821+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dee"}}
29841	450c18d7	21	2026-10-30 18:37:59.228858+00	{"TakeStrategyCard": {"card": "Trade", "player": "Eve"}}
8146	04831ded	57	2026-10-28 21:57:13.467541+00	{"EndTurn": {"player": "Tux"}}
29842	450c18d7	22	2026-10-30 18:37:59.665295+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Frank"}}
29843	450c18d7	23	2026-10-30 18:38:00.094513+00	"CompleteStrategyPhase"
8149	04831ded	58	2026-10-28 22:03:15.966598+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Gurr"}}
8150	04831ded	59	2026-10-28 22:03:26.790069+00	{"ResearchBookOfLatviniaTechs": {"player": "Gurr", "technologies": ["AntimassDeflectors", "AiDevelopmentAlgorithm"]}}
8151	04831ded	60	2026-10-28 22:04:17.014277+00	{"EndTurn": {"player": "Gurr"}}
29844	450c18d7	24	2026-10-30 18:38:12.312422+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alexandria Vandersloot"}}
29845	450c18d7	25	2026-10-30 18:38:18.852456+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29846	450c18d7	26	2026-10-30 18:38:19.19382+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
8137	3c42917a	0	2026-10-28 21:28:07.923562+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
8138	3c42917a	1	2026-10-28 21:29:27.792668+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
8139	3c42917a	2	2026-10-28 21:29:31.558572+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
8140	3c42917a	3	2026-10-28 21:29:32.862402+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
8145	3c42917a	4	2026-10-28 21:50:03.978033+00	"CreationDone"
29847	450c18d7	27	2026-10-30 18:38:19.544291+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29848	450c18d7	28	2026-10-30 18:38:19.876581+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29849	450c18d7	29	2026-10-30 18:38:20.210308+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29850	450c18d7	30	2026-10-30 18:38:25.505967+00	"StrategicActionCommit"
29851	450c18d7	31	2026-10-30 18:39:16.302593+00	{"EndTurn": {"player": "Alexandria Vandersloot"}}
29852	450c18d7	32	2026-10-30 18:39:17.14544+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29853	450c18d7	33	2026-10-30 18:39:25.165366+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29854	450c18d7	34	2026-10-30 18:39:25.460509+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29855	450c18d7	35	2026-10-30 18:39:25.741749+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29856	450c18d7	36	2026-10-30 18:39:26.021613+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29857	450c18d7	37	2026-10-30 18:39:26.30462+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29858	450c18d7	38	2026-10-30 18:39:26.588095+00	"StrategicActionCommit"
29859	450c18d7	39	2026-10-30 18:39:27.229739+00	{"EndTurn": {"player": "Bob"}}
29860	450c18d7	40	2026-10-30 18:39:27.862799+00	{"StrategicActionBegin": {"card": "Politics", "player": "Charlie"}}
29861	450c18d7	41	2026-10-30 18:39:35.678558+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Charlie"}}, "player": "Charlie"}}
29862	450c18d7	42	2026-10-30 18:39:36.21098+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29863	450c18d7	43	2026-10-30 18:39:36.494008+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29864	450c18d7	44	2026-10-30 18:39:36.772005+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29865	450c18d7	45	2026-10-30 18:39:37.071542+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29866	450c18d7	46	2026-10-30 18:39:37.352428+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29867	450c18d7	47	2026-10-30 18:39:37.655222+00	"StrategicActionCommit"
29868	450c18d7	48	2026-10-30 18:39:38.284739+00	{"EndTurn": {"player": "Charlie"}}
29869	450c18d7	49	2026-10-30 18:39:38.917255+00	{"StrategicActionBegin": {"card": "Construction", "player": "Dee"}}
29870	450c18d7	50	2026-10-30 18:39:46.324656+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29871	450c18d7	51	2026-10-30 18:39:46.608551+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29872	450c18d7	52	2026-10-30 18:39:46.910057+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29873	450c18d7	53	2026-10-30 18:39:47.191713+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29874	450c18d7	54	2026-10-30 18:39:47.475761+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29875	450c18d7	55	2026-10-30 18:39:47.77718+00	"StrategicActionCommit"
29876	450c18d7	56	2026-10-30 18:39:48.407165+00	{"EndTurn": {"player": "Dee"}}
29877	450c18d7	57	2026-10-30 18:39:49.028449+00	{"TacticalActionBegin": {"player": "Eve"}}
29878	450c18d7	58	2026-10-30 18:40:26.090631+00	{"TacticalActionTakePlanet": {"planet": "AhkCreuxx", "player": "Eve"}}
29879	450c18d7	59	2026-10-30 18:40:36.067299+00	{"TacticalActionCommit": {"player": "Eve"}}
29880	450c18d7	60	2026-10-30 18:40:36.697741+00	{"EndTurn": {"player": "Eve"}}
29887	450c18d7	61	2026-10-30 18:42:59.349475+00	{"SetRelicOwner": {"relic": "BookOfLatvinia", "player": "Frank"}}
29888	450c18d7	62	2026-10-30 18:44:13.277642+00	{"ResearchBookOfLatviniaTechs": {"player": "Frank", "technologies": []}}
29889	450c18d7	63	2026-10-30 18:44:14.682123+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Frank"}}
29890	450c18d7	64	2026-10-30 18:44:15.318213+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29891	450c18d7	65	2026-10-30 18:44:15.958389+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29892	450c18d7	66	2026-10-30 18:44:16.590178+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29893	450c18d7	67	2026-10-30 18:44:17.222836+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29894	450c18d7	68	2026-10-30 18:44:17.855839+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29895	450c18d7	69	2026-10-30 18:44:18.494036+00	"StrategicActionCommit"
29896	450c18d7	70	2026-10-30 18:44:19.125625+00	{"EndTurn": {"player": "Frank"}}
29897	450c18d7	71	2026-10-30 18:44:19.760079+00	{"PassAction": {"player": "Alexandria Vandersloot"}}
29898	450c18d7	72	2026-10-30 18:44:20.393844+00	{"EndTurn": {"player": "Alexandria Vandersloot"}}
29899	450c18d7	73	2026-10-30 18:44:21.025523+00	{"PassAction": {"player": "Bob"}}
29900	450c18d7	74	2026-10-30 18:44:21.678517+00	{"EndTurn": {"player": "Bob"}}
29901	450c18d7	75	2026-10-30 18:44:22.314122+00	{"PassAction": {"player": "Charlie"}}
29902	450c18d7	76	2026-10-30 18:44:22.947207+00	{"EndTurn": {"player": "Charlie"}}
29903	450c18d7	77	2026-10-30 18:44:23.589108+00	{"PassAction": {"player": "Dee"}}
29904	450c18d7	78	2026-10-30 18:44:24.242141+00	{"EndTurn": {"player": "Dee"}}
29905	450c18d7	79	2026-10-30 18:44:24.892558+00	{"StrategicActionBegin": {"card": "Trade", "player": "Eve"}}
29906	450c18d7	80	2026-10-30 18:44:25.539357+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29907	450c18d7	81	2026-10-30 18:44:26.1749+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29908	450c18d7	82	2026-10-30 18:44:26.807878+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29909	450c18d7	83	2026-10-30 18:44:27.442627+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29910	450c18d7	84	2026-10-30 18:44:28.073999+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29911	450c18d7	85	2026-10-30 18:44:28.722252+00	"StrategicActionCommit"
29912	450c18d7	86	2026-10-30 18:44:29.372882+00	{"EndTurn": {"player": "Eve"}}
29913	450c18d7	87	2026-10-30 18:44:30.040685+00	{"PassAction": {"player": "Frank"}}
29914	450c18d7	88	2026-10-30 18:44:30.690892+00	{"EndTurn": {"player": "Frank"}}
29915	450c18d7	89	2026-10-30 18:44:31.34056+00	{"PassAction": {"player": "Eve"}}
29916	450c18d7	90	2026-10-30 18:44:32.007427+00	{"EndTurn": {"player": "Eve"}}
29917	450c18d7	91	2026-10-30 18:44:47.519169+00	{"ScorePublicObjective": {"player": "Alexandria Vandersloot", "objective": null}}
29918	450c18d7	92	2026-10-30 18:44:47.808171+00	{"ScoreSecretObjective": {"player": "Alexandria Vandersloot", "objective": null}}
29919	450c18d7	93	2026-10-30 18:44:48.094037+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29920	450c18d7	94	2026-10-30 18:44:48.375146+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29921	450c18d7	95	2026-10-30 18:44:48.677464+00	{"ScorePublicObjective": {"player": "Charlie", "objective": null}}
29922	450c18d7	96	2026-10-30 18:44:48.968403+00	{"ScoreSecretObjective": {"player": "Charlie", "objective": null}}
29923	450c18d7	97	2026-10-30 18:44:49.266055+00	{"ScorePublicObjective": {"player": "Dee", "objective": null}}
29924	450c18d7	98	2026-10-30 18:44:49.550052+00	{"ScoreSecretObjective": {"player": "Dee", "objective": null}}
29925	450c18d7	99	2026-10-30 18:44:49.832075+00	{"ScorePublicObjective": {"player": "Eve", "objective": null}}
29926	450c18d7	100	2026-10-30 18:44:50.135805+00	{"ScoreSecretObjective": {"player": "Eve", "objective": null}}
29927	450c18d7	101	2026-10-30 18:44:50.416053+00	{"ScorePublicObjective": {"player": "Frank", "objective": null}}
29928	450c18d7	102	2026-10-30 18:44:50.699134+00	{"ScoreSecretObjective": {"player": "Frank", "objective": null}}
29929	450c18d7	103	2026-10-30 18:44:51.009311+00	{"RevealPublicObjective": {"objective": "CornerTheMarket"}}
29930	450c18d7	104	2026-10-30 18:44:51.545263+00	"CompleteStatusPhase"
29931	450c18d7	105	2026-10-30 18:45:01.512902+00	{"SetCustodians": {"player": "Alexandria Vandersloot"}}
29932	450c18d7	106	2026-10-30 18:45:02.918404+00	{"TakeStrategyCard": {"card": "Technology", "player": "Charlie"}}
29933	450c18d7	107	2026-10-30 18:45:03.300165+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Dee"}}
29934	450c18d7	108	2026-10-30 18:45:03.683849+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Eve"}}
29935	450c18d7	109	2026-10-30 18:45:04.058355+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Frank"}}
29936	450c18d7	110	2026-10-30 18:45:04.443815+00	{"TakeStrategyCard": {"card": "Politics", "player": "Alexandria Vandersloot"}}
29937	450c18d7	111	2026-10-30 18:45:04.82404+00	{"TakeStrategyCard": {"card": "Construction", "player": "Bob"}}
29938	450c18d7	112	2026-10-30 18:45:05.197542+00	"CompleteStrategyPhase"
29939	450c18d7	113	2026-10-30 18:45:15.246224+00	{"TacticalActionBegin": {"player": "Eve"}}
29940	450c18d7	114	2026-10-30 18:45:15.775918+00	{"TacticalActionCommit": {"player": "Eve"}}
29941	450c18d7	115	2026-10-30 18:45:16.305843+00	{"EndTurn": {"player": "Eve"}}
29942	450c18d7	116	2026-10-30 18:45:16.93744+00	{"TacticalActionBegin": {"player": "Frank"}}
29943	450c18d7	117	2026-10-30 18:45:17.465846+00	{"TacticalActionCommit": {"player": "Frank"}}
29944	450c18d7	118	2026-10-30 18:45:18.000791+00	{"EndTurn": {"player": "Frank"}}
29945	450c18d7	119	2026-10-30 18:45:18.632473+00	{"TacticalActionBegin": {"player": "Alexandria Vandersloot"}}
29946	450c18d7	120	2026-10-30 18:45:19.168193+00	{"TacticalActionCommit": {"player": "Alexandria Vandersloot"}}
29947	450c18d7	121	2026-10-30 18:45:19.706503+00	{"EndTurn": {"player": "Alexandria Vandersloot"}}
29948	450c18d7	122	2026-10-30 18:45:20.33659+00	{"TacticalActionBegin": {"player": "Bob"}}
29949	450c18d7	123	2026-10-30 18:45:20.871426+00	{"TacticalActionCommit": {"player": "Bob"}}
29950	450c18d7	124	2026-10-30 18:45:21.404988+00	{"EndTurn": {"player": "Bob"}}
29951	450c18d7	125	2026-10-30 18:45:22.036099+00	{"StrategicActionBegin": {"card": "Technology", "player": "Charlie"}}
29952	450c18d7	126	2026-10-30 18:45:31.502307+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": null, "second_tech": null}}, "player": "Charlie"}}
29953	450c18d7	127	2026-10-30 18:45:31.843732+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29954	450c18d7	128	2026-10-30 18:45:32.174539+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29955	450c18d7	129	2026-10-30 18:45:32.516007+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29956	450c18d7	130	2026-10-30 18:45:32.848794+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29957	450c18d7	131	2026-10-30 18:45:33.182825+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29958	450c18d7	132	2026-10-30 18:45:33.516976+00	"StrategicActionCommit"
29959	450c18d7	133	2026-10-30 18:45:34.152707+00	{"EndTurn": {"player": "Charlie"}}
29960	450c18d7	134	2026-10-30 18:45:34.791317+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Dee"}}
29961	450c18d7	135	2026-10-30 18:45:43.709913+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": null}}, "player": "Dee"}}
29962	450c18d7	136	2026-10-30 18:45:44.269325+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29963	450c18d7	137	2026-10-30 18:45:44.798342+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29964	450c18d7	138	2026-10-30 18:45:45.334374+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29965	450c18d7	139	2026-10-30 18:45:45.868429+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29966	450c18d7	140	2026-10-30 18:45:46.411354+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29967	450c18d7	141	2026-10-30 18:45:46.959528+00	"StrategicActionCommit"
29968	450c18d7	142	2026-10-30 18:45:47.496546+00	{"EndTurn": {"player": "Dee"}}
29969	450c18d7	143	2026-10-30 18:45:48.028711+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Eve"}}
29970	450c18d7	144	2026-10-30 18:45:48.575631+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29971	450c18d7	145	2026-10-30 18:45:49.114936+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29972	450c18d7	146	2026-10-30 18:45:49.647214+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29973	450c18d7	147	2026-10-30 18:45:50.179918+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29974	450c18d7	148	2026-10-30 18:45:50.713522+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29975	450c18d7	149	2026-10-30 18:45:51.281752+00	"StrategicActionCommit"
29976	450c18d7	150	2026-10-30 18:45:51.831593+00	{"EndTurn": {"player": "Eve"}}
29977	450c18d7	151	2026-10-30 18:45:52.375815+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Frank"}}
29978	450c18d7	152	2026-10-30 18:45:52.925933+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29979	450c18d7	153	2026-10-30 18:45:53.464767+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29980	450c18d7	154	2026-10-30 18:45:54.004401+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29981	450c18d7	155	2026-10-30 18:45:54.548172+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29982	450c18d7	156	2026-10-30 18:45:55.079642+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29983	450c18d7	157	2026-10-30 18:45:55.629967+00	"StrategicActionCommit"
29984	450c18d7	158	2026-10-30 18:45:56.180698+00	{"EndTurn": {"player": "Frank"}}
29985	450c18d7	159	2026-10-30 18:45:56.730163+00	{"StrategicActionBegin": {"card": "Politics", "player": "Alexandria Vandersloot"}}
29986	450c18d7	160	2026-10-30 18:45:57.293052+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alexandria Vandersloot"}}, "player": "Alexandria Vandersloot"}}
29987	450c18d7	161	2026-10-30 18:45:57.828941+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29988	450c18d7	162	2026-10-30 18:45:58.364539+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29989	450c18d7	163	2026-10-30 18:45:58.894946+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29990	450c18d7	164	2026-10-30 18:45:59.43716+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29991	450c18d7	165	2026-10-30 18:45:59.968497+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
29992	450c18d7	166	2026-10-30 18:46:00.51974+00	"StrategicActionCommit"
29993	450c18d7	167	2026-10-30 18:46:01.067802+00	{"EndTurn": {"player": "Alexandria Vandersloot"}}
29994	450c18d7	168	2026-10-30 18:46:01.605375+00	{"StrategicActionBegin": {"card": "Construction", "player": "Bob"}}
29995	450c18d7	169	2026-10-30 18:46:02.136466+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alexandria Vandersloot"}}
29996	450c18d7	170	2026-10-30 18:46:02.67503+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Charlie"}}
29997	450c18d7	171	2026-10-30 18:46:03.207655+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
29998	450c18d7	172	2026-10-30 18:46:03.74089+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Eve"}}
29999	450c18d7	173	2026-10-30 18:46:04.291703+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Frank"}}
30000	450c18d7	174	2026-10-30 18:46:04.841764+00	"StrategicActionCommit"
30001	450c18d7	175	2026-10-30 18:46:05.391846+00	{"EndTurn": {"player": "Bob"}}
30002	450c18d7	176	2026-10-30 18:46:05.941969+00	{"PassAction": {"player": "Charlie"}}
30003	450c18d7	177	2026-10-30 18:46:06.492037+00	{"EndTurn": {"player": "Charlie"}}
30004	450c18d7	178	2026-10-30 18:46:07.04535+00	{"PassAction": {"player": "Dee"}}
30005	450c18d7	179	2026-10-30 18:46:07.598744+00	{"EndTurn": {"player": "Dee"}}
30006	450c18d7	180	2026-10-30 18:46:08.146026+00	{"PassAction": {"player": "Eve"}}
30007	450c18d7	181	2026-10-30 18:46:08.699679+00	{"EndTurn": {"player": "Eve"}}
30008	450c18d7	182	2026-10-30 18:46:09.249318+00	{"PassAction": {"player": "Frank"}}
30009	450c18d7	183	2026-10-30 18:46:09.800486+00	{"EndTurn": {"player": "Frank"}}
30010	450c18d7	184	2026-10-30 18:46:10.350074+00	{"PassAction": {"player": "Alexandria Vandersloot"}}
30011	450c18d7	185	2026-10-30 18:46:10.900213+00	{"EndTurn": {"player": "Alexandria Vandersloot"}}
30012	450c18d7	186	2026-10-30 18:46:11.450282+00	{"PassAction": {"player": "Bob"}}
30013	450c18d7	187	2026-10-30 18:46:12.0043+00	{"EndTurn": {"player": "Bob"}}
30014	450c18d7	188	2026-10-30 18:46:12.556001+00	{"ScorePublicObjective": {"player": "Eve", "objective": null}}
30015	450c18d7	189	2026-10-30 18:46:13.091771+00	{"ScoreSecretObjective": {"player": "Eve", "objective": null}}
30016	450c18d7	190	2026-10-30 18:46:13.626515+00	{"ScorePublicObjective": {"player": "Frank", "objective": null}}
30017	450c18d7	191	2026-10-30 18:46:14.176762+00	{"ScoreSecretObjective": {"player": "Frank", "objective": null}}
30018	450c18d7	192	2026-10-30 18:46:14.706813+00	{"ScorePublicObjective": {"player": "Alexandria Vandersloot", "objective": null}}
30019	450c18d7	193	2026-10-30 18:46:15.246037+00	{"ScoreSecretObjective": {"player": "Alexandria Vandersloot", "objective": null}}
30020	450c18d7	194	2026-10-30 18:46:15.779894+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30021	450c18d7	195	2026-10-30 18:46:16.319748+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30022	450c18d7	196	2026-10-30 18:46:16.851952+00	{"ScorePublicObjective": {"player": "Charlie", "objective": null}}
30023	450c18d7	197	2026-10-30 18:46:17.381345+00	{"ScoreSecretObjective": {"player": "Charlie", "objective": null}}
30024	450c18d7	198	2026-10-30 18:46:17.918474+00	{"ScorePublicObjective": {"player": "Dee", "objective": null}}
30025	450c18d7	199	2026-10-30 18:46:18.451987+00	{"ScoreSecretObjective": {"player": "Dee", "objective": null}}
30026	450c18d7	200	2026-10-30 18:46:19.015739+00	{"RevealPublicObjective": {"objective": "DevelopWeaponry"}}
30027	450c18d7	201	2026-10-30 18:46:19.551763+00	"CompleteStatusPhase"
30028	450c18d7	202	2026-10-30 18:46:26.414057+00	{"RevealAgenda": {"agenda": "MinisterOfAntiques"}}
30029	450c18d7	203	2026-10-30 18:46:39.714252+00	{"CastAgendaVote": {"votes": 0, "player": "Alexandria Vandersloot", "outcome": null}}
30030	450c18d7	204	2026-10-30 18:46:40.046068+00	{"CastAgendaVote": {"votes": 0, "player": "Bob", "outcome": null}}
30031	450c18d7	205	2026-10-30 18:46:40.378969+00	{"CastAgendaVote": {"votes": 0, "player": "Charlie", "outcome": null}}
30032	450c18d7	206	2026-10-30 18:46:40.705712+00	{"CastAgendaVote": {"votes": 0, "player": "Dee", "outcome": null}}
30033	450c18d7	207	2026-10-30 18:46:41.043125+00	{"CastAgendaVote": {"votes": 0, "player": "Eve", "outcome": null}}
30034	450c18d7	208	2026-10-30 18:46:41.374853+00	{"CastAgendaVote": {"votes": 0, "player": "Frank", "outcome": null}}
30035	450c18d7	209	2026-10-30 18:46:42.022683+00	{"ResolveAgenda": {"outcome": {"value": "Alexandria Vandersloot", "electKind": "Player"}}}
30036	450c18d7	210	2026-10-30 18:47:03.249855+00	{"GainMinisterOfAntiquesRelic": {"relic": null, "player": "Alexandria Vandersloot"}}
30037	450c18d7	211	2026-10-30 18:47:05.050941+00	{"AddAgendaBegin": {"agenda": "AntiIntellectualRevolution"}}
30038	450c18d7	212	2026-10-30 18:47:45.652704+00	{"AddAgendaResolve": {"electedOutcome": {"value": "For", "electKind": "ForOrAgainst"}}}
31185	942cbdd8	63	2026-11-05 23:11:02.509969+00	{"RevealPublicObjective": {"objective": "DiversifyResearch"}}
31186	942cbdd8	64	2026-11-05 23:11:02.516951+00	"CompleteStatusPhase"
31187	ccc97b0b	1	2026-11-05 23:11:02.811623+00	{"SetGalacticEvents": {"events": ["ConventionsOfWarAbandoned"]}}
31188	ccc97b0b	2	2026-11-05 23:11:02.820431+00	"CreationDone"
31189	ccc97b0b	3	2026-11-05 23:11:02.827037+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31190	ccc97b0b	4	2026-11-05 23:11:02.833897+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31191	ccc97b0b	5	2026-11-05 23:11:02.840754+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31192	ccc97b0b	6	2026-11-05 23:11:02.848776+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31193	ccc97b0b	7	2026-11-05 23:11:02.856876+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31194	ccc97b0b	8	2026-11-05 23:11:02.863862+00	"StartGame"
31195	ccc97b0b	9	2026-11-05 23:11:02.871129+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31196	ccc97b0b	10	2026-11-05 23:11:02.877931+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31197	ccc97b0b	11	2026-11-05 23:11:02.885136+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31198	ccc97b0b	12	2026-11-05 23:11:02.891625+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31199	ccc97b0b	13	2026-11-05 23:11:02.898643+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31200	ccc97b0b	14	2026-11-05 23:11:02.905252+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31201	ccc97b0b	15	2026-11-05 23:11:02.911578+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31202	ccc97b0b	16	2026-11-05 23:11:02.920384+00	"CompleteStrategyPhase"
31203	ccc97b0b	17	2026-11-05 23:11:02.927579+00	{"AddTechToPlayer": {"tech": "X89BacterialWeaponOmegaOmega", "player": "Tux"}}
31204	dc46c53a	1	2026-11-05 23:11:03.217337+00	{"SetGalacticEvents": {"events": ["AgeOfExploration"]}}
31205	dc46c53a	2	2026-11-05 23:11:03.233113+00	"CreationDone"
31206	dc46c53a	3	2026-11-05 23:11:03.240633+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31207	dc46c53a	4	2026-11-05 23:11:03.250391+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31208	dc46c53a	5	2026-11-05 23:11:03.263569+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31209	dc46c53a	6	2026-11-05 23:11:03.272307+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31210	dc46c53a	7	2026-11-05 23:11:03.281239+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31211	dc46c53a	8	2026-11-05 23:11:03.288498+00	"StartGame"
31212	dc46c53a	9	2026-11-05 23:11:03.296528+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
30039	450c18d7	213	2026-10-30 19:11:03.272996+00	{"AddAgendaBegin": {"agenda": "ArchivedSecret"}}
30040	450c18d7	214	2026-10-30 19:11:04.576045+00	"AddAgendaCancel"
30041	450c18d7	215	2026-10-30 19:20:25.270863+00	{"RevealAgenda": {"agenda": "ArchivedSecret"}}
30042	450c18d7	216	2026-10-30 19:20:25.826199+00	{"CastAgendaVote": {"votes": 0, "player": "Alexandria Vandersloot", "outcome": null}}
30043	450c18d7	217	2026-10-30 19:20:26.37468+00	{"CastAgendaVote": {"votes": 0, "player": "Bob", "outcome": null}}
30044	450c18d7	218	2026-10-30 19:20:26.904748+00	{"CastAgendaVote": {"votes": 0, "player": "Charlie", "outcome": null}}
30045	450c18d7	219	2026-10-30 19:20:27.439432+00	{"CastAgendaVote": {"votes": 0, "player": "Dee", "outcome": null}}
30046	450c18d7	220	2026-10-30 19:20:27.970693+00	{"CastAgendaVote": {"votes": 0, "player": "Eve", "outcome": null}}
30047	450c18d7	221	2026-10-30 19:20:28.506399+00	{"CastAgendaVote": {"votes": 0, "player": "Frank", "outcome": null}}
30048	450c18d7	222	2026-10-30 19:20:29.057676+00	{"ResolveAgenda": {"outcome": null}}
30049	450c18d7	223	2026-10-30 19:20:36.970244+00	"CompleteAgendaPhase"
30050	450c18d7	224	2026-11-01 13:14:14.849604+00	{"SetRelicOwner": {"relic": "MetaliVoidArmaments", "player": "Charlie"}}
30051	450c18d7	225	2026-11-01 13:14:16.627067+00	{"SetRelicOwner": {"relic": "MetaliVoidArmaments", "player": null}}
30052	450c18d7	226	2026-11-01 13:14:17.855428+00	{"SetRelicOwner": {"relic": "BookOfLatvinia", "player": null}}
30053	450c18d7	227	2026-11-01 13:14:37.9327+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Alexandria Vandersloot"}}
30054	450c18d7	228	2026-11-01 13:14:38.841697+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30055	450c18d7	229	2026-11-01 13:14:39.457964+00	{"TakeStrategyCard": {"card": "Construction", "player": "Charlie"}}
30056	450c18d7	230	2026-11-01 13:14:40.783595+00	{"TakeStrategyCard": {"card": "Politics", "player": "Dee"}}
30057	450c18d7	231	2026-11-01 13:14:41.197503+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Eve"}}
30058	450c18d7	232	2026-11-01 13:14:42.130095+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Frank"}}
30059	450c18d7	233	2026-11-01 13:14:43.242655+00	"CompleteStrategyPhase"
30060	450c18d7	234	2026-11-01 13:14:44.585909+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Eve"}}
30061	450c18d7	235	2026-11-01 13:14:45.663158+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Alexandria Vandersloot"}}
30062	450c18d7	236	2026-11-01 13:14:46.333032+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
30063	450c18d7	237	2026-11-01 13:14:47.13711+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Charlie"}}
30064	450c18d7	238	2026-11-01 13:14:47.750599+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dee"}}
30065	450c18d7	239	2026-11-01 13:14:48.827374+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Frank"}}
30066	450c18d7	240	2026-11-01 13:14:49.386603+00	"StrategicActionCommit"
30067	450c18d7	241	2026-11-01 20:44:08.197672+00	{"SetRelicOwner": {"relic": "TheCodex", "player": "Charlie"}}
30068	db9f6e36	0	2026-11-02 18:08:31.453587+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30069	db9f6e36	1	2026-11-02 18:08:37.090296+00	{"SetGalacticEvents": {"events": ["TotalWar"]}}
30070	db9f6e36	2	2026-11-02 18:08:37.166463+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics"]}}
30071	db9f6e36	3	2026-11-02 18:08:41.373543+00	{"SetGalacticEvents": {"events": ["StellarAtomics"]}}
30072	db9f6e36	4	2026-11-02 18:08:42.421442+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics"]}}
30073	db9f6e36	5	2026-11-02 18:08:56.88931+00	{"AddPlayer": {"player": {"name": "Ann", "color": "Black", "faction": "LastBastion"}}}
30074	db9f6e36	6	2026-11-02 18:08:57.092269+00	{"AddPlayer": {"player": {"name": "Ben", "color": "Black", "faction": "SardakkNorr"}}}
30075	db9f6e36	7	2026-11-02 18:08:57.408609+00	{"AddPlayer": {"player": {"name": "Cid", "color": "Blue", "faction": "Arborec"}}}
30076	db9f6e36	8	2026-11-02 18:08:57.500669+00	"CreationDone"
30077	23e3bcec	0	2026-11-02 18:09:04.040749+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
30081	53e02a07	0	2026-11-02 18:55:19.870768+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30093	53e02a07	1	2026-11-02 21:23:14.311357+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
30121	31c1512f	0	2026-11-03 10:11:46.713665+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30122	31c1512f	1	2026-11-03 10:11:55.697657+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30123	31c1512f	2	2026-11-03 10:11:56.313061+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30124	31c1512f	3	2026-11-03 10:11:56.911445+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30125	31c1512f	4	2026-11-03 10:11:57.482308+00	{"SetGalacticEvents": {"events": ["ZealousOrthodoxy"]}}
30126	31c1512f	5	2026-11-03 10:12:47.91252+00	{"ScoreExtraSecretObjective": {"player": "Bob", "objective": "AdaptNewStrategies"}}
30127	31c1512f	6	2026-11-03 10:12:48.763843+00	{"ScoreExtraSecretObjective": {"player": "Bob", "objective": "BecomeAMartyr"}}
30128	257ca62a	0	2026-11-03 12:01:50.823398+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": false, "prophecyOfKings": false}}}}
30129	257ca62a	1	2026-11-03 12:02:02.259897+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30130	257ca62a	2	2026-11-03 12:02:03.488481+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "EmiratesOfHacan"}}}
30131	257ca62a	3	2026-11-03 12:02:04.704916+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30078	ad779ebc	0	2026-11-02 18:10:08.595555+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "74", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "113", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "100", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "39", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "102", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "36", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "78", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "59", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "34", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "32", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "68", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "47", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "49", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "50", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 0}}}, {"system": "64", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "24", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "35", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "80", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 0}}}, {"system": "107", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Gurr": {"name": "Gurr", "order": 0, "faction": "FirmamentObsidian"}, "Yoda": {"name": "Yoda", "order": 3, "faction": "RalNelConsortium"}, "Vidde": {"name": "Vidde", "order": 2, "faction": "CrimsonRebellion"}, "Swexbe": {"name": "Swexbe", "order": 1, "faction": "DeepwroughtScholarate"}, "Portals": {"name": "Portals", "order": 4, "faction": "CouncilKeleres"}}, "game_name": "Operation Itchy Friendship", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30079	ad779ebc	1	2026-11-02 18:10:08.856729+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
30080	ad779ebc	2	2026-11-02 18:10:13.04329+00	"CreationDone"
30082	3fc75ce7	0	2026-11-02 19:07:06.991914+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30083	3fc75ce7	1	2026-11-02 19:07:15.111656+00	{"SetGalacticEvents": {"events": ["TotalWar"]}}
30084	3fc75ce7	2	2026-11-02 19:07:16.288314+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics"]}}
30085	3fc75ce7	3	2026-11-02 19:07:28.219654+00	{"SetGalacticEvents": {"events": ["StellarAtomics"]}}
30086	3fc75ce7	4	2026-11-02 19:07:36.365699+00	{"AddPlayer": {"player": {"name": "P1", "color": "Black", "faction": "LastBastion"}}}
30087	3fc75ce7	5	2026-11-02 19:07:37.491491+00	{"AddPlayer": {"player": {"name": "P2", "color": "Black", "faction": "SardakkNorr"}}}
30088	3fc75ce7	6	2026-11-02 19:07:38.617245+00	{"AddPlayer": {"player": {"name": "P3", "color": "Blue", "faction": "Arborec"}}}
30089	3fc75ce7	7	2026-11-02 19:07:39.35554+00	"CreationDone"
30090	d9741780	0	2026-11-02 19:07:47.173591+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
30091	ea4edebf	0	2026-11-02 19:18:53.817418+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30092	ea4edebf	1	2026-11-02 19:18:54.453741+00	{"SetGalacticEvents": {"events": ["TotalWar"]}}
30094	54757682	0	2026-11-02 21:49:31.457864+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30095	54757682	1	2026-11-02 21:50:19.863326+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce"]}}
30096	54757682	2	2026-11-02 21:50:19.92475+00	{"SetGalacticEvents": {"events": ["MinorFactions", "AgeOfCommerce"]}}
30097	54757682	3	2026-11-02 21:50:20.010464+00	{"SetGalacticEvents": {"events": ["MinorFactions", "TotalWar", "AgeOfCommerce"]}}
30098	54757682	4	2026-11-02 21:50:20.078469+00	{"SetGalacticEvents": {"events": ["MinorFactions", "TotalWar", "AgeOfCommerce", "CosmicPhenomenae"]}}
30099	53e02a07	2	2026-11-02 21:56:28.561308+00	{"SetGalacticEvents": {"events": ["MinorFactions", "ConventionsOfWarAbandoned"]}}
30132	257ca62a	4	2026-11-03 12:02:05.55755+00	{"SetGalacticEvents": {"events": ["TotalWar"]}}
30133	257ca62a	5	2026-11-03 12:02:09.404937+00	"CreationDone"
30134	257ca62a	6	2026-11-03 12:02:15.955372+00	{"SetupSpeaker": {"player": "Alice"}}
30135	257ca62a	7	2026-11-03 12:02:17.104217+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30136	257ca62a	8	2026-11-03 12:02:17.92088+00	"StartGame"
30137	257ca62a	9	2026-11-03 12:02:23.758036+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30138	257ca62a	10	2026-11-03 12:02:24.269833+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30139	257ca62a	11	2026-11-03 12:02:24.789+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30140	257ca62a	12	2026-11-03 12:02:25.303348+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30141	257ca62a	13	2026-11-03 12:02:25.821206+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30142	257ca62a	14	2026-11-03 12:02:26.33763+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30143	257ca62a	15	2026-11-03 12:02:26.853937+00	"CompleteStrategyPhase"
30144	257ca62a	16	2026-11-03 12:02:46.887796+00	{"PlayGalacticEventAction": {"action": "TotalWar", "player": "Alice"}}
30145	257ca62a	17	2026-11-03 12:02:53.136444+00	{"EndTurn": {"player": "Alice"}}
30146	ba1a832b	0	2026-11-03 12:03:13.294557+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": false, "prophecyOfKings": false}}}}
30147	ba1a832b	1	2026-11-03 12:03:15.670758+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30148	ba1a832b	2	2026-11-03 12:03:16.884611+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "EmiratesOfHacan"}}}
30149	ba1a832b	3	2026-11-03 12:03:18.103228+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30150	ba1a832b	4	2026-11-03 12:03:18.918449+00	"CreationDone"
30151	ba1a832b	5	2026-11-03 12:03:20.434417+00	{"SetupSpeaker": {"player": "Alice"}}
30152	ba1a832b	6	2026-11-03 12:03:21.588037+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30153	ba1a832b	7	2026-11-03 12:03:22.400807+00	"StartGame"
30154	ba1a832b	8	2026-11-03 12:03:23.920287+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30155	ba1a832b	9	2026-11-03 12:03:24.434426+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30156	ba1a832b	10	2026-11-03 12:03:24.950708+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30100	60fff71a	0	2026-11-02 22:12:16.371099+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30101	60fff71a	1	2026-11-02 22:12:16.563278+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce"]}}
30102	60fff71a	2	2026-11-02 22:12:16.637633+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30103	60fff71a	3	2026-11-02 22:12:16.724425+00	{"SetGalacticEvents": {"events": ["MinorFactions", "AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30104	60fff71a	4	2026-11-02 22:12:16.806337+00	{"SetGalacticEvents": {"events": ["MinorFactions", "TotalWar", "AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30105	60fff71a	5	2026-11-02 22:12:44.918989+00	{"SetGalacticEvents": {"events": ["MinorFactions", "AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30157	ba1a832b	11	2026-11-03 12:03:25.468251+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30158	ba1a832b	12	2026-11-03 12:03:25.98545+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30159	ba1a832b	13	2026-11-03 12:03:26.502415+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30160	ba1a832b	14	2026-11-03 12:03:27.021133+00	"CompleteStrategyPhase"
31213	dc46c53a	10	2026-11-05 23:11:03.302927+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31214	dc46c53a	11	2026-11-05 23:11:03.310425+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31215	dc46c53a	12	2026-11-05 23:11:03.317032+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31216	dc46c53a	13	2026-11-05 23:11:03.323844+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31217	dc46c53a	14	2026-11-05 23:11:03.33145+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31218	dc46c53a	15	2026-11-05 23:11:03.337334+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31219	dc46c53a	16	2026-11-05 23:11:03.345004+00	"CompleteStrategyPhase"
31220	dc46c53a	17	2026-11-05 23:11:03.351001+00	{"AddTechToPlayer": {"tech": "DarkEnergyTap", "player": "Tux"}}
31221	03d042d0	1	2026-11-05 23:11:03.661219+00	{"SetGalacticEvents": {"events": ["WildWildGalaxy"]}}
31222	03d042d0	2	2026-11-05 23:11:03.678767+00	"CreationDone"
31223	03d042d0	3	2026-11-05 23:11:03.68775+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31224	03d042d0	4	2026-11-05 23:11:03.696499+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31225	03d042d0	5	2026-11-05 23:11:03.704923+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31226	03d042d0	6	2026-11-05 23:11:03.713558+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31227	03d042d0	7	2026-11-05 23:11:03.721982+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31228	03d042d0	8	2026-11-05 23:11:03.729917+00	"StartGame"
31229	03d042d0	9	2026-11-05 23:11:03.737889+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31230	03d042d0	10	2026-11-05 23:11:03.745828+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31231	03d042d0	11	2026-11-05 23:11:03.752963+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31232	03d042d0	12	2026-11-05 23:11:03.759753+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31233	03d042d0	13	2026-11-05 23:11:03.766687+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31234	03d042d0	14	2026-11-05 23:11:03.773314+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31235	03d042d0	15	2026-11-05 23:11:03.780335+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31236	03d042d0	16	2026-11-05 23:11:03.78709+00	"CompleteStrategyPhase"
31237	03d042d0	17	2026-11-05 23:11:03.793633+00	{"SetRelicOwner": {"relic": "StellarConverter", "player": "Tux"}}
31238	d4406362	1	2026-11-05 23:11:04.080404+00	{"SetGalacticEvents": {"events": ["DangerousWilds"]}}
31239	d4406362	2	2026-11-05 23:11:04.096154+00	"CreationDone"
31240	d4406362	3	2026-11-05 23:11:04.10386+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31241	d4406362	4	2026-11-05 23:11:04.112003+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31242	d4406362	5	2026-11-05 23:11:04.119898+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31243	d4406362	6	2026-11-05 23:11:04.127876+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31244	d4406362	7	2026-11-05 23:11:04.135658+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31245	d4406362	8	2026-11-05 23:11:04.143148+00	"StartGame"
31246	d4406362	9	2026-11-05 23:11:04.149986+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31247	d4406362	10	2026-11-05 23:11:04.157671+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31248	d4406362	11	2026-11-05 23:11:04.164759+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31249	d4406362	12	2026-11-05 23:11:04.17177+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31250	d4406362	13	2026-11-05 23:11:04.179144+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31251	d4406362	14	2026-11-05 23:11:04.186158+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31252	d4406362	15	2026-11-05 23:11:04.192713+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31253	d4406362	16	2026-11-05 23:11:04.200162+00	"CompleteStrategyPhase"
31254	63b0393b	1	2026-11-05 23:11:04.490791+00	{"SetGalacticEvents": {"events": ["MonumentsToTheAges"]}}
31255	63b0393b	2	2026-11-05 23:11:04.508882+00	"CreationDone"
31256	63b0393b	3	2026-11-05 23:11:04.516798+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31257	63b0393b	4	2026-11-05 23:11:04.524508+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31258	63b0393b	5	2026-11-05 23:11:04.53291+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
30106	53e02a07	3	2026-11-02 22:33:18.416116+00	{"AddPlayer": {"player": {"name": "Pepe", "color": "Red", "faction": "EmbersOfMuaat"}}}
30107	53e02a07	4	2026-11-02 22:33:23.571833+00	{"AddPlayer": {"player": {"name": "Dafuq", "color": "Orange", "faction": "CouncilKeleres"}}}
30161	450c18d7	242	2026-11-03 13:48:04.415389+00	{"EndTurn": {"player": "Eve"}}
30162	53e02a07	5	2026-11-03 13:49:55.985574+00	{"AddPlayer": {"player": {"name": "Davud", "color": "Blue", "faction": "FederationOfSol"}}}
30163	53e02a07	6	2026-11-03 13:49:59.709863+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
30164	53e02a07	7	2026-11-03 13:50:00.645657+00	{"SetGalacticEvents": {"events": []}}
30165	53e02a07	8	2026-11-03 13:50:12.047237+00	{"SetGalacticEvents": {"events": ["MercenariesForHire"]}}
30166	53e02a07	9	2026-11-03 13:50:18.323407+00	{"SetGalacticEvents": {"events": ["StellarAtomics", "MercenariesForHire"]}}
30167	53e02a07	10	2026-11-03 13:50:32.037948+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics", "MercenariesForHire"]}}
30168	53e02a07	11	2026-11-03 13:50:33.043587+00	"CreationDone"
30169	53e02a07	12	2026-11-03 13:51:34.004081+00	{"SetupPlayerTechs": {"player": "Dafuq", "technologies": ["AntimassDeflectors", "PlasmaScoring"]}}
30170	53e02a07	13	2026-11-03 13:51:35.912945+00	{"SetupTheTribunii": {"player": "Dafuq", "faction": "MentakCoalition"}}
30171	53e02a07	14	2026-11-03 13:51:41.25941+00	{"RevealInitialObjectives": {"firstObjective": "DevelopWeaponry", "secondObjective": "ExploreDeepSpace"}}
30172	53e02a07	15	2026-11-03 13:51:50.466763+00	{"SetupSpeaker": {"player": "Dafuq"}}
30173	53e02a07	16	2026-11-03 13:51:52.776029+00	"StartGame"
30174	53e02a07	17	2026-11-03 13:52:20.306883+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Dafuq"}}
30175	53e02a07	18	2026-11-03 13:52:20.809147+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Davud"}}
30176	53e02a07	19	2026-11-03 13:52:21.210895+00	{"TakeStrategyCard": {"card": "Politics", "player": "Pepe"}}
30177	53e02a07	20	2026-11-03 13:52:22.052268+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Dafuq"}}
30178	53e02a07	21	2026-11-03 13:52:23.507331+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Davud"}}
30179	53e02a07	22	2026-11-03 13:52:23.835413+00	{"TakeStrategyCard": {"card": "Technology", "player": "Pepe"}}
30180	53e02a07	23	2026-11-03 13:52:24.316917+00	"CompleteStrategyPhase"
30181	53e02a07	24	2026-11-03 13:52:31.258539+00	{"PlayGalacticEventAction": {"action": "StellarAtomics", "player": "Dafuq"}}
30182	53e02a07	25	2026-11-03 13:52:33.176284+00	{"EndTurn": {"player": "Dafuq"}}
30183	53e02a07	26	2026-11-03 13:52:40.15619+00	{"PlayGalacticEventAction": {"action": "TotalWar", "player": "Davud"}}
30184	53e02a07	27	2026-11-03 13:52:41.611644+00	{"EndTurn": {"player": "Davud"}}
30185	53e02a07	28	2026-11-03 13:52:46.036338+00	{"PlayGalacticEventAction": {"action": "MercenariesForHire", "player": "Pepe"}}
30186	53e02a07	29	2026-11-03 13:52:47.686729+00	{"EndTurn": {"player": "Pepe"}}
30187	aeffde45	0	2026-11-03 15:11:32.79153+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30188	aeffde45	1	2026-11-03 15:11:41.278364+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30189	aeffde45	2	2026-11-03 15:11:42.585685+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30190	aeffde45	3	2026-11-03 15:11:43.805702+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30191	aeffde45	4	2026-11-03 15:11:45.0015+00	{"SetGalacticEvents": {"events": ["WildWildGalaxy"]}}
30192	aeffde45	5	2026-11-03 15:11:49.404315+00	"CreationDone"
30193	aeffde45	6	2026-11-03 15:11:55.32809+00	{"SetupSpeaker": {"player": "Alice"}}
30194	aeffde45	7	2026-11-03 15:11:56.817876+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
30195	aeffde45	8	2026-11-03 15:11:57.650468+00	"StartGame"
30196	aeffde45	9	2026-11-03 15:12:04.667378+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30197	aeffde45	10	2026-11-03 15:12:05.203493+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30198	aeffde45	11	2026-11-03 15:12:05.73649+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30199	aeffde45	12	2026-11-03 15:12:06.269647+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30200	aeffde45	13	2026-11-03 15:12:06.803175+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30201	aeffde45	14	2026-11-03 15:12:07.339083+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30202	aeffde45	15	2026-11-03 15:12:07.887131+00	"CompleteStrategyPhase"
30203	aeffde45	16	2026-11-03 15:12:15.210624+00	{"SetRelicOwner": {"relic": "StellarConverter", "player": "Alice"}}
30204	aeffde45	17	2026-11-03 15:12:28.721663+00	{"RelicActionBegin": {"relic": "StellarConverter", "player": "Alice"}}
30205	aeffde45	18	2026-11-03 15:12:38.293925+00	{"RelicActionCommit": {"data": {"StellarConverter": {"planet": "MecatolRexOmega"}}, "player": "Alice"}}
30206	e0a811de	0	2026-11-04 08:53:57.118026+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30207	e0a811de	1	2026-11-04 08:54:15.857735+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30208	e0a811de	2	2026-11-04 08:54:17.036724+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30209	e0a811de	3	2026-11-04 08:54:18.219501+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30210	e0a811de	4	2026-11-04 08:54:19.352841+00	{"SetGalacticEvents": {"events": ["ConventionsOfWarAbandoned"]}}
30211	e0a811de	5	2026-11-04 08:54:24.036153+00	"CreationDone"
30212	e0a811de	6	2026-11-04 08:54:30.599049+00	{"SetupSpeaker": {"player": "Alice"}}
30213	e0a811de	7	2026-11-04 08:54:32.031555+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
30214	e0a811de	8	2026-11-04 08:54:32.84887+00	"StartGame"
28247	27d0c2d4	20	2026-10-30 12:54:23.047684+00	"StrategicActionCommit"
28248	27d0c2d4	21	2026-10-30 12:55:23.047684+00	{"EndTurn": {"player": "Bob"}}
28249	27d0c2d4	22	2026-10-30 12:56:23.047684+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28250	27d0c2d4	23	2026-10-30 12:57:23.047684+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28251	27d0c2d4	24	2026-10-30 12:58:23.047684+00	"StrategicActionCommit"
28252	27d0c2d4	25	2026-10-30 12:59:23.047684+00	{"EndTurn": {"player": "Carol"}}
28253	27d0c2d4	26	2026-10-30 13:00:23.047684+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28254	27d0c2d4	27	2026-10-30 13:01:23.047684+00	"StrategicActionCommit"
28255	27d0c2d4	28	2026-10-30 13:02:23.047684+00	{"EndTurn": {"player": "Alice"}}
28256	27d0c2d4	29	2026-10-30 13:03:23.047684+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28257	27d0c2d4	30	2026-10-30 13:04:23.047684+00	"StrategicActionCommit"
28258	27d0c2d4	31	2026-10-30 13:05:23.047684+00	{"EndTurn": {"player": "Bob"}}
28259	27d0c2d4	32	2026-10-30 13:06:23.047684+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28260	27d0c2d4	33	2026-10-30 13:07:23.047684+00	"StrategicActionCommit"
28261	27d0c2d4	34	2026-10-30 13:08:23.047684+00	{"EndTurn": {"player": "Carol"}}
28262	27d0c2d4	35	2026-10-30 13:09:23.047684+00	{"PassAction": {"player": "Alice"}}
28263	27d0c2d4	36	2026-10-30 13:10:23.047684+00	{"PassAction": {"player": "Bob"}}
28264	27d0c2d4	37	2026-10-30 13:11:23.047684+00	{"PassAction": {"player": "Carol"}}
28265	27d0c2d4	38	2026-10-30 13:12:23.047684+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28266	27d0c2d4	39	2026-10-30 13:13:23.047684+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28267	27d0c2d4	40	2026-10-30 13:14:23.047684+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28268	27d0c2d4	41	2026-10-30 13:15:23.047684+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28269	27d0c2d4	42	2026-10-30 13:16:23.047684+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28270	27d0c2d4	43	2026-10-30 13:17:23.047684+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28271	27d0c2d4	44	2026-10-30 13:18:23.047684+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28272	27d0c2d4	45	2026-10-30 13:19:23.047684+00	"CompleteStatusPhase"
28273	27d0c2d4	46	2026-10-30 13:20:23.047684+00	{"AddExtraPoints": {"value": 2, "player": "Bob"}}
28274	27d0c2d4	47	2026-10-30 13:21:23.047684+00	{"RevealAgenda": {"agenda": "Mutiny"}}
28275	27d0c2d4	48	2026-10-30 13:22:23.047684+00	{"CastAgendaVote": {"votes": 3, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28276	27d0c2d4	49	2026-10-30 13:23:23.047684+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28277	27d0c2d4	50	2026-10-30 13:24:23.047684+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28278	27d0c2d4	51	2026-10-30 13:25:23.047684+00	{"RevealAgenda": {"agenda": "SeedOfAnEmpire"}}
28279	27d0c2d4	52	2026-10-30 13:26:23.047684+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28280	27d0c2d4	53	2026-10-30 13:27:23.047684+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28281	27d0c2d4	54	2026-10-30 13:28:23.047684+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28282	9a5c9ddc	0	2026-10-30 12:34:23.292805+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28283	9a5c9ddc	1	2026-10-30 12:35:23.292805+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28284	9a5c9ddc	2	2026-10-30 12:36:23.292805+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28285	9a5c9ddc	3	2026-10-30 12:37:23.292805+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "NekroVirus"}}}
28286	9a5c9ddc	4	2026-10-30 12:38:23.292805+00	"CreationDone"
28287	9a5c9ddc	5	2026-10-30 12:39:23.292805+00	{"SetupSpeaker": {"player": "Alice"}}
28288	9a5c9ddc	6	2026-10-30 12:40:23.292805+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28289	9a5c9ddc	7	2026-10-30 12:41:23.292805+00	"StartGame"
28290	9a5c9ddc	8	2026-10-30 12:42:23.292805+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28291	9a5c9ddc	9	2026-10-30 12:43:23.292805+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28292	9a5c9ddc	10	2026-10-30 12:44:23.292805+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28293	9a5c9ddc	11	2026-10-30 12:45:23.292805+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28294	9a5c9ddc	12	2026-10-30 12:46:23.292805+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28295	9a5c9ddc	13	2026-10-30 12:47:23.292805+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28296	9a5c9ddc	14	2026-10-30 12:48:23.292805+00	"CompleteStrategyPhase"
28297	9a5c9ddc	15	2026-10-30 12:49:23.292805+00	{"SetCustodians": {"player": "Alice"}}
28298	9a5c9ddc	16	2026-10-30 12:50:23.292805+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28299	9a5c9ddc	17	2026-10-30 12:51:23.292805+00	"StrategicActionCommit"
28300	9a5c9ddc	18	2026-10-30 12:52:23.292805+00	{"EndTurn": {"player": "Alice"}}
28301	9a5c9ddc	19	2026-10-30 12:53:23.292805+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28302	9a5c9ddc	20	2026-10-30 12:54:23.292805+00	"StrategicActionCommit"
28303	9a5c9ddc	21	2026-10-30 12:55:23.292805+00	{"EndTurn": {"player": "Bob"}}
28304	9a5c9ddc	22	2026-10-30 12:56:23.292805+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28305	9a5c9ddc	23	2026-10-30 12:57:23.292805+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30110	434a0278	0	2026-11-02 22:47:21.910188+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": false}}}}
30111	434a0278	1	2026-11-02 22:47:29.559627+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce"]}}
30112	434a0278	2	2026-11-02 22:47:30.022475+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30113	434a0278	3	2026-11-02 22:47:30.488654+00	{"SetGalacticEvents": {"events": ["MinorFactions", "AgeOfCommerce", "ConventionsOfWarAbandoned"]}}
30114	434a0278	4	2026-11-02 22:47:40.047744+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "Arborec"}}}
30115	434a0278	5	2026-11-02 22:47:40.567863+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "Winnu"}}}
30116	434a0278	6	2026-11-02 22:47:41.10047+00	{"AddPlayer": {"player": {"name": "Cara", "color": "Green", "faction": "NekroVirus"}}}
30118	434a0278	7	2026-11-02 22:48:59.46208+00	{"SetGalacticEvents": {"events": ["MinorFactions", "AgeOfCommerce"]}}
30119	434a0278	8	2026-11-02 22:48:59.891514+00	{"SetGalacticEvents": {"events": ["AgeOfCommerce"]}}
30120	434a0278	9	2026-11-02 22:49:00.324975+00	"CreationDone"
30215	e0a811de	9	2026-11-04 08:54:38.722813+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30216	e0a811de	10	2026-11-04 08:54:39.438027+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30217	e0a811de	11	2026-11-04 08:54:40.160193+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30218	e0a811de	12	2026-11-04 08:54:40.875649+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30219	e0a811de	13	2026-11-04 08:54:41.593762+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30220	e0a811de	14	2026-11-04 08:54:42.309141+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30221	e0a811de	15	2026-11-04 08:54:43.025699+00	"CompleteStrategyPhase"
30222	e0a811de	16	2026-11-04 08:55:11.045111+00	{"AddTechToPlayer": {"tech": "X89BacterialWeapon", "player": "Alice"}}
30223	e0a811de	17	2026-11-04 08:55:34.592027+00	{"PlayGalacticEventAction": {"action": {"ConventionsOfWarAbandoned": {"planet": "Arretze"}}, "player": "Alice"}}
30224	e0a811de	18	2026-11-04 08:55:54.296173+00	{"EndTurn": {"player": "Alice"}}
30225	e0a811de	19	2026-11-04 08:55:55.330334+00	{"TacticalActionBegin": {"player": "Bob"}}
31259	63b0393b	6	2026-11-05 23:11:04.541837+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31260	63b0393b	7	2026-11-05 23:11:04.549888+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31261	63b0393b	8	2026-11-05 23:11:04.558784+00	"StartGame"
31262	63b0393b	9	2026-11-05 23:11:04.566993+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31263	63b0393b	10	2026-11-05 23:11:04.575416+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31264	63b0393b	11	2026-11-05 23:11:04.583216+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31265	63b0393b	12	2026-11-05 23:11:04.592125+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31266	63b0393b	13	2026-11-05 23:11:04.599143+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31267	63b0393b	14	2026-11-05 23:11:04.60692+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31268	63b0393b	15	2026-11-05 23:11:04.614286+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31269	63b0393b	16	2026-11-05 23:11:04.621856+00	"CompleteStrategyPhase"
31270	63b0393b	17	2026-11-05 23:11:04.628686+00	{"SetPlanetOwner": {"planet": "Thibah", "player": "Tux"}}
31271	63b0393b	18	2026-11-05 23:11:04.63542+00	{"SetPlanetOwner": {"planet": "Accoen", "player": "Tux"}}
31272	63b0393b	19	2026-11-05 23:11:04.641897+00	{"SetPlanetOwner": {"planet": "Lazar", "player": "Vidde"}}
31273	63b0393b	20	2026-11-05 23:11:04.647878+00	{"SetPlanetOwner": {"planet": "Torkan", "player": "Vidde"}}
31274	d8bc49b5	1	2026-11-05 23:11:04.946821+00	{"SetGalacticEvents": {"events": ["ZealousOrthodoxy"]}}
31275	d8bc49b5	2	2026-11-05 23:11:04.962915+00	"CreationDone"
31276	d8bc49b5	3	2026-11-05 23:11:04.969464+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31277	d8bc49b5	4	2026-11-05 23:11:04.975994+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31278	d8bc49b5	5	2026-11-05 23:11:04.984361+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31279	d8bc49b5	6	2026-11-05 23:11:04.99134+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31280	d8bc49b5	7	2026-11-05 23:11:04.99992+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31281	d8bc49b5	8	2026-11-05 23:11:05.008555+00	"StartGame"
31282	d8bc49b5	9	2026-11-05 23:11:05.018088+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31283	d8bc49b5	10	2026-11-05 23:11:05.02635+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31284	d8bc49b5	11	2026-11-05 23:11:05.034009+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31285	d8bc49b5	12	2026-11-05 23:11:05.04128+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31286	d8bc49b5	13	2026-11-05 23:11:05.04797+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31287	d8bc49b5	14	2026-11-05 23:11:05.055804+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31288	d8bc49b5	15	2026-11-05 23:11:05.063309+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31289	d8bc49b5	16	2026-11-05 23:11:05.070819+00	"CompleteStrategyPhase"
31290	d8bc49b5	17	2026-11-05 23:11:05.078124+00	{"ScoreExtraSecretObjective": {"player": "Tux", "objective": "DestroyTheirGreatestShip"}}
31291	d8bc49b5	18	2026-11-05 23:11:05.085852+00	{"ScoreExtraSecretObjective": {"player": "Vidde", "objective": "BecomeAMartyr"}}
31292	0bc7835e	1	2026-11-05 23:11:05.376415+00	{"SetGalacticEvents": {"events": ["HiddenAgenda"]}}
31293	0bc7835e	2	2026-11-05 23:11:05.393956+00	"CreationDone"
31294	0bc7835e	3	2026-11-05 23:11:05.401929+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31295	0bc7835e	4	2026-11-05 23:11:05.410317+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
30226	e0a811de	20	2026-11-04 09:28:44.228772+00	{"TacticalActionTakePlanet": {"planet": "AhkCreuxx", "player": "Bob"}}
30227	e0a811de	21	2026-11-04 09:28:44.956971+00	{"TacticalActionCommit": {"player": "Bob"}}
30228	e0a811de	22	2026-11-04 09:28:46.864971+00	{"EndTurn": {"player": "Bob"}}
30229	e0a811de	23	2026-11-04 09:29:06.092342+00	{"AddTechToPlayer": {"tech": "X89BacterialWeapon", "player": "Bob"}}
30230	e0a811de	24	2026-11-04 09:29:06.269926+00	{"RemoveTechFromPlayer": {"tech": "X89BacterialWeapon", "player": "Bob"}}
30231	e0a811de	25	2026-11-04 09:29:07.355792+00	{"AddTechToPlayer": {"tech": "X89BacterialWeapon", "player": "Carol"}}
30232	e0a811de	26	2026-11-04 09:29:09.018894+00	{"AddTechToPlayer": {"tech": "X89BacterialWeapon", "player": "Bob"}}
30233	e0a811de	27	2026-11-04 09:31:53.343793+00	{"PlayGalacticEventAction": {"action": {"ConventionsOfWarAbandoned": {"planet": "AhkCreuxx"}}, "player": "Carol"}}
30234	e0a811de	28	2026-11-04 09:32:05.596963+00	{"EndTurn": {"player": "Carol"}}
30266	e0a811de	29	2026-11-04 13:23:16.634705+00	{"SetPlanetDestroyed": {"planet": "Darien", "destroyed": true}}
30267	e0a811de	30	2026-11-04 13:23:17.640972+00	{"SetPlanetDestroyed": {"planet": "Darien", "destroyed": false}}
31296	0bc7835e	5	2026-11-05 23:11:05.418828+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31297	0bc7835e	6	2026-11-05 23:11:05.427119+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31298	0bc7835e	7	2026-11-05 23:11:05.435511+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31299	0bc7835e	8	2026-11-05 23:11:05.442603+00	"StartGame"
31300	0bc7835e	9	2026-11-05 23:11:05.450326+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31301	0bc7835e	10	2026-11-05 23:11:05.457998+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31302	0bc7835e	11	2026-11-05 23:11:05.465389+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31303	0bc7835e	12	2026-11-05 23:11:05.472314+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31304	0bc7835e	13	2026-11-05 23:11:05.479385+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31305	0bc7835e	14	2026-11-05 23:11:05.485681+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31306	0bc7835e	15	2026-11-05 23:11:05.493303+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31307	0bc7835e	16	2026-11-05 23:11:05.500826+00	"CompleteStrategyPhase"
31308	0bc7835e	17	2026-11-05 23:11:05.509014+00	{"SetCustodians": {"player": "Tux"}}
31309	0bc7835e	18	2026-11-05 23:11:05.515435+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Tux"}}
31310	0bc7835e	19	2026-11-05 23:11:05.521583+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31311	0bc7835e	20	2026-11-05 23:11:05.52824+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Yoda"}}
31312	0bc7835e	21	2026-11-05 23:11:05.535229+00	"StrategicActionCommit"
31313	0bc7835e	22	2026-11-05 23:11:05.541906+00	{"EndTurn": {"player": "Tux"}}
31314	0bc7835e	23	2026-11-05 23:11:05.549344+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}
31315	0bc7835e	24	2026-11-05 23:11:05.557543+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31316	0bc7835e	25	2026-11-05 23:11:05.565852+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Yoda"}}
31317	0bc7835e	26	2026-11-05 23:11:05.573749+00	"StrategicActionCommit"
31318	0bc7835e	27	2026-11-05 23:11:05.581324+00	{"EndTurn": {"player": "Vidde"}}
31319	0bc7835e	28	2026-11-05 23:11:05.589552+00	{"StrategicActionBegin": {"card": "Politics", "player": "Yoda"}}
31320	0bc7835e	29	2026-11-05 23:11:05.597651+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Yoda"}}, "player": "Yoda"}}
31321	0bc7835e	30	2026-11-05 23:11:05.605125+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31322	0bc7835e	31	2026-11-05 23:11:05.612145+00	{"StrategicActionSecondary": {"action": "Politics", "player": "Vidde"}}
31323	0bc7835e	32	2026-11-05 23:11:05.617805+00	"StrategicActionCommit"
31324	0bc7835e	33	2026-11-05 23:11:05.623159+00	{"EndTurn": {"player": "Yoda"}}
31325	0bc7835e	34	2026-11-05 23:11:05.627802+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
31326	0bc7835e	35	2026-11-05 23:11:05.63331+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31327	0bc7835e	36	2026-11-05 23:11:05.639299+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Yoda"}}
31328	0bc7835e	37	2026-11-05 23:11:05.645027+00	"StrategicActionCommit"
31329	0bc7835e	38	2026-11-05 23:11:05.650363+00	{"EndTurn": {"player": "Tux"}}
31330	0bc7835e	39	2026-11-05 23:11:05.654898+00	{"StrategicActionBegin": {"card": "Trade", "player": "Vidde"}}
31331	0bc7835e	40	2026-11-05 23:11:05.659797+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Tux"}}
31332	0bc7835e	41	2026-11-05 23:11:05.665344+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Yoda"}}
31333	0bc7835e	42	2026-11-05 23:11:05.671747+00	"StrategicActionCommit"
31334	0bc7835e	43	2026-11-05 23:11:05.677448+00	{"EndTurn": {"player": "Vidde"}}
31335	0bc7835e	44	2026-11-05 23:11:05.682191+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Yoda"}}
31336	0bc7835e	45	2026-11-05 23:11:05.687391+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Tux"}}
31337	0bc7835e	46	2026-11-05 23:11:05.693447+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Vidde"}}
31338	0bc7835e	47	2026-11-05 23:11:05.6988+00	"StrategicActionCommit"
31339	0bc7835e	48	2026-11-05 23:11:05.703704+00	{"EndTurn": {"player": "Yoda"}}
31340	0bc7835e	49	2026-11-05 23:11:05.709145+00	{"PassAction": {"player": "Tux"}}
31341	0bc7835e	50	2026-11-05 23:11:05.714374+00	{"EndTurn": {"player": "Tux"}}
31342	0bc7835e	51	2026-11-05 23:11:05.719655+00	{"PassAction": {"player": "Vidde"}}
31343	0bc7835e	52	2026-11-05 23:11:05.726598+00	{"EndTurn": {"player": "Vidde"}}
31344	0bc7835e	53	2026-11-05 23:11:05.733554+00	{"PassAction": {"player": "Yoda"}}
31345	0bc7835e	54	2026-11-05 23:11:05.739914+00	{"EndTurn": {"player": "Yoda"}}
31346	0bc7835e	55	2026-11-05 23:11:05.746296+00	{"ScorePublicObjective": {"player": "Tux", "objective": null}}
30235	cf738caf	0	2026-11-04 10:15:17.7874+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30236	cf738caf	1	2026-11-04 10:15:25.369196+00	{"SetGalacticEvents": {"events": ["AgeOfFighters"]}}
30237	cf738caf	2	2026-11-04 10:15:37.655759+00	{"SetGalacticEvents": {"events": ["StellarAtomics", "AgeOfFighters"]}}
30238	cf738caf	3	2026-11-04 10:15:38.202188+00	{"SetGalacticEvents": {"events": ["StellarAtomics", "AgeOfFighters", "ZealousOrthodoxy"]}}
30239	cf738caf	4	2026-11-04 10:15:38.754259+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics", "AgeOfFighters", "ZealousOrthodoxy"]}}
30240	cf738caf	5	2026-11-04 10:15:39.303777+00	{"SetGalacticEvents": {"events": ["TotalWar", "StellarAtomics", "AgeOfFighters", "ZealousOrthodoxy", "ConventionsOfWarAbandoned"]}}
30241	cf738caf	6	2026-11-04 10:15:39.902765+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30242	cf738caf	7	2026-11-04 10:15:40.702552+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30243	cf738caf	8	2026-11-04 10:15:41.507203+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30244	cf738caf	9	2026-11-04 10:16:00.895973+00	{"SetStellarAtomicsUsed": {"used": true, "player": "Alice"}}
30245	cf738caf	10	2026-11-04 10:16:01.61179+00	{"SetStellarAtomicsUsed": {"used": false, "player": "Alice"}}
30246	cf738caf	11	2026-11-04 10:16:02.33017+00	{"SetStellarAtomicsUsed": {"used": true, "player": "Bob"}}
30247	cf738caf	12	2026-11-04 10:16:11.405516+00	{"SetZealousOrthodoxyOwner": {"player": "Bob"}}
30248	cf738caf	13	2026-11-04 10:16:13.717851+00	{"SetZealousOrthodoxyOwner": {"player": "Carol"}}
30249	cf738caf	14	2026-11-04 10:16:15.961368+00	{"SetZealousOrthodoxyOwner": {"player": null}}
30250	cf738caf	15	2026-11-04 10:16:27.291159+00	{"SetTotalWarPoints": {"player": "Alice", "points": 1}}
30251	cf738caf	16	2026-11-04 10:16:27.802697+00	{"SetTotalWarPoints": {"player": "Alice", "points": 2}}
30252	cf738caf	17	2026-11-04 10:16:28.323101+00	{"SetTotalWarPoints": {"player": "Bob", "points": 1}}
30253	cf738caf	18	2026-11-04 10:16:28.836809+00	{"SetTotalWarPoints": {"player": "Alice", "points": 1}}
30254	cf738caf	19	2026-11-04 10:16:29.345114+00	{"SetZealousOrthodoxyOwner": {"player": "Carol"}}
30255	cf738caf	20	2026-11-04 10:16:44.468091+00	{"AddImperial": {"value": 1, "player": "Alice"}}
30256	cf738caf	21	2026-11-04 10:16:45.090155+00	{"AddExtraPoints": {"value": 1, "player": "Alice"}}
30257	cf738caf	22	2026-11-04 10:16:45.705777+00	{"AddImperial": {"value": -1, "player": "Alice"}}
30258	cf738caf	23	2026-11-04 10:16:46.321559+00	{"AddExtraPoints": {"value": -1, "player": "Alice"}}
30259	cf738caf	24	2026-11-04 10:16:58.57145+00	{"SetPlanetOwner": {"planet": "Lodor", "player": "Alice"}}
30260	cf738caf	25	2026-11-04 10:17:00.562212+00	{"SetPlanetDestroyed": {"planet": "Lodor", "destroyed": true}}
30261	cf738caf	26	2026-11-04 10:18:09.938038+00	{"SetPlanetDestroyed": {"planet": "Lodor", "destroyed": false}}
30262	cf738caf	27	2026-11-04 10:18:11.297817+00	{"SetPlanetOwner": {"planet": "Lodor", "player": "Bob"}}
31347	0bc7835e	56	2026-11-05 23:11:05.752272+00	{"ScorePublicObjective": {"player": "Vidde", "objective": null}}
31348	0bc7835e	57	2026-11-05 23:11:05.758124+00	{"ScorePublicObjective": {"player": "Yoda", "objective": null}}
31349	0bc7835e	58	2026-11-05 23:11:05.765327+00	{"ScoreSecretObjective": {"player": "Tux", "objective": null}}
31350	0bc7835e	59	2026-11-05 23:11:05.772192+00	{"ScoreSecretObjective": {"player": "Vidde", "objective": null}}
31351	0bc7835e	60	2026-11-05 23:11:05.77874+00	{"ScoreSecretObjective": {"player": "Yoda", "objective": null}}
31352	0bc7835e	61	2026-11-05 23:11:05.786061+00	{"RevealPublicObjective": {"objective": "DiversifyResearch"}}
31353	0bc7835e	62	2026-11-05 23:11:05.793238+00	"CompleteStatusPhase"
31354	185090c7	1	2026-11-05 23:11:06.089468+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
31355	185090c7	2	2026-11-05 23:11:06.108808+00	"CreationDone"
31356	185090c7	3	2026-11-05 23:11:06.120221+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31357	185090c7	4	2026-11-05 23:11:06.129394+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31358	185090c7	5	2026-11-05 23:11:06.138614+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31359	185090c7	6	2026-11-05 23:11:06.147682+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31360	1aac2dae	1	2026-11-05 23:11:06.442063+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
31361	1aac2dae	2	2026-11-05 23:11:06.458938+00	"CreationDone"
31362	1aac2dae	3	2026-11-05 23:11:06.465736+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31363	1aac2dae	4	2026-11-05 23:11:06.473204+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31364	1aac2dae	5	2026-11-05 23:11:06.480249+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31365	1aac2dae	6	2026-11-05 23:11:06.486495+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31366	1aac2dae	7	2026-11-05 23:11:06.493341+00	{"RevealAllPublicObjectives": {"stageOne": ["AmassWealth", "BuildDefenses", "CornerTheMarket", "DiversifyResearch", "ExpandBorders"], "stageTwo": ["AchieveSupremacy", "CentralizeGalacticTrade", "ConquerTheWeak", "FormGalacticBrainTrust", "FoundAGoldenAge"]}}
31367	1aac2dae	8	2026-11-05 23:11:06.500719+00	"StartGame"
31368	1aac2dae	9	2026-11-05 23:11:06.508269+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31369	1aac2dae	10	2026-11-05 23:11:06.515275+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31370	1aac2dae	11	2026-11-05 23:11:06.522799+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31371	1aac2dae	12	2026-11-05 23:11:06.530962+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31372	1aac2dae	13	2026-11-05 23:11:06.53911+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31373	1aac2dae	14	2026-11-05 23:11:06.54711+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
30272	4b5546f9	0	2026-11-04 14:42:19.087835+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30273	4b5546f9	1	2026-11-04 14:42:36.354696+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30274	4b5546f9	2	2026-11-04 14:42:37.589733+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30275	4b5546f9	3	2026-11-04 14:42:38.688951+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30276	4b5546f9	4	2026-11-04 14:42:39.627533+00	{"SetGalacticEvents": {"events": ["DangerousWilds"]}}
30277	4b5546f9	5	2026-11-04 14:42:45.523511+00	"CreationDone"
6106	dbe8cd53	0	2026-10-27 23:01:25.183894+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": true, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
6107	dbe8cd53	1	2026-10-27 23:01:51.314393+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "Arborec"}}}
6108	dbe8cd53	2	2026-10-27 23:01:52.108193+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "EmiratesOfHacan"}}}
6109	dbe8cd53	3	2026-10-27 23:01:52.882943+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
6110	dbe8cd53	4	2026-10-27 23:01:56.081712+00	"CreationDone"
6111	dbe8cd53	5	2026-10-27 23:02:01.802414+00	{"SetupSpeaker": {"player": "Alice"}}
6112	dbe8cd53	6	2026-10-27 23:02:02.555406+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
6113	dbe8cd53	7	2026-10-27 23:02:02.87102+00	"StartGame"
6114	dbe8cd53	8	2026-10-27 23:02:09.038809+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
6115	dbe8cd53	9	2026-10-27 23:02:09.360093+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
6116	dbe8cd53	10	2026-10-27 23:02:09.694299+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
6117	dbe8cd53	11	2026-10-27 23:02:10.016606+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
6118	dbe8cd53	12	2026-10-27 23:02:10.341746+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
6119	dbe8cd53	13	2026-10-27 23:02:10.67548+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
6120	dbe8cd53	14	2026-10-27 23:02:11.01058+00	"CompleteStrategyPhase"
6121	01bde7ff	0	2026-10-27 23:02:24.443637+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": true, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
6122	01bde7ff	1	2026-10-27 23:02:25.705435+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "Arborec"}}}
6123	01bde7ff	2	2026-10-27 23:02:26.498106+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Blue", "faction": "EmiratesOfHacan"}}}
6124	01bde7ff	3	2026-10-27 23:02:27.291727+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
6125	01bde7ff	4	2026-10-27 23:02:27.82538+00	"CreationDone"
6126	01bde7ff	5	2026-10-27 23:02:28.340683+00	{"SetupSpeaker": {"player": "Alice"}}
6127	01bde7ff	6	2026-10-27 23:02:29.111905+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
6128	01bde7ff	7	2026-10-27 23:02:29.450466+00	"StartGame"
6129	01bde7ff	8	2026-10-27 23:02:29.983649+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
6130	01bde7ff	9	2026-10-27 23:02:30.31393+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
6131	01bde7ff	10	2026-10-27 23:02:30.63086+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
6132	01bde7ff	11	2026-10-27 23:02:30.964227+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
6133	01bde7ff	12	2026-10-27 23:02:31.298168+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
6134	01bde7ff	13	2026-10-27 23:02:31.630553+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
6135	01bde7ff	14	2026-10-27 23:02:31.947302+00	"CompleteStrategyPhase"
30278	4b5546f9	6	2026-11-04 14:42:51.740437+00	{"SetupSpeaker": {"player": "Alice"}}
30279	4b5546f9	7	2026-11-04 14:42:52.641347+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
30280	4b5546f9	8	2026-11-04 14:42:53.419855+00	"StartGame"
30281	4b5546f9	9	2026-11-04 14:43:05.577656+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30282	4b5546f9	10	2026-11-04 14:43:06.265535+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30283	4b5546f9	11	2026-11-04 14:43:06.941009+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30284	4b5546f9	12	2026-11-04 14:43:07.602442+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30285	4b5546f9	13	2026-11-04 14:43:08.271206+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30286	4b5546f9	14	2026-11-04 14:43:08.942125+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30287	4b5546f9	15	2026-11-04 14:43:09.624666+00	"CompleteStrategyPhase"
30288	4b5546f9	16	2026-11-04 14:43:15.843424+00	{"TacticalActionBegin": {"player": "Alice"}}
30289	4b5546f9	17	2026-11-04 14:43:16.990006+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Alice"}}
30290	4b5546f9	18	2026-11-04 14:43:29.928804+00	{"TacticalActionResearchTechnology": {"tech": "FleetLogistics", "planet": "Abyz", "player": "Alice"}}
30291	4b5546f9	19	2026-11-04 14:43:40.336535+00	{"TacticalActionCommit": {"player": "Alice"}}
30292	4b5546f9	20	2026-11-04 14:43:41.417659+00	{"EndTurn": {"player": "Alice"}}
30293	4b5546f9	21	2026-11-04 14:44:01.936796+00	{"TacticalActionBegin": {"player": "Bob"}}
30294	4b5546f9	22	2026-11-04 14:44:03.065295+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Bob"}}
30295	4b5546f9	23	2026-11-04 14:44:14.918552+00	{"TacticalActionTakePlanet": {"planet": "Fria", "player": "Bob"}}
30296	4b5546f9	24	2026-11-04 14:44:17.370754+00	{"TacticalActionCommit": {"player": "Bob"}}
30297	4b5546f9	25	2026-11-04 14:44:18.434015+00	{"EndTurn": {"player": "Bob"}}
30298	4b5546f9	26	2026-11-04 14:44:19.4966+00	{"TacticalActionBegin": {"player": "Carol"}}
30299	4b5546f9	27	2026-11-04 14:44:20.597334+00	{"TacticalActionTakePlanet": {"planet": "Centauri", "player": "Carol"}}
30300	4b5546f9	28	2026-11-04 14:44:29.224552+00	{"TacticalActionCommit": {"player": "Carol"}}
30301	4b5546f9	29	2026-11-04 14:44:30.278334+00	{"EndTurn": {"player": "Carol"}}
31374	1aac2dae	15	2026-11-05 23:11:06.553667+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31375	1aac2dae	16	2026-11-05 23:11:06.560301+00	"CompleteStrategyPhase"
31376	1aac2dae	17	2026-11-05 23:11:06.567387+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Tux"}}
31377	1aac2dae	18	2026-11-05 23:11:06.573582+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31378	1aac2dae	19	2026-11-05 23:11:06.580117+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Yoda"}}
31379	1aac2dae	20	2026-11-05 23:11:06.58655+00	"StrategicActionCommit"
31380	1aac2dae	21	2026-11-05 23:11:06.59346+00	{"EndTurn": {"player": "Tux"}}
31381	1aac2dae	22	2026-11-05 23:11:06.601093+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}
31382	1aac2dae	23	2026-11-05 23:11:06.609033+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31383	1aac2dae	24	2026-11-05 23:11:06.616919+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Yoda"}}
31384	1aac2dae	25	2026-11-05 23:11:06.623508+00	"StrategicActionCommit"
31385	1aac2dae	26	2026-11-05 23:11:06.628919+00	{"EndTurn": {"player": "Vidde"}}
31386	1aac2dae	27	2026-11-05 23:11:06.635495+00	{"StrategicActionBegin": {"card": "Politics", "player": "Yoda"}}
31387	1aac2dae	28	2026-11-05 23:11:06.64184+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Yoda"}}, "player": "Yoda"}}
31388	1aac2dae	29	2026-11-05 23:11:06.648141+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31389	1aac2dae	30	2026-11-05 23:11:06.654408+00	{"StrategicActionSecondary": {"action": "Politics", "player": "Vidde"}}
31390	1aac2dae	31	2026-11-05 23:11:06.6608+00	"StrategicActionCommit"
31391	1aac2dae	32	2026-11-05 23:11:06.667047+00	{"EndTurn": {"player": "Yoda"}}
31392	1aac2dae	33	2026-11-05 23:11:06.673579+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
31393	1aac2dae	34	2026-11-05 23:11:06.679453+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31394	1aac2dae	35	2026-11-05 23:11:06.687026+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Yoda"}}
31395	1aac2dae	36	2026-11-05 23:11:06.694958+00	"StrategicActionCommit"
31396	1aac2dae	37	2026-11-05 23:11:06.702089+00	{"EndTurn": {"player": "Tux"}}
31397	1aac2dae	38	2026-11-05 23:11:06.709522+00	{"StrategicActionBegin": {"card": "Trade", "player": "Vidde"}}
31398	1aac2dae	39	2026-11-05 23:11:06.716505+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Tux"}}
31399	1aac2dae	40	2026-11-05 23:11:06.723536+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Yoda"}}
31400	1aac2dae	41	2026-11-05 23:11:06.731127+00	"StrategicActionCommit"
31401	1aac2dae	42	2026-11-05 23:11:06.737962+00	{"EndTurn": {"player": "Vidde"}}
31402	1aac2dae	43	2026-11-05 23:11:06.744988+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Yoda"}}
31403	1aac2dae	44	2026-11-05 23:11:06.752095+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Tux"}}
31404	1aac2dae	45	2026-11-05 23:11:06.75888+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Vidde"}}
31405	1aac2dae	46	2026-11-05 23:11:06.765702+00	"StrategicActionCommit"
31406	1aac2dae	47	2026-11-05 23:11:06.772886+00	{"EndTurn": {"player": "Yoda"}}
31407	1aac2dae	48	2026-11-05 23:11:06.779928+00	{"PassAction": {"player": "Tux"}}
31408	1aac2dae	49	2026-11-05 23:11:06.78747+00	{"EndTurn": {"player": "Tux"}}
31409	1aac2dae	50	2026-11-05 23:11:06.793633+00	{"PassAction": {"player": "Vidde"}}
31410	1aac2dae	51	2026-11-05 23:11:06.800043+00	{"EndTurn": {"player": "Vidde"}}
31411	1aac2dae	52	2026-11-05 23:11:06.807889+00	{"PassAction": {"player": "Yoda"}}
31412	1aac2dae	53	2026-11-05 23:11:06.81398+00	{"EndTurn": {"player": "Yoda"}}
31413	7d8a64d9	1	2026-11-05 23:11:07.107779+00	{"SetGalacticEvents": {"events": ["CulturalExchangeProgram"]}}
31414	7d8a64d9	2	2026-11-05 23:11:07.125332+00	"CreationDone"
31415	7d8a64d9	3	2026-11-05 23:11:07.133217+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31416	7d8a64d9	4	2026-11-05 23:11:07.141843+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31417	7d8a64d9	5	2026-11-05 23:11:07.150439+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31418	7d8a64d9	6	2026-11-05 23:11:07.160334+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31419	7d8a64d9	7	2026-11-05 23:11:07.169052+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31420	260cb545	1	2026-11-05 23:11:07.464091+00	{"SetGalacticEvents": {"events": ["CulturalExchangeProgram"]}}
31421	260cb545	2	2026-11-05 23:11:07.480678+00	"CreationDone"
31422	260cb545	3	2026-11-05 23:11:07.488774+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31423	260cb545	4	2026-11-05 23:11:07.496224+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31424	260cb545	5	2026-11-05 23:11:07.503764+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31425	260cb545	6	2026-11-05 23:11:07.511228+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31426	260cb545	7	2026-11-05 23:11:07.519239+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31427	260cb545	8	2026-11-05 23:11:07.525701+00	{"SetupCulturalExchange": {"player": "Tux", "faction": "CrimsonRebellion"}}
31428	260cb545	9	2026-11-05 23:11:07.531325+00	{"SetupCulturalExchange": {"player": "Yoda", "faction": "CouncilKeleres"}}
31429	260cb545	10	2026-11-05 23:11:07.537481+00	"StartGame"
27569	69d97f21	10	2026-10-30 13:24:19.99373+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Carol"}}
31430	260cb545	11	2026-11-05 23:11:07.544394+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
30302	4a9e1d07	0	2026-11-04 16:31:24.536431+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30303	4a9e1d07	1	2026-11-04 16:31:25.536431+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
30304	4a9e1d07	2	2026-11-04 16:31:26.536431+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30305	4a9e1d07	3	2026-11-04 16:31:27.536431+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30306	4a9e1d07	4	2026-11-04 16:31:28.536431+00	{"SetGalacticEvents": {"events": ["StellarAtomics", "HiddenAgenda"]}}
30307	4a9e1d07	5	2026-11-04 16:31:29.536431+00	"CreationDone"
30308	4a9e1d07	6	2026-11-04 16:31:30.536431+00	{"SetupSpeaker": {"player": "Alice"}}
30309	4a9e1d07	7	2026-11-04 16:31:31.536431+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30310	4a9e1d07	8	2026-11-04 16:31:32.536431+00	"StartGame"
30311	4a9e1d07	9	2026-11-04 16:31:33.536431+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30312	4a9e1d07	10	2026-11-04 16:31:34.536431+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30313	4a9e1d07	11	2026-11-04 16:31:35.536431+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30314	4a9e1d07	12	2026-11-04 16:31:36.536431+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30315	4a9e1d07	13	2026-11-04 16:31:37.536431+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30316	4a9e1d07	14	2026-11-04 16:31:38.536431+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30317	4a9e1d07	15	2026-11-04 16:31:39.536431+00	"CompleteStrategyPhase"
30318	4a9e1d07	16	2026-11-04 16:31:40.536431+00	{"PlayGalacticEventAction": {"action": "StellarAtomics", "player": "Alice"}}
30319	4a9e1d07	17	2026-11-04 16:31:41.536431+00	{"EndTurn": {"player": "Alice"}}
30320	4a9e1d07	18	2026-11-04 16:31:42.536431+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30321	4a9e1d07	19	2026-11-04 16:31:43.536431+00	"StrategicActionCommit"
30322	4a9e1d07	20	2026-11-04 16:31:44.536431+00	{"EndTurn": {"player": "Bob"}}
30323	4a9e1d07	21	2026-11-04 16:31:45.536431+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30324	4a9e1d07	22	2026-11-04 16:31:46.536431+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30325	4a9e1d07	23	2026-11-04 16:31:47.536431+00	"StrategicActionCommit"
30326	4a9e1d07	24	2026-11-04 16:31:48.536431+00	{"EndTurn": {"player": "Carol"}}
30327	4a9e1d07	25	2026-11-04 16:31:49.536431+00	{"AddAgendaBegin": {"agenda": "Mutiny"}}
30328	4a9e1d07	26	2026-11-04 16:31:50.536431+00	"AddAgendaCancel"
30329	4a9e1d07	27	2026-11-04 16:31:51.536431+00	{"SetCustodians": {"player": "Alice"}}
30330	4a9e1d07	28	2026-11-04 16:31:52.536431+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30331	4a9e1d07	29	2026-11-04 16:31:53.536431+00	"StrategicActionCommit"
30332	4a9e1d07	30	2026-11-04 16:31:54.536431+00	{"EndTurn": {"player": "Alice"}}
30333	4a9e1d07	31	2026-11-04 16:31:55.536431+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30334	4a9e1d07	32	2026-11-04 16:31:56.536431+00	"StrategicActionCommit"
30335	4a9e1d07	33	2026-11-04 16:31:57.536431+00	{"EndTurn": {"player": "Bob"}}
30336	4a9e1d07	34	2026-11-04 16:31:58.536431+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30337	4a9e1d07	35	2026-11-04 16:31:59.536431+00	"StrategicActionCommit"
30338	4a9e1d07	36	2026-11-04 16:32:00.536431+00	{"EndTurn": {"player": "Carol"}}
30339	4a9e1d07	37	2026-11-04 16:32:01.536431+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30340	4a9e1d07	38	2026-11-04 16:32:02.536431+00	"StrategicActionCommit"
30341	4a9e1d07	39	2026-11-04 16:32:03.536431+00	{"EndTurn": {"player": "Alice"}}
30342	4a9e1d07	40	2026-11-04 16:32:04.536431+00	{"PassAction": {"player": "Bob"}}
30343	4a9e1d07	41	2026-11-04 16:32:05.536431+00	{"EndTurn": {"player": "Bob"}}
30344	4a9e1d07	42	2026-11-04 16:32:06.536431+00	{"PassAction": {"player": "Carol"}}
30345	4a9e1d07	43	2026-11-04 16:32:07.536431+00	{"EndTurn": {"player": "Carol"}}
30346	4a9e1d07	44	2026-11-04 16:32:08.536431+00	{"PassAction": {"player": "Alice"}}
30347	4a9e1d07	45	2026-11-04 16:32:09.536431+00	{"EndTurn": {"player": "Alice"}}
30348	4a9e1d07	46	2026-11-04 16:32:10.536431+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
30349	4a9e1d07	47	2026-11-04 16:32:11.536431+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30350	4a9e1d07	48	2026-11-04 16:32:12.536431+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30351	4a9e1d07	49	2026-11-04 16:32:13.536431+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30352	4a9e1d07	50	2026-11-04 16:32:14.536431+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30353	4a9e1d07	51	2026-11-04 16:32:15.536431+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30354	4a9e1d07	52	2026-11-04 16:32:16.536431+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
30355	4a9e1d07	53	2026-11-04 16:32:17.536431+00	"CompleteStatusPhase"
30356	4a9e1d07	54	2026-11-04 16:32:18.536431+00	{"RevealAgenda": {"agenda": "Mutiny"}}
30357	4a9e1d07	55	2026-11-04 16:32:19.536431+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
30358	4a9e1d07	56	2026-11-04 16:32:20.536431+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
30359	4a9e1d07	57	2026-11-04 16:32:21.536431+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
30364	11751527	0	2026-11-04 20:30:51.747448+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
31431	260cb545	12	2026-11-05 23:11:07.551195+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
30360	4a9e1d07	58	2026-11-04 16:34:42.640815+00	{"RevealAgenda": {"agenda": "ArmsReduction"}}
30361	4a9e1d07	59	2026-11-04 16:35:01.968892+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
30362	4a9e1d07	60	2026-11-04 16:35:02.847954+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
30363	4a9e1d07	61	2026-11-04 16:35:03.697034+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
30365	11751527	1	2026-11-04 20:31:51.747448+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
30366	11751527	2	2026-11-04 20:32:51.747448+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30367	11751527	3	2026-11-04 20:33:51.747448+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30368	11751527	4	2026-11-04 20:34:51.747448+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
30369	11751527	5	2026-11-04 20:35:51.747448+00	"CreationDone"
30370	11751527	6	2026-11-04 20:36:51.747448+00	{"SetupSpeaker": {"player": "Alice"}}
30371	5554afee	0	2026-11-04 20:30:51.750443+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
30372	5554afee	1	2026-11-04 20:31:51.750443+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
30373	5554afee	2	2026-11-04 20:32:51.750443+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30374	5554afee	3	2026-11-04 20:33:51.750443+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30375	5554afee	4	2026-11-04 20:34:51.750443+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
30376	5554afee	5	2026-11-04 20:35:51.750443+00	"CreationDone"
30377	5554afee	6	2026-11-04 20:36:51.750443+00	{"SetupSpeaker": {"player": "Alice"}}
30378	5554afee	7	2026-11-04 20:37:51.750443+00	{"RevealAllPublicObjectives": {"stageOne": ["CornerTheMarket", "DevelopWeaponry", "DiversifyResearch", "ErectAMonument", "ExpandBorders"], "stageTwo": ["CentralizeGalacticTrade", "ConquerTheWeak", "FormGalacticBrainTrust", "FoundAGoldenAge", "GalvanizeThePeople"]}}
30379	5554afee	8	2026-11-04 20:38:51.750443+00	"StartGame"
30380	5554afee	9	2026-11-04 20:39:51.750443+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30381	5554afee	10	2026-11-04 20:40:51.750443+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30382	5554afee	11	2026-11-04 20:41:51.750443+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30383	5554afee	12	2026-11-04 20:42:51.750443+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30384	5554afee	13	2026-11-04 20:43:51.750443+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30385	5554afee	14	2026-11-04 20:44:51.750443+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30386	5554afee	15	2026-11-04 20:45:51.750443+00	"CompleteStrategyPhase"
30387	5554afee	16	2026-11-04 20:46:51.750443+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30388	5554afee	17	2026-11-04 20:47:51.750443+00	"StrategicActionCommit"
30389	5554afee	18	2026-11-04 20:48:51.750443+00	{"EndTurn": {"player": "Alice"}}
30390	5554afee	19	2026-11-04 20:49:51.750443+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30391	5554afee	20	2026-11-04 20:50:51.750443+00	"StrategicActionCommit"
30392	5554afee	21	2026-11-04 20:51:51.750443+00	{"EndTurn": {"player": "Bob"}}
30393	5554afee	22	2026-11-04 20:52:51.750443+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30394	5554afee	23	2026-11-04 20:53:51.750443+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30395	5554afee	24	2026-11-04 20:54:51.750443+00	"StrategicActionCommit"
30396	5554afee	25	2026-11-04 20:55:51.750443+00	{"EndTurn": {"player": "Carol"}}
30397	5554afee	26	2026-11-04 20:56:51.750443+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30398	5554afee	27	2026-11-04 20:57:51.750443+00	"StrategicActionCommit"
30399	5554afee	28	2026-11-04 20:58:51.750443+00	{"EndTurn": {"player": "Alice"}}
30400	5554afee	29	2026-11-04 20:59:51.750443+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30401	5554afee	30	2026-11-04 21:00:51.750443+00	"StrategicActionCommit"
30402	5554afee	31	2026-11-04 21:01:51.750443+00	{"EndTurn": {"player": "Bob"}}
30403	5554afee	32	2026-11-04 21:02:51.750443+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30404	5554afee	33	2026-11-04 21:03:51.750443+00	"StrategicActionCommit"
30405	5554afee	34	2026-11-04 21:04:51.750443+00	{"EndTurn": {"player": "Carol"}}
30406	5554afee	35	2026-11-04 21:05:51.750443+00	{"PassAction": {"player": "Alice"}}
30407	5554afee	36	2026-11-04 21:06:51.750443+00	{"EndTurn": {"player": "Alice"}}
30408	5554afee	37	2026-11-04 21:07:51.750443+00	{"PassAction": {"player": "Bob"}}
30409	5554afee	38	2026-11-04 21:08:51.750443+00	{"EndTurn": {"player": "Bob"}}
30410	5554afee	39	2026-11-04 21:09:51.750443+00	{"PassAction": {"player": "Carol"}}
30411	5554afee	40	2026-11-04 21:10:51.750443+00	{"EndTurn": {"player": "Carol"}}
30412	33ae89db	0	2026-11-04 20:48:51.762416+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30413	33ae89db	1	2026-11-04 20:49:51.762416+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "DeepwroughtScholarate"}}}
30414	33ae89db	2	2026-11-04 20:50:51.762416+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "MahactGeneSorcerers"}}}
30415	33ae89db	3	2026-11-04 20:51:51.762416+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "Arborec"}}}
30416	33ae89db	4	2026-11-04 20:52:51.762416+00	"CreationDone"
30417	33ae89db	5	2026-11-04 20:53:51.762416+00	{"SetupPlayerTechs": {"player": "Alice", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
30418	33ae89db	6	2026-11-04 20:54:51.762416+00	{"SetupSpeaker": {"player": "Alice"}}
30419	33ae89db	7	2026-11-04 20:55:51.762416+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30420	33ae89db	8	2026-11-04 20:56:51.762416+00	"StartGame"
30421	33ae89db	9	2026-11-04 20:57:51.762416+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30422	33ae89db	10	2026-11-04 20:58:51.762416+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30423	33ae89db	11	2026-11-04 20:59:51.762416+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30424	33ae89db	12	2026-11-04 21:00:51.762416+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30425	33ae89db	13	2026-11-04 21:01:51.762416+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30426	33ae89db	14	2026-11-04 21:02:51.762416+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30427	33ae89db	15	2026-11-04 21:03:51.762416+00	"CompleteStrategyPhase"
31432	260cb545	13	2026-11-05 23:11:07.558494+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31433	260cb545	14	2026-11-05 23:11:07.565815+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31434	260cb545	15	2026-11-05 23:11:07.571455+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31435	260cb545	16	2026-11-05 23:11:07.576332+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31436	260cb545	17	2026-11-05 23:11:07.582172+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31437	260cb545	18	2026-11-05 23:11:07.587054+00	"CompleteStrategyPhase"
31438	d64c88d1	1	2026-11-05 23:11:07.878559+00	{"SetGalacticEvents": {"events": ["RapidMobilization"]}}
31439	d64c88d1	2	2026-11-05 23:11:07.895504+00	"CreationDone"
31440	d64c88d1	3	2026-11-05 23:11:07.904648+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31441	d64c88d1	4	2026-11-05 23:11:07.913268+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31442	d64c88d1	5	2026-11-05 23:11:07.922083+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31443	d64c88d1	6	2026-11-05 23:11:07.930622+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31444	d64c88d1	7	2026-11-05 23:11:07.938968+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31445	d64c88d1	8	2026-11-05 23:11:07.946614+00	"StartGame"
31446	551dc1fe	1	2026-11-05 23:11:08.249255+00	{"SetGalacticEvents": {"events": ["AgeOfFighters"]}}
31447	551dc1fe	2	2026-11-05 23:11:08.265573+00	"CreationDone"
31448	551dc1fe	3	2026-11-05 23:11:08.27444+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31449	551dc1fe	4	2026-11-05 23:11:08.283055+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31450	551dc1fe	5	2026-11-05 23:11:08.291925+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31451	551dc1fe	6	2026-11-05 23:11:08.300412+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31452	551dc1fe	7	2026-11-05 23:11:08.307984+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31453	adf05cd3	1	2026-11-05 23:11:08.604744+00	{"SetGalacticEvents": {"events": ["AdventOfTheWarSun"]}}
31454	adf05cd3	2	2026-11-05 23:11:08.619948+00	"CreationDone"
31455	adf05cd3	3	2026-11-05 23:11:08.626394+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31456	adf05cd3	4	2026-11-05 23:11:08.633088+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31457	adf05cd3	5	2026-11-05 23:11:08.638988+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31458	adf05cd3	6	2026-11-05 23:11:08.644434+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31459	adf05cd3	7	2026-11-05 23:11:08.64993+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31460	1ac9a493	0	2026-11-05 23:11:40.116518+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31461	1ac9a493	1	2026-11-05 23:11:47.721856+00	{"AddPlayer": {"player": {"name": "Tux", "color": "Purple", "faction": "CouncilKeleres"}}}
31462	1ac9a493	2	2026-11-05 23:11:47.739187+00	{"AddPlayer": {"player": {"name": "Vidde", "color": "Black", "faction": "FirmamentObsidian"}}}
31463	1ac9a493	3	2026-11-05 23:11:47.747825+00	{"AddPlayer": {"player": {"name": "Yoda", "color": "Red", "faction": "CrimsonRebellion"}}}
31468	1ac9a493	4	2026-11-05 23:12:00.35173+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
31469	1ac9a493	5	2026-11-05 23:12:00.358642+00	"CreationDone"
31470	1ac9a493	6	2026-11-05 23:12:00.366333+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31471	1ac9a493	7	2026-11-05 23:12:00.371109+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31472	1ac9a493	8	2026-11-05 23:12:00.376247+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31473	1ac9a493	9	2026-11-05 23:12:00.380993+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31474	1ac9a493	10	2026-11-05 23:12:00.385668+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31475	1ac9a493	11	2026-11-05 23:12:00.39071+00	{"SetupSpeaker": {"player": "Tux"}}
31476	09d3557e	4	2026-11-05 23:12:00.684191+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
31477	09d3557e	5	2026-11-05 23:12:00.700256+00	"CreationDone"
31478	09d3557e	6	2026-11-05 23:12:00.70689+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31479	09d3557e	7	2026-11-05 23:12:00.713465+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31480	09d3557e	8	2026-11-05 23:12:00.72097+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31481	09d3557e	9	2026-11-05 23:12:00.728358+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
30428	11751527	7	2026-11-04 21:23:13.61996+00	{"RevealAllPublicObjectives": {"stageOne": ["CornerTheMarket", "DevelopWeaponry", "DiversifyResearch", "ErectAMonument", "ExpandBorders"], "stageTwo": ["CentralizeGalacticTrade", "ConquerTheWeak", "FormGalacticBrainTrust", "FoundAGoldenAge", "GalvanizeThePeople"]}}
30429	11751527	8	2026-11-04 21:23:14.669059+00	"StartGame"
30430	5554afee	41	2026-11-04 21:23:35.530948+00	{"ScorePublicObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
30431	5554afee	42	2026-11-04 21:23:36.630628+00	{"ScoreExtraPublicObjective": {"player": "Alice", "objective": "DevelopWeaponry"}}
30432	5554afee	43	2026-11-04 21:23:37.681858+00	{"ScoreExtraPublicObjective": {"player": "Alice", "objective": "CentralizeGalacticTrade"}}
30433	5554afee	44	2026-11-04 21:24:07.525073+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30434	5554afee	45	2026-11-04 21:24:08.16479+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30435	5554afee	46	2026-11-04 21:24:08.81615+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30436	5554afee	47	2026-11-04 21:24:09.466216+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30437	5554afee	48	2026-11-04 21:24:10.117057+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30438	5554afee	49	2026-11-04 21:24:10.764401+00	"CompleteStatusPhase"
30439	33ae89db	16	2026-11-04 21:24:52.232983+00	{"SetBreakthrough": {"player": "Alice", "breakthrough": "VisionariaSelect"}}
30440	33ae89db	17	2026-11-04 21:25:16.79797+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
30441	33ae89db	18	2026-11-04 21:25:18.494563+00	{"SetBreakthrough": {"player": "Bob", "breakthrough": "VaultsOfTheHeir"}}
30442	33ae89db	19	2026-11-04 21:25:20.21306+00	{"RollForTheFracture": {"player": "Bob", "entered_play": false}}
30443	33ae89db	20	2026-11-04 21:25:21.925323+00	{"SetBreakthrough": {"player": "Carol", "breakthrough": "Psychospore"}}
30444	33ae89db	21	2026-11-04 21:25:23.648432+00	{"RollForTheFracture": {"player": "Carol", "entered_play": false}}
30445	33ae89db	22	2026-11-04 21:25:43.59529+00	{"PlayBreakthroughAction": {"action": {"VisionariaSelect": {"researched": {"Bob": "GravityDrive"}}}, "player": "Alice"}}
30446	33ae89db	23	2026-11-04 21:26:01.411136+00	{"EndTurn": {"player": "Alice"}}
30447	33ae89db	24	2026-11-04 21:26:11.884718+00	{"PlayBreakthroughAction": {"action": {"VaultsOfTheHeir": {"relic": "TheCodex", "purgedTech": "BioStims"}}, "player": "Bob"}}
30448	33ae89db	25	2026-11-04 21:26:35.279822+00	{"EndTurn": {"player": "Bob"}}
30449	33ae89db	26	2026-11-04 21:26:38.203202+00	{"PlayBreakthroughAction": {"action": "Psychospore", "player": "Carol"}}
31464	09d3557e	0	2026-11-05 23:11:40.116518+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31465	09d3557e	1	2026-11-05 23:11:47.721856+00	{"AddPlayer": {"player": {"name": "Tux", "color": "Purple", "faction": "CouncilKeleres"}}}
31466	09d3557e	2	2026-11-05 23:11:47.739187+00	{"AddPlayer": {"player": {"name": "Vidde", "color": "Black", "faction": "FirmamentObsidian"}}}
31467	09d3557e	3	2026-11-05 23:11:47.747825+00	{"AddPlayer": {"player": {"name": "Yoda", "color": "Red", "faction": "CrimsonRebellion"}}}
30450	8abd8371	0	2026-11-04 21:31:19.628352+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30451	8abd8371	1	2026-11-04 21:32:19.628352+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "Arborec"}}}
30452	8abd8371	2	2026-11-04 21:33:19.628352+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "XxchaKingdom"}}}
30453	8abd8371	3	2026-11-04 21:34:19.628352+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "YssarilTribes"}}}
30454	8abd8371	4	2026-11-04 21:35:19.628352+00	{"AddPlayer": {"player": {"name": "Dave", "color": "Red", "faction": "FirmamentObsidian"}}}
30455	8abd8371	5	2026-11-04 21:36:19.628352+00	{"SetGalacticEvents": {"events": ["CulturalExchangeProgram"]}}
30456	8abd8371	6	2026-11-04 21:37:19.628352+00	"CreationDone"
30457	8abd8371	7	2026-11-04 21:38:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": "Arborec"}}
30458	8abd8371	8	2026-11-04 21:39:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": null}}
30459	8abd8371	9	2026-11-04 21:40:19.628352+00	{"SetupPlayerTechs": {"player": "Dave", "technologies": ["NeuralMotivator"]}}
30460	8abd8371	10	2026-11-04 21:41:19.628352+00	{"SetupSpeaker": {"player": "Alice"}}
30461	8abd8371	11	2026-11-04 21:42:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": "XxchaKingdom"}}
30462	8abd8371	12	2026-11-04 21:43:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": "YssarilTribes"}}
30463	8abd8371	13	2026-11-04 21:44:19.628352+00	{"SetupCulturalExchange": {"player": "Carol", "faction": "Arborec"}}
30464	8abd8371	14	2026-11-04 21:45:19.628352+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30465	8abd8371	15	2026-11-04 21:46:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": null}}
30466	8abd8371	16	2026-11-04 21:47:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": "YssarilTribes"}}
30467	f3b35c18	0	2026-11-04 21:21:19.628352+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30468	f3b35c18	1	2026-11-04 21:22:19.628352+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "Arborec"}}}
30469	f3b35c18	2	2026-11-04 21:23:19.628352+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "XxchaKingdom"}}}
30470	f3b35c18	3	2026-11-04 21:24:19.628352+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "YssarilTribes"}}}
30471	f3b35c18	4	2026-11-04 21:25:19.628352+00	{"AddPlayer": {"player": {"name": "Dave", "color": "Red", "faction": "FirmamentObsidian"}}}
30472	f3b35c18	5	2026-11-04 21:26:19.628352+00	{"SetGalacticEvents": {"events": ["CulturalExchangeProgram"]}}
30473	f3b35c18	6	2026-11-04 21:27:19.628352+00	"CreationDone"
30474	f3b35c18	7	2026-11-04 21:28:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": "Arborec"}}
30475	f3b35c18	8	2026-11-04 21:29:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": null}}
30476	f3b35c18	9	2026-11-04 21:30:19.628352+00	{"SetupPlayerTechs": {"player": "Dave", "technologies": ["NeuralMotivator"]}}
30477	f3b35c18	10	2026-11-04 21:31:19.628352+00	{"SetupSpeaker": {"player": "Alice"}}
30478	f3b35c18	11	2026-11-04 21:32:19.628352+00	{"SetupCulturalExchange": {"player": "Alice", "faction": "XxchaKingdom"}}
30479	f3b35c18	12	2026-11-04 21:33:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": "YssarilTribes"}}
30480	f3b35c18	13	2026-11-04 21:34:19.628352+00	{"SetupCulturalExchange": {"player": "Carol", "faction": "Arborec"}}
30481	f3b35c18	14	2026-11-04 21:35:19.628352+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30482	f3b35c18	15	2026-11-04 21:36:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": null}}
30483	f3b35c18	16	2026-11-04 21:37:19.628352+00	{"SetupCulturalExchange": {"player": "Bob", "faction": "YssarilTribes"}}
30484	f3b35c18	17	2026-11-04 21:38:19.628352+00	"StartGame"
30485	f3b35c18	18	2026-11-04 21:39:19.628352+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30486	f3b35c18	19	2026-11-04 21:40:19.628352+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30487	f3b35c18	20	2026-11-04 21:41:19.628352+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30488	f3b35c18	21	2026-11-04 21:42:19.628352+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dave"}}
30489	f3b35c18	22	2026-11-04 21:43:19.628352+00	{"TakeStrategyCard": {"card": "Trade", "player": "Alice"}}
30490	f3b35c18	23	2026-11-04 21:44:19.628352+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Bob"}}
30491	f3b35c18	24	2026-11-04 21:45:19.628352+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
30492	f3b35c18	25	2026-11-04 21:46:19.628352+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Dave"}}
30493	f3b35c18	26	2026-11-04 21:47:19.628352+00	"CompleteStrategyPhase"
30499	2cf183d9	0	2026-11-04 20:18:07.749675+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
30500	2cf183d9	1	2026-11-04 20:19:07.749675+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
30501	2cf183d9	2	2026-11-04 20:20:07.749675+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30502	2cf183d9	3	2026-11-04 20:21:07.749675+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30503	2cf183d9	4	2026-11-04 20:22:07.749675+00	{"SetGalacticEvents": {"events": ["MonumentsToTheAges"]}}
30504	2cf183d9	5	2026-11-04 20:23:07.749675+00	"CreationDone"
30505	2cf183d9	6	2026-11-04 20:24:07.749675+00	{"SetupSpeaker": {"player": "Alice"}}
30506	2cf183d9	7	2026-11-04 20:25:07.749675+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30507	2cf183d9	8	2026-11-04 20:26:07.749675+00	"StartGame"
30508	2cf183d9	9	2026-11-04 20:27:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30494	8abd8371	17	2026-11-04 21:48:37.886829+00	{"SetupCulturalExchange": {"player": "Alice", "faction": null}}
30495	8abd8371	18	2026-11-04 21:48:43.719484+00	{"SetupCulturalExchange": {"player": "Alice", "faction": "XxchaKingdom"}}
30496	8abd8371	19	2026-11-04 21:48:45.246328+00	"StartGame"
30497	f3b35c18	27	2026-11-04 21:49:24.608351+00	{"LeaderActionBegin": {"leader": "GgrucotoRinn", "player": "Alice"}}
30498	f3b35c18	28	2026-11-04 21:49:30.650577+00	{"EndTurn": {"player": "Alice"}}
30509	2cf183d9	10	2026-11-04 20:28:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30510	2cf183d9	11	2026-11-04 20:29:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30511	2cf183d9	12	2026-11-04 20:30:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30512	2cf183d9	13	2026-11-04 20:31:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30513	2cf183d9	14	2026-11-04 20:32:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30514	2cf183d9	15	2026-11-04 20:33:07.749675+00	"CompleteStrategyPhase"
30515	2cf183d9	16	2026-11-04 20:34:07.749675+00	{"SetPlanetOwner": {"planet": "ThundersEdge", "player": "Alice"}}
30516	2cf183d9	17	2026-11-04 20:35:07.749675+00	{"SetPlanetOwner": {"planet": "ThundersEdge", "player": null}}
30517	2cf183d9	18	2026-11-04 20:36:07.749675+00	{"TacticalActionBegin": {"player": "Alice"}}
30518	2cf183d9	19	2026-11-04 20:37:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Alice"}}
30519	2cf183d9	20	2026-11-04 20:38:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Fria", "player": "Alice"}}
30520	2cf183d9	21	2026-11-04 20:39:07.749675+00	{"TacticalActionCommit": {"player": "Alice"}}
30521	2cf183d9	22	2026-11-04 20:40:07.749675+00	{"EndTurn": {"player": "Alice"}}
30522	2cf183d9	23	2026-11-04 20:41:07.749675+00	{"TacticalActionBegin": {"player": "Bob"}}
30523	2cf183d9	24	2026-11-04 20:42:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Arinam", "player": "Bob"}}
30524	2cf183d9	25	2026-11-04 20:43:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Meer", "player": "Bob"}}
30525	2cf183d9	26	2026-11-04 20:44:07.749675+00	{"TacticalActionCommit": {"player": "Bob"}}
30526	2cf183d9	27	2026-11-04 20:45:07.749675+00	{"EndTurn": {"player": "Bob"}}
30527	2cf183d9	28	2026-11-04 20:46:07.749675+00	{"BuildMonument": {"planet": "Abyz", "player": "Alice"}}
30528	2cf183d9	29	2026-11-04 20:47:07.749675+00	{"BuildMonument": {"planet": "Fria", "player": "Alice"}}
30529	2cf183d9	30	2026-11-04 20:48:07.749675+00	{"BuildMonument": {"planet": "Arinam", "player": "Bob"}}
30530	2cf183d9	31	2026-11-04 20:49:07.749675+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30531	2cf183d9	32	2026-11-04 20:50:07.749675+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30532	2cf183d9	33	2026-11-04 20:51:07.749675+00	"StrategicActionCommit"
30533	2cf183d9	34	2026-11-04 20:52:07.749675+00	{"EndTurn": {"player": "Carol"}}
30534	2cf183d9	35	2026-11-04 20:53:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30535	2cf183d9	36	2026-11-04 20:54:07.749675+00	"StrategicActionCommit"
30536	2cf183d9	37	2026-11-04 20:55:07.749675+00	{"EndTurn": {"player": "Alice"}}
30537	2cf183d9	38	2026-11-04 20:56:07.749675+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30538	2cf183d9	39	2026-11-04 20:57:07.749675+00	"StrategicActionCommit"
30539	2cf183d9	40	2026-11-04 20:58:07.749675+00	{"EndTurn": {"player": "Bob"}}
30540	2cf183d9	41	2026-11-04 20:59:07.749675+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30541	2cf183d9	42	2026-11-04 21:00:07.749675+00	"StrategicActionCommit"
30542	2cf183d9	43	2026-11-04 21:01:07.749675+00	{"EndTurn": {"player": "Carol"}}
30543	2cf183d9	44	2026-11-04 21:02:07.749675+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30544	2cf183d9	45	2026-11-04 21:03:07.749675+00	"StrategicActionCommit"
30545	2cf183d9	46	2026-11-04 21:04:07.749675+00	{"EndTurn": {"player": "Alice"}}
30546	2cf183d9	47	2026-11-04 21:05:07.749675+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30547	2cf183d9	48	2026-11-04 21:06:07.749675+00	"StrategicActionCommit"
30548	2cf183d9	49	2026-11-04 21:07:07.749675+00	{"EndTurn": {"player": "Bob"}}
30549	2cf183d9	50	2026-11-04 21:08:07.749675+00	{"PassAction": {"player": "Carol"}}
30550	2cf183d9	51	2026-11-04 21:09:07.749675+00	{"EndTurn": {"player": "Carol"}}
30551	2cf183d9	52	2026-11-04 21:10:07.749675+00	{"PassAction": {"player": "Alice"}}
30552	2cf183d9	53	2026-11-04 21:11:07.749675+00	{"EndTurn": {"player": "Alice"}}
30553	2cf183d9	54	2026-11-04 21:12:07.749675+00	{"PassAction": {"player": "Bob"}}
30554	2cf183d9	55	2026-11-04 21:13:07.749675+00	{"EndTurn": {"player": "Bob"}}
30555	2cf183d9	56	2026-11-04 21:14:07.749675+00	{"SetMonumentCommodities": {"planet": "Fria", "commodities": 0}}
30556	2cf183d9	57	2026-11-04 21:15:07.749675+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
30557	2cf183d9	58	2026-11-04 21:16:07.749675+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30558	2cf183d9	59	2026-11-04 21:17:07.749675+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30559	2cf183d9	60	2026-11-04 21:18:07.749675+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30560	2cf183d9	61	2026-11-04 21:19:07.749675+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30561	2cf183d9	62	2026-11-04 21:20:07.749675+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30562	2cf183d9	63	2026-11-04 21:21:07.749675+00	{"RevealPublicObjective": {"objective": "DiversifyResearch"}}
30563	2cf183d9	64	2026-11-04 21:22:07.749675+00	"CompleteStatusPhase"
30564	2cf183d9	65	2026-11-04 21:23:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30565	2cf183d9	66	2026-11-04 21:24:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30566	2cf183d9	67	2026-11-04 21:25:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30567	2cf183d9	68	2026-11-04 21:26:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30568	2cf183d9	69	2026-11-04 21:27:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30569	2cf183d9	70	2026-11-04 21:28:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30570	2cf183d9	71	2026-11-04 21:29:07.749675+00	"CompleteStrategyPhase"
30571	2cf183d9	72	2026-11-04 21:30:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30572	2cf183d9	73	2026-11-04 21:31:07.749675+00	"StrategicActionCommit"
30573	2cf183d9	74	2026-11-04 21:32:07.749675+00	{"EndTurn": {"player": "Alice"}}
30574	2cf183d9	75	2026-11-04 21:33:07.749675+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30575	2cf183d9	76	2026-11-04 21:34:07.749675+00	"StrategicActionCommit"
30576	2cf183d9	77	2026-11-04 21:35:07.749675+00	{"EndTurn": {"player": "Bob"}}
30577	2cf183d9	78	2026-11-04 21:36:07.749675+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30578	2cf183d9	79	2026-11-04 21:37:07.749675+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30579	2cf183d9	80	2026-11-04 21:38:07.749675+00	"StrategicActionCommit"
30580	2cf183d9	81	2026-11-04 21:39:07.749675+00	{"EndTurn": {"player": "Carol"}}
30581	2cf183d9	82	2026-11-04 21:40:07.749675+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30582	2cf183d9	83	2026-11-04 21:41:07.749675+00	"StrategicActionCommit"
30583	2cf183d9	84	2026-11-04 21:42:07.749675+00	{"EndTurn": {"player": "Alice"}}
30584	2cf183d9	85	2026-11-04 21:43:07.749675+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30585	2cf183d9	86	2026-11-04 21:44:07.749675+00	"StrategicActionCommit"
30586	2cf183d9	87	2026-11-04 21:45:07.749675+00	{"EndTurn": {"player": "Bob"}}
30587	2cf183d9	88	2026-11-04 21:46:07.749675+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30588	2cf183d9	89	2026-11-04 21:47:07.749675+00	"StrategicActionCommit"
30589	2cf183d9	90	2026-11-04 21:48:07.749675+00	{"EndTurn": {"player": "Carol"}}
30590	2cf183d9	91	2026-11-04 21:49:07.749675+00	{"PassAction": {"player": "Alice"}}
30591	2cf183d9	92	2026-11-04 21:50:07.749675+00	{"EndTurn": {"player": "Alice"}}
30592	2cf183d9	93	2026-11-04 21:51:07.749675+00	{"PassAction": {"player": "Bob"}}
30593	2cf183d9	94	2026-11-04 21:52:07.749675+00	{"EndTurn": {"player": "Bob"}}
30594	2cf183d9	95	2026-11-04 21:53:07.749675+00	{"PassAction": {"player": "Carol"}}
30595	2cf183d9	96	2026-11-04 21:54:07.749675+00	{"EndTurn": {"player": "Carol"}}
30596	2cf183d9	97	2026-11-04 21:55:07.749675+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
30597	2cf183d9	98	2026-11-04 21:56:07.749675+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30598	2cf183d9	99	2026-11-04 21:57:07.749675+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30599	2cf183d9	100	2026-11-04 21:58:07.749675+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30600	2cf183d9	101	2026-11-04 21:59:07.749675+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30601	2cf183d9	102	2026-11-04 22:00:07.749675+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30602	2cf183d9	103	2026-11-04 22:01:07.749675+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
30603	2cf183d9	104	2026-11-04 22:02:07.749675+00	"CompleteStatusPhase"
30604	2cf183d9	105	2026-11-04 22:03:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30605	2cf183d9	106	2026-11-04 22:04:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30606	2cf183d9	107	2026-11-04 22:05:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30607	2cf183d9	108	2026-11-04 22:06:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30608	2cf183d9	109	2026-11-04 22:07:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30609	2cf183d9	110	2026-11-04 22:08:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30610	2cf183d9	111	2026-11-04 22:09:07.749675+00	"CompleteStrategyPhase"
30611	2cf183d9	112	2026-11-04 22:10:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30612	2cf183d9	113	2026-11-04 22:11:07.749675+00	"StrategicActionCommit"
30613	2cf183d9	114	2026-11-04 22:12:07.749675+00	{"EndTurn": {"player": "Alice"}}
30614	b645aaf4	0	2026-11-04 19:52:07.749675+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
30615	b645aaf4	1	2026-11-04 19:53:07.749675+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
30616	b645aaf4	2	2026-11-04 19:54:07.749675+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30617	b645aaf4	3	2026-11-04 19:55:07.749675+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
30618	b645aaf4	4	2026-11-04 19:56:07.749675+00	{"SetGalacticEvents": {"events": ["MonumentsToTheAges"]}}
30619	b645aaf4	5	2026-11-04 19:57:07.749675+00	"CreationDone"
30620	b645aaf4	6	2026-11-04 19:58:07.749675+00	{"SetupSpeaker": {"player": "Alice"}}
30621	b645aaf4	7	2026-11-04 19:59:07.749675+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30622	b645aaf4	8	2026-11-04 20:00:07.749675+00	"StartGame"
30623	b645aaf4	9	2026-11-04 20:01:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30624	b645aaf4	10	2026-11-04 20:02:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30625	b645aaf4	11	2026-11-04 20:03:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30626	b645aaf4	12	2026-11-04 20:04:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30627	b645aaf4	13	2026-11-04 20:05:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30628	b645aaf4	14	2026-11-04 20:06:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30629	b645aaf4	15	2026-11-04 20:07:07.749675+00	"CompleteStrategyPhase"
30630	b645aaf4	16	2026-11-04 20:08:07.749675+00	{"SetPlanetOwner": {"planet": "ThundersEdge", "player": "Alice"}}
30631	b645aaf4	17	2026-11-04 20:09:07.749675+00	{"SetPlanetOwner": {"planet": "ThundersEdge", "player": null}}
30632	b645aaf4	18	2026-11-04 20:10:07.749675+00	{"TacticalActionBegin": {"player": "Alice"}}
28306	9a5c9ddc	24	2026-10-30 12:58:23.292805+00	"StrategicActionCommit"
30633	b645aaf4	19	2026-11-04 20:11:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Alice"}}
30634	b645aaf4	20	2026-11-04 20:12:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Fria", "player": "Alice"}}
30635	b645aaf4	21	2026-11-04 20:13:07.749675+00	{"TacticalActionCommit": {"player": "Alice"}}
30636	b645aaf4	22	2026-11-04 20:14:07.749675+00	{"EndTurn": {"player": "Alice"}}
30637	b645aaf4	23	2026-11-04 20:15:07.749675+00	{"TacticalActionBegin": {"player": "Bob"}}
30638	b645aaf4	24	2026-11-04 20:16:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Arinam", "player": "Bob"}}
30639	b645aaf4	25	2026-11-04 20:17:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Meer", "player": "Bob"}}
30640	b645aaf4	26	2026-11-04 20:18:07.749675+00	{"TacticalActionCommit": {"player": "Bob"}}
30641	b645aaf4	27	2026-11-04 20:19:07.749675+00	{"EndTurn": {"player": "Bob"}}
30642	b645aaf4	28	2026-11-04 20:20:07.749675+00	{"BuildMonument": {"planet": "Abyz", "player": "Alice"}}
30643	b645aaf4	29	2026-11-04 20:21:07.749675+00	{"BuildMonument": {"planet": "Fria", "player": "Alice"}}
30644	b645aaf4	30	2026-11-04 20:22:07.749675+00	{"BuildMonument": {"planet": "Arinam", "player": "Bob"}}
30645	b645aaf4	31	2026-11-04 20:23:07.749675+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30646	b645aaf4	32	2026-11-04 20:24:07.749675+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30647	b645aaf4	33	2026-11-04 20:25:07.749675+00	"StrategicActionCommit"
30648	b645aaf4	34	2026-11-04 20:26:07.749675+00	{"EndTurn": {"player": "Carol"}}
30649	b645aaf4	35	2026-11-04 20:27:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30650	b645aaf4	36	2026-11-04 20:28:07.749675+00	"StrategicActionCommit"
30651	b645aaf4	37	2026-11-04 20:29:07.749675+00	{"EndTurn": {"player": "Alice"}}
30652	b645aaf4	38	2026-11-04 20:30:07.749675+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30653	b645aaf4	39	2026-11-04 20:31:07.749675+00	"StrategicActionCommit"
30654	b645aaf4	40	2026-11-04 20:32:07.749675+00	{"EndTurn": {"player": "Bob"}}
30655	b645aaf4	41	2026-11-04 20:33:07.749675+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30656	b645aaf4	42	2026-11-04 20:34:07.749675+00	"StrategicActionCommit"
30657	b645aaf4	43	2026-11-04 20:35:07.749675+00	{"EndTurn": {"player": "Carol"}}
30658	b645aaf4	44	2026-11-04 20:36:07.749675+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30659	b645aaf4	45	2026-11-04 20:37:07.749675+00	"StrategicActionCommit"
30660	b645aaf4	46	2026-11-04 20:38:07.749675+00	{"EndTurn": {"player": "Alice"}}
30661	b645aaf4	47	2026-11-04 20:39:07.749675+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30662	b645aaf4	48	2026-11-04 20:40:07.749675+00	"StrategicActionCommit"
30663	b645aaf4	49	2026-11-04 20:41:07.749675+00	{"EndTurn": {"player": "Bob"}}
30664	b645aaf4	50	2026-11-04 20:42:07.749675+00	{"PassAction": {"player": "Carol"}}
30665	b645aaf4	51	2026-11-04 20:43:07.749675+00	{"EndTurn": {"player": "Carol"}}
30666	b645aaf4	52	2026-11-04 20:44:07.749675+00	{"PassAction": {"player": "Alice"}}
30667	b645aaf4	53	2026-11-04 20:45:07.749675+00	{"EndTurn": {"player": "Alice"}}
30668	b645aaf4	54	2026-11-04 20:46:07.749675+00	{"PassAction": {"player": "Bob"}}
30669	b645aaf4	55	2026-11-04 20:47:07.749675+00	{"EndTurn": {"player": "Bob"}}
30670	b645aaf4	56	2026-11-04 20:48:07.749675+00	{"SetMonumentCommodities": {"planet": "Fria", "commodities": 0}}
30671	b645aaf4	57	2026-11-04 20:49:07.749675+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
30672	b645aaf4	58	2026-11-04 20:50:07.749675+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30673	b645aaf4	59	2026-11-04 20:51:07.749675+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30674	b645aaf4	60	2026-11-04 20:52:07.749675+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30675	b645aaf4	61	2026-11-04 20:53:07.749675+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30676	b645aaf4	62	2026-11-04 20:54:07.749675+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30677	b645aaf4	63	2026-11-04 20:55:07.749675+00	{"RevealPublicObjective": {"objective": "DiversifyResearch"}}
30678	b645aaf4	64	2026-11-04 20:56:07.749675+00	"CompleteStatusPhase"
30679	b645aaf4	65	2026-11-04 20:57:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30680	b645aaf4	66	2026-11-04 20:58:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30681	b645aaf4	67	2026-11-04 20:59:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30682	b645aaf4	68	2026-11-04 21:00:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30683	b645aaf4	69	2026-11-04 21:01:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30684	b645aaf4	70	2026-11-04 21:02:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30685	b645aaf4	71	2026-11-04 21:03:07.749675+00	"CompleteStrategyPhase"
30686	b645aaf4	72	2026-11-04 21:04:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30687	b645aaf4	73	2026-11-04 21:05:07.749675+00	"StrategicActionCommit"
30688	b645aaf4	74	2026-11-04 21:06:07.749675+00	{"EndTurn": {"player": "Alice"}}
30689	b645aaf4	75	2026-11-04 21:07:07.749675+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30690	b645aaf4	76	2026-11-04 21:08:07.749675+00	"StrategicActionCommit"
30691	b645aaf4	77	2026-11-04 21:09:07.749675+00	{"EndTurn": {"player": "Bob"}}
30692	b645aaf4	78	2026-11-04 21:10:07.749675+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30693	b645aaf4	79	2026-11-04 21:11:07.749675+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30694	b645aaf4	80	2026-11-04 21:12:07.749675+00	"StrategicActionCommit"
30695	b645aaf4	81	2026-11-04 21:13:07.749675+00	{"EndTurn": {"player": "Carol"}}
30696	b645aaf4	82	2026-11-04 21:14:07.749675+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30697	b645aaf4	83	2026-11-04 21:15:07.749675+00	"StrategicActionCommit"
30698	b645aaf4	84	2026-11-04 21:16:07.749675+00	{"EndTurn": {"player": "Alice"}}
30699	b645aaf4	85	2026-11-04 21:17:07.749675+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30700	b645aaf4	86	2026-11-04 21:18:07.749675+00	"StrategicActionCommit"
30701	b645aaf4	87	2026-11-04 21:19:07.749675+00	{"EndTurn": {"player": "Bob"}}
30702	b645aaf4	88	2026-11-04 21:20:07.749675+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30703	b645aaf4	89	2026-11-04 21:21:07.749675+00	"StrategicActionCommit"
30704	b645aaf4	90	2026-11-04 21:22:07.749675+00	{"EndTurn": {"player": "Carol"}}
30705	b645aaf4	91	2026-11-04 21:23:07.749675+00	{"PassAction": {"player": "Alice"}}
30706	b645aaf4	92	2026-11-04 21:24:07.749675+00	{"EndTurn": {"player": "Alice"}}
30707	b645aaf4	93	2026-11-04 21:25:07.749675+00	{"PassAction": {"player": "Bob"}}
30708	b645aaf4	94	2026-11-04 21:26:07.749675+00	{"EndTurn": {"player": "Bob"}}
30709	b645aaf4	95	2026-11-04 21:27:07.749675+00	{"PassAction": {"player": "Carol"}}
30710	b645aaf4	96	2026-11-04 21:28:07.749675+00	{"EndTurn": {"player": "Carol"}}
30711	b645aaf4	97	2026-11-04 21:29:07.749675+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
30712	b645aaf4	98	2026-11-04 21:30:07.749675+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
30713	b645aaf4	99	2026-11-04 21:31:07.749675+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
30714	b645aaf4	100	2026-11-04 21:32:07.749675+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
30715	b645aaf4	101	2026-11-04 21:33:07.749675+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
30716	b645aaf4	102	2026-11-04 21:34:07.749675+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
30717	b645aaf4	103	2026-11-04 21:35:07.749675+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
30718	b645aaf4	104	2026-11-04 21:36:07.749675+00	"CompleteStatusPhase"
30719	b645aaf4	105	2026-11-04 21:37:07.749675+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30720	b645aaf4	106	2026-11-04 21:38:07.749675+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30721	b645aaf4	107	2026-11-04 21:39:07.749675+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30722	b645aaf4	108	2026-11-04 21:40:07.749675+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30723	b645aaf4	109	2026-11-04 21:41:07.749675+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30724	b645aaf4	110	2026-11-04 21:42:07.749675+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30725	b645aaf4	111	2026-11-04 21:43:07.749675+00	"CompleteStrategyPhase"
30726	b645aaf4	112	2026-11-04 21:44:07.749675+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
30727	b645aaf4	113	2026-11-04 21:45:07.749675+00	"StrategicActionCommit"
30728	b645aaf4	114	2026-11-04 21:46:07.749675+00	{"EndTurn": {"player": "Alice"}}
30729	b645aaf4	115	2026-11-04 21:47:07.749675+00	{"TacticalActionBegin": {"player": "Bob"}}
30730	b645aaf4	116	2026-11-04 21:48:07.749675+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Bob"}}
30731	b645aaf4	117	2026-11-04 21:49:07.749675+00	{"TacticalActionCommit": {"player": "Bob"}}
30732	b645aaf4	118	2026-11-04 21:50:07.749675+00	{"EndTurn": {"player": "Bob"}}
30733	b645aaf4	119	2026-11-04 21:51:07.749675+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
30734	b645aaf4	120	2026-11-04 21:52:07.749675+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
30735	b645aaf4	121	2026-11-04 21:53:07.749675+00	"StrategicActionCommit"
30736	b645aaf4	122	2026-11-04 21:54:07.749675+00	{"EndTurn": {"player": "Carol"}}
30737	b645aaf4	123	2026-11-04 21:55:07.749675+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
30738	b645aaf4	124	2026-11-04 21:56:07.749675+00	"StrategicActionCommit"
30739	b645aaf4	125	2026-11-04 21:57:07.749675+00	{"EndTurn": {"player": "Alice"}}
30740	b645aaf4	126	2026-11-04 21:58:07.749675+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
30741	b645aaf4	127	2026-11-04 21:59:07.749675+00	"StrategicActionCommit"
30742	b645aaf4	128	2026-11-04 22:00:07.749675+00	{"EndTurn": {"player": "Bob"}}
30743	b645aaf4	129	2026-11-04 22:01:07.749675+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
30744	b645aaf4	130	2026-11-04 22:02:07.749675+00	"StrategicActionCommit"
30745	b645aaf4	131	2026-11-04 22:03:07.749675+00	{"EndTurn": {"player": "Carol"}}
30746	b645aaf4	132	2026-11-04 22:04:07.749675+00	{"PassAction": {"player": "Alice"}}
30747	b645aaf4	133	2026-11-04 22:05:07.749675+00	{"EndTurn": {"player": "Alice"}}
30748	b645aaf4	134	2026-11-04 22:06:07.749675+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
30749	b645aaf4	135	2026-11-04 22:07:07.749675+00	"StrategicActionCommit"
30750	b645aaf4	136	2026-11-04 22:08:07.749675+00	{"EndTurn": {"player": "Bob"}}
30751	b645aaf4	137	2026-11-04 22:09:07.749675+00	{"PassAction": {"player": "Carol"}}
30752	b645aaf4	138	2026-11-04 22:10:07.749675+00	{"EndTurn": {"player": "Carol"}}
30753	b645aaf4	139	2026-11-04 22:11:07.749675+00	{"PassAction": {"player": "Bob"}}
30754	b645aaf4	140	2026-11-04 22:12:07.749675+00	{"EndTurn": {"player": "Bob"}}
31482	09d3557e	10	2026-11-05 23:12:00.73577+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31483	09d3557e	11	2026-11-05 23:12:00.742484+00	{"SetupSpeaker": {"player": "Tux"}}
31484	09d3557e	12	2026-11-05 23:12:00.749495+00	{"SetupMinorFaction": {"player": "Tux", "faction": "Arborec"}}
31485	09d3557e	13	2026-11-05 23:12:00.755444+00	{"SetupMinorFaction": {"player": "Vidde", "faction": "Winnu"}}
31486	09d3557e	14	2026-11-05 23:12:00.761084+00	{"SetupMinorFaction": {"player": "Yoda", "faction": "YinBrotherhood"}}
31487	09d3557e	15	2026-11-05 23:12:00.766926+00	"StartGame"
31488	09d3557e	16	2026-11-05 23:12:00.773604+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31489	09d3557e	17	2026-11-05 23:12:00.779442+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31490	09d3557e	18	2026-11-05 23:12:00.785055+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31491	09d3557e	19	2026-11-05 23:12:00.790973+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31492	09d3557e	20	2026-11-05 23:12:00.797381+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
30755	2cf183d9	115	2026-11-04 22:14:55.579507+00	{"TacticalActionBegin": {"player": "Bob"}}
30756	2cf183d9	116	2026-11-04 22:14:57.632613+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "Bob"}}
30757	2cf183d9	117	2026-11-04 22:15:03.050212+00	{"DestroyMonument": {"planet": "Abyz"}}
30758	2cf183d9	118	2026-11-04 22:15:12.722355+00	{"BuildMonument": {"planet": "Abyz", "player": "Bob"}}
30759	b645aaf4	141	2026-11-04 22:15:35.06505+00	{"SetMonumentCommodities": {"planet": "Fria", "commodities": 3}}
30760	b645aaf4	142	2026-11-04 22:15:36.295094+00	{"DestroyMonument": {"planet": "Abyz"}}
31493	09d3557e	21	2026-11-05 23:12:00.802793+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31494	09d3557e	22	2026-11-05 23:12:00.808472+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31495	09d3557e	23	2026-11-05 23:12:00.815004+00	"CompleteStrategyPhase"
30761	b645aaf4	143	2026-11-04 22:18:15.706189+00	{"SetMonumentCommodities": {"planet": "Fria", "commodities": 2}}
30762	b645aaf4	144	2026-11-04 22:26:02.13858+00	{"SetMonumentCommodities": {"planet": "Fria", "commodities": 3}}
30800	7a9d0b11	0	2025-01-01 12:00:00+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": true}}}}
30801	7a9d0b11	1	2025-01-01 12:01:00+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "Nomad"}}}
30802	7a9d0b11	2	2025-01-01 12:02:00+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "NekroVirus"}}}
30803	7a9d0b11	3	2025-01-01 12:03:00+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "EmbersOfMuaat"}}}
30804	7a9d0b11	4	2025-01-01 12:04:00+00	{"SetGalacticEvents": {"events": ["RapidMobilization"]}}
30805	7a9d0b11	5	2025-01-01 12:05:00+00	"CreationDone"
30806	7a9d0b11	6	2025-01-01 12:06:00+00	{"SetupSpeaker": {"player": "Alice"}}
30807	7a9d0b11	7	2025-01-01 12:07:00+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30808	7a9d0b11	8	2025-01-01 12:08:00+00	"StartGame"
27523	e15c9638	0	2026-10-30 13:21:19.787818+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27524	e15c9638	1	2026-10-30 13:22:19.787818+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27525	e15c9638	2	2026-10-30 13:23:19.787818+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
30763	0e9e7177	0	2026-11-04 23:10:14.779061+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30764	0e9e7177	1	2026-11-04 23:10:23.024751+00	{"AddPlayer": {"player": {"name": "AAA", "color": "Green", "faction": "Arborec"}}}
30765	0e9e7177	2	2026-11-04 23:10:28.504029+00	{"AddPlayer": {"player": {"name": "BBB", "color": "Pink", "faction": "EmiratesOfHacan"}}}
30766	0e9e7177	3	2026-11-04 23:10:33.520877+00	{"AddPlayer": {"player": {"name": "CCC", "color": "Red", "faction": "EmbersOfMuaat"}}}
30767	0e9e7177	4	2026-11-04 23:10:39.380742+00	{"SetGalacticEvents": {"events": ["CulturalExchangeProgram"]}}
30775	0e9e7177	5	2026-11-04 23:11:43.27268+00	{"SetGalacticEvents": {"events": []}}
30776	0e9e7177	6	2026-11-04 23:11:45.809095+00	{"SetGalacticEvents": {"events": ["CivilizedSociety"]}}
30777	0e9e7177	7	2026-11-04 23:11:47.668371+00	"CreationDone"
30778	0e9e7177	8	2026-11-04 23:11:49.217+00	{"SetupSpeaker": {"player": "AAA"}}
31496	f1a70001	0	2026-11-06 09:17:23.686196+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31497	f1a70001	1	2026-11-06 09:17:23.686196+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
31498	f1a70001	2	2026-11-06 09:17:23.686196+00	"CreationDone"
31499	f1a70001	3	2026-11-06 09:17:23.686196+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31500	f1a70001	4	2026-11-06 09:17:23.686196+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31501	f1a70001	5	2026-11-06 09:17:23.686196+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31502	f1a70001	6	2026-11-06 09:17:23.686196+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31503	f1a70001	7	2026-11-06 09:17:23.686196+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
30781	b6e8aaef	0	2026-11-04 23:24:15.84587+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "74", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "113", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "100", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "39", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "102", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "36", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "78", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "59", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "34", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "32", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "68", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "47", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "49", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "50", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 0}}}, {"system": "64", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "24", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "35", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "80", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 0}}}, {"system": "107", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Ann": {"name": "Ann", "order": 2, "faction": "Arborec"}, "Ben": {"name": "Ben", "order": 0, "faction": "NaazRokhaAlliance"}, "Cid": {"name": "Cid", "order": 1, "faction": "RalNelConsortium"}, "Dee": {"name": "Dee", "order": 3, "faction": "DeepwroughtScholarate"}, "Eve": {"name": "Eve", "order": 4, "faction": "FirmamentObsidian"}}, "game_name": "Scenario", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30782	b6e8aaef	1	2026-11-04 23:25:15.84587+00	{"SetGalacticEvents": {"events": ["AgeOfExploration"]}}
30783	b6e8aaef	2	2026-11-04 23:26:15.84587+00	"CreationDone"
30784	b6e8aaef	3	2026-11-04 23:27:15.84587+00	{"SetupPlayerTechs": {"player": "Cid", "technologies": ["NeuralMotivator"]}}
30785	b6e8aaef	4	2026-11-04 23:28:15.84587+00	{"SetupPlayerTechs": {"player": "Dee", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
30786	b6e8aaef	5	2026-11-04 23:29:15.84587+00	{"SetupPlayerTechs": {"player": "Eve", "technologies": ["NeuralMotivator"]}}
30787	b6e8aaef	6	2026-11-04 23:30:15.84587+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
30788	b6e8aaef	7	2026-11-04 23:31:15.84587+00	"StartGame"
30789	b6e8aaef	8	2026-11-04 23:32:15.84587+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Ben"}}
30790	b6e8aaef	9	2026-11-04 23:33:15.84587+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Cid"}}
30791	b6e8aaef	10	2026-11-04 23:34:15.84587+00	{"TakeStrategyCard": {"card": "Politics", "player": "Ann"}}
30792	b6e8aaef	11	2026-11-04 23:35:15.84587+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dee"}}
30793	b6e8aaef	12	2026-11-04 23:36:15.84587+00	{"TakeStrategyCard": {"card": "Trade", "player": "Eve"}}
30812	66a56181	0	2026-11-05 13:23:59.327775+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
30813	66a56181	1	2026-11-05 13:24:09.037717+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Black", "faction": "FederationOfSol"}}}
30814	66a56181	2	2026-11-05 13:24:09.632522+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Black", "faction": "EmiratesOfHacan"}}}
30815	66a56181	3	2026-11-05 13:24:10.232828+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Blue", "faction": "XxchaKingdom"}}}
30816	66a56181	4	2026-11-05 13:24:10.784479+00	{"SetGalacticEvents": {"events": ["MinorFactions"]}}
30817	66a56181	5	2026-11-05 13:24:17.916164+00	"CreationDone"
30818	66a56181	6	2026-11-05 13:24:28.746832+00	{"SetupMinorFaction": {"player": "Alice", "faction": "MentakCoalition"}}
30819	66a56181	7	2026-11-05 13:24:38.423028+00	{"SetupSpeaker": {"player": "Alice"}}
30820	66a56181	8	2026-11-05 13:24:38.886147+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "ErectAMonument"}}
30821	66a56181	9	2026-11-05 13:24:39.39913+00	{"SetupMinorFaction": {"player": "Bob", "faction": "NaaluCollective"}}
30822	66a56181	10	2026-11-05 13:24:57.818003+00	{"SetupMinorFaction": {"player": "Carol", "faction": "Winnu"}}
30823	66a56181	11	2026-11-05 13:24:58.346202+00	"StartGame"
30824	66a56181	12	2026-11-05 13:25:06.840108+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
30825	66a56181	13	2026-11-05 13:25:07.256025+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
30826	66a56181	14	2026-11-05 13:25:07.672897+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
30827	66a56181	15	2026-11-05 13:25:08.095191+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
30828	66a56181	16	2026-11-05 13:25:08.511926+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
30829	66a56181	17	2026-11-05 13:25:08.927877+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
30830	66a56181	18	2026-11-05 13:25:09.350865+00	"CompleteStrategyPhase"
30831	66a56181	19	2026-11-05 13:25:16.027602+00	{"TacticalActionBegin": {"player": "Alice"}}
30832	66a56181	20	2026-11-05 13:25:28.2243+00	{"TacticalActionTakePlanet": {"planet": "MollPrimus", "player": "Alice"}}
30833	66a56181	21	2026-11-05 13:25:43.615461+00	{"TacticalActionCommit": {"player": "Alice"}}
31512	099a8a18	17	2026-11-06 23:08:08.65224+00	{"PlayGalacticEventAction": {"action": "TotalWar", "player": "Tux"}}
30798	b6e8aaef	13	2026-11-05 00:05:02.407295+00	"CompleteStrategyPhase"
30799	b6e8aaef	14	2026-11-05 00:05:05.116298+00	{"AddTechToPlayer": {"tech": "DarkEnergyTap", "player": "Ben"}}
30878	55f31401	0	2026-11-05 21:18:44.87523+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30879	d4bd829a	0	2026-11-05 21:19:27.873608+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30880	d4bd829a	1	2026-11-05 21:19:39.059151+00	"CreationDone"
30881	d4bd829a	2	2026-11-05 21:19:41.393698+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
30882	d4bd829a	3	2026-11-05 21:19:43.566614+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
30883	d4bd829a	4	2026-11-05 21:19:45.14097+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
30884	d4bd829a	5	2026-11-05 21:19:48.534823+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
30885	d4bd829a	6	2026-11-05 21:19:53.848702+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
30886	d4bd829a	7	2026-11-05 21:19:55.422163+00	"StartGame"
30887	d4bd829a	8	2026-11-05 21:19:57.586241+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
30888	d4bd829a	9	2026-11-05 21:19:58.970396+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
30889	d4bd829a	10	2026-11-05 21:20:02.152322+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
30890	d4bd829a	11	2026-11-05 21:20:02.911662+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
30891	d4bd829a	12	2026-11-05 21:20:03.592068+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
30892	d4bd829a	13	2026-11-05 21:20:05.062046+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
30893	d4bd829a	14	2026-11-05 21:20:07.621758+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
30894	d4bd829a	15	2026-11-05 21:20:08.687089+00	"CompleteStrategyPhase"
30895	d4bd829a	16	2026-11-05 21:20:10.714977+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Tux"}}
30896	d4bd829a	17	2026-11-05 21:20:12.149082+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
30897	d4bd829a	18	2026-11-05 21:20:13.251552+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Yoda"}}
30898	d4bd829a	19	2026-11-05 21:20:16.797319+00	"StrategicActionCommit"
30899	d4bd829a	20	2026-11-05 21:20:18.276662+00	{"EndTurn": {"player": "Tux"}}
30900	d4bd829a	21	2026-11-05 21:20:19.518772+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}
30901	d4bd829a	22	2026-11-05 21:20:20.715373+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
30902	d4bd829a	23	2026-11-05 21:20:21.42475+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Yoda"}}
30903	d4bd829a	24	2026-11-05 21:20:22.464521+00	"StrategicActionCommit"
30904	d4bd829a	25	2026-11-05 21:20:23.719596+00	{"EndTurn": {"player": "Vidde"}}
30905	d4bd829a	26	2026-11-05 21:20:25.012942+00	{"StrategicActionBegin": {"card": "Politics", "player": "Yoda"}}
30906	d4bd829a	27	2026-11-05 21:20:27.791629+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Yoda"}}, "player": "Yoda"}}
30907	d4bd829a	28	2026-11-05 21:20:28.859916+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
30908	d4bd829a	29	2026-11-05 21:20:29.505564+00	{"StrategicActionSecondary": {"action": "Politics", "player": "Vidde"}}
30909	d4bd829a	30	2026-11-05 21:20:30.080579+00	"StrategicActionCommit"
30910	d4bd829a	31	2026-11-05 21:20:31.672702+00	{"EndTurn": {"player": "Yoda"}}
30911	d4bd829a	32	2026-11-05 21:20:32.573812+00	{"StrategicActionBegin": {"card": "Construction", "player": "Tux"}}
30912	d4bd829a	33	2026-11-05 21:20:33.685204+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
30913	d4bd829a	34	2026-11-05 21:20:34.449202+00	{"StrategicActionSecondary": {"action": "Construction", "player": "Yoda"}}
30914	d4bd829a	35	2026-11-05 21:20:35.081459+00	"StrategicActionCommit"
30915	d4bd829a	36	2026-11-05 21:20:36.636296+00	{"EndTurn": {"player": "Tux"}}
30916	d4bd829a	37	2026-11-05 21:20:37.950815+00	{"StrategicActionBegin": {"card": "Trade", "player": "Vidde"}}
30917	d4bd829a	38	2026-11-05 21:20:39.272862+00	{"StrategicActionSecondary": {"action": "Trade", "player": "Tux"}}
30918	d4bd829a	39	2026-11-05 21:20:39.86009+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Yoda"}}
30919	d4bd829a	40	2026-11-05 21:20:40.464449+00	"StrategicActionCommit"
30920	d4bd829a	41	2026-11-05 21:20:41.942314+00	{"EndTurn": {"player": "Vidde"}}
30921	d4bd829a	42	2026-11-05 21:20:42.975933+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Yoda"}}
30922	d4bd829a	43	2026-11-05 21:20:44.775996+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Tux"}}
30923	d4bd829a	44	2026-11-05 21:20:45.400855+00	{"StrategicActionSecondary": {"action": "Warfare", "player": "Vidde"}}
30924	d4bd829a	45	2026-11-05 21:20:46.255593+00	"StrategicActionCommit"
30925	d4bd829a	46	2026-11-05 21:20:47.505541+00	{"EndTurn": {"player": "Yoda"}}
30926	d4bd829a	47	2026-11-05 21:20:48.69185+00	{"PassAction": {"player": "Tux"}}
30927	d4bd829a	48	2026-11-05 21:20:49.760977+00	{"EndTurn": {"player": "Tux"}}
30928	d4bd829a	49	2026-11-05 21:20:50.65187+00	{"PassAction": {"player": "Vidde"}}
30929	d4bd829a	50	2026-11-05 21:20:51.501704+00	{"EndTurn": {"player": "Vidde"}}
30930	d4bd829a	51	2026-11-05 21:20:52.30085+00	{"PassAction": {"player": "Yoda"}}
30931	d4bd829a	52	2026-11-05 21:20:53.161676+00	{"EndTurn": {"player": "Yoda"}}
30932	d4bd829a	53	2026-11-05 21:21:02.125525+00	{"ScorePublicObjective": {"player": "Tux", "objective": "BuildDefenses"}}
30933	d4bd829a	54	2026-11-05 21:21:05.579053+00	{"ScoreSecretObjective": {"player": "Tux", "objective": null}}
30934	0806fe00	0	2026-11-05 21:22:30.659634+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
30935	0806fe00	1	2026-11-05 21:22:36.324869+00	{"AddPlayer": {"player": {"name": "AAAA", "color": "Green", "faction": "Arborec"}}}
30936	0806fe00	2	2026-11-05 21:22:41.266647+00	{"AddPlayer": {"player": {"name": "BBB", "color": "Pink", "faction": "ClanOfSaar"}}}
30937	0806fe00	3	2026-11-05 21:22:47.635153+00	{"AddPlayer": {"player": {"name": "CCC", "color": "Blue", "faction": "FederationOfSol"}}}
30938	0806fe00	4	2026-11-05 21:22:50.389699+00	"CreationDone"
30939	0806fe00	5	2026-11-05 21:22:52.149577+00	{"SetupSpeaker": {"player": "AAAA"}}
30940	0806fe00	6	2026-11-05 21:22:54.845944+00	{"RevealInitialObjectives": {"firstObjective": "AmassWealth", "secondObjective": "BuildDefenses"}}
30941	0806fe00	7	2026-11-05 21:22:55.595207+00	"StartGame"
30942	0806fe00	8	2026-11-05 21:22:57.22813+00	{"TakeStrategyCard": {"card": "Leadership", "player": "AAAA"}}
30943	0806fe00	9	2026-11-05 21:22:58.509914+00	{"TakeStrategyCard": {"card": "Politics", "player": "BBB"}}
30944	0806fe00	10	2026-11-05 21:23:00.583883+00	{"TakeStrategyCard": {"card": "Trade", "player": "CCC"}}
30945	0806fe00	11	2026-11-05 21:23:01.828263+00	{"TakeStrategyCard": {"card": "Technology", "player": "AAAA"}}
30946	0806fe00	12	2026-11-05 21:23:02.804697+00	{"TakeStrategyCard": {"card": "Imperial", "player": "BBB"}}
30947	0806fe00	13	2026-11-05 21:23:03.692945+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "CCC"}}
30948	0806fe00	14	2026-11-05 21:23:07.805057+00	"CompleteStrategyPhase"
30949	0806fe00	15	2026-11-05 21:23:10.945266+00	{"StrategicActionBegin": {"card": "Leadership", "player": "AAAA"}}
30950	0806fe00	16	2026-11-05 21:23:14.749359+00	{"StrategicActionSecondary": {"action": "Skip", "player": "BBB"}}
30951	0806fe00	17	2026-11-05 21:23:16.750294+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "CCC"}}
30952	0806fe00	18	2026-11-05 21:23:17.316242+00	"StrategicActionCommit"
30953	0806fe00	19	2026-11-05 21:23:19.132086+00	{"CommitToThundersEdgeExpedition": {"slot": "SecretObjective", "player": "AAAA"}}
30954	0806fe00	20	2026-11-05 21:23:20.888752+00	{"RollForTheFracture": {"player": "AAAA", "entered_play": false}}
30955	0806fe00	21	2026-11-05 21:23:25.68553+00	{"EndTurn": {"player": "AAAA"}}
30956	0806fe00	22	2026-11-05 21:23:27.792816+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "CCC"}}
30957	0806fe00	23	2026-11-05 21:23:28.619386+00	{"StrategicActionSecondary": {"action": "Skip", "player": "AAAA"}}
30958	0806fe00	24	2026-11-05 21:23:29.250577+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "BBB"}}
30959	0806fe00	25	2026-11-05 21:23:30.031855+00	"StrategicActionCommit"
30960	0806fe00	26	2026-11-05 21:23:31.7713+00	{"EndTurn": {"player": "CCC"}}
30961	0806fe00	27	2026-11-05 21:23:32.712603+00	{"StrategicActionBegin": {"card": "Politics", "player": "BBB"}}
30962	0806fe00	28	2026-11-05 21:23:35.390882+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "CCC"}}, "player": "BBB"}}
30963	0806fe00	29	2026-11-05 21:23:36.197558+00	{"StrategicActionSecondary": {"action": "Skip", "player": "AAAA"}}
30964	0806fe00	30	2026-11-05 21:23:36.869294+00	{"StrategicActionSecondary": {"action": "Politics", "player": "CCC"}}
30965	0806fe00	31	2026-11-05 21:23:37.648156+00	"StrategicActionCommit"
30966	0806fe00	32	2026-11-05 21:23:39.234526+00	{"EndTurn": {"player": "BBB"}}
30967	0806fe00	33	2026-11-05 21:23:40.254458+00	{"StrategicActionBegin": {"card": "Technology", "player": "AAAA"}}
30968	0806fe00	34	2026-11-05 21:23:44.314012+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": "Bioplasmosis", "second_tech": null}}, "player": "AAAA"}}
30969	0806fe00	35	2026-11-05 21:23:47.761207+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "HyperMetabolism"}}, "player": "BBB"}}
30970	0806fe00	36	2026-11-05 21:23:53.315859+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "LightWaveDeflector"}}, "player": "CCC"}}
30971	0806fe00	37	2026-11-05 21:23:54.917651+00	"StrategicActionCommit"
30972	0806fe00	38	2026-11-05 21:23:56.576218+00	{"EndTurn": {"player": "AAAA"}}
30973	0806fe00	39	2026-11-05 21:23:58.094085+00	{"StrategicActionBegin": {"card": "Trade", "player": "CCC"}}
30974	0806fe00	40	2026-11-05 21:23:59.071805+00	{"StrategicActionSecondary": {"action": "Skip", "player": "AAAA"}}
30975	0806fe00	41	2026-11-05 21:23:59.741128+00	{"StrategicActionSecondary": {"action": "Trade", "player": "BBB"}}
30976	0806fe00	42	2026-11-05 21:24:00.295913+00	"StrategicActionCommit"
30977	0806fe00	43	2026-11-05 21:24:01.835003+00	{"EndTurn": {"player": "CCC"}}
30978	0806fe00	44	2026-11-05 21:24:02.941199+00	{"StrategicActionBegin": {"card": "Imperial", "player": "BBB"}}
30979	0806fe00	45	2026-11-05 21:24:05.882709+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "AmassWealth"}}, "player": "BBB"}}
30980	0806fe00	46	2026-11-05 21:24:06.680579+00	{"StrategicActionSecondary": {"action": "Skip", "player": "AAAA"}}
30981	0806fe00	47	2026-11-05 21:24:07.227466+00	{"StrategicActionSecondary": {"action": "Imperial", "player": "CCC"}}
30982	0806fe00	48	2026-11-05 21:24:07.832254+00	"StrategicActionCommit"
30983	0806fe00	49	2026-11-05 21:24:09.718209+00	{"EndTurn": {"player": "BBB"}}
30984	0806fe00	50	2026-11-05 21:24:11.507643+00	{"TacticalActionBegin": {"player": "AAAA"}}
30985	0806fe00	51	2026-11-05 21:24:13.585168+00	{"TacticalActionTakePlanet": {"planet": "Abyz", "player": "AAAA"}}
30986	0806fe00	52	2026-11-05 21:24:16.280574+00	{"TacticalActionAttachPlanetAttachment": {"planet": "Abyz", "player": "AAAA", "attachment": "MiningWorld"}}
30987	0806fe00	53	2026-11-05 21:24:17.409884+00	{"TacticalActionCommit": {"player": "AAAA"}}
30988	0806fe00	54	2026-11-05 21:24:18.869272+00	{"EndTurn": {"player": "AAAA"}}
30989	0806fe00	55	2026-11-05 21:24:19.752212+00	{"PassAction": {"player": "CCC"}}
30990	0806fe00	56	2026-11-05 21:24:20.757303+00	{"EndTurn": {"player": "CCC"}}
30991	0806fe00	57	2026-11-05 21:24:21.756502+00	{"PassAction": {"player": "BBB"}}
30992	0806fe00	58	2026-11-05 21:24:23.212957+00	{"EndTurn": {"player": "BBB"}}
30993	0806fe00	59	2026-11-05 21:24:24.25522+00	{"PassAction": {"player": "AAAA"}}
30994	0806fe00	60	2026-11-05 21:24:26.049077+00	{"EndTurn": {"player": "AAAA"}}
28548	2f2807a6	30	2026-10-30 12:50:24.387337+00	"StrategicActionCommit"
31056	0806fe00	61	2026-11-05 22:18:26.457354+00	{"GainEntropicScarTech": {"tech": "LetaniWarriorII", "player": "AAAA"}}
31057	099a8a18	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31075	099a8a18	1	2026-11-05 23:10:56.041389+00	{"SetGalacticEvents": {"events": ["TotalWar"]}}
31076	099a8a18	2	2026-11-05 23:10:56.049002+00	"CreationDone"
31077	099a8a18	3	2026-11-05 23:10:56.056602+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31078	099a8a18	4	2026-11-05 23:10:56.062962+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31079	099a8a18	5	2026-11-05 23:10:56.069568+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31080	099a8a18	6	2026-11-05 23:10:56.075543+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31081	099a8a18	7	2026-11-05 23:10:56.08095+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31082	099a8a18	8	2026-11-05 23:10:56.086371+00	"StartGame"
31083	099a8a18	9	2026-11-05 23:10:56.091531+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31084	099a8a18	10	2026-11-05 23:10:56.096248+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31085	099a8a18	11	2026-11-05 23:10:56.10128+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31086	099a8a18	12	2026-11-05 23:10:56.106188+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31087	099a8a18	13	2026-11-05 23:10:56.111168+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31088	099a8a18	14	2026-11-05 23:10:56.115995+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31089	099a8a18	15	2026-11-05 23:10:56.120959+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31090	099a8a18	16	2026-11-05 23:10:56.126768+00	"CompleteStrategyPhase"
31091	100a9fd2	1	2026-11-05 23:11:01.282403+00	{"SetGalacticEvents": {"events": ["MercenariesForHire"]}}
31092	100a9fd2	2	2026-11-05 23:11:01.299274+00	"CreationDone"
31093	100a9fd2	3	2026-11-05 23:11:01.308246+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31094	100a9fd2	4	2026-11-05 23:11:01.315878+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31095	100a9fd2	5	2026-11-05 23:11:01.323777+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31096	100a9fd2	6	2026-11-05 23:11:01.331463+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31097	100a9fd2	7	2026-11-05 23:11:01.338825+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31098	100a9fd2	8	2026-11-05 23:11:01.346737+00	"StartGame"
31099	100a9fd2	9	2026-11-05 23:11:01.354517+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31100	100a9fd2	10	2026-11-05 23:11:01.361976+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31101	100a9fd2	11	2026-11-05 23:11:01.368948+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31102	100a9fd2	12	2026-11-05 23:11:01.374319+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31103	100a9fd2	13	2026-11-05 23:11:01.379505+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31104	100a9fd2	14	2026-11-05 23:11:01.384861+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31105	100a9fd2	15	2026-11-05 23:11:01.391739+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31106	100a9fd2	16	2026-11-05 23:11:01.398486+00	"CompleteStrategyPhase"
31107	20e0b4a1	1	2026-11-05 23:11:01.68537+00	{"SetGalacticEvents": {"events": ["StellarAtomics"]}}
31108	20e0b4a1	2	2026-11-05 23:11:01.701991+00	"CreationDone"
31109	20e0b4a1	3	2026-11-05 23:11:01.709925+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31110	20e0b4a1	4	2026-11-05 23:11:01.718042+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31111	20e0b4a1	5	2026-11-05 23:11:01.726009+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31112	20e0b4a1	6	2026-11-05 23:11:01.733446+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31113	20e0b4a1	7	2026-11-05 23:11:01.740427+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31114	20e0b4a1	8	2026-11-05 23:11:01.747774+00	"StartGame"
31115	20e0b4a1	9	2026-11-05 23:11:01.754601+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31116	20e0b4a1	10	2026-11-05 23:11:01.760323+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31117	20e0b4a1	11	2026-11-05 23:11:01.766504+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31118	20e0b4a1	12	2026-11-05 23:11:01.773221+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31119	20e0b4a1	13	2026-11-05 23:11:01.780897+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31120	20e0b4a1	14	2026-11-05 23:11:01.787755+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31121	20e0b4a1	15	2026-11-05 23:11:01.795839+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31122	20e0b4a1	16	2026-11-05 23:11:01.803896+00	"CompleteStrategyPhase"
31123	942cbdd8	1	2026-11-05 23:11:02.099113+00	{"SetGalacticEvents": {"events": ["StellarAtomics"]}}
31124	942cbdd8	2	2026-11-05 23:11:02.105663+00	"CreationDone"
31058	100a9fd2	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31059	20e0b4a1	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31060	942cbdd8	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31061	ccc97b0b	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31062	dc46c53a	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31063	03d042d0	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31064	d4406362	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31065	63b0393b	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31066	d8bc49b5	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31067	0bc7835e	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31068	185090c7	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31069	1aac2dae	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31070	7d8a64d9	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31071	260cb545	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
27570	69d97f21	11	2026-10-30 13:25:19.99373+00	{"TakeStrategyCard": {"card": "Trade", "player": "Alice"}}
31072	d64c88d1	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31073	551dc1fe	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
31074	adf05cd3	0	2026-11-05 23:06:03.965982+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 4}}}, {"system": "68", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "41", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 2}}}, {"system": "73", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 4}}}, {"system": "20", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 2}}}, {"system": "28", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "21", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 4}}}, {"system": "69", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "87A", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 2}}}, {"system": "77", "position": {"Pos": {"ring": 3, "position": 1, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 2, "rotation": 1}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 4}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 4, "rotation": 2}}}, {"system": "42", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "65", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 8, "rotation": 0}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 10, "rotation": 1}}}, {"system": "67", "position": {"Pos": {"ring": 3, "position": 11, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "31", "position": {"Pos": {"ring": 3, "position": 13, "rotation": 0}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 14, "rotation": 2}}}, {"system": "85A", "position": {"Pos": {"ring": 3, "position": 15, "rotation": 2}}}, {"system": "83A", "position": {"Pos": {"ring": 3, "position": 16, "rotation": 0}}}, {"system": "63", "position": {"Pos": {"ring": 3, "position": 17, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Tux": {"name": "Tux", "order": 0, "faction": "CouncilKeleres"}, "Yoda": {"name": "Yoda", "order": 2, "faction": "CrimsonRebellion"}, "Vidde": {"name": "Vidde", "order": 1, "faction": "FirmamentObsidian"}}, "game_name": "Test Game no uno", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
27526	e15c9638	3	2026-10-30 13:24:19.787818+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27527	e15c9638	4	2026-10-30 13:25:19.787818+00	"CreationDone"
27528	e15c9638	5	2026-10-30 13:26:19.787818+00	{"SetupSpeaker": {"player": "Alice"}}
27529	e15c9638	6	2026-10-30 13:27:19.787818+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27530	e15c9638	7	2026-10-30 13:28:19.787818+00	"StartGame"
27531	f4445ee6	0	2026-10-30 13:17:19.838616+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
27532	f4445ee6	1	2026-10-30 13:18:19.838616+00	{"AddPlayer": {"player": {"name": "Wendy", "color": "Pink", "faction": "Winnu"}}}
27571	69d97f21	12	2026-10-30 13:26:19.99373+00	{"TakeStrategyCard": {"card": "Technology", "player": "Bob"}}
27572	69d97f21	13	2026-10-30 13:27:19.99373+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Carol"}}
27573	69d97f21	14	2026-10-30 13:28:19.99373+00	"CompleteStrategyPhase"
27574	36242911	0	2026-10-30 12:52:20.067525+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27575	36242911	1	2026-10-30 12:53:20.067525+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "NaaluCollective"}}}
27576	36242911	2	2026-10-30 12:54:20.067525+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "FederationOfSol"}}}
27577	36242911	3	2026-10-30 12:55:20.067525+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "EmiratesOfHacan"}}}
27578	36242911	4	2026-10-30 12:56:20.067525+00	"CreationDone"
27579	36242911	5	2026-10-30 12:57:20.067525+00	{"SetupSpeaker": {"player": "Alice"}}
27580	36242911	6	2026-10-30 12:58:20.067525+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27581	36242911	7	2026-10-30 12:59:20.067525+00	"StartGame"
27582	36242911	8	2026-10-30 13:00:20.067525+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Alice"}}
27583	36242911	9	2026-10-30 13:01:20.067525+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Bob"}}
27584	36242911	10	2026-10-30 13:02:20.067525+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Carol"}}
27585	36242911	11	2026-10-30 13:03:20.067525+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Alice"}}
27586	36242911	12	2026-10-30 13:04:20.067525+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27587	36242911	13	2026-10-30 13:05:20.067525+00	{"TakeStrategyCard": {"card": "Construction", "player": "Carol"}}
27588	36242911	14	2026-10-30 13:06:20.067525+00	"CompleteStrategyPhase"
27589	36242911	15	2026-10-30 13:07:20.067525+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Alice"}}
27590	36242911	16	2026-10-30 13:08:20.067525+00	"StrategicActionCommit"
27591	36242911	17	2026-10-30 13:09:20.067525+00	{"EndTurn": {"player": "Alice"}}
27592	36242911	18	2026-10-30 13:10:20.067525+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Bob"}}
27593	36242911	19	2026-10-30 13:11:20.067525+00	"StrategicActionCommit"
27594	36242911	20	2026-10-30 13:12:20.067525+00	{"EndTurn": {"player": "Bob"}}
27595	36242911	21	2026-10-30 13:13:20.067525+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Carol"}}
27596	36242911	22	2026-10-30 13:14:20.067525+00	"StrategicActionCommit"
27597	36242911	23	2026-10-30 13:15:20.067525+00	{"EndTurn": {"player": "Carol"}}
27598	36242911	24	2026-10-30 13:16:20.067525+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Alice"}}
27599	36242911	25	2026-10-30 13:17:20.067525+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": null}}, "player": "Alice"}}
27600	36242911	26	2026-10-30 13:18:20.067525+00	"StrategicActionCommit"
27601	36242911	27	2026-10-30 13:19:20.067525+00	{"EndTurn": {"player": "Alice"}}
27602	36242911	28	2026-10-30 13:20:20.067525+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
27603	36242911	29	2026-10-30 13:21:20.067525+00	"StrategicActionCommit"
27604	36242911	30	2026-10-30 13:22:20.067525+00	{"EndTurn": {"player": "Bob"}}
27605	36242911	31	2026-10-30 13:23:20.067525+00	{"StrategicActionBegin": {"card": "Construction", "player": "Carol"}}
27606	36242911	32	2026-10-30 13:24:20.067525+00	"StrategicActionCommit"
27607	36242911	33	2026-10-30 13:25:20.067525+00	{"EndTurn": {"player": "Carol"}}
27608	36242911	34	2026-10-30 13:26:20.067525+00	{"PassAction": {"player": "Alice"}}
27609	36242911	35	2026-10-30 13:27:20.067525+00	{"PassAction": {"player": "Bob"}}
27610	36242911	36	2026-10-30 13:28:20.067525+00	{"PassAction": {"player": "Carol"}}
27611	cdd60863	0	2026-10-30 12:43:20.226707+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27612	cdd60863	1	2026-10-30 12:44:20.226707+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "NaaluCollective"}}}
27613	cdd60863	2	2026-10-30 12:45:20.226707+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "FederationOfSol"}}}
27614	cdd60863	3	2026-10-30 12:46:20.226707+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "EmiratesOfHacan"}}}
27615	cdd60863	4	2026-10-30 12:47:20.226707+00	"CreationDone"
27616	cdd60863	5	2026-10-30 12:48:20.226707+00	{"SetupSpeaker": {"player": "Alice"}}
27617	cdd60863	6	2026-10-30 12:49:20.226707+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27618	cdd60863	7	2026-10-30 12:50:20.226707+00	"StartGame"
27619	cdd60863	8	2026-10-30 12:51:20.226707+00	{"PlayGiftOfPrescience": {"player": "Bob"}}
27620	cdd60863	9	2026-10-30 12:52:20.226707+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27621	cdd60863	10	2026-10-30 12:53:20.226707+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Bob"}}
27622	cdd60863	11	2026-10-30 12:54:20.226707+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Carol"}}
27623	cdd60863	12	2026-10-30 12:55:20.226707+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Alice"}}
27624	cdd60863	13	2026-10-30 12:56:20.226707+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27625	cdd60863	14	2026-10-30 12:57:20.226707+00	{"TakeStrategyCard": {"card": "Construction", "player": "Carol"}}
27626	cdd60863	15	2026-10-30 12:58:20.226707+00	"CompleteStrategyPhase"
27627	cdd60863	16	2026-10-30 12:59:20.226707+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
27628	cdd60863	17	2026-10-30 13:00:20.226707+00	"StrategicActionCommit"
27629	cdd60863	18	2026-10-30 13:01:20.226707+00	{"EndTurn": {"player": "Bob"}}
27630	cdd60863	19	2026-10-30 13:02:20.226707+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
27631	cdd60863	20	2026-10-30 13:03:20.226707+00	"StrategicActionCommit"
27632	cdd60863	21	2026-10-30 13:04:20.226707+00	{"EndTurn": {"player": "Alice"}}
27633	cdd60863	22	2026-10-30 13:05:20.226707+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Carol"}}
27634	cdd60863	23	2026-10-30 13:06:20.226707+00	"StrategicActionCommit"
27635	cdd60863	24	2026-10-30 13:07:20.226707+00	{"EndTurn": {"player": "Carol"}}
27636	cdd60863	25	2026-10-30 13:08:20.226707+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Bob"}}
27637	cdd60863	26	2026-10-30 13:09:20.226707+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": null}}, "player": "Bob"}}
27638	cdd60863	27	2026-10-30 13:10:20.226707+00	"StrategicActionCommit"
27639	cdd60863	28	2026-10-30 13:11:20.226707+00	{"EndTurn": {"player": "Bob"}}
27640	cdd60863	29	2026-10-30 13:12:20.226707+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Alice"}}
27641	cdd60863	30	2026-10-30 13:13:20.226707+00	"StrategicActionCommit"
27642	cdd60863	31	2026-10-30 13:14:20.226707+00	{"EndTurn": {"player": "Alice"}}
27643	cdd60863	32	2026-10-30 13:15:20.226707+00	{"StrategicActionBegin": {"card": "Construction", "player": "Carol"}}
27644	cdd60863	33	2026-10-30 13:16:20.226707+00	"StrategicActionCommit"
27645	cdd60863	34	2026-10-30 13:17:20.226707+00	{"EndTurn": {"player": "Carol"}}
27646	cdd60863	35	2026-10-30 13:18:20.226707+00	{"PassAction": {"player": "Bob"}}
27647	cdd60863	36	2026-10-30 13:19:20.226707+00	{"PassAction": {"player": "Alice"}}
27648	cdd60863	37	2026-10-30 13:20:20.226707+00	{"PassAction": {"player": "Carol"}}
27649	cdd60863	38	2026-10-30 13:21:20.226707+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
27650	cdd60863	39	2026-10-30 13:22:20.226707+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
27651	cdd60863	40	2026-10-30 13:23:20.226707+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
27652	cdd60863	41	2026-10-30 13:24:20.226707+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
27653	cdd60863	42	2026-10-30 13:25:20.226707+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
27654	cdd60863	43	2026-10-30 13:26:20.226707+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
27655	cdd60863	44	2026-10-30 13:27:20.226707+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
27656	cdd60863	45	2026-10-30 13:28:20.226707+00	"CompleteStatusPhase"
27657	dd2da702	0	2026-10-30 12:46:20.419403+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27658	dd2da702	1	2026-10-30 12:47:20.419403+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27659	dd2da702	2	2026-10-30 12:48:20.419403+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27660	dd2da702	3	2026-10-30 12:49:20.419403+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27661	dd2da702	4	2026-10-30 12:50:20.419403+00	"CreationDone"
27662	dd2da702	5	2026-10-30 12:51:20.419403+00	{"SetupSpeaker": {"player": "Alice"}}
27663	dd2da702	6	2026-10-30 12:52:20.419403+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27664	dd2da702	7	2026-10-30 12:53:20.419403+00	"StartGame"
27665	dd2da702	8	2026-10-30 12:54:20.419403+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27666	dd2da702	9	2026-10-30 12:55:20.419403+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27667	dd2da702	10	2026-10-30 12:56:20.419403+00	{"TakeStrategyCard": {"card": "Construction", "player": "Carol"}}
27668	dd2da702	11	2026-10-30 12:57:20.419403+00	{"TakeStrategyCard": {"card": "Trade", "player": "Alice"}}
27669	dd2da702	12	2026-10-30 12:58:20.419403+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Bob"}}
27670	dd2da702	13	2026-10-30 12:59:20.419403+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
27671	dd2da702	14	2026-10-30 13:00:20.419403+00	"CompleteStrategyPhase"
27672	dd2da702	15	2026-10-30 13:01:20.419403+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
27673	dd2da702	16	2026-10-30 13:02:20.419403+00	"StrategicActionCommit"
27674	dd2da702	17	2026-10-30 13:03:20.419403+00	{"EndTurn": {"player": "Alice"}}
27675	dd2da702	18	2026-10-30 13:04:20.419403+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27676	dd2da702	19	2026-10-30 13:05:20.419403+00	"StrategicActionCommit"
27677	dd2da702	20	2026-10-30 13:06:20.419403+00	{"EndTurn": {"player": "Bob"}}
27678	dd2da702	21	2026-10-30 13:07:20.419403+00	{"StrategicActionBegin": {"card": "Construction", "player": "Carol"}}
27679	dd2da702	22	2026-10-30 13:08:20.419403+00	"StrategicActionCommit"
27680	dd2da702	23	2026-10-30 13:09:20.419403+00	{"EndTurn": {"player": "Carol"}}
27681	dd2da702	24	2026-10-30 13:10:20.419403+00	{"StrategicActionBegin": {"card": "Trade", "player": "Alice"}}
27682	dd2da702	25	2026-10-30 13:11:20.419403+00	"StrategicActionCommit"
27683	dd2da702	26	2026-10-30 13:12:20.419403+00	{"TakeAnotherTurn": {"player": "Alice"}}
27684	dd2da702	27	2026-10-30 13:13:20.419403+00	{"TacticalActionBegin": {"player": "Alice"}}
27685	dd2da702	28	2026-10-30 13:14:20.419403+00	{"TacticalActionCommit": {"player": "Alice"}}
27686	dd2da702	29	2026-10-30 13:15:20.419403+00	{"EndTurn": {"player": "Alice"}}
27687	dd2da702	30	2026-10-30 13:16:20.419403+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Bob"}}
27688	dd2da702	31	2026-10-30 13:17:20.419403+00	"StrategicActionCommit"
27689	dd2da702	32	2026-10-30 13:18:20.419403+00	{"EndTurn": {"player": "Bob"}}
27690	dd2da702	33	2026-10-30 13:19:20.419403+00	{"StrategicActionBegin": {"card": "Technology", "player": "Carol"}}
27691	dd2da702	34	2026-10-30 13:20:20.419403+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": null, "second_tech": null}}, "player": "Carol"}}
27692	dd2da702	35	2026-10-30 13:21:20.419403+00	"StrategicActionCommit"
27693	dd2da702	36	2026-10-30 13:22:20.419403+00	{"EndTurn": {"player": "Carol"}}
27694	dd2da702	37	2026-10-30 13:23:20.419403+00	{"PassAction": {"player": "Alice"}}
27695	dd2da702	38	2026-10-30 13:24:20.419403+00	{"PassAction": {"player": "Bob"}}
27696	dd2da702	39	2026-10-30 13:25:20.419403+00	{"TacticalActionBegin": {"player": "Carol"}}
27697	dd2da702	40	2026-10-30 13:26:20.419403+00	{"TacticalActionCommit": {"player": "Carol"}}
27698	dd2da702	41	2026-10-30 13:27:20.419403+00	{"EndTurn": {"player": "Carol"}}
27699	dd2da702	42	2026-10-30 13:28:20.419403+00	{"PassAction": {"player": "Carol"}}
27700	c828f2f2	0	2026-10-30 12:58:20.616767+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
27701	c828f2f2	1	2026-10-30 12:59:20.616767+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27702	c828f2f2	2	2026-10-30 13:00:20.616767+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27703	c828f2f2	3	2026-10-30 13:01:20.616767+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27704	c828f2f2	4	2026-10-30 13:02:20.616767+00	"CreationDone"
27705	c828f2f2	5	2026-10-30 13:03:20.616767+00	{"SetupSpeaker": {"player": "Alice"}}
27706	c828f2f2	6	2026-10-30 13:04:20.616767+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27707	c828f2f2	7	2026-10-30 13:05:20.616767+00	"StartGame"
27708	c828f2f2	8	2026-10-30 13:06:20.616767+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27709	c828f2f2	9	2026-10-30 13:07:20.616767+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27710	c828f2f2	10	2026-10-30 13:08:20.616767+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27711	c828f2f2	11	2026-10-30 13:09:20.616767+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27712	c828f2f2	12	2026-10-30 13:10:20.616767+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27713	c828f2f2	13	2026-10-30 13:11:20.616767+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27714	c828f2f2	14	2026-10-30 13:12:20.616767+00	"CompleteStrategyPhase"
27715	c828f2f2	15	2026-10-30 13:13:20.616767+00	{"TacticalActionBegin": {"player": "Alice"}}
27716	c828f2f2	16	2026-10-30 13:14:20.616767+00	{"TacticalActionTakePlanet": {"planet": "Centauri", "player": "Alice"}}
27717	c828f2f2	17	2026-10-30 13:15:20.616767+00	{"TacticalActionAttachPlanetAttachment": {"planet": "Centauri", "player": "Alice", "attachment": "DysonSphere"}}
27718	c828f2f2	18	2026-10-30 13:16:20.616767+00	{"TacticalActionTakePlanet": {"planet": "Gral", "player": "Alice"}}
27719	c828f2f2	19	2026-10-30 13:17:20.616767+00	{"TacticalActionAttachPlanetAttachment": {"planet": "Gral", "player": "Alice", "attachment": "BioticResearchFacility"}}
27720	c828f2f2	20	2026-10-30 13:18:20.616767+00	{"TacticalActionTakePlanet": {"planet": "Arinam", "player": "Alice"}}
27721	c828f2f2	21	2026-10-30 13:19:20.616767+00	{"TacticalActionCommit": {"player": "Alice"}}
27722	c828f2f2	22	2026-10-30 13:20:20.616767+00	{"EndTurn": {"player": "Alice"}}
27723	c828f2f2	23	2026-10-30 13:21:20.616767+00	{"TacticalActionBegin": {"player": "Bob"}}
27724	c828f2f2	24	2026-10-30 13:22:20.616767+00	{"TacticalActionTakePlanet": {"planet": "Centauri", "player": "Bob"}}
27725	c828f2f2	25	2026-10-30 13:23:20.616767+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Bob"}}
27726	c828f2f2	26	2026-10-30 13:24:20.616767+00	{"TacticalActionCommit": {"player": "Bob"}}
27727	c828f2f2	27	2026-10-30 13:25:20.616767+00	{"EndTurn": {"player": "Bob"}}
27728	c828f2f2	28	2026-10-30 13:26:20.616767+00	{"TacticalActionBegin": {"player": "Carol"}}
27729	c828f2f2	29	2026-10-30 13:27:20.616767+00	{"TacticalActionCommit": {"player": "Carol"}}
27730	c828f2f2	30	2026-10-30 13:28:20.616767+00	{"EndTurn": {"player": "Carol"}}
27731	bcebac25	0	2026-10-30 13:02:20.759965+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27732	bcebac25	1	2026-10-30 13:03:20.759965+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27733	bcebac25	2	2026-10-30 13:04:20.759965+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "UniversitiesOfJolNar"}}}
27734	bcebac25	3	2026-10-30 13:05:20.759965+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "NekroVirus"}}}
27735	bcebac25	4	2026-10-30 13:06:20.759965+00	"CreationDone"
27736	bcebac25	5	2026-10-30 13:07:20.759965+00	{"SetupSpeaker": {"player": "Alice"}}
27737	bcebac25	6	2026-10-30 13:08:20.759965+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27738	bcebac25	7	2026-10-30 13:09:20.759965+00	"StartGame"
27739	bcebac25	8	2026-10-30 13:10:20.759965+00	{"TakeStrategyCard": {"card": "Technology", "player": "Alice"}}
27740	bcebac25	9	2026-10-30 13:11:20.759965+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Bob"}}
27741	bcebac25	10	2026-10-30 13:12:20.759965+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Carol"}}
27742	bcebac25	11	2026-10-30 13:13:20.759965+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27743	bcebac25	12	2026-10-30 13:14:20.759965+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27744	bcebac25	13	2026-10-30 13:15:20.759965+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27745	bcebac25	14	2026-10-30 13:16:20.759965+00	"CompleteStrategyPhase"
27746	bcebac25	15	2026-10-30 13:17:20.759965+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Bob"}}
27747	bcebac25	16	2026-10-30 13:18:20.759965+00	"StrategicActionCommit"
27748	bcebac25	17	2026-10-30 13:19:20.759965+00	{"EndTurn": {"player": "Bob"}}
27749	bcebac25	18	2026-10-30 13:20:20.759965+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Carol"}}
27750	bcebac25	19	2026-10-30 13:21:20.759965+00	"StrategicActionCommit"
27751	bcebac25	20	2026-10-30 13:22:20.759965+00	{"EndTurn": {"player": "Carol"}}
27752	bcebac25	21	2026-10-30 13:23:20.759965+00	{"StrategicActionBegin": {"card": "Technology", "player": "Alice"}}
27753	bcebac25	22	2026-10-30 13:24:20.759965+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": "GravityDrive", "second_tech": "PlasmaScoring"}}, "player": "Alice"}}
27754	bcebac25	23	2026-10-30 13:25:20.759965+00	{"StrategicActionSecondary": {"action": {"TechnologyJolNar": {"firstTech": "GravityDrive", "secondTech": "DacxiveAnimators"}}, "player": "Bob"}}
27755	bcebac25	24	2026-10-30 13:26:20.759965+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
27756	bcebac25	25	2026-10-30 13:27:20.759965+00	"StrategicActionCommit"
27757	bcebac25	26	2026-10-30 13:28:20.759965+00	{"EndTurn": {"player": "Alice"}}
27758	e3857a9f	0	2026-10-30 13:04:20.878904+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27759	e3857a9f	1	2026-10-30 13:05:20.878904+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27760	e3857a9f	2	2026-10-30 13:06:20.878904+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "UniversitiesOfJolNar"}}}
27761	e3857a9f	3	2026-10-30 13:07:20.878904+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "NekroVirus"}}}
27762	e3857a9f	4	2026-10-30 13:08:20.878904+00	"CreationDone"
27763	e3857a9f	5	2026-10-30 13:09:20.878904+00	{"SetupSpeaker": {"player": "Alice"}}
27764	e3857a9f	6	2026-10-30 13:10:20.878904+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27765	e3857a9f	7	2026-10-30 13:11:20.878904+00	"StartGame"
27766	e3857a9f	8	2026-10-30 13:12:20.878904+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27767	e3857a9f	9	2026-10-30 13:13:20.878904+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27768	e3857a9f	10	2026-10-30 13:14:20.878904+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
27769	e3857a9f	11	2026-10-30 13:15:20.878904+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27770	e3857a9f	12	2026-10-30 13:16:20.878904+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27771	e3857a9f	13	2026-10-30 13:17:20.878904+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27772	e3857a9f	14	2026-10-30 13:18:20.878904+00	"CompleteStrategyPhase"
27773	e3857a9f	15	2026-10-30 13:19:20.878904+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
27774	e3857a9f	16	2026-10-30 13:20:20.878904+00	"StrategicActionCommit"
27775	e3857a9f	17	2026-10-30 13:21:20.878904+00	{"EndTurn": {"player": "Alice"}}
27776	e3857a9f	18	2026-10-30 13:22:20.878904+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27777	e3857a9f	19	2026-10-30 13:23:20.878904+00	"StrategicActionCommit"
27778	e3857a9f	20	2026-10-30 13:24:20.878904+00	{"EndTurn": {"player": "Bob"}}
27779	e3857a9f	21	2026-10-30 13:25:20.878904+00	{"StrategicActionBegin": {"card": "Technology", "player": "Carol"}}
27780	e3857a9f	22	2026-10-30 13:26:20.878904+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "GravityDrive"}}, "player": "Alice"}}
27781	e3857a9f	23	2026-10-30 13:27:20.878904+00	"StrategicActionCommit"
27782	e3857a9f	24	2026-10-30 13:28:20.878904+00	{"EndTurn": {"player": "Carol"}}
27783	a68a7f72	0	2026-10-30 12:51:20.989928+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27784	a68a7f72	1	2026-10-30 12:52:20.989928+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27785	a68a7f72	2	2026-10-30 12:53:20.989928+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27786	a68a7f72	3	2026-10-30 12:54:20.989928+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27787	a68a7f72	4	2026-10-30 12:55:20.989928+00	"CreationDone"
27788	a68a7f72	5	2026-10-30 12:56:20.989928+00	{"SetupSpeaker": {"player": "Alice"}}
27789	a68a7f72	6	2026-10-30 12:57:20.989928+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27790	a68a7f72	7	2026-10-30 12:58:20.989928+00	"StartGame"
27791	a68a7f72	8	2026-10-30 12:59:20.989928+00	{"TakeStrategyCard": {"card": "Politics", "player": "Alice"}}
27792	a68a7f72	9	2026-10-30 13:00:20.989928+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Bob"}}
27793	a68a7f72	10	2026-10-30 13:01:20.989928+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Carol"}}
27794	a68a7f72	11	2026-10-30 13:02:20.989928+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Alice"}}
27795	a68a7f72	12	2026-10-30 13:03:20.989928+00	{"TakeStrategyCard": {"card": "Construction", "player": "Bob"}}
27796	a68a7f72	13	2026-10-30 13:04:20.989928+00	{"TakeStrategyCard": {"card": "Trade", "player": "Carol"}}
27797	a68a7f72	14	2026-10-30 13:05:20.989928+00	"CompleteStrategyPhase"
27798	a68a7f72	15	2026-10-30 13:06:20.989928+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Carol"}}
27799	a68a7f72	16	2026-10-30 13:07:20.989928+00	"StrategicActionCommit"
27800	a68a7f72	17	2026-10-30 13:08:20.989928+00	{"EndTurn": {"player": "Carol"}}
27801	a68a7f72	18	2026-10-30 13:09:20.989928+00	{"StrategicActionBegin": {"card": "Politics", "player": "Alice"}}
27802	a68a7f72	19	2026-10-30 13:10:20.989928+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Bob"}}, "player": "Alice"}}
27803	a68a7f72	20	2026-10-30 13:11:20.989928+00	{"StrategicActionSecondary": {"action": "Politics", "player": "Carol"}}
27804	a68a7f72	21	2026-10-30 13:12:20.989928+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
27805	a68a7f72	22	2026-10-30 13:13:20.989928+00	"StrategicActionCommit"
27806	a68a7f72	23	2026-10-30 13:14:20.989928+00	{"EndTurn": {"player": "Alice"}}
27807	a68a7f72	24	2026-10-30 13:15:20.989928+00	{"TacticalActionBegin": {"player": "Bob"}}
27808	a68a7f72	25	2026-10-30 13:16:20.989928+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Bob"}}
27809	a68a7f72	26	2026-10-30 13:17:20.989928+00	{"TacticalActionCommit": {"player": "Bob"}}
27810	a68a7f72	27	2026-10-30 13:18:20.989928+00	{"EndTurn": {"player": "Bob"}}
27811	a68a7f72	28	2026-10-30 13:19:20.989928+00	{"StrategicActionBegin": {"card": "Trade", "player": "Carol"}}
27812	a68a7f72	29	2026-10-30 13:20:20.989928+00	"StrategicActionCommit"
27813	a68a7f72	30	2026-10-30 13:21:20.989928+00	{"EndTurn": {"player": "Carol"}}
27814	a68a7f72	31	2026-10-30 13:22:20.989928+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Alice"}}
27815	a68a7f72	32	2026-10-30 13:23:20.989928+00	"StrategicActionCommit"
27816	a68a7f72	33	2026-10-30 13:24:20.989928+00	{"EndTurn": {"player": "Alice"}}
27817	a68a7f72	34	2026-10-30 13:25:20.989928+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Bob"}}
27875	71b34856	26	2026-10-30 13:28:21.284584+00	{"EndTurn": {"player": "Alice"}}
27818	a68a7f72	35	2026-10-30 13:26:20.989928+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "CornerTheMarket"}}, "player": "Bob"}}
27819	a68a7f72	36	2026-10-30 13:27:20.989928+00	"StrategicActionCommit"
27820	a68a7f72	37	2026-10-30 13:28:20.989928+00	{"EndTurn": {"player": "Bob"}}
27821	2ec3d2d7	0	2026-10-30 13:01:21.158261+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
27822	2ec3d2d7	1	2026-10-30 13:02:21.158261+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27823	2ec3d2d7	2	2026-10-30 13:03:21.158261+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27824	2ec3d2d7	3	2026-10-30 13:04:21.158261+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27825	2ec3d2d7	4	2026-10-30 13:05:21.158261+00	"CreationDone"
27826	2ec3d2d7	5	2026-10-30 13:06:21.158261+00	{"SetupSpeaker": {"player": "Alice"}}
27827	2ec3d2d7	6	2026-10-30 13:07:21.158261+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27828	2ec3d2d7	7	2026-10-30 13:08:21.158261+00	"StartGame"
27829	2ec3d2d7	8	2026-10-30 13:09:21.158261+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27830	2ec3d2d7	9	2026-10-30 13:10:21.158261+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27831	2ec3d2d7	10	2026-10-30 13:11:21.158261+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27832	2ec3d2d7	11	2026-10-30 13:12:21.158261+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27833	2ec3d2d7	12	2026-10-30 13:13:21.158261+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27834	2ec3d2d7	13	2026-10-30 13:14:21.158261+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27835	2ec3d2d7	14	2026-10-30 13:15:21.158261+00	"CompleteStrategyPhase"
27836	2ec3d2d7	15	2026-10-30 13:16:21.158261+00	{"ActionCardActionBegin": {"card": "FocusedResearch", "player": "Alice"}}
27837	2ec3d2d7	16	2026-10-30 13:17:21.158261+00	{"ActionCardActionCommit": {"data": {"FocusedResearch": {"tech": "GravityDrive"}}, "player": "Alice"}}
27838	2ec3d2d7	17	2026-10-30 13:18:21.158261+00	{"EndTurn": {"player": "Alice"}}
27839	2ec3d2d7	18	2026-10-30 13:19:21.158261+00	{"AddTechToPlayer": {"tech": "ProductionBiomes", "player": "Bob"}}
27840	2ec3d2d7	19	2026-10-30 13:20:21.158261+00	{"ActionCardActionBegin": {"card": "DivertFunding", "player": "Bob"}}
27841	2ec3d2d7	20	2026-10-30 13:21:21.158261+00	{"ActionCardActionCommit": {"data": {"DivertFunding": {"takeTech": "GravityDrive", "removeTech": "SarweenTools"}}, "player": "Bob"}}
27842	2ec3d2d7	21	2026-10-30 13:22:21.158261+00	{"EndTurn": {"player": "Bob"}}
27843	2ec3d2d7	22	2026-10-30 13:23:21.158261+00	{"ActionCardActionBegin": {"card": "Plagiarize", "player": "Carol"}}
27844	2ec3d2d7	23	2026-10-30 13:24:21.158261+00	{"ActionCardActionCommit": {"data": {"Plagiarize": {"tech": "NeuralMotivator"}}, "player": "Carol"}}
27845	2ec3d2d7	24	2026-10-30 13:25:21.158261+00	{"EndTurn": {"player": "Carol"}}
27846	2ec3d2d7	25	2026-10-30 13:26:21.158261+00	{"ActionCardActionBegin": {"card": "UnstablePlanet", "player": "Alice"}}
27847	2ec3d2d7	26	2026-10-30 13:27:21.158261+00	{"ActionCardActionCommit": {"data": null, "player": "Alice"}}
27848	2ec3d2d7	27	2026-10-30 13:28:21.158261+00	{"EndTurn": {"player": "Alice"}}
27849	71b34856	0	2026-10-30 13:02:21.284584+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
27850	71b34856	1	2026-10-30 13:03:21.284584+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "Arborec"}}}
27851	71b34856	2	2026-10-30 13:04:21.284584+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27852	71b34856	3	2026-10-30 13:05:21.284584+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27853	71b34856	4	2026-10-30 13:06:21.284584+00	"CreationDone"
27854	71b34856	5	2026-10-30 13:07:21.284584+00	{"SetupSpeaker": {"player": "Alice"}}
27855	71b34856	6	2026-10-30 13:08:21.284584+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27856	71b34856	7	2026-10-30 13:09:21.284584+00	"StartGame"
27857	71b34856	8	2026-10-30 13:10:21.284584+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27858	71b34856	9	2026-10-30 13:11:21.284584+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27859	71b34856	10	2026-10-30 13:12:21.284584+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27860	71b34856	11	2026-10-30 13:13:21.284584+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27861	71b34856	12	2026-10-30 13:14:21.284584+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27862	71b34856	13	2026-10-30 13:15:21.284584+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27863	71b34856	14	2026-10-30 13:16:21.284584+00	"CompleteStrategyPhase"
27864	71b34856	15	2026-10-30 13:17:21.284584+00	{"LeaderActionBegin": {"leader": "LetaniOspha", "player": "Alice"}}
27865	71b34856	16	2026-10-30 13:18:21.284584+00	{"EndTurn": {"player": "Alice"}}
27866	71b34856	17	2026-10-30 13:19:21.284584+00	{"ScoreExtraPublicObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
27867	71b34856	18	2026-10-30 13:20:21.284584+00	{"ScoreExtraPublicObjective": {"player": "Alice", "objective": "DevelopWeaponry"}}
27868	71b34856	19	2026-10-30 13:21:21.284584+00	{"ScoreExtraSecretObjective": {"player": "Alice", "objective": "BecomeTheGatekeeper"}}
27869	71b34856	20	2026-10-30 13:22:21.284584+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27870	71b34856	21	2026-10-30 13:23:21.284584+00	"StrategicActionCommit"
27871	71b34856	22	2026-10-30 13:24:21.284584+00	{"EndTurn": {"player": "Bob"}}
27872	71b34856	23	2026-10-30 13:25:21.284584+00	{"LeaderActionBegin": {"leader": "GgrucotoRinn", "player": "Carol"}}
27873	71b34856	24	2026-10-30 13:26:21.284584+00	{"EndTurn": {"player": "Carol"}}
27874	71b34856	25	2026-10-30 13:27:21.284584+00	{"LeaderActionBegin": {"leader": "LetaniMiasmiala", "player": "Alice"}}
27876	eaf9b225	0	2026-10-30 12:57:21.42571+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
27877	eaf9b225	1	2026-10-30 12:58:21.42571+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27878	eaf9b225	2	2026-10-30 12:59:21.42571+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "NekroVirus"}}}
27879	eaf9b225	3	2026-10-30 13:00:21.42571+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27880	eaf9b225	4	2026-10-30 13:01:21.42571+00	"CreationDone"
27881	eaf9b225	5	2026-10-30 13:02:21.42571+00	{"SetupSpeaker": {"player": "Alice"}}
27882	eaf9b225	6	2026-10-30 13:03:21.42571+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27883	eaf9b225	7	2026-10-30 13:04:21.42571+00	"StartGame"
27884	eaf9b225	8	2026-10-30 13:05:21.42571+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27885	eaf9b225	9	2026-10-30 13:06:21.42571+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27886	eaf9b225	10	2026-10-30 13:07:21.42571+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27887	eaf9b225	11	2026-10-30 13:08:21.42571+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27888	eaf9b225	12	2026-10-30 13:09:21.42571+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27889	eaf9b225	13	2026-10-30 13:10:21.42571+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27890	eaf9b225	14	2026-10-30 13:11:21.42571+00	"CompleteStrategyPhase"
27891	eaf9b225	15	2026-10-30 13:12:21.42571+00	{"FrontierCardActionBegin": {"card": "EnigmaticDevice", "player": "Alice"}}
27892	eaf9b225	16	2026-10-30 13:13:21.42571+00	{"FrontierCardActionCommit": {"data": {"EnigmaticDevice": {"tech": "GravityDrive"}}, "player": "Alice"}}
27893	eaf9b225	17	2026-10-30 13:14:21.42571+00	{"EndTurn": {"player": "Alice"}}
27894	eaf9b225	18	2026-10-30 13:15:21.42571+00	{"FrontierCardActionBegin": {"card": "EnigmaticDevice", "player": "Bob"}}
27895	eaf9b225	19	2026-10-30 13:16:21.42571+00	{"FrontierCardActionCommit": {"data": null, "player": "Bob"}}
27896	eaf9b225	20	2026-10-30 13:17:21.42571+00	{"EndTurn": {"player": "Bob"}}
27897	eaf9b225	21	2026-10-30 13:18:21.42571+00	{"FrontierCardActionBegin": {"card": "Mirage", "player": "Carol"}}
27898	eaf9b225	22	2026-10-30 13:19:21.42571+00	{"FrontierCardActionCommit": {"data": {"Mirage": {"system": null, "attachment": "ParadiseWorld"}}, "player": "Carol"}}
27899	eaf9b225	23	2026-10-30 13:20:21.42571+00	{"EndTurn": {"player": "Carol"}}
27900	eaf9b225	24	2026-10-30 13:21:21.42571+00	{"TacticalActionBegin": {"player": "Alice"}}
27901	eaf9b225	25	2026-10-30 13:22:21.42571+00	{"TacticalActionTakePlanet": {"planet": "Mirage", "player": "Alice"}}
27902	eaf9b225	26	2026-10-30 13:23:21.42571+00	{"TacticalActionCommit": {"player": "Alice"}}
27903	eaf9b225	27	2026-10-30 13:24:21.42571+00	{"EndTurn": {"player": "Alice"}}
27904	eaf9b225	28	2026-10-30 13:26:21.42571+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27905	eaf9b225	29	2026-10-30 13:27:21.42571+00	"StrategicActionCommit"
27906	eaf9b225	30	2026-10-30 13:28:21.42571+00	{"EndTurn": {"player": "Bob"}}
27907	459c2e20	0	2026-10-30 12:31:21.578509+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
27908	459c2e20	1	2026-10-30 12:32:21.578509+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27909	459c2e20	2	2026-10-30 12:33:21.578509+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27910	459c2e20	3	2026-10-30 12:34:21.578509+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27911	459c2e20	4	2026-10-30 12:35:21.578509+00	"CreationDone"
27912	459c2e20	5	2026-10-30 12:36:21.578509+00	{"SetupSpeaker": {"player": "Alice"}}
27913	459c2e20	6	2026-10-30 12:37:21.578509+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27914	459c2e20	7	2026-10-30 12:38:21.578509+00	"StartGame"
27915	459c2e20	8	2026-10-30 12:39:21.578509+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27916	459c2e20	9	2026-10-30 12:40:21.578509+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27917	459c2e20	10	2026-10-30 12:41:21.578509+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27918	459c2e20	11	2026-10-30 12:42:21.578509+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27919	459c2e20	12	2026-10-30 12:43:21.578509+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27920	459c2e20	13	2026-10-30 12:44:21.578509+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27921	459c2e20	14	2026-10-30 12:45:21.578509+00	"CompleteStrategyPhase"
27922	459c2e20	15	2026-10-30 12:46:21.578509+00	{"SetPlanetOwner": {"planet": "Primor", "player": "Alice"}}
27923	459c2e20	16	2026-10-30 12:47:21.578509+00	{"SetPlanetOwner": {"planet": "Centauri", "player": "Alice"}}
27924	459c2e20	17	2026-10-30 12:48:21.578509+00	{"SetPlanetOwner": {"planet": "Lodor", "player": "Bob"}}
27925	459c2e20	18	2026-10-30 12:49:21.578509+00	{"GainRelicAction": {"relic": "ShardOfTheThrone", "player": "Alice"}}
27926	459c2e20	19	2026-10-30 12:50:21.578509+00	{"EndTurn": {"player": "Alice"}}
27927	459c2e20	20	2026-10-30 12:51:21.578509+00	{"TacticalActionBegin": {"player": "Bob"}}
27928	459c2e20	21	2026-10-30 12:52:21.578509+00	{"TacticalActionTakePlanet": {"planet": "Primor", "player": "Bob"}}
27929	459c2e20	22	2026-10-30 12:53:21.578509+00	{"TacticalActionCommit": {"player": "Bob"}}
27930	459c2e20	23	2026-10-30 12:54:21.578509+00	{"EndTurn": {"player": "Bob"}}
27931	459c2e20	24	2026-10-30 12:55:21.578509+00	{"GainRelicAction": {"relic": "StellarConverter", "player": "Carol"}}
27932	459c2e20	25	2026-10-30 12:56:21.578509+00	{"EndTurn": {"player": "Carol"}}
27933	459c2e20	26	2026-10-30 12:57:21.578509+00	{"GainRelicAction": {"relic": "NanoForge", "player": "Alice"}}
27934	459c2e20	27	2026-10-30 12:58:21.578509+00	{"EndTurn": {"player": "Alice"}}
27935	459c2e20	28	2026-10-30 12:59:21.578509+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27936	459c2e20	29	2026-10-30 13:00:21.578509+00	"StrategicActionCommit"
27937	459c2e20	30	2026-10-30 13:01:21.578509+00	{"EndTurn": {"player": "Bob"}}
27938	459c2e20	31	2026-10-30 13:02:21.578509+00	{"RelicActionBegin": {"relic": "StellarConverter", "player": "Carol"}}
27939	459c2e20	32	2026-10-30 13:03:21.578509+00	{"RelicActionCommit": {"data": {"StellarConverter": {"planet": "Lodor"}}, "player": "Carol"}}
27940	459c2e20	33	2026-10-30 13:04:21.578509+00	{"EndTurn": {"player": "Carol"}}
27941	459c2e20	34	2026-10-30 13:05:21.578509+00	{"RelicActionBegin": {"relic": "NanoForge", "player": "Alice"}}
27942	459c2e20	35	2026-10-30 13:06:21.578509+00	{"RelicActionCommit": {"data": {"NanoForge": {"planet": "Centauri"}}, "player": "Alice"}}
27943	459c2e20	36	2026-10-30 13:07:21.578509+00	{"EndTurn": {"player": "Alice"}}
27944	459c2e20	37	2026-10-30 13:08:21.578509+00	{"GainRelicAction": {"relic": "JrXs4550", "player": "Bob"}}
27945	459c2e20	38	2026-10-30 13:09:21.578509+00	{"EndTurn": {"player": "Bob"}}
27946	459c2e20	39	2026-10-30 13:10:21.578509+00	{"GainRelicAction": {"relic": "CircletOfTheVoid", "player": "Carol"}}
27947	459c2e20	40	2026-10-30 13:11:21.578509+00	{"EndTurn": {"player": "Carol"}}
27948	459c2e20	41	2026-10-30 13:12:21.578509+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
27949	459c2e20	42	2026-10-30 13:13:21.578509+00	"StrategicActionCommit"
27950	459c2e20	43	2026-10-30 13:14:21.578509+00	{"EndTurn": {"player": "Alice"}}
27951	459c2e20	44	2026-10-30 13:15:21.578509+00	{"RelicActionBegin": {"relic": "JrXs4550", "player": "Bob"}}
27952	459c2e20	45	2026-10-30 13:16:21.578509+00	{"RelicActionCommit": {"data": null, "player": "Bob"}}
27953	459c2e20	46	2026-10-30 13:17:21.578509+00	{"EndTurn": {"player": "Bob"}}
27954	459c2e20	47	2026-10-30 13:18:21.578509+00	{"RelicActionBegin": {"relic": "CircletOfTheVoid", "player": "Carol"}}
27955	459c2e20	48	2026-10-30 13:19:21.578509+00	{"RelicActionCommit": {"data": null, "player": "Carol"}}
27956	459c2e20	49	2026-10-30 13:20:21.578509+00	{"EndTurn": {"player": "Carol"}}
27957	459c2e20	50	2026-10-30 13:21:21.578509+00	{"TacticalActionBegin": {"player": "Alice"}}
27958	459c2e20	51	2026-10-30 13:22:21.578509+00	{"TacticalActionTakePlanet": {"planet": "Arretze", "player": "Alice"}}
27959	459c2e20	52	2026-10-30 13:23:21.578509+00	{"TacticalActionCommit": {"player": "Alice"}}
27960	459c2e20	53	2026-10-30 13:24:21.578509+00	{"EndTurn": {"player": "Alice"}}
27961	459c2e20	54	2026-10-30 13:25:21.578509+00	{"TacticalActionBegin": {"player": "Bob"}}
27962	459c2e20	55	2026-10-30 13:26:21.578509+00	{"TacticalActionTakePlanet": {"planet": "Arretze", "player": "Bob"}}
27963	459c2e20	56	2026-10-30 13:27:21.578509+00	{"TacticalActionCommit": {"player": "Bob"}}
27964	459c2e20	57	2026-10-30 13:28:21.578509+00	{"EndTurn": {"player": "Bob"}}
27965	cc8bb7bd	0	2026-10-30 12:44:21.826125+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
27966	cc8bb7bd	1	2026-10-30 12:45:21.826125+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
27967	cc8bb7bd	2	2026-10-30 12:46:21.826125+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
27968	cc8bb7bd	3	2026-10-30 12:47:21.826125+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
27969	cc8bb7bd	4	2026-10-30 12:48:21.826125+00	"CreationDone"
27970	cc8bb7bd	5	2026-10-30 12:49:21.826125+00	{"SetupSpeaker": {"player": "Alice"}}
27971	cc8bb7bd	6	2026-10-30 12:50:21.826125+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
27972	cc8bb7bd	7	2026-10-30 12:51:21.826125+00	"StartGame"
27973	cc8bb7bd	8	2026-10-30 12:52:21.826125+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
27974	cc8bb7bd	9	2026-10-30 12:53:21.826125+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
27975	cc8bb7bd	10	2026-10-30 12:54:21.826125+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
27976	cc8bb7bd	11	2026-10-30 12:55:21.826125+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
27977	cc8bb7bd	12	2026-10-30 12:56:21.826125+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
27978	cc8bb7bd	13	2026-10-30 12:57:21.826125+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
27979	cc8bb7bd	14	2026-10-30 12:58:21.826125+00	"CompleteStrategyPhase"
27980	cc8bb7bd	15	2026-10-30 12:59:21.826125+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
27981	cc8bb7bd	16	2026-10-30 13:00:21.826125+00	"StrategicActionCommit"
27982	cc8bb7bd	17	2026-10-30 13:01:21.826125+00	{"EndTurn": {"player": "Alice"}}
27983	cc8bb7bd	18	2026-10-30 13:02:21.826125+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
27984	cc8bb7bd	19	2026-10-30 13:03:21.826125+00	"StrategicActionCommit"
27985	cc8bb7bd	20	2026-10-30 13:04:21.826125+00	{"EndTurn": {"player": "Bob"}}
27986	cc8bb7bd	21	2026-10-30 13:05:21.826125+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
27987	cc8bb7bd	22	2026-10-30 13:06:21.826125+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
27988	cc8bb7bd	23	2026-10-30 13:07:21.826125+00	"StrategicActionCommit"
27989	cc8bb7bd	24	2026-10-30 13:08:21.826125+00	{"EndTurn": {"player": "Carol"}}
27990	cc8bb7bd	25	2026-10-30 13:09:21.826125+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
27991	cc8bb7bd	26	2026-10-30 13:10:21.826125+00	"StrategicActionCommit"
27992	cc8bb7bd	27	2026-10-30 13:11:21.826125+00	{"EndTurn": {"player": "Alice"}}
27993	cc8bb7bd	28	2026-10-30 13:12:21.826125+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
27994	cc8bb7bd	29	2026-10-30 13:13:21.826125+00	"StrategicActionCommit"
27995	cc8bb7bd	30	2026-10-30 13:14:21.826125+00	{"EndTurn": {"player": "Bob"}}
27996	cc8bb7bd	31	2026-10-30 13:15:21.826125+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
27997	cc8bb7bd	32	2026-10-30 13:16:21.826125+00	"StrategicActionCommit"
27998	cc8bb7bd	33	2026-10-30 13:17:21.826125+00	{"EndTurn": {"player": "Carol"}}
27999	cc8bb7bd	34	2026-10-30 13:18:21.826125+00	{"PassAction": {"player": "Alice"}}
28000	cc8bb7bd	35	2026-10-30 13:19:21.826125+00	{"PassAction": {"player": "Bob"}}
28001	cc8bb7bd	36	2026-10-30 13:20:21.826125+00	{"PassAction": {"player": "Carol"}}
28002	cc8bb7bd	37	2026-10-30 13:21:21.826125+00	{"ScorePublicObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
28003	cc8bb7bd	38	2026-10-30 13:22:21.826125+00	{"ScoreSecretObjective": {"player": "Bob", "objective": "BecomeTheGatekeeper"}}
28004	cc8bb7bd	39	2026-10-30 13:23:21.826125+00	{"ScorePublicObjective": {"player": "Carol", "objective": "CornerTheMarket"}}
28005	cc8bb7bd	40	2026-10-30 13:24:21.826125+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28006	cc8bb7bd	41	2026-10-30 13:25:21.826125+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28007	cc8bb7bd	42	2026-10-30 13:26:21.826125+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28008	cc8bb7bd	43	2026-10-30 13:27:21.826125+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28009	cc8bb7bd	44	2026-10-30 13:28:21.826125+00	"CompleteStatusPhase"
28010	f19ebc78	0	2026-10-30 12:41:22.027897+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28011	f19ebc78	1	2026-10-30 12:42:22.027897+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28012	f19ebc78	2	2026-10-30 12:43:22.027897+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28013	f19ebc78	3	2026-10-30 12:44:22.027897+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28014	f19ebc78	4	2026-10-30 12:45:22.027897+00	"CreationDone"
28015	f19ebc78	5	2026-10-30 12:46:22.027897+00	{"SetupSpeaker": {"player": "Alice"}}
28016	f19ebc78	6	2026-10-30 12:47:22.027897+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28017	f19ebc78	7	2026-10-30 12:48:22.027897+00	"StartGame"
28018	f19ebc78	8	2026-10-30 12:49:22.027897+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28019	f19ebc78	9	2026-10-30 12:50:22.027897+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28020	f19ebc78	10	2026-10-30 12:51:22.027897+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28021	f19ebc78	11	2026-10-30 12:52:22.027897+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28022	f19ebc78	12	2026-10-30 12:53:22.027897+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28023	f19ebc78	13	2026-10-30 12:54:22.027897+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28024	f19ebc78	14	2026-10-30 12:55:22.027897+00	"CompleteStrategyPhase"
28025	f19ebc78	15	2026-10-30 12:56:22.027897+00	{"RevealExtraPublicObjective": {"objective": "ErectAMonument"}}
28026	f19ebc78	16	2026-10-30 12:57:22.027897+00	{"RevealExtraPublicObjective": {"objective": "ExpandBorders"}}
28027	f19ebc78	17	2026-10-30 12:58:22.027897+00	{"RevealExtraPublicObjective": {"objective": "SwayTheCouncil"}}
28028	f19ebc78	18	2026-10-30 12:59:22.027897+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28029	f19ebc78	19	2026-10-30 13:00:22.027897+00	"StrategicActionCommit"
28030	f19ebc78	20	2026-10-30 13:01:22.027897+00	{"EndTurn": {"player": "Alice"}}
28031	f19ebc78	21	2026-10-30 13:02:22.027897+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28032	f19ebc78	22	2026-10-30 13:03:22.027897+00	"StrategicActionCommit"
28033	f19ebc78	23	2026-10-30 13:04:22.027897+00	{"EndTurn": {"player": "Bob"}}
28034	f19ebc78	24	2026-10-30 13:05:22.027897+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28035	f19ebc78	25	2026-10-30 13:06:22.027897+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28036	f19ebc78	26	2026-10-30 13:07:22.027897+00	"StrategicActionCommit"
28037	f19ebc78	27	2026-10-30 13:08:22.027897+00	{"EndTurn": {"player": "Carol"}}
28038	f19ebc78	28	2026-10-30 13:09:22.027897+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28039	f19ebc78	29	2026-10-30 13:10:22.027897+00	"StrategicActionCommit"
28040	f19ebc78	30	2026-10-30 13:11:22.027897+00	{"EndTurn": {"player": "Alice"}}
28041	f19ebc78	31	2026-10-30 13:12:22.027897+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28042	f19ebc78	32	2026-10-30 13:13:22.027897+00	"StrategicActionCommit"
28043	f19ebc78	33	2026-10-30 13:14:22.027897+00	{"EndTurn": {"player": "Bob"}}
28044	f19ebc78	34	2026-10-30 13:15:22.027897+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28045	f19ebc78	35	2026-10-30 13:16:22.027897+00	"StrategicActionCommit"
28046	f19ebc78	36	2026-10-30 13:17:22.027897+00	{"EndTurn": {"player": "Carol"}}
28047	f19ebc78	37	2026-10-30 13:18:22.027897+00	{"PassAction": {"player": "Alice"}}
28048	f19ebc78	38	2026-10-30 13:19:22.027897+00	{"PassAction": {"player": "Bob"}}
28049	f19ebc78	39	2026-10-30 13:20:22.027897+00	{"PassAction": {"player": "Carol"}}
28050	f19ebc78	40	2026-10-30 13:21:22.027897+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28051	f19ebc78	41	2026-10-30 13:22:22.027897+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28052	f19ebc78	42	2026-10-30 13:23:22.027897+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28053	f19ebc78	43	2026-10-30 13:24:22.027897+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28054	f19ebc78	44	2026-10-30 13:25:22.027897+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28055	f19ebc78	45	2026-10-30 13:26:22.027897+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28056	f19ebc78	46	2026-10-30 13:27:22.027897+00	{"RevealPublicObjective": {"objective": "CentralizeGalacticTrade"}}
28057	f19ebc78	47	2026-10-30 13:28:22.027897+00	"CompleteStatusPhase"
28058	3b29b712	0	2026-10-30 12:34:22.270264+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28059	3b29b712	1	2026-10-30 12:35:22.270264+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28123	1ef78e6b	10	2026-10-30 12:41:22.53398+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28060	3b29b712	2	2026-10-30 12:36:22.270264+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28061	3b29b712	3	2026-10-30 12:37:22.270264+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28062	3b29b712	4	2026-10-30 12:38:22.270264+00	"CreationDone"
28063	3b29b712	5	2026-10-30 12:39:22.270264+00	{"SetupSpeaker": {"player": "Alice"}}
28064	3b29b712	6	2026-10-30 12:40:22.270264+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28065	3b29b712	7	2026-10-30 12:41:22.270264+00	"StartGame"
28066	3b29b712	8	2026-10-30 12:42:22.270264+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28067	3b29b712	9	2026-10-30 12:43:22.270264+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28068	3b29b712	10	2026-10-30 12:44:22.270264+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28069	3b29b712	11	2026-10-30 12:45:22.270264+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28070	3b29b712	12	2026-10-30 12:46:22.270264+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28071	3b29b712	13	2026-10-30 12:47:22.270264+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28072	3b29b712	14	2026-10-30 12:48:22.270264+00	"CompleteStrategyPhase"
28073	3b29b712	15	2026-10-30 12:49:22.270264+00	{"SetCustodians": {"player": "Alice"}}
28074	3b29b712	16	2026-10-30 12:50:22.270264+00	{"SetPlanetOwner": {"planet": "Centauri", "player": "Bob"}}
28075	3b29b712	17	2026-10-30 12:51:22.270264+00	{"AddPlanetAttachment": {"planet": "Centauri", "player": "Bob", "attachment": "TombOfEmphidia"}}
28076	3b29b712	18	2026-10-30 12:52:22.270264+00	{"GainRelicAction": {"relic": "MawOfWorlds", "player": "Alice"}}
28077	3b29b712	19	2026-10-30 12:53:22.270264+00	{"EndTurn": {"player": "Alice"}}
28078	3b29b712	20	2026-10-30 12:54:22.270264+00	{"GainRelicAction": {"relic": "TheCrownOfEmphidia", "player": "Bob"}}
28079	3b29b712	21	2026-10-30 12:55:22.270264+00	{"EndTurn": {"player": "Bob"}}
28080	3b29b712	22	2026-10-30 12:56:22.270264+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28081	3b29b712	23	2026-10-30 12:57:22.270264+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28082	3b29b712	24	2026-10-30 12:58:22.270264+00	"StrategicActionCommit"
28083	3b29b712	25	2026-10-30 12:59:22.270264+00	{"EndTurn": {"player": "Carol"}}
28084	3b29b712	26	2026-10-30 13:00:22.270264+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28085	3b29b712	27	2026-10-30 13:01:22.270264+00	"StrategicActionCommit"
28086	3b29b712	28	2026-10-30 13:02:22.270264+00	{"EndTurn": {"player": "Alice"}}
28087	3b29b712	29	2026-10-30 13:03:22.270264+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28088	3b29b712	30	2026-10-30 13:04:22.270264+00	"StrategicActionCommit"
28089	3b29b712	31	2026-10-30 13:05:22.270264+00	{"EndTurn": {"player": "Bob"}}
28090	3b29b712	32	2026-10-30 13:06:22.270264+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28091	3b29b712	33	2026-10-30 13:07:22.270264+00	"StrategicActionCommit"
28092	3b29b712	34	2026-10-30 13:08:22.270264+00	{"EndTurn": {"player": "Carol"}}
28093	3b29b712	35	2026-10-30 13:09:22.270264+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28094	3b29b712	36	2026-10-30 13:10:22.270264+00	"StrategicActionCommit"
28095	3b29b712	37	2026-10-30 13:11:22.270264+00	{"EndTurn": {"player": "Alice"}}
28096	3b29b712	38	2026-10-30 13:12:22.270264+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28097	3b29b712	39	2026-10-30 13:13:22.270264+00	"StrategicActionCommit"
28098	3b29b712	40	2026-10-30 13:14:22.270264+00	{"EndTurn": {"player": "Bob"}}
28099	3b29b712	41	2026-10-30 13:15:22.270264+00	{"PassAction": {"player": "Carol"}}
28100	3b29b712	42	2026-10-30 13:16:22.270264+00	{"PassAction": {"player": "Alice"}}
28101	3b29b712	43	2026-10-30 13:17:22.270264+00	{"PassAction": {"player": "Bob"}}
28102	3b29b712	44	2026-10-30 13:18:22.270264+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28103	3b29b712	45	2026-10-30 13:19:22.270264+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28104	3b29b712	46	2026-10-30 13:20:22.270264+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28105	3b29b712	47	2026-10-30 13:21:22.270264+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28106	3b29b712	48	2026-10-30 13:22:22.270264+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28107	3b29b712	49	2026-10-30 13:23:22.270264+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28108	3b29b712	50	2026-10-30 13:24:22.270264+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28109	3b29b712	51	2026-10-30 13:25:22.270264+00	"CompleteStatusPhase"
28110	3b29b712	52	2026-10-30 13:26:22.270264+00	{"PlayMawOfWorlds": {"tech": "GravityDrive", "player": "Alice"}}
28111	3b29b712	53	2026-10-30 13:27:22.270264+00	{"PlayCrownOfEmphidia": {"player": "Bob"}}
28112	3b29b712	54	2026-10-30 13:28:22.270264+00	"CompleteRelicsPhase"
28113	1ef78e6b	0	2026-10-30 12:31:22.53398+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28114	1ef78e6b	1	2026-10-30 12:32:22.53398+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28115	1ef78e6b	2	2026-10-30 12:33:22.53398+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28116	1ef78e6b	3	2026-10-30 12:34:22.53398+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28117	1ef78e6b	4	2026-10-30 12:35:22.53398+00	"CreationDone"
28118	1ef78e6b	5	2026-10-30 12:36:22.53398+00	{"SetupSpeaker": {"player": "Alice"}}
28119	1ef78e6b	6	2026-10-30 12:37:22.53398+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28120	1ef78e6b	7	2026-10-30 12:38:22.53398+00	"StartGame"
28121	1ef78e6b	8	2026-10-30 12:39:22.53398+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28122	1ef78e6b	9	2026-10-30 12:40:22.53398+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28124	1ef78e6b	11	2026-10-30 12:42:22.53398+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28125	1ef78e6b	12	2026-10-30 12:43:22.53398+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28126	1ef78e6b	13	2026-10-30 12:44:22.53398+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28127	1ef78e6b	14	2026-10-30 12:45:22.53398+00	"CompleteStrategyPhase"
28128	1ef78e6b	15	2026-10-30 12:46:22.53398+00	{"SetCustodians": {"player": "Alice"}}
28129	1ef78e6b	16	2026-10-30 12:47:22.53398+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28130	1ef78e6b	17	2026-10-30 12:48:22.53398+00	"StrategicActionCommit"
28131	1ef78e6b	18	2026-10-30 12:49:22.53398+00	{"EndTurn": {"player": "Alice"}}
28132	1ef78e6b	19	2026-10-30 12:50:22.53398+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28133	1ef78e6b	20	2026-10-30 12:51:22.53398+00	"StrategicActionCommit"
28134	1ef78e6b	21	2026-10-30 12:52:22.53398+00	{"EndTurn": {"player": "Bob"}}
28135	1ef78e6b	22	2026-10-30 12:53:22.53398+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28136	1ef78e6b	23	2026-10-30 12:54:22.53398+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28137	1ef78e6b	24	2026-10-30 12:55:22.53398+00	"StrategicActionCommit"
28138	1ef78e6b	25	2026-10-30 12:56:22.53398+00	{"EndTurn": {"player": "Carol"}}
28139	1ef78e6b	26	2026-10-30 12:57:22.53398+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28140	1ef78e6b	27	2026-10-30 12:58:22.53398+00	"StrategicActionCommit"
28141	1ef78e6b	28	2026-10-30 12:59:22.53398+00	{"EndTurn": {"player": "Alice"}}
28142	1ef78e6b	29	2026-10-30 13:00:22.53398+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28143	1ef78e6b	30	2026-10-30 13:01:22.53398+00	"StrategicActionCommit"
28144	1ef78e6b	31	2026-10-30 13:02:22.53398+00	{"EndTurn": {"player": "Bob"}}
28145	1ef78e6b	32	2026-10-30 13:03:22.53398+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28146	1ef78e6b	33	2026-10-30 13:04:22.53398+00	"StrategicActionCommit"
28147	1ef78e6b	34	2026-10-30 13:05:22.53398+00	{"EndTurn": {"player": "Carol"}}
28148	1ef78e6b	35	2026-10-30 13:06:22.53398+00	{"PassAction": {"player": "Alice"}}
28149	1ef78e6b	36	2026-10-30 13:07:22.53398+00	{"PassAction": {"player": "Bob"}}
28150	1ef78e6b	37	2026-10-30 13:08:22.53398+00	{"PassAction": {"player": "Carol"}}
28151	1ef78e6b	38	2026-10-30 13:09:22.53398+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28152	1ef78e6b	39	2026-10-30 13:10:22.53398+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28153	1ef78e6b	40	2026-10-30 13:11:22.53398+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28154	1ef78e6b	41	2026-10-30 13:12:22.53398+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28155	1ef78e6b	42	2026-10-30 13:13:22.53398+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28156	1ef78e6b	43	2026-10-30 13:14:22.53398+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28157	1ef78e6b	44	2026-10-30 13:15:22.53398+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28158	1ef78e6b	45	2026-10-30 13:16:22.53398+00	"CompleteStatusPhase"
28159	1ef78e6b	46	2026-10-30 13:17:22.53398+00	{"RevealAgenda": {"agenda": "ConventionsOfWar"}}
28160	1ef78e6b	47	2026-10-30 13:18:22.53398+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28161	1ef78e6b	48	2026-10-30 13:19:22.53398+00	{"CastAgendaVote": {"votes": 2, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28162	1ef78e6b	49	2026-10-30 13:20:22.53398+00	{"CastAgendaVote": {"votes": 2, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28163	1ef78e6b	50	2026-10-30 13:21:22.53398+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28164	1ef78e6b	51	2026-10-30 13:22:22.53398+00	{"RevealAgenda": {"agenda": "PoliticalCensure"}}
28165	1ef78e6b	52	2026-10-30 13:23:22.53398+00	{"CastAgendaVote": {"votes": 3, "player": "Bob", "outcome": {"value": "Carol", "electKind": "Player"}}}
28166	1ef78e6b	53	2026-10-30 13:24:22.53398+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Carol", "electKind": "Player"}}}
28167	1ef78e6b	54	2026-10-30 13:25:22.53398+00	{"CastAgendaVote": {"votes": 2, "player": "Alice", "outcome": {"value": "Alice", "electKind": "Player"}}}
28168	1ef78e6b	55	2026-10-30 13:26:22.53398+00	{"ResolveAgenda": {"outcome": {"value": "Carol", "electKind": "Player"}}}
28169	1ef78e6b	56	2026-10-30 13:27:22.53398+00	"CompleteAgendaPhase"
28170	1ef78e6b	57	2026-10-30 13:28:22.53398+00	{"RepealLaw": {"law": "PoliticalCensure"}}
28171	ae72c5d9	0	2026-10-30 12:33:22.808991+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28172	ae72c5d9	1	2026-10-30 12:34:22.808991+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28173	ae72c5d9	2	2026-10-30 12:35:22.808991+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28174	ae72c5d9	3	2026-10-30 12:36:22.808991+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28175	ae72c5d9	4	2026-10-30 12:37:22.808991+00	"CreationDone"
28176	ae72c5d9	5	2026-10-30 12:38:22.808991+00	{"SetupSpeaker": {"player": "Alice"}}
28177	ae72c5d9	6	2026-10-30 12:39:22.808991+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28178	ae72c5d9	7	2026-10-30 12:40:22.808991+00	"StartGame"
28179	ae72c5d9	8	2026-10-30 12:41:22.808991+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28180	ae72c5d9	9	2026-10-30 12:42:22.808991+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28181	ae72c5d9	10	2026-10-30 12:43:22.808991+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28182	ae72c5d9	11	2026-10-30 12:44:22.808991+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28183	ae72c5d9	12	2026-10-30 12:45:22.808991+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28184	ae72c5d9	13	2026-10-30 12:46:22.808991+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28185	ae72c5d9	14	2026-10-30 12:47:22.808991+00	"CompleteStrategyPhase"
28186	ae72c5d9	15	2026-10-30 12:48:22.808991+00	{"SetCustodians": {"player": "Alice"}}
28187	ae72c5d9	16	2026-10-30 12:49:22.808991+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28188	ae72c5d9	17	2026-10-30 12:50:22.808991+00	"StrategicActionCommit"
28189	ae72c5d9	18	2026-10-30 12:51:22.808991+00	{"EndTurn": {"player": "Alice"}}
28190	ae72c5d9	19	2026-10-30 12:52:22.808991+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28191	ae72c5d9	20	2026-10-30 12:53:22.808991+00	"StrategicActionCommit"
28192	ae72c5d9	21	2026-10-30 12:54:22.808991+00	{"EndTurn": {"player": "Bob"}}
28193	ae72c5d9	22	2026-10-30 12:55:22.808991+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28194	ae72c5d9	23	2026-10-30 12:56:22.808991+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28195	ae72c5d9	24	2026-10-30 12:57:22.808991+00	"StrategicActionCommit"
28196	ae72c5d9	25	2026-10-30 12:58:22.808991+00	{"EndTurn": {"player": "Carol"}}
28197	ae72c5d9	26	2026-10-30 12:59:22.808991+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28198	ae72c5d9	27	2026-10-30 13:00:22.808991+00	"StrategicActionCommit"
28199	ae72c5d9	28	2026-10-30 13:01:22.808991+00	{"EndTurn": {"player": "Alice"}}
28200	ae72c5d9	29	2026-10-30 13:02:22.808991+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28201	ae72c5d9	30	2026-10-30 13:03:22.808991+00	"StrategicActionCommit"
28202	ae72c5d9	31	2026-10-30 13:04:22.808991+00	{"EndTurn": {"player": "Bob"}}
28203	ae72c5d9	32	2026-10-30 13:05:22.808991+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28204	ae72c5d9	33	2026-10-30 13:06:22.808991+00	"StrategicActionCommit"
28205	ae72c5d9	34	2026-10-30 13:07:22.808991+00	{"EndTurn": {"player": "Carol"}}
28206	ae72c5d9	35	2026-10-30 13:08:22.808991+00	{"PassAction": {"player": "Alice"}}
28207	ae72c5d9	36	2026-10-30 13:09:22.808991+00	{"PassAction": {"player": "Bob"}}
28208	ae72c5d9	37	2026-10-30 13:10:22.808991+00	{"PassAction": {"player": "Carol"}}
28209	ae72c5d9	38	2026-10-30 13:11:22.808991+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28210	ae72c5d9	39	2026-10-30 13:12:22.808991+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28211	ae72c5d9	40	2026-10-30 13:13:22.808991+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28212	ae72c5d9	41	2026-10-30 13:14:22.808991+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28213	ae72c5d9	42	2026-10-30 13:15:22.808991+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28214	ae72c5d9	43	2026-10-30 13:16:22.808991+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28215	ae72c5d9	44	2026-10-30 13:17:22.808991+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28216	ae72c5d9	45	2026-10-30 13:18:22.808991+00	"CompleteStatusPhase"
28217	ae72c5d9	46	2026-10-30 13:19:22.808991+00	{"RevealAgenda": {"agenda": "FleetRegulations"}}
28218	ae72c5d9	47	2026-10-30 13:20:22.808991+00	"VetoAgenda"
28219	ae72c5d9	48	2026-10-30 13:21:22.808991+00	{"RevealAgenda": {"agenda": "ConventionsOfWar"}}
28220	ae72c5d9	49	2026-10-30 13:22:22.808991+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28221	ae72c5d9	50	2026-10-30 13:23:22.808991+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28222	ae72c5d9	51	2026-10-30 13:24:22.808991+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28223	ae72c5d9	52	2026-10-30 13:25:22.808991+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28224	ae72c5d9	53	2026-10-30 13:26:22.808991+00	{"RevealAgenda": {"agenda": "MinisterOfWar"}}
28225	ae72c5d9	54	2026-10-30 13:27:22.808991+00	{"ResolveAgenda": {"outcome": null}}
28226	ae72c5d9	55	2026-10-30 13:28:22.808991+00	"CompleteAgendaPhase"
28227	27d0c2d4	0	2026-10-30 12:34:23.047684+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28228	27d0c2d4	1	2026-10-30 12:35:23.047684+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28229	27d0c2d4	2	2026-10-30 12:36:23.047684+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28230	27d0c2d4	3	2026-10-30 12:37:23.047684+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "NekroVirus"}}}
28231	27d0c2d4	4	2026-10-30 12:38:23.047684+00	"CreationDone"
28232	27d0c2d4	5	2026-10-30 12:39:23.047684+00	{"SetupSpeaker": {"player": "Alice"}}
28233	27d0c2d4	6	2026-10-30 12:40:23.047684+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28234	27d0c2d4	7	2026-10-30 12:41:23.047684+00	"StartGame"
28235	27d0c2d4	8	2026-10-30 12:42:23.047684+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28236	27d0c2d4	9	2026-10-30 12:43:23.047684+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28237	27d0c2d4	10	2026-10-30 12:44:23.047684+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28238	27d0c2d4	11	2026-10-30 12:45:23.047684+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28239	27d0c2d4	12	2026-10-30 12:46:23.047684+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28240	27d0c2d4	13	2026-10-30 12:47:23.047684+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28241	27d0c2d4	14	2026-10-30 12:48:23.047684+00	"CompleteStrategyPhase"
28242	27d0c2d4	15	2026-10-30 12:49:23.047684+00	{"SetCustodians": {"player": "Alice"}}
28243	27d0c2d4	16	2026-10-30 12:50:23.047684+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28244	27d0c2d4	17	2026-10-30 12:51:23.047684+00	"StrategicActionCommit"
28245	27d0c2d4	18	2026-10-30 12:52:23.047684+00	{"EndTurn": {"player": "Alice"}}
28246	27d0c2d4	19	2026-10-30 12:53:23.047684+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28307	9a5c9ddc	25	2026-10-30 12:59:23.292805+00	{"EndTurn": {"player": "Carol"}}
28308	9a5c9ddc	26	2026-10-30 13:00:23.292805+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28309	9a5c9ddc	27	2026-10-30 13:01:23.292805+00	"StrategicActionCommit"
28310	9a5c9ddc	28	2026-10-30 13:02:23.292805+00	{"EndTurn": {"player": "Alice"}}
28311	9a5c9ddc	29	2026-10-30 13:03:23.292805+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28312	9a5c9ddc	30	2026-10-30 13:04:23.292805+00	"StrategicActionCommit"
28313	9a5c9ddc	31	2026-10-30 13:05:23.292805+00	{"EndTurn": {"player": "Bob"}}
28314	9a5c9ddc	32	2026-10-30 13:06:23.292805+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28315	9a5c9ddc	33	2026-10-30 13:07:23.292805+00	"StrategicActionCommit"
28316	9a5c9ddc	34	2026-10-30 13:08:23.292805+00	{"EndTurn": {"player": "Carol"}}
28317	9a5c9ddc	35	2026-10-30 13:09:23.292805+00	{"PassAction": {"player": "Alice"}}
28318	9a5c9ddc	36	2026-10-30 13:10:23.292805+00	{"PassAction": {"player": "Bob"}}
28319	9a5c9ddc	37	2026-10-30 13:11:23.292805+00	{"PassAction": {"player": "Carol"}}
28320	9a5c9ddc	38	2026-10-30 13:12:23.292805+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28321	9a5c9ddc	39	2026-10-30 13:13:23.292805+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28322	9a5c9ddc	40	2026-10-30 13:14:23.292805+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28323	9a5c9ddc	41	2026-10-30 13:15:23.292805+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28324	9a5c9ddc	42	2026-10-30 13:16:23.292805+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28325	9a5c9ddc	43	2026-10-30 13:17:23.292805+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28326	9a5c9ddc	44	2026-10-30 13:18:23.292805+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28327	9a5c9ddc	45	2026-10-30 13:19:23.292805+00	"CompleteStatusPhase"
28328	9a5c9ddc	46	2026-10-30 13:20:23.292805+00	{"AddExtraPoints": {"value": 2, "player": "Bob"}}
28329	9a5c9ddc	47	2026-10-30 13:21:23.292805+00	{"RevealAgenda": {"agenda": "Mutiny"}}
28330	9a5c9ddc	48	2026-10-30 13:22:23.292805+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28331	9a5c9ddc	49	2026-10-30 13:23:23.292805+00	{"CastAgendaVote": {"votes": 3, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28332	9a5c9ddc	50	2026-10-30 13:24:23.292805+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28333	9a5c9ddc	51	2026-10-30 13:25:23.292805+00	{"RevealAgenda": {"agenda": "SeedOfAnEmpire"}}
28334	9a5c9ddc	52	2026-10-30 13:26:23.292805+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28335	9a5c9ddc	53	2026-10-30 13:27:23.292805+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28336	9a5c9ddc	54	2026-10-30 13:28:23.292805+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
28337	49897d7d	0	2026-10-30 11:48:23.559357+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28338	49897d7d	1	2026-10-30 11:49:23.559357+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28339	49897d7d	2	2026-10-30 11:50:23.559357+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28340	49897d7d	3	2026-10-30 11:51:23.559357+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28341	49897d7d	4	2026-10-30 11:52:23.559357+00	"CreationDone"
28342	49897d7d	5	2026-10-30 11:53:23.559357+00	{"SetupSpeaker": {"player": "Alice"}}
28343	49897d7d	6	2026-10-30 11:54:23.559357+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28344	49897d7d	7	2026-10-30 11:55:23.559357+00	"StartGame"
28345	49897d7d	8	2026-10-30 11:56:23.559357+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28346	49897d7d	9	2026-10-30 11:57:23.559357+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28347	49897d7d	10	2026-10-30 11:58:23.559357+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28348	49897d7d	11	2026-10-30 11:59:23.559357+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28349	49897d7d	12	2026-10-30 12:00:23.559357+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28350	49897d7d	13	2026-10-30 12:01:23.559357+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28351	49897d7d	14	2026-10-30 12:02:23.559357+00	"CompleteStrategyPhase"
28352	49897d7d	15	2026-10-30 12:03:23.559357+00	{"SetCustodians": {"player": "Alice"}}
28353	49897d7d	16	2026-10-30 12:04:23.559357+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28354	49897d7d	17	2026-10-30 12:05:23.559357+00	"StrategicActionCommit"
28355	49897d7d	18	2026-10-30 12:06:23.559357+00	{"EndTurn": {"player": "Alice"}}
28356	49897d7d	19	2026-10-30 12:07:23.559357+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28357	49897d7d	20	2026-10-30 12:08:23.559357+00	"StrategicActionCommit"
28358	49897d7d	21	2026-10-30 12:09:23.559357+00	{"EndTurn": {"player": "Bob"}}
28359	49897d7d	22	2026-10-30 12:10:23.559357+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28360	49897d7d	23	2026-10-30 12:11:23.559357+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28361	49897d7d	24	2026-10-30 12:12:23.559357+00	"StrategicActionCommit"
28362	49897d7d	25	2026-10-30 12:13:23.559357+00	{"EndTurn": {"player": "Carol"}}
28363	49897d7d	26	2026-10-30 12:14:23.559357+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28364	49897d7d	27	2026-10-30 12:15:23.559357+00	"StrategicActionCommit"
28365	49897d7d	28	2026-10-30 12:16:23.559357+00	{"EndTurn": {"player": "Alice"}}
28366	49897d7d	29	2026-10-30 12:17:23.559357+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28367	49897d7d	30	2026-10-30 12:18:23.559357+00	"StrategicActionCommit"
28368	49897d7d	31	2026-10-30 12:19:23.559357+00	{"EndTurn": {"player": "Bob"}}
28369	49897d7d	32	2026-10-30 12:20:23.559357+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28370	49897d7d	33	2026-10-30 12:21:23.559357+00	"StrategicActionCommit"
28371	49897d7d	34	2026-10-30 12:22:23.559357+00	{"EndTurn": {"player": "Carol"}}
28372	49897d7d	35	2026-10-30 12:23:23.559357+00	{"PassAction": {"player": "Alice"}}
28373	49897d7d	36	2026-10-30 12:24:23.559357+00	{"PassAction": {"player": "Bob"}}
28374	49897d7d	37	2026-10-30 12:25:23.559357+00	{"PassAction": {"player": "Carol"}}
28375	49897d7d	38	2026-10-30 12:26:23.559357+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28376	49897d7d	39	2026-10-30 12:27:23.559357+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28377	49897d7d	40	2026-10-30 12:28:23.559357+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28378	49897d7d	41	2026-10-30 12:29:23.559357+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28379	49897d7d	42	2026-10-30 12:30:23.559357+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28380	49897d7d	43	2026-10-30 12:31:23.559357+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28381	49897d7d	44	2026-10-30 12:32:23.559357+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28382	49897d7d	45	2026-10-30 12:33:23.559357+00	"CompleteStatusPhase"
28383	49897d7d	46	2026-10-30 12:34:23.559357+00	{"ScoreExtraSecretObjective": {"player": "Carol", "objective": "BecomeTheGatekeeper"}}
28384	49897d7d	47	2026-10-30 12:35:23.559357+00	{"ScoreExtraSecretObjective": {"player": "Bob", "objective": "AdaptNewStrategies"}}
28385	49897d7d	48	2026-10-30 12:36:23.559357+00	{"RevealAgenda": {"agenda": "ConventionsOfWar"}}
28386	49897d7d	49	2026-10-30 12:37:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28387	49897d7d	50	2026-10-30 12:38:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28388	49897d7d	51	2026-10-30 12:39:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28389	49897d7d	52	2026-10-30 12:40:23.559357+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28390	49897d7d	53	2026-10-30 12:41:23.559357+00	{"RevealAgenda": {"agenda": "ClassifiedDocumentLeaks"}}
28391	49897d7d	54	2026-10-30 12:42:23.559357+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "BecomeTheGatekeeper", "electKind": "SecretObjective"}}}
28392	49897d7d	55	2026-10-30 12:43:23.559357+00	{"CastAgendaVote": {"votes": 0, "player": "Carol", "outcome": null}}
28393	49897d7d	56	2026-10-30 12:44:23.559357+00	{"CastAgendaVote": {"votes": 0, "player": "Alice", "outcome": null}}
28394	49897d7d	57	2026-10-30 12:45:23.559357+00	{"ResolveAgenda": {"outcome": {"value": "BecomeTheGatekeeper", "electKind": "SecretObjective"}}}
28395	49897d7d	58	2026-10-30 12:46:23.559357+00	"CompleteAgendaPhase"
28396	49897d7d	59	2026-10-30 12:47:23.559357+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28397	49897d7d	60	2026-10-30 12:48:23.559357+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28398	49897d7d	61	2026-10-30 12:49:23.559357+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28399	49897d7d	62	2026-10-30 12:50:23.559357+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28400	49897d7d	63	2026-10-30 12:51:23.559357+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28401	49897d7d	64	2026-10-30 12:52:23.559357+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28402	49897d7d	65	2026-10-30 12:53:23.559357+00	"CompleteStrategyPhase"
28403	49897d7d	66	2026-10-30 12:54:23.559357+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28404	49897d7d	67	2026-10-30 12:55:23.559357+00	"StrategicActionCommit"
28405	49897d7d	68	2026-10-30 12:56:23.559357+00	{"EndTurn": {"player": "Alice"}}
28406	49897d7d	69	2026-10-30 12:57:23.559357+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28407	49897d7d	70	2026-10-30 12:58:23.559357+00	"StrategicActionCommit"
28408	49897d7d	71	2026-10-30 12:59:23.559357+00	{"EndTurn": {"player": "Bob"}}
28409	49897d7d	72	2026-10-30 13:00:23.559357+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28410	49897d7d	73	2026-10-30 13:01:23.559357+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28411	49897d7d	74	2026-10-30 13:02:23.559357+00	"StrategicActionCommit"
28412	49897d7d	75	2026-10-30 13:03:23.559357+00	{"EndTurn": {"player": "Carol"}}
28413	49897d7d	76	2026-10-30 13:04:23.559357+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28414	49897d7d	77	2026-10-30 13:05:23.559357+00	"StrategicActionCommit"
28415	49897d7d	78	2026-10-30 13:06:23.559357+00	{"EndTurn": {"player": "Alice"}}
28416	49897d7d	79	2026-10-30 13:07:23.559357+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28417	49897d7d	80	2026-10-30 13:08:23.559357+00	"StrategicActionCommit"
28418	49897d7d	81	2026-10-30 13:09:23.559357+00	{"EndTurn": {"player": "Bob"}}
28419	49897d7d	82	2026-10-30 13:10:23.559357+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28420	49897d7d	83	2026-10-30 13:11:23.559357+00	"StrategicActionCommit"
28421	49897d7d	84	2026-10-30 13:12:23.559357+00	{"EndTurn": {"player": "Carol"}}
28422	49897d7d	85	2026-10-30 13:13:23.559357+00	{"PassAction": {"player": "Alice"}}
28423	49897d7d	86	2026-10-30 13:14:23.559357+00	{"PassAction": {"player": "Bob"}}
28424	49897d7d	87	2026-10-30 13:15:23.559357+00	{"PassAction": {"player": "Carol"}}
28425	49897d7d	88	2026-10-30 13:16:23.559357+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28426	49897d7d	89	2026-10-30 13:17:23.559357+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28427	49897d7d	90	2026-10-30 13:18:23.559357+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28428	49897d7d	91	2026-10-30 13:19:23.559357+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28429	49897d7d	92	2026-10-30 13:20:23.559357+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28430	49897d7d	93	2026-10-30 13:21:23.559357+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28431	49897d7d	94	2026-10-30 13:22:23.559357+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
28432	49897d7d	95	2026-10-30 13:23:23.559357+00	"CompleteStatusPhase"
28433	49897d7d	96	2026-10-30 13:24:23.559357+00	{"RevealAgenda": {"agenda": "JudicialAbolishment"}}
28434	49897d7d	97	2026-10-30 13:25:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "ConventionsOfWar", "electKind": "Law"}}}
28435	49897d7d	98	2026-10-30 13:26:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "ConventionsOfWar", "electKind": "Law"}}}
28436	49897d7d	99	2026-10-30 13:27:23.559357+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "ConventionsOfWar", "electKind": "Law"}}}
28437	49897d7d	100	2026-10-30 13:28:23.559357+00	{"ResolveAgenda": {"outcome": {"value": "ConventionsOfWar", "electKind": "Law"}}}
28438	d624536b	0	2026-10-30 13:08:24.022789+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28439	d624536b	1	2026-10-30 13:09:24.022789+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28440	d624536b	2	2026-10-30 13:10:24.022789+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28441	d624536b	3	2026-10-30 13:11:24.022789+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28442	d624536b	4	2026-10-30 13:12:24.022789+00	"CreationDone"
28443	d624536b	5	2026-10-30 13:13:24.022789+00	{"SetupSpeaker": {"player": "Alice"}}
28444	d624536b	6	2026-10-30 13:14:24.022789+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28445	d624536b	7	2026-10-30 13:15:24.022789+00	"StartGame"
28446	d624536b	8	2026-10-30 13:16:24.022789+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28447	d624536b	9	2026-10-30 13:17:24.022789+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28448	d624536b	10	2026-10-30 13:18:24.022789+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28449	d624536b	11	2026-10-30 13:19:24.022789+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28450	d624536b	12	2026-10-30 13:20:24.022789+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28451	d624536b	13	2026-10-30 13:21:24.022789+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28452	d624536b	14	2026-10-30 13:22:24.022789+00	"CompleteStrategyPhase"
28453	d624536b	15	2026-10-30 13:23:24.022789+00	{"AddAgendaBegin": {"agenda": "Mutiny"}}
28454	d624536b	16	2026-10-30 13:24:24.022789+00	{"AddAgendaPlayerVote": {"votes": 2, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28455	d624536b	17	2026-10-30 13:25:24.022789+00	"AddAgendaCancel"
28456	d624536b	18	2026-10-30 13:26:24.022789+00	{"AddAgendaBegin": {"agenda": "MinisterOfWar"}}
28457	d624536b	19	2026-10-30 13:27:24.022789+00	{"AddAgendaPlayerVote": {"votes": 2, "player": "Bob", "outcome": {"value": "Bob", "electKind": "Player"}}}
28458	d624536b	20	2026-10-30 13:28:24.022789+00	{"AddAgendaResolve": {"electedOutcome": {"value": "Bob", "electKind": "Player"}}}
28459	d0a685ae	0	2026-10-30 12:30:24.116562+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
28460	d0a685ae	1	2026-10-30 12:31:24.116562+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28461	d0a685ae	2	2026-10-30 12:32:24.116562+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28462	d0a685ae	3	2026-10-30 12:33:24.116562+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28463	d0a685ae	4	2026-10-30 12:34:24.116562+00	"CreationDone"
28464	d0a685ae	5	2026-10-30 12:35:24.116562+00	{"SetupSpeaker": {"player": "Alice"}}
28465	d0a685ae	6	2026-10-30 12:36:24.116562+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28466	d0a685ae	7	2026-10-30 12:37:24.116562+00	"StartGame"
28467	d0a685ae	8	2026-10-30 12:38:24.116562+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28468	d0a685ae	9	2026-10-30 12:39:24.116562+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28469	d0a685ae	10	2026-10-30 12:40:24.116562+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28470	d0a685ae	11	2026-10-30 12:41:24.116562+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28471	d0a685ae	12	2026-10-30 12:42:24.116562+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28472	d0a685ae	13	2026-10-30 12:43:24.116562+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28473	d0a685ae	14	2026-10-30 12:44:24.116562+00	"CompleteStrategyPhase"
28474	d0a685ae	15	2026-10-30 12:45:24.116562+00	{"SetCustodians": {"player": "Alice"}}
28475	d0a685ae	16	2026-10-30 12:46:24.116562+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28476	d0a685ae	17	2026-10-30 12:47:24.116562+00	"StrategicActionCommit"
28477	d0a685ae	18	2026-10-30 12:48:24.116562+00	{"EndTurn": {"player": "Alice"}}
28478	d0a685ae	19	2026-10-30 12:49:24.116562+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28479	d0a685ae	20	2026-10-30 12:50:24.116562+00	"StrategicActionCommit"
28480	d0a685ae	21	2026-10-30 12:51:24.116562+00	{"EndTurn": {"player": "Bob"}}
28481	d0a685ae	22	2026-10-30 12:52:24.116562+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28482	d0a685ae	23	2026-10-30 12:53:24.116562+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28483	d0a685ae	24	2026-10-30 12:54:24.116562+00	"StrategicActionCommit"
28484	d0a685ae	25	2026-10-30 12:55:24.116562+00	{"EndTurn": {"player": "Carol"}}
28485	d0a685ae	26	2026-10-30 12:56:24.116562+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28486	d0a685ae	27	2026-10-30 12:57:24.116562+00	"StrategicActionCommit"
28487	d0a685ae	28	2026-10-30 12:58:24.116562+00	{"EndTurn": {"player": "Alice"}}
28488	d0a685ae	29	2026-10-30 12:59:24.116562+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28489	d0a685ae	30	2026-10-30 13:00:24.116562+00	"StrategicActionCommit"
28490	d0a685ae	31	2026-10-30 13:01:24.116562+00	{"EndTurn": {"player": "Bob"}}
28491	d0a685ae	32	2026-10-30 13:02:24.116562+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28492	d0a685ae	33	2026-10-30 13:03:24.116562+00	"StrategicActionCommit"
28493	d0a685ae	34	2026-10-30 13:04:24.116562+00	{"EndTurn": {"player": "Carol"}}
28494	d0a685ae	35	2026-10-30 13:05:24.116562+00	{"PassAction": {"player": "Alice"}}
28495	d0a685ae	36	2026-10-30 13:06:24.116562+00	{"PassAction": {"player": "Bob"}}
28496	d0a685ae	37	2026-10-30 13:07:24.116562+00	{"PassAction": {"player": "Carol"}}
28497	d0a685ae	38	2026-10-30 13:08:24.116562+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28498	d0a685ae	39	2026-10-30 13:09:24.116562+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28499	d0a685ae	40	2026-10-30 13:10:24.116562+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28500	d0a685ae	41	2026-10-30 13:11:24.116562+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28501	d0a685ae	42	2026-10-30 13:12:24.116562+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28502	d0a685ae	43	2026-10-30 13:13:24.116562+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28503	d0a685ae	44	2026-10-30 13:14:24.116562+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28504	d0a685ae	45	2026-10-30 13:15:24.116562+00	"CompleteStatusPhase"
28505	d0a685ae	46	2026-10-30 13:16:24.116562+00	{"SetPlanetOwner": {"planet": "Quann", "player": "Alice"}}
28506	d0a685ae	47	2026-10-30 13:17:24.116562+00	{"SetPlanetOwner": {"planet": "Arnor", "player": "Bob"}}
28507	d0a685ae	48	2026-10-30 13:18:24.116562+00	{"AddPlanetAttachment": {"planet": "Arnor", "player": "Bob", "attachment": "Terraform"}}
28508	d0a685ae	49	2026-10-30 13:19:24.116562+00	{"SetPlanetOwner": {"planet": "Wellon", "player": "Carol"}}
28509	d0a685ae	50	2026-10-30 13:20:24.116562+00	{"RevealAgenda": {"agenda": "HolyPlanetOfIxth"}}
28510	d0a685ae	51	2026-10-30 13:21:24.116562+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Quann", "electKind": "CulturalPlanet"}}}
28511	d0a685ae	52	2026-10-30 13:22:24.116562+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Quann", "electKind": "CulturalPlanet"}}}
28512	d0a685ae	53	2026-10-30 13:23:24.116562+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Quann", "electKind": "CulturalPlanet"}}}
28513	d0a685ae	54	2026-10-30 13:24:24.116562+00	{"ResolveAgenda": {"outcome": {"value": "Quann", "electKind": "CulturalPlanet"}}}
28514	d0a685ae	55	2026-10-30 13:25:24.116562+00	{"SetPlanetOwner": {"planet": "Quann", "player": null}}
28515	d0a685ae	56	2026-10-30 13:26:24.116562+00	{"RevealAgenda": {"agenda": "ResearchTeamCybernetic"}}
28516	d0a685ae	57	2026-10-30 13:27:24.116562+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "Wellon", "electKind": "IndustrialPlanet"}}}
28517	d0a685ae	58	2026-10-30 13:28:24.116562+00	{"ResolveAgenda": {"outcome": {"value": "Wellon", "electKind": "IndustrialPlanet"}}}
28518	2f2807a6	0	2026-10-30 12:20:24.387337+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28519	2f2807a6	1	2026-10-30 12:21:24.387337+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28520	2f2807a6	2	2026-10-30 12:22:24.387337+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28521	2f2807a6	3	2026-10-30 12:23:24.387337+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28522	2f2807a6	4	2026-10-30 12:24:24.387337+00	"CreationDone"
28523	2f2807a6	5	2026-10-30 12:25:24.387337+00	{"SetupSpeaker": {"player": "Alice"}}
28524	2f2807a6	6	2026-10-30 12:26:24.387337+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28525	2f2807a6	7	2026-10-30 12:27:24.387337+00	"StartGame"
28526	2f2807a6	8	2026-10-30 12:28:24.387337+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28527	2f2807a6	9	2026-10-30 12:29:24.387337+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28528	2f2807a6	10	2026-10-30 12:30:24.387337+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28529	2f2807a6	11	2026-10-30 12:31:24.387337+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28530	2f2807a6	12	2026-10-30 12:32:24.387337+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28531	2f2807a6	13	2026-10-30 12:33:24.387337+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28532	2f2807a6	14	2026-10-30 12:34:24.387337+00	"CompleteStrategyPhase"
28533	2f2807a6	15	2026-10-30 12:35:24.387337+00	{"SetCustodians": {"player": "Alice"}}
28534	2f2807a6	16	2026-10-30 12:36:24.387337+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28535	2f2807a6	17	2026-10-30 12:37:24.387337+00	"StrategicActionCommit"
28536	2f2807a6	18	2026-10-30 12:38:24.387337+00	{"EndTurn": {"player": "Alice"}}
28537	2f2807a6	19	2026-10-30 12:39:24.387337+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28538	2f2807a6	20	2026-10-30 12:40:24.387337+00	"StrategicActionCommit"
28539	2f2807a6	21	2026-10-30 12:41:24.387337+00	{"EndTurn": {"player": "Bob"}}
28540	2f2807a6	22	2026-10-30 12:42:24.387337+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28541	2f2807a6	23	2026-10-30 12:43:24.387337+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28542	2f2807a6	24	2026-10-30 12:44:24.387337+00	"StrategicActionCommit"
28543	2f2807a6	25	2026-10-30 12:45:24.387337+00	{"EndTurn": {"player": "Carol"}}
28544	2f2807a6	26	2026-10-30 12:46:24.387337+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28545	2f2807a6	27	2026-10-30 12:47:24.387337+00	"StrategicActionCommit"
28546	2f2807a6	28	2026-10-30 12:48:24.387337+00	{"EndTurn": {"player": "Alice"}}
28547	2f2807a6	29	2026-10-30 12:49:24.387337+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28549	2f2807a6	31	2026-10-30 12:51:24.387337+00	{"EndTurn": {"player": "Bob"}}
28550	2f2807a6	32	2026-10-30 12:52:24.387337+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28551	2f2807a6	33	2026-10-30 12:53:24.387337+00	"StrategicActionCommit"
28552	2f2807a6	34	2026-10-30 12:54:24.387337+00	{"EndTurn": {"player": "Carol"}}
28553	2f2807a6	35	2026-10-30 12:55:24.387337+00	{"PassAction": {"player": "Alice"}}
28554	2f2807a6	36	2026-10-30 12:56:24.387337+00	{"PassAction": {"player": "Bob"}}
28555	2f2807a6	37	2026-10-30 12:57:24.387337+00	{"PassAction": {"player": "Carol"}}
28556	2f2807a6	38	2026-10-30 12:58:24.387337+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
28557	2f2807a6	39	2026-10-30 12:59:24.387337+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
28558	2f2807a6	40	2026-10-30 13:00:24.387337+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
28559	2f2807a6	41	2026-10-30 13:01:24.387337+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
28560	2f2807a6	42	2026-10-30 13:02:24.387337+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
28561	2f2807a6	43	2026-10-30 13:03:24.387337+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
28562	2f2807a6	44	2026-10-30 13:04:24.387337+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
28563	2f2807a6	45	2026-10-30 13:05:24.387337+00	"CompleteStatusPhase"
28564	2f2807a6	46	2026-10-30 13:06:24.387337+00	{"SetRelicOwner": {"relic": "StellarConverter", "player": "Carol"}}
28565	2f2807a6	47	2026-10-30 13:07:24.387337+00	{"RevealAgenda": {"agenda": "MinisterOfAntiques"}}
28566	2f2807a6	48	2026-10-30 13:08:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Bob", "electKind": "Player"}}}
28567	2f2807a6	49	2026-10-30 13:09:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Bob", "electKind": "Player"}}}
28568	2f2807a6	50	2026-10-30 13:10:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Bob", "electKind": "Player"}}}
28569	2f2807a6	51	2026-10-30 13:11:24.387337+00	{"ResolveAgenda": {"outcome": {"value": "Bob", "electKind": "Player"}}}
28570	2f2807a6	52	2026-10-30 13:12:24.387337+00	{"GainMinisterOfAntiquesRelic": {"relic": "TheCodex", "player": "Bob"}}
28571	2f2807a6	53	2026-10-30 13:13:24.387337+00	{"RevealAgenda": {"agenda": "ConventionsOfWar"}}
28572	2f2807a6	54	2026-10-30 13:14:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28573	2f2807a6	55	2026-10-30 13:15:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28574	2f2807a6	56	2026-10-30 13:16:24.387337+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28575	2f2807a6	57	2026-10-30 13:17:24.387337+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
28576	2f2807a6	58	2026-10-30 13:18:24.387337+00	"CompleteAgendaPhase"
28577	2f2807a6	59	2026-10-30 13:19:24.387337+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28578	2f2807a6	60	2026-10-30 13:20:24.387337+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28579	2f2807a6	61	2026-10-30 13:21:24.387337+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28580	2f2807a6	62	2026-10-30 13:22:24.387337+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28581	2f2807a6	63	2026-10-30 13:23:24.387337+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28582	2f2807a6	64	2026-10-30 13:24:24.387337+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28583	2f2807a6	65	2026-10-30 13:25:24.387337+00	"CompleteStrategyPhase"
28584	2f2807a6	66	2026-10-30 13:26:24.387337+00	{"AddAgendaBegin": {"agenda": "MinisterOfAntiques"}}
28585	2f2807a6	67	2026-10-30 13:27:24.387337+00	{"AddAgendaResolve": {"electedOutcome": {"value": "Carol", "electKind": "Player"}}}
28586	2f2807a6	68	2026-10-30 13:28:24.387337+00	{"GainMinisterOfAntiquesRelic": {"relic": null, "player": "Carol"}}
28587	9b227a4f	0	2026-10-30 13:03:24.697466+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28588	9b227a4f	1	2026-10-30 13:04:24.697466+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28589	9b227a4f	2	2026-10-30 13:05:24.697466+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28590	9b227a4f	3	2026-10-30 13:06:24.697466+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28591	9b227a4f	4	2026-10-30 13:07:24.697466+00	"CreationDone"
28592	9b227a4f	5	2026-10-30 13:08:24.697466+00	{"SetupSpeaker": {"player": "Alice"}}
28593	9b227a4f	6	2026-10-30 13:09:24.697466+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28594	9b227a4f	7	2026-10-30 13:10:24.697466+00	"StartGame"
28595	9b227a4f	8	2026-10-30 13:11:24.697466+00	{"SetCustodians": {"player": "Alice"}}
28596	9b227a4f	9	2026-10-30 13:12:24.697466+00	{"GiveSupportForTheThrone": {"giver": "Bob", "receiver": "Alice"}}
28597	9b227a4f	10	2026-10-30 13:13:24.697466+00	{"GiveSupportForTheThrone": {"giver": "Carol", "receiver": "Alice"}}
28598	9b227a4f	11	2026-10-30 13:14:24.697466+00	{"GiveSupportForTheThrone": {"giver": "Bob", "receiver": "Carol"}}
28599	9b227a4f	12	2026-10-30 13:15:24.697466+00	{"AddExtraPoints": {"value": 2, "player": "Carol"}}
28600	9b227a4f	13	2026-10-30 13:16:24.697466+00	{"AddExtraPoints": {"value": -1, "player": "Carol"}}
28601	9b227a4f	14	2026-10-30 13:17:24.697466+00	{"AddImperial": {"value": 1, "player": "Bob"}}
28602	9b227a4f	15	2026-10-30 13:18:24.697466+00	{"ScoreExtraPublicObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
28603	9b227a4f	16	2026-10-30 13:19:24.697466+00	{"RevealExtraPublicObjective": {"objective": "ErectAMonument"}}
28604	9b227a4f	17	2026-10-30 13:20:24.697466+00	{"ScoreExtraPublicObjective": {"player": "Bob", "objective": "ErectAMonument"}}
28605	9b227a4f	18	2026-10-30 13:21:24.697466+00	{"UnscoreObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
28606	9b227a4f	19	2026-10-30 13:22:24.697466+00	{"ScoreExtraSecretObjective": {"player": "Bob", "objective": "BecomeTheGatekeeper"}}
28607	9b227a4f	20	2026-10-30 13:23:24.697466+00	{"ScoreExtraSecretObjective": {"player": "Carol", "objective": "AdaptNewStrategies"}}
28608	9b227a4f	21	2026-10-30 13:24:24.697466+00	{"UnscoreSecretObjective": {"player": "Carol", "objective": "AdaptNewStrategies"}}
28609	9b227a4f	22	2026-10-30 13:25:24.697466+00	{"SetShardForTheThroneOwner": {"player": "Carol"}}
28610	9b227a4f	23	2026-10-30 13:26:24.697466+00	{"SetShardForTheThroneOwner": {"player": "Bob"}}
28611	9b227a4f	24	2026-10-30 13:27:24.697466+00	{"SetCrownOfEmphidiaOwner": {"player": "Bob"}}
28612	9b227a4f	25	2026-10-30 13:28:24.697466+00	{"SetCrownOfEmphidiaOwner": {"player": null}}
28613	69a4c6a8	0	2026-10-30 13:09:24.82222+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
28614	69a4c6a8	1	2026-10-30 13:10:24.82222+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28615	69a4c6a8	2	2026-10-30 13:11:24.82222+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28616	69a4c6a8	3	2026-10-30 13:12:24.82222+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28617	69a4c6a8	4	2026-10-30 13:13:24.82222+00	"CreationDone"
28618	69a4c6a8	5	2026-10-30 13:14:24.82222+00	{"SetupSpeaker": {"player": "Alice"}}
28619	69a4c6a8	6	2026-10-30 13:15:24.82222+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28620	69a4c6a8	7	2026-10-30 13:16:24.82222+00	"StartGame"
28621	69a4c6a8	8	2026-10-30 13:17:24.82222+00	{"SetPlanetOwner": {"planet": "MecatolRex", "player": "Bob"}}
28622	69a4c6a8	9	2026-10-30 13:18:24.82222+00	{"SetPlanetOwner": {"planet": "MecatolRex", "player": "Carol"}}
28623	69a4c6a8	10	2026-10-30 13:19:24.82222+00	{"SetPlanetOwner": {"planet": "MecatolRex", "player": null}}
28624	69a4c6a8	11	2026-10-30 13:20:24.82222+00	{"SetPlanetOwner": {"planet": "Elysium", "player": "Alice"}}
28625	69a4c6a8	12	2026-10-30 13:21:24.82222+00	{"SetPlanetOwner": {"planet": "Lodor", "player": "Alice"}}
28626	69a4c6a8	13	2026-10-30 13:22:24.82222+00	{"SetPlanetOwner": {"planet": "Primor", "player": "Alice"}}
28627	69a4c6a8	14	2026-10-30 13:23:24.82222+00	{"AddPlanetAttachment": {"planet": "Elysium", "player": "Alice", "attachment": "Geoform"}}
28628	69a4c6a8	15	2026-10-30 13:24:24.82222+00	{"AddPlanetAttachment": {"planet": "Lodor", "player": "Alice", "attachment": "Terraform"}}
28629	69a4c6a8	16	2026-10-30 13:25:24.82222+00	{"AddPlanetAttachment": {"planet": "Lodor", "player": "Alice", "attachment": "DysonSphere"}}
28630	69a4c6a8	17	2026-10-30 13:26:24.82222+00	{"RemovePlanetAttachment": {"planet": "Lodor", "player": "Alice", "attachment": "Terraform"}}
28631	69a4c6a8	18	2026-10-30 13:27:24.82222+00	{"AddTechToPlayer": {"tech": "GravityDrive", "player": "Carol"}}
28632	69a4c6a8	19	2026-10-30 13:28:24.82222+00	{"RemoveTechFromPlayer": {"tech": "GravitonLaserSystem", "player": "Carol"}}
28633	0f0e0005	0	2026-10-30 13:10:24.91745+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": true}}}}
28634	0f0e0005	1	2026-10-30 13:11:24.91745+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28635	0f0e0005	2	2026-10-30 13:12:24.91745+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28636	0f0e0005	3	2026-10-30 13:13:24.91745+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28637	0f0e0005	4	2026-10-30 13:14:24.91745+00	"CreationDone"
28638	0f0e0005	5	2026-10-30 13:15:24.91745+00	{"SetupSpeaker": {"player": "Alice"}}
28639	0f0e0005	6	2026-10-30 13:16:24.91745+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28640	0f0e0005	7	2026-10-30 13:17:24.91745+00	"StartGame"
28641	0f0e0005	8	2026-10-30 13:18:24.91745+00	{"SetRelicOwner": {"relic": "StellarConverter", "player": "Alice"}}
28642	0f0e0005	9	2026-10-30 13:19:24.91745+00	{"SetRelicOwner": {"relic": "StellarConverter", "player": "Bob"}}
28643	0f0e0005	10	2026-10-30 13:20:24.91745+00	{"SetRelicOwner": {"relic": "ShardOfTheThrone", "player": "Carol"}}
28644	0f0e0005	11	2026-10-30 13:21:24.91745+00	{"SetRelicOwner": {"relic": "ShardOfTheThrone", "player": null}}
28645	0f0e0005	12	2026-10-30 13:22:24.91745+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28646	0f0e0005	13	2026-10-30 13:23:24.91745+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28647	0f0e0005	14	2026-10-30 13:24:24.91745+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28648	0f0e0005	15	2026-10-30 13:25:24.91745+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28649	0f0e0005	16	2026-10-30 13:26:24.91745+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28650	0f0e0005	17	2026-10-30 13:27:24.91745+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28651	0f0e0005	18	2026-10-30 13:28:24.91745+00	"CompleteStrategyPhase"
28652	006fa03a	0	2026-10-30 12:43:25.00599+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28653	006fa03a	1	2026-10-30 12:44:25.00599+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28654	006fa03a	2	2026-10-30 12:45:25.00599+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28655	006fa03a	3	2026-10-30 12:46:25.00599+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28656	006fa03a	4	2026-10-30 12:47:25.00599+00	"CreationDone"
28657	006fa03a	5	2026-10-30 12:48:25.00599+00	{"SetupSpeaker": {"player": "Alice"}}
28658	006fa03a	6	2026-10-30 12:49:25.00599+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28659	006fa03a	7	2026-10-30 12:50:25.00599+00	"StartGame"
28660	006fa03a	8	2026-10-30 12:51:25.00599+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28661	006fa03a	9	2026-10-30 12:52:25.00599+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28662	006fa03a	10	2026-10-30 12:53:25.00599+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28663	006fa03a	11	2026-10-30 12:54:25.00599+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28664	006fa03a	12	2026-10-30 12:55:25.00599+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28665	006fa03a	13	2026-10-30 12:56:25.00599+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28666	006fa03a	14	2026-10-30 12:57:25.00599+00	"CompleteStrategyPhase"
28667	006fa03a	15	2026-10-30 12:58:25.00599+00	{"TacticalActionBegin": {"player": "Alice"}}
28668	006fa03a	16	2026-10-30 12:59:25.00599+00	{"TacticalActionCommit": {"player": "Alice"}}
28669	006fa03a	17	2026-10-30 13:00:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Alice"}}
28670	006fa03a	18	2026-10-30 13:01:25.00599+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
28671	006fa03a	19	2026-10-30 13:02:25.00599+00	{"TakeAnotherTurn": {"player": "Alice"}}
28672	006fa03a	20	2026-10-30 13:03:25.00599+00	{"TacticalActionBegin": {"player": "Alice"}}
28673	006fa03a	21	2026-10-30 13:04:25.00599+00	{"TacticalActionCommit": {"player": "Alice"}}
28674	006fa03a	22	2026-10-30 13:05:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "Influence", "player": "Alice"}}
28675	006fa03a	23	2026-10-30 13:06:25.00599+00	{"EndTurn": {"player": "Alice"}}
28676	006fa03a	24	2026-10-30 13:07:25.00599+00	{"TacticalActionBegin": {"player": "Bob"}}
28677	006fa03a	25	2026-10-30 13:08:25.00599+00	{"TacticalActionCommit": {"player": "Bob"}}
28678	006fa03a	26	2026-10-30 13:09:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "ActionCards", "player": "Bob"}}
28679	006fa03a	27	2026-10-30 13:10:25.00599+00	{"RollForTheFracture": {"player": "Bob", "entered_play": false}}
28680	006fa03a	28	2026-10-30 13:11:25.00599+00	{"EndTurn": {"player": "Bob"}}
28681	006fa03a	29	2026-10-30 13:12:25.00599+00	{"TacticalActionBegin": {"player": "Carol"}}
28682	006fa03a	30	2026-10-30 13:13:25.00599+00	{"TacticalActionCommit": {"player": "Carol"}}
28683	006fa03a	31	2026-10-30 13:14:25.00599+00	{"EndTurn": {"player": "Carol"}}
28684	006fa03a	32	2026-10-30 13:15:25.00599+00	{"TacticalActionBegin": {"player": "Alice"}}
28685	006fa03a	33	2026-10-30 13:16:25.00599+00	{"TacticalActionCommit": {"player": "Alice"}}
28686	006fa03a	34	2026-10-30 13:17:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "TradeGoods", "player": "Alice"}}
28687	006fa03a	35	2026-10-30 13:18:25.00599+00	{"EndTurn": {"player": "Alice"}}
28688	006fa03a	36	2026-10-30 13:19:25.00599+00	{"TacticalActionBegin": {"player": "Bob"}}
28689	006fa03a	37	2026-10-30 13:20:25.00599+00	{"TacticalActionCommit": {"player": "Bob"}}
28690	006fa03a	38	2026-10-30 13:21:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "SecretObjective", "player": "Bob"}}
28691	006fa03a	39	2026-10-30 13:22:25.00599+00	{"EndTurn": {"player": "Bob"}}
28692	006fa03a	40	2026-10-30 13:23:25.00599+00	{"TacticalActionBegin": {"player": "Carol"}}
28693	006fa03a	41	2026-10-30 13:24:25.00599+00	{"TacticalActionCommit": {"player": "Carol"}}
28694	006fa03a	42	2026-10-30 13:25:25.00599+00	{"CommitToThundersEdgeExpedition": {"slot": "TechSpecialityPlanet", "player": "Carol"}}
28695	006fa03a	43	2026-10-30 13:26:25.00599+00	{"RollForTheFracture": {"player": "Carol", "entered_play": false}}
28696	006fa03a	44	2026-10-30 13:27:25.00599+00	{"CompleteThundersEdgeExpedition": {"player": "Carol", "won_by": "Alice", "placed_in": "114"}}
28697	006fa03a	45	2026-10-30 13:28:25.00599+00	{"EndTurn": {"player": "Carol"}}
28698	38720452	0	2026-10-30 12:47:25.200605+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28699	38720452	1	2026-10-30 12:48:25.200605+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28700	38720452	2	2026-10-30 12:49:25.200605+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28701	38720452	3	2026-10-30 12:50:25.200605+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28702	38720452	4	2026-10-30 12:51:25.200605+00	"CreationDone"
28703	38720452	5	2026-10-30 12:52:25.200605+00	{"SetupSpeaker": {"player": "Alice"}}
28704	38720452	6	2026-10-30 12:53:25.200605+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28705	38720452	7	2026-10-30 12:54:25.200605+00	"StartGame"
28706	38720452	8	2026-10-30 12:55:25.200605+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28707	38720452	9	2026-10-30 12:56:25.200605+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28708	38720452	10	2026-10-30 12:57:25.200605+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28709	38720452	11	2026-10-30 12:58:25.200605+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28710	38720452	12	2026-10-30 12:59:25.200605+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28711	38720452	13	2026-10-30 13:00:25.200605+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28712	38720452	14	2026-10-30 13:01:25.200605+00	"CompleteStrategyPhase"
28713	38720452	15	2026-10-30 13:02:25.200605+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28714	38720452	16	2026-10-30 13:03:25.200605+00	"StrategicActionCommit"
28715	38720452	17	2026-10-30 13:04:25.200605+00	{"EndTurn": {"player": "Alice"}}
28716	38720452	18	2026-10-30 13:05:25.200605+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28717	38720452	19	2026-10-30 13:06:25.200605+00	"StrategicActionCommit"
28718	38720452	20	2026-10-30 13:07:25.200605+00	{"EndTurn": {"player": "Bob"}}
28719	38720452	21	2026-10-30 13:08:25.200605+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28720	38720452	22	2026-10-30 13:09:25.200605+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28721	38720452	23	2026-10-30 13:10:25.200605+00	"StrategicActionCommit"
28722	38720452	24	2026-10-30 13:11:25.200605+00	{"EndTurn": {"player": "Carol"}}
28723	38720452	25	2026-10-30 13:12:25.200605+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28724	38720452	26	2026-10-30 13:13:25.200605+00	"StrategicActionCommit"
28725	38720452	27	2026-10-30 13:14:25.200605+00	{"TakeAnotherTurn": {"player": "Alice"}}
28726	38720452	28	2026-10-30 13:15:25.200605+00	{"PassAction": {"player": "Alice"}}
28727	38720452	29	2026-10-30 13:16:25.200605+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Alice"}}
28728	38720452	30	2026-10-30 13:17:25.200605+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
28729	38720452	31	2026-10-30 13:18:25.200605+00	{"EndTurn": {"player": "Alice"}}
28730	38720452	32	2026-10-30 13:19:25.200605+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28731	38720452	33	2026-10-30 13:20:25.200605+00	"StrategicActionCommit"
28732	38720452	34	2026-10-30 13:21:25.200605+00	{"EndTurn": {"player": "Bob"}}
28733	38720452	35	2026-10-30 13:22:25.200605+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28734	38720452	36	2026-10-30 13:23:25.200605+00	"StrategicActionCommit"
28735	38720452	37	2026-10-30 13:24:25.200605+00	{"EndTurn": {"player": "Carol"}}
28736	38720452	38	2026-10-30 13:25:25.200605+00	{"PassAction": {"player": "Bob"}}
28737	38720452	39	2026-10-30 13:26:25.200605+00	{"EndTurn": {"player": "Bob"}}
28738	38720452	40	2026-10-30 13:27:25.200605+00	{"PassAction": {"player": "Carol"}}
28739	38720452	41	2026-10-30 13:28:25.200605+00	{"EndTurn": {"player": "Carol"}}
28740	a19cc3eb	0	2026-10-30 13:03:25.397753+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28741	a19cc3eb	1	2026-10-30 13:04:25.397753+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28742	a19cc3eb	2	2026-10-30 13:05:25.397753+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28743	a19cc3eb	3	2026-10-30 13:06:25.397753+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28744	a19cc3eb	4	2026-10-30 13:07:25.397753+00	"CreationDone"
28745	a19cc3eb	5	2026-10-30 13:08:25.397753+00	{"SetupSpeaker": {"player": "Alice"}}
28746	a19cc3eb	6	2026-10-30 13:09:25.397753+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28747	a19cc3eb	7	2026-10-30 13:10:25.397753+00	"StartGame"
28748	a19cc3eb	8	2026-10-30 13:11:25.397753+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28749	a19cc3eb	9	2026-10-30 13:12:25.397753+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28750	a19cc3eb	10	2026-10-30 13:13:25.397753+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28751	a19cc3eb	11	2026-10-30 13:14:25.397753+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28752	a19cc3eb	12	2026-10-30 13:15:25.397753+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28753	a19cc3eb	13	2026-10-30 13:16:25.397753+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28754	a19cc3eb	14	2026-10-30 13:17:25.397753+00	"CompleteStrategyPhase"
28755	a19cc3eb	15	2026-10-30 13:18:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Resources", "player": "Alice"}}
28756	a19cc3eb	16	2026-10-30 13:19:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "ActionCards", "player": "Alice"}}
28757	a19cc3eb	17	2026-10-30 13:20:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Influence", "player": "Alice"}}
28758	a19cc3eb	18	2026-10-30 13:21:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "SecretObjective", "player": "Alice"}}
28759	a19cc3eb	19	2026-10-30 13:22:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TechSpecialityPlanet", "player": "Alice"}}
28760	a19cc3eb	20	2026-10-30 13:23:25.397753+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TradeGoods", "player": "Alice"}}
28761	a19cc3eb	21	2026-10-30 13:24:25.397753+00	{"TacticalActionBegin": {"player": "Alice"}}
28762	a19cc3eb	22	2026-10-30 13:25:25.397753+00	{"TacticalActionCommit": {"player": "Alice"}}
28763	a19cc3eb	23	2026-10-30 13:26:25.397753+00	{"EndTurn": {"player": "Alice"}}
28764	a19cc3eb	24	2026-10-30 13:27:25.397753+00	{"SetThundersEdgeCompletion": {"new_completion": {"won_by": "Alice", "placed_in": "114"}}}
28765	a19cc3eb	25	2026-10-30 13:28:25.397753+00	{"SetThundersEdgeCompletion": {"new_completion": {"won_by": "Bob", "placed_in": "114"}}}
28766	b436b5db	0	2026-10-30 12:44:25.511415+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28767	b436b5db	1	2026-10-30 12:45:25.511415+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FirmamentObsidian"}}}
28768	b436b5db	2	2026-10-30 12:46:25.511415+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28769	b436b5db	3	2026-10-30 12:47:25.511415+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28770	b436b5db	4	2026-10-30 12:48:25.511415+00	"CreationDone"
28771	b436b5db	5	2026-10-30 12:49:25.511415+00	{"SetupPlayerTechs": {"player": "Alice", "technologies": ["NeuralMotivator"]}}
28772	b436b5db	6	2026-10-30 12:50:25.511415+00	{"SetupSpeaker": {"player": "Alice"}}
28773	b436b5db	7	2026-10-30 12:51:25.511415+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28774	b436b5db	8	2026-10-30 12:52:25.511415+00	"StartGame"
28775	b436b5db	9	2026-10-30 12:53:25.511415+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28776	b436b5db	10	2026-10-30 12:54:25.511415+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28777	b436b5db	11	2026-10-30 12:55:25.511415+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28778	b436b5db	12	2026-10-30 12:56:25.511415+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28779	b436b5db	13	2026-10-30 12:57:25.511415+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28780	b436b5db	14	2026-10-30 12:58:25.511415+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28781	b436b5db	15	2026-10-30 12:59:25.511415+00	"CompleteStrategyPhase"
28782	b436b5db	16	2026-10-30 13:00:25.511415+00	{"SetBreakthrough": {"player": "Alice", "breakthrough": "TheSowing"}}
28783	b436b5db	17	2026-10-30 13:01:25.511415+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
28784	b436b5db	18	2026-10-30 13:02:25.511415+00	{"PuppetsOfTheBlade": {"player": "Alice"}}
28785	b436b5db	19	2026-10-30 13:03:25.511415+00	{"EndTurn": {"player": "Alice"}}
28786	b436b5db	20	2026-10-30 13:04:25.511415+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
28787	b436b5db	21	2026-10-30 13:05:25.511415+00	"StrategicActionCommit"
28788	b436b5db	22	2026-10-30 13:06:25.511415+00	{"EndTurn": {"player": "Bob"}}
28789	b436b5db	23	2026-10-30 13:07:25.511415+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28790	b436b5db	24	2026-10-30 13:08:25.511415+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
28791	b436b5db	25	2026-10-30 13:09:25.511415+00	"StrategicActionCommit"
28792	b436b5db	26	2026-10-30 13:10:25.511415+00	{"EndTurn": {"player": "Carol"}}
28793	b436b5db	27	2026-10-30 13:11:25.511415+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28794	b436b5db	28	2026-10-30 13:12:25.511415+00	"StrategicActionCommit"
28795	b436b5db	29	2026-10-30 13:13:25.511415+00	{"EndTurn": {"player": "Alice"}}
28796	b436b5db	30	2026-10-30 13:14:25.511415+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
28797	b436b5db	31	2026-10-30 13:15:25.511415+00	"StrategicActionCommit"
28798	b436b5db	32	2026-10-30 13:16:25.511415+00	{"EndTurn": {"player": "Bob"}}
28799	b436b5db	33	2026-10-30 13:17:25.511415+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
28800	b436b5db	34	2026-10-30 13:18:25.511415+00	"StrategicActionCommit"
28801	b436b5db	35	2026-10-30 13:19:25.511415+00	{"EndTurn": {"player": "Carol"}}
28802	b436b5db	36	2026-10-30 13:20:25.511415+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
28803	b436b5db	37	2026-10-30 13:21:25.511415+00	"StrategicActionCommit"
28804	b436b5db	38	2026-10-30 13:22:25.511415+00	{"EndTurn": {"player": "Alice"}}
28805	b436b5db	39	2026-10-30 13:23:25.511415+00	{"PassAction": {"player": "Bob"}}
28806	b436b5db	40	2026-10-30 13:24:25.511415+00	{"EndTurn": {"player": "Bob"}}
28807	b436b5db	41	2026-10-30 13:25:25.511415+00	{"PassAction": {"player": "Carol"}}
28808	b436b5db	42	2026-10-30 13:26:25.511415+00	{"EndTurn": {"player": "Carol"}}
28809	b436b5db	43	2026-10-30 13:27:25.511415+00	{"PassAction": {"player": "Alice"}}
28810	b436b5db	44	2026-10-30 13:28:25.511415+00	{"EndTurn": {"player": "Alice"}}
28811	11ed8fa8	0	2026-10-30 12:58:25.706598+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28812	11ed8fa8	1	2026-10-30 12:59:25.706598+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28813	11ed8fa8	2	2026-10-30 13:00:25.706598+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28814	11ed8fa8	3	2026-10-30 13:01:25.706598+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28815	11ed8fa8	4	2026-10-30 13:02:25.706598+00	"CreationDone"
28816	11ed8fa8	5	2026-10-30 13:03:25.706598+00	{"SetupSpeaker": {"player": "Alice"}}
28817	11ed8fa8	6	2026-10-30 13:04:25.706598+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28818	11ed8fa8	7	2026-10-30 13:05:25.706598+00	"StartGame"
28819	11ed8fa8	8	2026-10-30 13:06:25.706598+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28820	11ed8fa8	9	2026-10-30 13:07:25.706598+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28821	11ed8fa8	10	2026-10-30 13:08:25.706598+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28822	11ed8fa8	11	2026-10-30 13:09:25.706598+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28823	11ed8fa8	12	2026-10-30 13:10:25.706598+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28824	11ed8fa8	13	2026-10-30 13:11:25.706598+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28825	11ed8fa8	14	2026-10-30 13:12:25.706598+00	"CompleteStrategyPhase"
28826	11ed8fa8	15	2026-10-30 13:13:25.706598+00	{"TacticalActionBegin": {"player": "Alice"}}
28827	11ed8fa8	16	2026-10-30 13:14:25.706598+00	{"SetFractureInPlay": {"in_play": true}}
28828	11ed8fa8	17	2026-10-30 13:15:25.706598+00	{"TacticalActionTakePlanet": {"planet": "Lethe", "player": "Alice"}}
28829	11ed8fa8	18	2026-10-30 13:16:25.706598+00	{"TacticalActionTakePlanet": {"planet": "Phlegethon", "player": "Alice"}}
28830	11ed8fa8	19	2026-10-30 13:17:25.706598+00	{"TacticalActionCommit": {"player": "Alice"}}
28831	11ed8fa8	20	2026-10-30 13:18:25.706598+00	{"EndTurn": {"player": "Alice"}}
28832	11ed8fa8	21	2026-10-30 13:19:25.706598+00	{"SetThundersEdgeCompletion": {"new_completion": {"won_by": "Alice", "placed_in": "114"}}}
28833	11ed8fa8	22	2026-10-30 13:20:25.706598+00	{"SetPlanetOwner": {"planet": "Avernus", "player": "Alice"}}
28834	11ed8fa8	23	2026-10-30 13:21:25.706598+00	{"TacticalActionBegin": {"player": "Bob"}}
28835	11ed8fa8	24	2026-10-30 13:22:25.706598+00	{"TacticalActionTakePlanet": {"planet": "ThundersEdge", "player": "Bob"}}
28836	11ed8fa8	25	2026-10-30 13:23:25.706598+00	{"TacticalActionCommit": {"player": "Bob"}}
28837	11ed8fa8	26	2026-10-30 13:24:25.706598+00	{"EndTurn": {"player": "Bob"}}
28838	11ed8fa8	27	2026-10-30 13:25:25.706598+00	{"TacticalActionBegin": {"player": "Carol"}}
28839	11ed8fa8	28	2026-10-30 13:26:25.706598+00	{"TacticalActionTakePlanet": {"planet": "Avernus", "player": "Carol"}}
28840	11ed8fa8	29	2026-10-30 13:27:25.706598+00	{"TacticalActionCommit": {"player": "Carol"}}
28841	11ed8fa8	30	2026-10-30 13:28:25.706598+00	{"EndTurn": {"player": "Carol"}}
28842	728830d9	0	2026-10-30 13:04:25.84329+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28843	728830d9	1	2026-10-30 13:05:25.84329+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28962	d92655e1	27	2026-10-30 13:26:26.277917+00	{"ActionCardActionBegin": {"card": "MercenaryContract", "player": "Carol"}}
28844	728830d9	2	2026-10-30 13:06:25.84329+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28845	728830d9	3	2026-10-30 13:07:25.84329+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28846	728830d9	4	2026-10-30 13:08:25.84329+00	"CreationDone"
28847	728830d9	5	2026-10-30 13:09:25.84329+00	{"SetupSpeaker": {"player": "Alice"}}
28848	728830d9	6	2026-10-30 13:10:25.84329+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28849	728830d9	7	2026-10-30 13:11:25.84329+00	"StartGame"
28850	728830d9	8	2026-10-30 13:12:25.84329+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28851	728830d9	9	2026-10-30 13:13:25.84329+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28852	728830d9	10	2026-10-30 13:14:25.84329+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28853	728830d9	11	2026-10-30 13:15:25.84329+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28854	728830d9	12	2026-10-30 13:16:25.84329+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28855	728830d9	13	2026-10-30 13:17:25.84329+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28856	728830d9	14	2026-10-30 13:18:25.84329+00	"CompleteStrategyPhase"
28857	728830d9	15	2026-10-30 13:19:25.84329+00	{"SetFractureInPlay": {"in_play": true}}
28858	728830d9	16	2026-10-30 13:20:25.84329+00	{"TacticalActionBegin": {"player": "Alice"}}
28859	728830d9	17	2026-10-30 13:21:25.84329+00	{"TacticalActionTakePlanet": {"planet": "Styx", "player": "Alice"}}
28860	728830d9	18	2026-10-30 13:22:25.84329+00	{"TacticalActionCommit": {"player": "Alice"}}
28861	728830d9	19	2026-10-30 13:23:25.84329+00	{"EndTurn": {"player": "Alice"}}
28862	728830d9	20	2026-10-30 13:24:25.84329+00	{"TacticalActionBegin": {"player": "Bob"}}
28863	728830d9	21	2026-10-30 13:25:25.84329+00	{"TacticalActionTakePlanet": {"planet": "Styx", "player": "Bob"}}
28864	728830d9	22	2026-10-30 13:26:25.84329+00	{"TacticalActionCommit": {"player": "Bob"}}
28865	728830d9	23	2026-10-30 13:27:25.84329+00	{"EndTurn": {"player": "Bob"}}
28866	728830d9	24	2026-10-30 13:28:25.84329+00	{"SetPlanetOwner": {"planet": "Styx", "player": null}}
28867	ab4e4e80	0	2026-10-30 12:52:25.966614+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
28868	ab4e4e80	1	2026-10-30 12:53:25.966614+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28869	ab4e4e80	2	2026-10-30 12:54:25.966614+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28870	ab4e4e80	3	2026-10-30 12:55:25.966614+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28871	ab4e4e80	4	2026-10-30 12:56:25.966614+00	{"AddPlayer": {"player": {"name": "Dave", "color": "Red", "faction": "VuilRaithCabal"}}}
28872	ab4e4e80	5	2026-10-30 12:57:25.966614+00	"CreationDone"
28873	ab4e4e80	6	2026-10-30 12:58:25.966614+00	{"SetupSpeaker": {"player": "Alice"}}
28874	ab4e4e80	7	2026-10-30 12:59:25.966614+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28875	ab4e4e80	8	2026-10-30 13:00:25.966614+00	"StartGame"
28876	ab4e4e80	9	2026-10-30 13:01:25.966614+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28877	ab4e4e80	10	2026-10-30 13:02:25.966614+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28878	ab4e4e80	11	2026-10-30 13:03:25.966614+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28879	ab4e4e80	12	2026-10-30 13:04:25.966614+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dave"}}
28880	ab4e4e80	13	2026-10-30 13:05:25.966614+00	{"TakeStrategyCard": {"card": "Trade", "player": "Alice"}}
28881	ab4e4e80	14	2026-10-30 13:06:25.966614+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Bob"}}
28882	ab4e4e80	15	2026-10-30 13:07:25.966614+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
28883	ab4e4e80	16	2026-10-30 13:08:25.966614+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Dave"}}
28884	ab4e4e80	17	2026-10-30 13:09:25.966614+00	"CompleteStrategyPhase"
28885	ab4e4e80	18	2026-10-30 13:10:25.966614+00	{"TacticalActionBegin": {"player": "Alice"}}
28886	ab4e4e80	19	2026-10-30 13:11:25.966614+00	{"TacticalActionCommit": {"player": "Alice"}}
28887	ab4e4e80	20	2026-10-30 13:12:25.966614+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Alice"}}
28888	ab4e4e80	21	2026-10-30 13:13:25.966614+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
28889	ab4e4e80	22	2026-10-30 13:14:25.966614+00	{"EndTurn": {"player": "Alice"}}
28890	ab4e4e80	23	2026-10-30 13:15:25.966614+00	{"SetFractureInPlay": {"in_play": true}}
28891	ab4e4e80	24	2026-10-30 13:16:25.966614+00	{"SetFractureInPlay": {"in_play": false}}
28892	ab4e4e80	25	2026-10-30 13:17:25.966614+00	{"GainRelicAction": {"relic": "TheQuantumcore", "player": "Bob"}}
28893	ab4e4e80	26	2026-10-30 13:18:25.966614+00	{"RollForTheFracture": {"player": "Bob", "entered_play": true}}
28894	ab4e4e80	27	2026-10-30 13:19:25.966614+00	{"EndTurn": {"player": "Bob"}}
28895	ab4e4e80	28	2026-10-30 13:20:25.966614+00	{"ActionCardActionBegin": {"card": "Brilliance", "player": "Carol"}}
28896	ab4e4e80	29	2026-10-30 13:21:25.966614+00	{"ActionCardActionCommit": {"data": {"Brilliance": {"player": "Carol"}}, "player": "Carol"}}
28897	ab4e4e80	30	2026-10-30 13:22:25.966614+00	{"EndTurn": {"player": "Carol"}}
28898	ab4e4e80	31	2026-10-30 13:23:25.966614+00	{"SetFractureInPlay": {"in_play": false}}
28899	ab4e4e80	32	2026-10-30 13:24:25.966614+00	{"SetBreakthrough": {"player": "Dave", "breakthrough": "AlRaithIxIanovar"}}
28900	ab4e4e80	33	2026-10-30 13:25:25.966614+00	{"TacticalActionBegin": {"player": "Dave"}}
28901	ab4e4e80	34	2026-10-30 13:26:25.966614+00	{"TacticalActionTakePlanet": {"planet": "Cocytus", "player": "Dave"}}
28902	ab4e4e80	35	2026-10-30 13:27:25.966614+00	{"TacticalActionCommit": {"player": "Dave"}}
28903	ab4e4e80	36	2026-10-30 13:28:25.966614+00	{"EndTurn": {"player": "Dave"}}
28963	d92655e1	28	2026-10-30 13:27:26.277917+00	{"ActionCardActionCommit": {"data": {"MercenaryContract": {"planet": "Abyz"}}, "player": "Carol"}}
28904	d71a8543	0	2026-10-30 12:58:26.135279+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
28905	d71a8543	1	2026-10-30 12:59:26.135279+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "CouncilKeleres"}}}
28906	d71a8543	2	2026-10-30 13:00:26.135279+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmbersOfMuaat"}}}
28907	d71a8543	3	2026-10-30 13:01:26.135279+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28908	d71a8543	4	2026-10-30 13:02:26.135279+00	"CreationDone"
28909	d71a8543	5	2026-10-30 13:03:26.135279+00	{"SetupTheTribunii": {"player": "Alice", "faction": "MentakCoalition"}}
28910	d71a8543	6	2026-10-30 13:04:26.135279+00	{"SetupPlayerTechs": {"player": "Alice", "technologies": ["GravitonLaserSystem", "PlasmaScoring"]}}
28911	d71a8543	7	2026-10-30 13:05:26.135279+00	{"SetupSpeaker": {"player": "Alice"}}
28912	d71a8543	8	2026-10-30 13:06:26.135279+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28913	d71a8543	9	2026-10-30 13:07:26.135279+00	"StartGame"
28914	d71a8543	10	2026-10-30 13:08:26.135279+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28915	d71a8543	11	2026-10-30 13:09:26.135279+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28916	d71a8543	12	2026-10-30 13:10:26.135279+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28917	d71a8543	13	2026-10-30 13:11:26.135279+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28918	d71a8543	14	2026-10-30 13:12:26.135279+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28919	d71a8543	15	2026-10-30 13:13:26.135279+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28920	d71a8543	16	2026-10-30 13:14:26.135279+00	"CompleteStrategyPhase"
28921	d71a8543	17	2026-10-30 13:15:26.135279+00	{"TacticalActionBegin": {"player": "Alice"}}
28922	d71a8543	18	2026-10-30 13:16:26.135279+00	{"TacticalActionCommit": {"player": "Alice"}}
28923	d71a8543	19	2026-10-30 13:17:26.135279+00	{"CommitToThundersEdgeExpedition": {"slot": "Resources", "player": "Alice"}}
28924	d71a8543	20	2026-10-30 13:18:26.135279+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
28925	d71a8543	21	2026-10-30 13:19:26.135279+00	{"EndTurn": {"player": "Alice"}}
28926	d71a8543	22	2026-10-30 13:20:26.135279+00	{"SetBreakthrough": {"player": "Bob", "breakthrough": "StellarGenesis"}}
28927	d71a8543	23	2026-10-30 13:21:26.135279+00	{"RollForTheFracture": {"player": "Bob", "entered_play": false}}
28928	d71a8543	24	2026-10-30 13:22:26.135279+00	{"TacticalActionBegin": {"player": "Bob"}}
28929	d71a8543	25	2026-10-30 13:23:26.135279+00	{"TacticalActionCommit": {"player": "Bob"}}
28930	d71a8543	26	2026-10-30 13:24:26.135279+00	{"EndTurn": {"player": "Bob"}}
28931	d71a8543	27	2026-10-30 13:25:26.135279+00	{"TacticalActionBegin": {"player": "Carol"}}
28932	d71a8543	28	2026-10-30 13:26:26.135279+00	{"TacticalActionTakePlanet": {"planet": "Avernus", "player": "Carol"}}
28933	d71a8543	29	2026-10-30 13:27:26.135279+00	{"TacticalActionCommit": {"player": "Carol"}}
28934	d71a8543	30	2026-10-30 13:28:26.135279+00	{"EndTurn": {"player": "Carol"}}
28935	d92655e1	0	2026-10-30 12:59:26.277917+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
28936	d92655e1	1	2026-10-30 13:00:26.277917+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28937	d92655e1	2	2026-10-30 13:01:26.277917+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28938	d92655e1	3	2026-10-30 13:02:26.277917+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28939	d92655e1	4	2026-10-30 13:03:26.277917+00	"CreationDone"
28940	d92655e1	5	2026-10-30 13:04:26.277917+00	{"SetupSpeaker": {"player": "Alice"}}
28941	d92655e1	6	2026-10-30 13:05:26.277917+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28942	d92655e1	7	2026-10-30 13:06:26.277917+00	"StartGame"
28943	d92655e1	8	2026-10-30 13:07:26.277917+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28944	d92655e1	9	2026-10-30 13:08:26.277917+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28945	d92655e1	10	2026-10-30 13:09:26.277917+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28946	d92655e1	11	2026-10-30 13:10:26.277917+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28947	d92655e1	12	2026-10-30 13:11:26.277917+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28948	d92655e1	13	2026-10-30 13:12:26.277917+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28949	d92655e1	14	2026-10-30 13:13:26.277917+00	"CompleteStrategyPhase"
28950	d92655e1	15	2026-10-30 13:14:26.277917+00	{"SetPlanetOwner": {"planet": "Abyz", "player": "Bob"}}
28951	d92655e1	16	2026-10-30 13:15:26.277917+00	{"SetPlanetOwner": {"planet": "Lazar", "player": "Carol"}}
28952	d92655e1	17	2026-10-30 13:16:26.277917+00	{"ActionCardActionBegin": {"card": "FighterConscriptionOmega", "player": "Alice"}}
28953	d92655e1	18	2026-10-30 13:17:26.277917+00	{"ActionCardActionCommit": {"data": null, "player": "Alice"}}
28954	d92655e1	19	2026-10-30 13:18:26.277917+00	{"TakeAnotherTurn": {"player": "Alice"}}
28955	d92655e1	20	2026-10-30 13:19:26.277917+00	{"ActionCardActionBegin": {"card": "Brilliance", "player": "Alice"}}
28956	d92655e1	21	2026-10-30 13:20:26.277917+00	{"ActionCardActionCommit": {"data": {"Brilliance": {"player": "Bob"}}, "player": "Alice"}}
28957	d92655e1	22	2026-10-30 13:21:26.277917+00	{"RollForTheFracture": {"player": "Bob", "entered_play": false}}
28958	d92655e1	23	2026-10-30 13:22:26.277917+00	{"EndTurn": {"player": "Alice"}}
28959	d92655e1	24	2026-10-30 13:23:26.277917+00	{"ActionCardActionBegin": {"card": "Brilliance", "player": "Bob"}}
28960	d92655e1	25	2026-10-30 13:24:26.277917+00	{"ActionCardActionCommit": {"data": {"Brilliance": {"player": null}}, "player": "Bob"}}
28961	d92655e1	26	2026-10-30 13:25:26.277917+00	{"EndTurn": {"player": "Bob"}}
28964	d92655e1	29	2026-10-30 13:28:26.277917+00	{"EndTurn": {"player": "Carol"}}
28965	65cba6c2	0	2026-10-30 12:55:26.415127+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
28966	65cba6c2	1	2026-10-30 12:56:26.415127+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
28967	65cba6c2	2	2026-10-30 12:57:26.415127+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
28968	65cba6c2	3	2026-10-30 12:58:26.415127+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
28969	65cba6c2	4	2026-10-30 12:59:26.415127+00	"CreationDone"
28970	65cba6c2	5	2026-10-30 13:00:26.415127+00	{"SetupSpeaker": {"player": "Alice"}}
28971	65cba6c2	6	2026-10-30 13:01:26.415127+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
28972	65cba6c2	7	2026-10-30 13:02:26.415127+00	"StartGame"
28973	65cba6c2	8	2026-10-30 13:03:26.415127+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
28974	65cba6c2	9	2026-10-30 13:04:26.415127+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
28975	65cba6c2	10	2026-10-30 13:05:26.415127+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
28976	65cba6c2	11	2026-10-30 13:06:26.415127+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
28977	65cba6c2	12	2026-10-30 13:07:26.415127+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
28978	65cba6c2	13	2026-10-30 13:08:26.415127+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
28979	65cba6c2	14	2026-10-30 13:09:26.415127+00	"CompleteStrategyPhase"
28980	65cba6c2	15	2026-10-30 13:10:26.415127+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
28981	65cba6c2	16	2026-10-30 13:11:26.415127+00	"StrategicActionCommit"
28982	65cba6c2	17	2026-10-30 13:12:26.415127+00	{"EndTurn": {"player": "Alice"}}
28983	65cba6c2	18	2026-10-30 13:13:26.415127+00	{"ActionCardActionBegin": {"card": "Overrule", "player": "Bob"}}
28984	65cba6c2	19	2026-10-30 13:14:26.415127+00	{"ActionCardActionCommit": {"data": {"Overrule": {"card": "Technology", "primary": {"Technology": {"first_tech": "GravityDrive", "second_tech": null}}}}, "player": "Bob"}}
28985	65cba6c2	20	2026-10-30 13:15:26.415127+00	{"EndTurn": {"player": "Bob"}}
28986	65cba6c2	21	2026-10-30 13:16:26.415127+00	{"ActionCardActionBegin": {"card": "Strategize", "player": "Carol"}}
28987	65cba6c2	22	2026-10-30 13:17:26.415127+00	{"ActionCardActionCommit": {"data": {"Strategize": {"card": "Technology", "secondary": {"Technology": {"tech": "PlasmaScoring"}}}}, "player": "Carol"}}
28988	65cba6c2	23	2026-10-30 13:18:26.415127+00	{"EndTurn": {"player": "Carol"}}
28989	65cba6c2	24	2026-10-30 13:19:26.415127+00	{"ActionCardActionBegin": {"card": "Overrule", "player": "Alice"}}
28990	65cba6c2	25	2026-10-30 13:20:26.415127+00	{"ActionCardActionCommit": {"data": {"Overrule": {"card": "Imperial", "primary": {"Imperial": {"scoreObjective": "CornerTheMarket"}}}}, "player": "Alice"}}
28991	65cba6c2	26	2026-10-30 13:21:26.415127+00	{"EndTurn": {"player": "Alice"}}
28992	65cba6c2	27	2026-10-30 13:22:26.415127+00	{"ActionCardActionBegin": {"card": "Overrule", "player": "Bob"}}
28993	65cba6c2	28	2026-10-30 13:23:26.415127+00	{"ActionCardActionCommit": {"data": {"Overrule": {"card": "Politics", "primary": {"Politics": {"newSpeaker": "Bob"}}}}, "player": "Bob"}}
28994	65cba6c2	29	2026-10-30 13:24:26.415127+00	{"EndTurn": {"player": "Bob"}}
28995	65cba6c2	30	2026-10-30 13:25:26.415127+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
28996	65cba6c2	31	2026-10-30 13:26:26.415127+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Bob"}}, "player": "Carol"}}
28997	65cba6c2	32	2026-10-30 13:27:26.415127+00	"StrategicActionCommit"
28998	65cba6c2	33	2026-10-30 13:28:26.415127+00	{"EndTurn": {"player": "Carol"}}
28999	980076ac	0	2026-10-30 12:37:26.557788+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": true, "prophecyOfKings": false}}}}
29000	980076ac	1	2026-10-30 12:38:26.557788+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29001	980076ac	2	2026-10-30 12:39:26.557788+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29002	980076ac	3	2026-10-30 12:40:26.557788+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29003	980076ac	4	2026-10-30 12:41:26.557788+00	"CreationDone"
29004	980076ac	5	2026-10-30 12:42:26.557788+00	{"SetupSpeaker": {"player": "Alice"}}
29005	980076ac	6	2026-10-30 12:43:26.557788+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29006	980076ac	7	2026-10-30 12:44:26.557788+00	"StartGame"
29007	980076ac	8	2026-10-30 12:45:26.557788+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29008	980076ac	9	2026-10-30 12:46:26.557788+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29009	980076ac	10	2026-10-30 12:47:26.557788+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29010	980076ac	11	2026-10-30 12:48:26.557788+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29011	980076ac	12	2026-10-30 12:49:26.557788+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29012	980076ac	13	2026-10-30 12:50:26.557788+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29013	980076ac	14	2026-10-30 12:51:26.557788+00	"CompleteStrategyPhase"
29014	980076ac	15	2026-10-30 12:52:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Resources", "player": "Alice"}}
29015	980076ac	16	2026-10-30 12:53:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "ActionCards", "player": "Alice"}}
29016	980076ac	17	2026-10-30 12:54:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "Influence", "player": "Alice"}}
29017	980076ac	18	2026-10-30 12:55:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "SecretObjective", "player": "Alice"}}
29018	980076ac	19	2026-10-30 12:56:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TechSpecialityPlanet", "player": "Alice"}}
29019	980076ac	20	2026-10-30 12:57:26.557788+00	{"ClaimThundersEdgeExpeditionSlot": {"slot": "TradeGoods", "player": "Alice"}}
29020	980076ac	21	2026-10-30 12:58:26.557788+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29021	980076ac	22	2026-10-30 12:59:26.557788+00	"StrategicActionCommit"
29022	980076ac	23	2026-10-30 13:00:26.557788+00	{"PlayCrisis": {"player": "Carol"}}
29023	980076ac	24	2026-10-30 13:01:26.557788+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
29024	980076ac	25	2026-10-30 13:02:26.557788+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
29025	980076ac	26	2026-10-30 13:03:26.557788+00	"StrategicActionCommit"
29026	980076ac	27	2026-10-30 13:04:26.557788+00	{"EndTurn": {"player": "Carol"}}
29027	980076ac	28	2026-10-30 13:05:26.557788+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
29028	980076ac	29	2026-10-30 13:06:26.557788+00	"StrategicActionCommit"
29029	980076ac	30	2026-10-30 13:07:26.557788+00	{"EndTurn": {"player": "Alice"}}
29030	980076ac	31	2026-10-30 13:08:26.557788+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29031	980076ac	32	2026-10-30 13:09:26.557788+00	"StrategicActionCommit"
29032	980076ac	33	2026-10-30 13:10:26.557788+00	{"EndTurn": {"player": "Bob"}}
29033	980076ac	34	2026-10-30 13:11:26.557788+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29034	980076ac	35	2026-10-30 13:12:26.557788+00	"StrategicActionCommit"
29035	980076ac	36	2026-10-30 13:13:26.557788+00	{"EndTurn": {"player": "Carol"}}
29036	980076ac	37	2026-10-30 13:14:26.557788+00	{"PassAction": {"player": "Alice"}}
29037	980076ac	38	2026-10-30 13:15:26.557788+00	{"EndTurn": {"player": "Alice"}}
29038	980076ac	39	2026-10-30 13:16:26.557788+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
29039	980076ac	40	2026-10-30 13:17:26.557788+00	"StrategicActionCommit"
29040	980076ac	41	2026-10-30 13:18:26.557788+00	{"PlayPuppetsOnAString": {"player": "Alice"}}
29041	980076ac	42	2026-10-30 13:19:26.557788+00	{"TacticalActionBegin": {"player": "Alice"}}
29042	980076ac	43	2026-10-30 13:20:26.557788+00	{"TacticalActionCommit": {"player": "Alice"}}
29043	980076ac	44	2026-10-30 13:21:26.557788+00	{"EndTurn": {"player": "Alice"}}
29044	980076ac	45	2026-10-30 13:22:26.557788+00	{"PassAction": {"player": "Carol"}}
29045	980076ac	46	2026-10-30 13:23:26.557788+00	{"EndTurn": {"player": "Carol"}}
29046	980076ac	47	2026-10-30 13:24:26.557788+00	{"PassAction": {"player": "Bob"}}
29047	980076ac	48	2026-10-30 13:25:26.557788+00	{"PlayPuppetsOnAString": {"player": "Carol"}}
29048	980076ac	49	2026-10-30 13:26:26.557788+00	{"TacticalActionBegin": {"player": "Carol"}}
29049	980076ac	50	2026-10-30 13:27:26.557788+00	{"TacticalActionCommit": {"player": "Carol"}}
29050	980076ac	51	2026-10-30 13:28:26.557788+00	{"EndTurn": {"player": "Carol"}}
29051	2ac89bef	0	2026-10-30 13:09:26.818888+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
29052	2ac89bef	1	2026-10-30 13:10:26.818888+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29053	2ac89bef	2	2026-10-30 13:11:26.818888+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29054	2ac89bef	3	2026-10-30 13:12:26.818888+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29055	2ac89bef	4	2026-10-30 13:13:26.818888+00	"CreationDone"
29056	2ac89bef	5	2026-10-30 13:14:26.818888+00	{"SetupSpeaker": {"player": "Alice"}}
29057	2ac89bef	6	2026-10-30 13:15:26.818888+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29058	2ac89bef	7	2026-10-30 13:16:26.818888+00	"StartGame"
29059	2ac89bef	8	2026-10-30 13:17:26.818888+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29060	2ac89bef	9	2026-10-30 13:18:26.818888+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29061	2ac89bef	10	2026-10-30 13:19:26.818888+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29062	2ac89bef	11	2026-10-30 13:20:26.818888+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29063	2ac89bef	12	2026-10-30 13:21:26.818888+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29064	2ac89bef	13	2026-10-30 13:22:26.818888+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29065	2ac89bef	14	2026-10-30 13:23:26.818888+00	"CompleteStrategyPhase"
29066	2ac89bef	15	2026-10-30 13:24:26.818888+00	{"GainRelicAction": {"relic": "TheQuantumcore", "player": "Alice"}}
29067	2ac89bef	16	2026-10-30 13:25:26.818888+00	{"RollForTheFracture": {"player": "Alice", "entered_play": false}}
29068	2ac89bef	17	2026-10-30 13:26:26.818888+00	{"EndTurn": {"player": "Alice"}}
29069	2ac89bef	18	2026-10-30 13:27:26.818888+00	{"SetRelicOwner": {"relic": "TheQuantumcore", "player": "Bob"}}
29070	2ac89bef	19	2026-10-30 13:28:26.818888+00	{"RollForTheFracture": {"player": "Bob", "entered_play": false}}
29071	a44dae41	0	2026-10-30 12:47:26.935455+00	{"ImportFromMilty": {"max_points": 10, "milty_data": {"hex_map": {"tiles": [{"system": "112", "position": {"Pos": {"ring": 0, "position": 0, "rotation": 0}}}, {"system": "97", "position": {"Pos": {"ring": 1, "position": 0, "rotation": 0}}}, {"system": "98", "position": {"Pos": {"ring": 1, "position": 1, "rotation": 0}}}, {"system": "99", "position": {"Pos": {"ring": 1, "position": 2, "rotation": 0}}}, {"system": "100", "position": {"Pos": {"ring": 1, "position": 3, "rotation": 0}}}, {"system": "101", "position": {"Pos": {"ring": 1, "position": 4, "rotation": 0}}}, {"system": "102", "position": {"Pos": {"ring": 1, "position": 5, "rotation": 0}}}, {"system": "103", "position": {"Pos": {"ring": 2, "position": 0, "rotation": 0}}}, {"system": "104", "position": {"Pos": {"ring": 2, "position": 1, "rotation": 0}}}, {"system": "105", "position": {"Pos": {"ring": 2, "position": 2, "rotation": 0}}}, {"system": "106", "position": {"Pos": {"ring": 2, "position": 3, "rotation": 0}}}, {"system": "107", "position": {"Pos": {"ring": 2, "position": 4, "rotation": 0}}}, {"system": "108", "position": {"Pos": {"ring": 2, "position": 5, "rotation": 0}}}, {"system": "109", "position": {"Pos": {"ring": 2, "position": 6, "rotation": 0}}}, {"system": "110", "position": {"Pos": {"ring": 2, "position": 7, "rotation": 0}}}, {"system": "111", "position": {"Pos": {"ring": 2, "position": 8, "rotation": 0}}}, {"system": "115", "position": {"Pos": {"ring": 2, "position": 9, "rotation": 0}}}, {"system": "116", "position": {"Pos": {"ring": 2, "position": 10, "rotation": 0}}}, {"system": "117", "position": {"Pos": {"ring": 2, "position": 11, "rotation": 0}}}, {"system": "96A", "position": {"Pos": {"ring": 3, "position": 0, "rotation": 0}}}, {"system": "95", "position": {"Pos": {"ring": 3, "position": 3, "rotation": 0}}}, {"system": "113", "position": {"Pos": {"ring": 3, "position": 5, "rotation": 0}}}, {"system": "94", "position": {"Pos": {"ring": 3, "position": 6, "rotation": 0}}}, {"system": "114", "position": {"Pos": {"ring": 3, "position": 7, "rotation": 0}}}, {"system": "93", "position": {"Pos": {"ring": 3, "position": 9, "rotation": 0}}}, {"system": "92", "position": {"Pos": {"ring": 3, "position": 12, "rotation": 0}}}, {"system": "82", "position": "OutsideGalaxy"}, {"system": "118", "position": "OutsideGalaxy"}, {"system": "125", "position": "OutsideGalaxy"}, {"system": "126", "position": "OutsideGalaxy"}, {"system": "127", "position": "OutsideGalaxy"}], "ring_count": 4}, "players": {"Ann": {"name": "Ann", "order": 0, "faction": "FirmamentObsidian"}, "Ben": {"name": "Ben", "order": 1, "faction": "DeepwroughtScholarate"}, "Cid": {"name": "Cid", "order": 2, "faction": "CrimsonRebellion"}, "Dee": {"name": "Dee", "order": 3, "faction": "RalNelConsortium"}, "Eve": {"name": "Eve", "order": 4, "faction": "LastBastion"}}, "game_name": "Scenario", "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": true, "prophecyOfKings": true}}}}
29072	a44dae41	1	2026-10-30 12:48:26.935455+00	"CreationDone"
29073	a44dae41	2	2026-10-30 12:49:26.935455+00	{"SetupPlayerTechs": {"player": "Ann", "technologies": ["NeuralMotivator"]}}
29074	a44dae41	3	2026-10-30 12:50:26.935455+00	{"SetupPlayerTechs": {"player": "Ben", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
29075	a44dae41	4	2026-10-30 12:51:26.935455+00	{"SetupPlayerTechs": {"player": "Cid", "technologies": ["PlasmaScoring"]}}
29076	a44dae41	5	2026-10-30 12:52:26.935455+00	{"SetupPlayerTechs": {"player": "Dee", "technologies": ["NeuralMotivator"]}}
29077	a44dae41	6	2026-10-30 12:53:26.935455+00	{"SetupPlayerTechs": {"player": "Eve", "technologies": ["SarweenTools"]}}
29078	a44dae41	7	2026-10-30 12:54:26.935455+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29079	a44dae41	8	2026-10-30 12:55:26.935455+00	"StartGame"
29080	a44dae41	9	2026-10-30 12:56:26.935455+00	{"RollForTheFracture": {"player": "Cid", "entered_play": false}}
29081	a44dae41	10	2026-10-30 12:57:26.935455+00	{"SetPlanetOwner": {"planet": "MecatolRexOmega", "player": "Ann"}}
29082	a44dae41	11	2026-10-30 12:58:26.935455+00	{"SetPlanetOwner": {"planet": "Faunus", "player": "Ben"}}
29083	a44dae41	12	2026-10-30 12:59:26.935455+00	{"SetPlanetOwner": {"planet": "Garbozia", "player": "Cid"}}
29084	a44dae41	13	2026-10-30 13:00:26.935455+00	{"SetPlanetOwner": {"planet": "Emelpar", "player": "Dee"}}
29085	a44dae41	14	2026-10-30 13:01:26.935455+00	{"SetPlanetOwner": {"planet": "Tempesta", "player": "Eve"}}
29086	a44dae41	15	2026-10-30 13:02:26.935455+00	{"SetPlanetOwner": {"planet": "Olergodt", "player": "Ann"}}
29087	a44dae41	16	2026-10-30 13:03:26.935455+00	{"SetPlanetOwner": {"planet": "Andeara", "player": "Ben"}}
29088	a44dae41	17	2026-10-30 13:04:26.935455+00	{"SetPlanetOwner": {"planet": "ViraPicsIII", "player": "Cid"}}
29089	a44dae41	18	2026-10-30 13:05:26.935455+00	{"SetPlanetOwner": {"planet": "Lesab", "player": "Dee"}}
29090	a44dae41	19	2026-10-30 13:06:26.935455+00	{"SetPlanetOwner": {"planet": "NewTerra", "player": "Eve"}}
29091	a44dae41	20	2026-10-30 13:07:26.935455+00	{"SetPlanetOwner": {"planet": "Tinnes", "player": "Ann"}}
29092	a44dae41	21	2026-10-30 13:08:26.935455+00	{"SetPlanetOwner": {"planet": "Cresius", "player": "Ben"}}
29093	a44dae41	22	2026-10-30 13:09:26.935455+00	{"SetPlanetOwner": {"planet": "LazulRex", "player": "Cid"}}
29094	a44dae41	23	2026-10-30 13:10:26.935455+00	{"SetPlanetOwner": {"planet": "Hercalor", "player": "Dee"}}
29095	a44dae41	24	2026-10-30 13:11:26.935455+00	{"SetPlanetOwner": {"planet": "Tiamat", "player": "Eve"}}
29096	a44dae41	25	2026-10-30 13:12:26.935455+00	{"SetPlanetOwner": {"planet": "Capha", "player": "Ann"}}
29097	a44dae41	26	2026-10-30 13:13:26.935455+00	{"SetPlanetOwner": {"planet": "Kostboth", "player": "Ben"}}
29098	a44dae41	27	2026-10-30 13:14:26.935455+00	{"SetPlanetOwner": {"planet": "Bellatrix", "player": "Cid"}}
29099	a44dae41	28	2026-10-30 13:15:26.935455+00	{"SetPlanetOwner": {"planet": "TsionStation", "player": "Dee"}}
29100	a44dae41	29	2026-10-30 13:16:26.935455+00	{"SetPlanetOwner": {"planet": "Elnath", "player": "Eve"}}
29101	a44dae41	30	2026-10-30 13:17:26.935455+00	{"SetPlanetOwner": {"planet": "LuthienVi", "player": "Ann"}}
29102	a44dae41	31	2026-10-30 13:18:26.935455+00	{"SetPlanetOwner": {"planet": "Horizon", "player": "Ben"}}
29103	a44dae41	32	2026-10-30 13:19:26.935455+00	{"SetPlanetOwner": {"planet": "Tarana", "player": "Cid"}}
29104	a44dae41	33	2026-10-30 13:20:26.935455+00	{"SetPlanetOwner": {"planet": "OluzStation", "player": "Dee"}}
29105	a44dae41	34	2026-10-30 13:21:26.935455+00	{"SetPlanetOwner": {"planet": "Industrex", "player": "Eve"}}
29106	a44dae41	35	2026-10-30 13:22:26.935455+00	{"SetPlanetOwner": {"planet": "Lemox", "player": "Ann"}}
29107	a44dae41	36	2026-10-30 13:23:26.935455+00	{"SetPlanetOwner": {"planet": "TheWatchtower", "player": "Ben"}}
29108	a44dae41	37	2026-10-30 13:24:26.935455+00	{"SetPlanetOwner": {"planet": "Cocytus", "player": "Cid"}}
29109	a44dae41	38	2026-10-30 13:25:26.935455+00	{"SetPlanetOwner": {"planet": "Lethe", "player": "Dee"}}
29110	a44dae41	39	2026-10-30 13:26:26.935455+00	{"SetPlanetOwner": {"planet": "Phlegethon", "player": "Eve"}}
29111	a44dae41	40	2026-10-30 13:27:26.935455+00	{"SetPlanetOwner": {"planet": "Styx", "player": "Ann"}}
29112	a44dae41	41	2026-10-30 13:28:26.935455+00	{"SetThundersEdgeCompletion": {"new_completion": {"won_by": "Eve", "placed_in": "114"}}}
29113	6cd7ff7b	0	2026-10-30 12:54:27.148347+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
29114	6cd7ff7b	1	2026-10-30 12:55:27.148347+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29115	6cd7ff7b	2	2026-10-30 12:56:27.148347+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29116	6cd7ff7b	3	2026-10-30 12:57:27.148347+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29117	6cd7ff7b	4	2026-10-30 12:58:27.148347+00	"CreationDone"
29118	6cd7ff7b	5	2026-10-30 12:59:27.148347+00	{"SetupSpeaker": {"player": "Alice"}}
29119	6cd7ff7b	6	2026-10-30 13:00:27.148347+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29120	6cd7ff7b	7	2026-10-30 13:01:27.148347+00	"StartGame"
29121	6cd7ff7b	8	2026-10-30 13:02:27.148347+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29122	6cd7ff7b	9	2026-10-30 13:03:27.148347+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29123	6cd7ff7b	10	2026-10-30 13:04:27.148347+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29124	6cd7ff7b	11	2026-10-30 13:05:27.148347+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29125	6cd7ff7b	12	2026-10-30 13:06:27.148347+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29126	6cd7ff7b	13	2026-10-30 13:07:27.148347+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29127	6cd7ff7b	14	2026-10-30 13:08:27.148347+00	"CompleteStrategyPhase"
29128	6cd7ff7b	15	2026-10-30 13:09:27.148347+00	{"SetPlanetOwner": {"planet": "NewAlbion", "player": "Bob"}}
29129	6cd7ff7b	16	2026-10-30 13:10:27.148347+00	{"SetPlanetOwner": {"planet": "Gral", "player": "Bob"}}
29130	6cd7ff7b	17	2026-10-30 13:11:27.148347+00	{"SetPlanetOwner": {"planet": "Lazar", "player": "Bob"}}
29131	6cd7ff7b	18	2026-10-30 13:12:27.148347+00	{"SetPlanetOwner": {"planet": "Abyz", "player": "Bob"}}
29132	6cd7ff7b	19	2026-10-30 13:13:27.148347+00	{"AddPlanetAttachment": {"planet": "Abyz", "player": "Bob", "attachment": "WarfareResearchFacility"}}
29133	6cd7ff7b	20	2026-10-30 13:14:27.148347+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29134	6cd7ff7b	21	2026-10-30 13:15:27.148347+00	"StrategicActionCommit"
29135	6cd7ff7b	22	2026-10-30 13:16:27.148347+00	{"EndTurn": {"player": "Alice"}}
29136	6cd7ff7b	23	2026-10-30 13:17:27.148347+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Bob"}}
29137	6cd7ff7b	24	2026-10-30 13:18:27.148347+00	{"TakeAnotherTurn": {"player": "Bob"}}
29138	6cd7ff7b	25	2026-10-30 13:19:27.148347+00	{"RelicActionBegin": {"relic": "BookOfLatvinia", "player": "Bob"}}
29139	6cd7ff7b	26	2026-10-30 13:20:27.148347+00	{"RelicActionCommit": {"data": null, "player": "Bob"}}
29140	6cd7ff7b	27	2026-10-30 13:21:27.148347+00	{"EndTurn": {"player": "Bob"}}
29141	6cd7ff7b	28	2026-10-30 13:22:27.148347+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Carol"}}
29142	6cd7ff7b	29	2026-10-30 13:23:27.148347+00	{"TakeAnotherTurn": {"player": "Carol"}}
29143	6cd7ff7b	30	2026-10-30 13:24:27.148347+00	{"RelicActionBegin": {"relic": "BookOfLatvinia", "player": "Carol"}}
29144	6cd7ff7b	31	2026-10-30 13:25:27.148347+00	{"RelicActionCommit": {"data": null, "player": "Carol"}}
29145	6cd7ff7b	32	2026-10-30 13:26:27.148347+00	{"EndTurn": {"player": "Carol"}}
29146	6cd7ff7b	33	2026-10-30 13:27:27.148347+00	{"SetBookOfLatviniaOwner": {"player": "Carol"}}
29147	6cd7ff7b	34	2026-10-30 13:28:27.148347+00	{"SetBookOfLatviniaOwner": {"player": null}}
29148	6762a561	0	2026-10-30 12:51:27.313954+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
29149	6762a561	1	2026-10-30 12:52:27.313954+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29150	6762a561	2	2026-10-30 12:53:27.313954+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29151	6762a561	3	2026-10-30 12:54:27.313954+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29152	6762a561	4	2026-10-30 12:55:27.313954+00	"CreationDone"
29153	6762a561	5	2026-10-30 12:56:27.313954+00	{"SetupSpeaker": {"player": "Alice"}}
29154	6762a561	6	2026-10-30 12:57:27.313954+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29155	6762a561	7	2026-10-30 12:58:27.313954+00	"StartGame"
29156	6762a561	8	2026-10-30 12:59:27.313954+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29157	6762a561	9	2026-10-30 13:00:27.313954+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29158	6762a561	10	2026-10-30 13:01:27.313954+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29159	6762a561	11	2026-10-30 13:02:27.313954+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29160	6762a561	12	2026-10-30 13:03:27.313954+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29161	6762a561	13	2026-10-30 13:04:27.313954+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29162	6762a561	14	2026-10-30 13:05:27.313954+00	"CompleteStrategyPhase"
29163	6762a561	15	2026-10-30 13:06:27.313954+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Alice"}}
29164	6762a561	16	2026-10-30 13:07:27.313954+00	{"ResearchBookOfLatviniaTechs": {"player": "Alice", "technologies": ["SarweenTools", "PlasmaScoring"]}}
29165	6762a561	17	2026-10-30 13:08:27.313954+00	{"EndTurn": {"player": "Alice"}}
29166	6762a561	18	2026-10-30 13:09:27.313954+00	{"TacticalActionBegin": {"player": "Bob"}}
29167	6762a561	19	2026-10-30 13:10:27.313954+00	{"TacticalActionCommit": {"player": "Bob"}}
29168	6762a561	20	2026-10-30 13:11:27.313954+00	{"EndTurn": {"player": "Bob"}}
29169	6762a561	21	2026-10-30 13:12:27.313954+00	{"TacticalActionBegin": {"player": "Carol"}}
29170	6762a561	22	2026-10-30 13:13:27.313954+00	{"TacticalActionCommit": {"player": "Carol"}}
29171	6762a561	23	2026-10-30 13:14:27.313954+00	{"EndTurn": {"player": "Carol"}}
29172	6762a561	24	2026-10-30 13:15:27.313954+00	{"RelicActionBegin": {"relic": "BookOfLatvinia", "player": "Alice"}}
29173	6762a561	25	2026-10-30 13:16:27.313954+00	{"RelicActionCommit": {"data": null, "player": "Alice"}}
29174	6762a561	26	2026-10-30 13:17:27.313954+00	{"EndTurn": {"player": "Alice"}}
29175	6762a561	27	2026-10-30 13:18:27.313954+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Bob"}}
29176	6762a561	28	2026-10-30 13:19:27.313954+00	{"ResearchBookOfLatviniaTechs": {"player": "Bob", "technologies": []}}
29177	6762a561	29	2026-10-30 13:20:27.313954+00	{"TakeAnotherTurn": {"player": "Bob"}}
29178	6762a561	30	2026-10-30 13:21:27.313954+00	{"RelicActionBegin": {"relic": "BookOfLatvinia", "player": "Bob"}}
29179	6762a561	31	2026-10-30 13:22:27.313954+00	{"RelicActionCommit": {"data": null, "player": "Bob"}}
29180	6762a561	32	2026-10-30 13:23:27.313954+00	{"EndTurn": {"player": "Bob"}}
29181	6762a561	33	2026-10-30 13:24:27.313954+00	{"GainRelicAction": {"relic": "BookOfLatvinia", "player": "Carol"}}
29182	6762a561	34	2026-10-30 13:25:27.313954+00	{"TakeAnotherTurn": {"player": "Carol"}}
29183	6762a561	35	2026-10-30 13:26:27.313954+00	{"RelicActionBegin": {"relic": "BookOfLatvinia", "player": "Carol"}}
29184	6762a561	36	2026-10-30 13:27:27.313954+00	{"RelicActionCommit": {"data": null, "player": "Carol"}}
29185	6762a561	37	2026-10-30 13:28:27.313954+00	{"EndTurn": {"player": "Carol"}}
29186	8125c3f2	0	2026-10-30 12:09:27.510518+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
29187	8125c3f2	1	2026-10-30 12:10:27.510518+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29188	8125c3f2	2	2026-10-30 12:11:27.510518+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29189	8125c3f2	3	2026-10-30 12:12:27.510518+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29190	8125c3f2	4	2026-10-30 12:13:27.510518+00	"CreationDone"
29191	8125c3f2	5	2026-10-30 12:14:27.510518+00	{"SetupSpeaker": {"player": "Alice"}}
29192	8125c3f2	6	2026-10-30 12:15:27.510518+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29193	8125c3f2	7	2026-10-30 12:16:27.510518+00	"StartGame"
29194	8125c3f2	8	2026-10-30 12:17:27.510518+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29195	8125c3f2	9	2026-10-30 12:18:27.510518+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29196	8125c3f2	10	2026-10-30 12:19:27.510518+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29197	8125c3f2	11	2026-10-30 12:20:27.510518+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29198	8125c3f2	12	2026-10-30 12:21:27.510518+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29199	8125c3f2	13	2026-10-30 12:22:27.510518+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29200	8125c3f2	14	2026-10-30 12:23:27.510518+00	"CompleteStrategyPhase"
29201	8125c3f2	15	2026-10-30 12:24:27.510518+00	{"GainRelicAction": {"relic": "Neuraloop", "player": "Alice"}}
29202	8125c3f2	16	2026-10-30 12:25:27.510518+00	{"TakeAnotherTurn": {"player": "Alice"}}
29203	8125c3f2	17	2026-10-30 12:26:27.510518+00	{"GainRelicAction": {"relic": "ShardOfTheThrone", "player": "Alice"}}
29204	8125c3f2	18	2026-10-30 12:27:27.510518+00	{"EndTurn": {"player": "Alice"}}
29205	8125c3f2	19	2026-10-30 12:28:27.510518+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29206	8125c3f2	20	2026-10-30 12:29:27.510518+00	"StrategicActionCommit"
29207	8125c3f2	21	2026-10-30 12:30:27.510518+00	{"EndTurn": {"player": "Bob"}}
29208	8125c3f2	22	2026-10-30 12:31:27.510518+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
29209	8125c3f2	23	2026-10-30 12:32:27.510518+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
29210	8125c3f2	24	2026-10-30 12:33:27.510518+00	"StrategicActionCommit"
29211	8125c3f2	25	2026-10-30 12:34:27.510518+00	{"EndTurn": {"player": "Carol"}}
29212	8125c3f2	26	2026-10-30 12:35:27.510518+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29213	8125c3f2	27	2026-10-30 12:36:27.510518+00	"StrategicActionCommit"
29214	8125c3f2	28	2026-10-30 12:37:27.510518+00	{"EndTurn": {"player": "Alice"}}
29215	8125c3f2	29	2026-10-30 12:38:27.510518+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
29216	8125c3f2	30	2026-10-30 12:39:27.510518+00	"StrategicActionCommit"
29217	8125c3f2	31	2026-10-30 12:40:27.510518+00	{"EndTurn": {"player": "Bob"}}
29218	8125c3f2	32	2026-10-30 12:41:27.510518+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29219	8125c3f2	33	2026-10-30 12:42:27.510518+00	"StrategicActionCommit"
29220	8125c3f2	34	2026-10-30 12:43:27.510518+00	{"EndTurn": {"player": "Carol"}}
29221	8125c3f2	35	2026-10-30 12:44:27.510518+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
29222	8125c3f2	36	2026-10-30 12:45:27.510518+00	"StrategicActionCommit"
29223	8125c3f2	37	2026-10-30 12:46:27.510518+00	{"EndTurn": {"player": "Alice"}}
29224	8125c3f2	38	2026-10-30 12:47:27.510518+00	{"PassAction": {"player": "Bob"}}
29225	8125c3f2	39	2026-10-30 12:48:27.510518+00	{"PassAction": {"player": "Carol"}}
29226	8125c3f2	40	2026-10-30 12:49:27.510518+00	{"PassAction": {"player": "Alice"}}
29227	8125c3f2	41	2026-10-30 12:50:27.510518+00	{"ScoreSecretObjective": {"player": "Bob", "objective": "BecomeTheGatekeeper"}}
29228	8125c3f2	42	2026-10-30 12:51:27.510518+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
29229	8125c3f2	43	2026-10-30 12:52:27.510518+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29230	8125c3f2	44	2026-10-30 12:53:27.510518+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29231	8125c3f2	45	2026-10-30 12:54:27.510518+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29232	8125c3f2	46	2026-10-30 12:55:27.510518+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29233	8125c3f2	47	2026-10-30 12:56:27.510518+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
29234	8125c3f2	48	2026-10-30 12:57:27.510518+00	{"PlayNeuraloop": {"player": "Alice", "replacement": "DestroyTheirGreatestShip", "purged_relic": "ShardOfTheThrone"}}
29235	8125c3f2	49	2026-10-30 12:58:27.510518+00	"CompleteStatusPhase"
29236	8125c3f2	50	2026-10-30 12:59:27.510518+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29237	8125c3f2	51	2026-10-30 13:00:27.510518+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29238	8125c3f2	52	2026-10-30 13:01:27.510518+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29239	8125c3f2	53	2026-10-30 13:02:27.510518+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29240	8125c3f2	54	2026-10-30 13:03:27.510518+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29241	8125c3f2	55	2026-10-30 13:04:27.510518+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29242	8125c3f2	56	2026-10-30 13:05:27.510518+00	"CompleteStrategyPhase"
29243	8125c3f2	57	2026-10-30 13:06:27.510518+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29244	8125c3f2	58	2026-10-30 13:07:27.510518+00	"StrategicActionCommit"
29245	8125c3f2	59	2026-10-30 13:08:27.510518+00	{"EndTurn": {"player": "Alice"}}
29246	8125c3f2	60	2026-10-30 13:09:27.510518+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29247	8125c3f2	61	2026-10-30 13:10:27.510518+00	"StrategicActionCommit"
29248	8125c3f2	62	2026-10-30 13:11:27.510518+00	{"EndTurn": {"player": "Bob"}}
29249	8125c3f2	63	2026-10-30 13:12:27.510518+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
29250	8125c3f2	64	2026-10-30 13:13:27.510518+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
29251	8125c3f2	65	2026-10-30 13:14:27.510518+00	"StrategicActionCommit"
29252	8125c3f2	66	2026-10-30 13:15:27.510518+00	{"EndTurn": {"player": "Carol"}}
29253	8125c3f2	67	2026-10-30 13:16:27.510518+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
29254	8125c3f2	68	2026-10-30 13:17:27.510518+00	"StrategicActionCommit"
29255	8125c3f2	69	2026-10-30 13:18:27.510518+00	{"EndTurn": {"player": "Alice"}}
29256	8125c3f2	70	2026-10-30 13:19:27.510518+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
29257	8125c3f2	71	2026-10-30 13:20:27.510518+00	"StrategicActionCommit"
29258	8125c3f2	72	2026-10-30 13:21:27.510518+00	{"EndTurn": {"player": "Bob"}}
29259	8125c3f2	73	2026-10-30 13:22:27.510518+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29260	8125c3f2	74	2026-10-30 13:23:27.510518+00	"StrategicActionCommit"
29261	8125c3f2	75	2026-10-30 13:24:27.510518+00	{"EndTurn": {"player": "Carol"}}
29262	8125c3f2	76	2026-10-30 13:25:27.510518+00	{"PassAction": {"player": "Alice"}}
29263	8125c3f2	77	2026-10-30 13:26:27.510518+00	{"PassAction": {"player": "Bob"}}
29264	8125c3f2	78	2026-10-30 13:27:27.510518+00	{"PassAction": {"player": "Carol"}}
29265	8125c3f2	79	2026-10-30 13:28:27.510518+00	{"ScorePublicObjective": {"player": "Alice", "objective": "DestroyTheirGreatestShip"}}
29266	e3f1e7db	0	2026-10-30 12:29:27.868538+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
29267	e3f1e7db	1	2026-10-30 12:29:27.868538+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29268	e3f1e7db	2	2026-10-30 12:29:27.868538+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29269	e3f1e7db	3	2026-10-30 12:29:27.868538+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29270	e3f1e7db	4	2026-10-30 12:29:27.868538+00	"CreationDone"
29271	e3f1e7db	5	2026-10-30 12:29:27.868538+00	{"SetupSpeaker": {"player": "Alice"}}
29272	e3f1e7db	6	2026-10-30 12:29:27.868538+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29273	e3f1e7db	7	2026-10-30 12:29:27.868538+00	"StartGame"
29274	e3f1e7db	8	2026-10-30 12:30:27.868538+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29275	e3f1e7db	9	2026-10-30 12:31:27.868538+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29276	e3f1e7db	10	2026-10-30 12:32:27.868538+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29277	e3f1e7db	11	2026-10-30 12:33:27.868538+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29278	e3f1e7db	12	2026-10-30 12:34:27.868538+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29279	e3f1e7db	13	2026-10-30 12:35:27.868538+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29280	e3f1e7db	14	2026-10-30 12:35:27.868538+00	"CompleteStrategyPhase"
29281	e3f1e7db	15	2026-10-30 12:40:27.868538+00	{"TrackTime": {"paused": true}}
29282	e3f1e7db	16	2026-10-30 13:10:27.868538+00	{"TrackTime": {"paused": false}}
29283	e3f1e7db	17	2026-10-30 13:12:27.868538+00	{"TacticalActionBegin": {"player": "Alice"}}
29284	e3f1e7db	18	2026-10-30 13:12:27.868538+00	{"TacticalActionCommit": {"player": "Alice"}}
29285	e3f1e7db	19	2026-10-30 13:12:27.868538+00	{"EndTurn": {"player": "Alice"}}
29286	e3f1e7db	20	2026-10-30 13:15:27.868538+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29287	e3f1e7db	21	2026-10-30 13:15:27.868538+00	"StrategicActionCommit"
29288	e3f1e7db	22	2026-10-30 13:15:27.868538+00	{"EndTurn": {"player": "Bob"}}
29289	e3f1e7db	23	2026-10-30 13:19:27.868538+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
29290	e3f1e7db	24	2026-10-30 13:19:27.868538+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Carol"}}
29291	e3f1e7db	25	2026-10-30 13:19:27.868538+00	"StrategicActionCommit"
29292	e3f1e7db	26	2026-10-30 13:19:27.868538+00	{"EndTurn": {"player": "Carol"}}
29293	e3f1e7db	27	2026-10-30 13:19:27.868538+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29294	e3f1e7db	28	2026-10-30 13:19:27.868538+00	"StrategicActionCommit"
29295	e3f1e7db	29	2026-10-30 13:19:27.868538+00	{"EndTurn": {"player": "Alice"}}
29296	e3f1e7db	30	2026-10-30 13:19:27.868538+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
29297	e3f1e7db	31	2026-10-30 13:19:27.868538+00	"StrategicActionCommit"
29298	e3f1e7db	32	2026-10-30 13:19:27.868538+00	{"EndTurn": {"player": "Bob"}}
29299	e3f1e7db	33	2026-10-30 13:19:27.868538+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29300	e3f1e7db	34	2026-10-30 13:19:27.868538+00	"StrategicActionCommit"
29301	e3f1e7db	35	2026-10-30 13:19:27.868538+00	{"EndTurn": {"player": "Carol"}}
29302	e3f1e7db	36	2026-10-30 13:19:27.868538+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
29303	e3f1e7db	37	2026-10-30 13:19:27.868538+00	"StrategicActionCommit"
29304	e3f1e7db	38	2026-10-30 13:19:27.868538+00	{"EndTurn": {"player": "Alice"}}
29305	e3f1e7db	39	2026-10-30 13:19:27.868538+00	{"PassAction": {"player": "Bob"}}
29306	e3f1e7db	40	2026-10-30 13:19:27.868538+00	{"PassAction": {"player": "Carol"}}
29307	e3f1e7db	41	2026-10-30 13:19:27.868538+00	{"PassAction": {"player": "Alice"}}
29308	e3f1e7db	42	2026-10-30 13:29:27.868538+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
29309	e3f1e7db	43	2026-10-30 13:29:27.868538+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29310	e3f1e7db	44	2026-10-30 13:29:27.868538+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29311	e3f1e7db	45	2026-10-30 13:29:27.868538+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29312	e3f1e7db	46	2026-10-30 13:29:27.868538+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29313	e3f1e7db	47	2026-10-30 13:29:27.868538+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29314	e3f1e7db	48	2026-10-30 13:29:27.868538+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
29315	e3f1e7db	49	2026-10-30 13:29:27.868538+00	"CompleteStatusPhase"
29316	d68a3307	0	2026-10-30 09:30:28.095821+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": false, "codex2": false, "codex3": false, "codex4": false, "thundersEdge": false, "prophecyOfKings": false}}}}
29317	d68a3307	1	2026-10-30 09:31:28.095821+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "FederationOfSol"}}}
29318	d68a3307	2	2026-10-30 09:32:28.095821+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "EmiratesOfHacan"}}}
29319	d68a3307	3	2026-10-30 09:33:28.095821+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "XxchaKingdom"}}}
29320	d68a3307	4	2026-10-30 09:34:28.095821+00	"CreationDone"
29321	d68a3307	5	2026-10-30 09:35:28.095821+00	{"SetupSpeaker": {"player": "Alice"}}
29322	d68a3307	6	2026-10-30 09:36:28.095821+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29323	d68a3307	7	2026-10-30 09:37:28.095821+00	"StartGame"
29503	d68a3307	187	2026-10-30 12:37:28.095821+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Bob"}}
29324	d68a3307	8	2026-10-30 09:38:28.095821+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29325	d68a3307	9	2026-10-30 09:39:28.095821+00	{"TakeStrategyCard": {"card": "Trade", "player": "Bob"}}
29326	d68a3307	10	2026-10-30 09:40:28.095821+00	{"TakeStrategyCard": {"card": "Politics", "player": "Carol"}}
29327	d68a3307	11	2026-10-30 09:41:28.095821+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Alice"}}
29328	d68a3307	12	2026-10-30 09:42:28.095821+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Bob"}}
29329	d68a3307	13	2026-10-30 09:43:28.095821+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
29330	d68a3307	14	2026-10-30 09:44:28.095821+00	"CompleteStrategyPhase"
29331	d68a3307	15	2026-10-30 09:45:28.095821+00	{"TacticalActionBegin": {"player": "Alice"}}
29332	d68a3307	16	2026-10-30 09:46:28.095821+00	{"TacticalActionTakePlanet": {"planet": "Centauri", "player": "Alice"}}
29333	d68a3307	17	2026-10-30 09:47:28.095821+00	{"TacticalActionTakePlanet": {"planet": "Lodor", "player": "Alice"}}
29334	d68a3307	18	2026-10-30 09:48:28.095821+00	{"TacticalActionTakePlanet": {"planet": "Quann", "player": "Alice"}}
29335	d68a3307	19	2026-10-30 09:49:28.095821+00	{"TacticalActionCommit": {"player": "Alice"}}
29336	d68a3307	20	2026-10-30 09:50:28.095821+00	{"EndTurn": {"player": "Alice"}}
29337	d68a3307	21	2026-10-30 09:51:28.095821+00	{"StrategicActionBegin": {"card": "Technology", "player": "Carol"}}
29338	d68a3307	22	2026-10-30 09:52:28.095821+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": "GravityDrive", "second_tech": null}}, "player": "Carol"}}
29339	d68a3307	23	2026-10-30 09:53:28.095821+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "SarweenTools"}}, "player": "Alice"}}
29340	d68a3307	24	2026-10-30 09:54:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29341	d68a3307	25	2026-10-30 09:55:28.095821+00	"StrategicActionCommit"
29342	d68a3307	26	2026-10-30 09:56:28.095821+00	{"EndTurn": {"player": "Carol"}}
29343	d68a3307	27	2026-10-30 09:57:28.095821+00	{"TacticalActionBegin": {"player": "Bob"}}
29344	d68a3307	28	2026-10-30 09:58:28.095821+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Bob"}}
29345	d68a3307	29	2026-10-30 09:59:28.095821+00	{"TacticalActionCommit": {"player": "Bob"}}
29346	d68a3307	30	2026-10-30 10:00:28.095821+00	{"EndTurn": {"player": "Bob"}}
29347	d68a3307	31	2026-10-30 10:01:28.095821+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29348	d68a3307	32	2026-10-30 10:02:28.095821+00	"StrategicActionCommit"
29349	d68a3307	33	2026-10-30 10:03:28.095821+00	{"EndTurn": {"player": "Alice"}}
29350	d68a3307	34	2026-10-30 10:04:28.095821+00	{"StrategicActionBegin": {"card": "Politics", "player": "Carol"}}
29351	d68a3307	35	2026-10-30 10:05:28.095821+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Bob"}}, "player": "Carol"}}
29352	d68a3307	36	2026-10-30 10:06:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
29353	d68a3307	37	2026-10-30 10:07:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29354	d68a3307	38	2026-10-30 10:08:28.095821+00	"StrategicActionCommit"
29355	d68a3307	39	2026-10-30 10:09:28.095821+00	{"EndTurn": {"player": "Carol"}}
29356	d68a3307	40	2026-10-30 10:10:28.095821+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Bob"}}
29357	d68a3307	41	2026-10-30 10:11:28.095821+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": null}}, "player": "Bob"}}
29358	d68a3307	42	2026-10-30 10:12:28.095821+00	"StrategicActionCommit"
29359	d68a3307	43	2026-10-30 10:13:28.095821+00	{"EndTurn": {"player": "Bob"}}
29360	d68a3307	44	2026-10-30 10:14:28.095821+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Alice"}}
29361	d68a3307	45	2026-10-30 10:15:28.095821+00	"StrategicActionCommit"
29362	d68a3307	46	2026-10-30 10:16:28.095821+00	{"EndTurn": {"player": "Alice"}}
29363	d68a3307	47	2026-10-30 10:17:28.095821+00	{"PassAction": {"player": "Carol"}}
29364	d68a3307	48	2026-10-30 10:18:28.095821+00	{"StrategicActionBegin": {"card": "Trade", "player": "Bob"}}
29365	d68a3307	49	2026-10-30 10:19:28.095821+00	"StrategicActionCommit"
29366	d68a3307	50	2026-10-30 10:20:28.095821+00	{"EndTurn": {"player": "Bob"}}
29367	d68a3307	51	2026-10-30 10:21:28.095821+00	{"PassAction": {"player": "Alice"}}
29368	d68a3307	52	2026-10-30 10:22:28.095821+00	{"PassAction": {"player": "Bob"}}
29369	d68a3307	53	2026-10-30 10:23:28.095821+00	{"ScorePublicObjective": {"player": "Alice", "objective": "CornerTheMarket"}}
29370	d68a3307	54	2026-10-30 10:24:28.095821+00	{"ScoreSecretObjective": {"player": "Bob", "objective": "BecomeTheGatekeeper"}}
29371	d68a3307	55	2026-10-30 10:25:28.095821+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29372	d68a3307	56	2026-10-30 10:26:28.095821+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29373	d68a3307	57	2026-10-30 10:27:28.095821+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29374	d68a3307	58	2026-10-30 10:28:28.095821+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29375	d68a3307	59	2026-10-30 10:29:28.095821+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
29376	d68a3307	60	2026-10-30 10:30:28.095821+00	"CompleteStatusPhase"
29377	d68a3307	61	2026-10-30 10:31:28.095821+00	{"RevealAgenda": {"agenda": "ConventionsOfWar"}}
29378	d68a3307	62	2026-10-30 10:32:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29379	d68a3307	63	2026-10-30 10:33:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Alice", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29380	d68a3307	64	2026-10-30 10:34:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29381	d68a3307	65	2026-10-30 10:35:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29382	d68a3307	66	2026-10-30 10:36:28.095821+00	{"RevealAgenda": {"agenda": "Mutiny"}}
29383	d68a3307	67	2026-10-30 10:37:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29504	d68a3307	188	2026-10-30 12:38:28.095821+00	{"TakeStrategyCard": {"card": "Trade", "player": "Carol"}}
29384	d68a3307	68	2026-10-30 10:38:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29385	d68a3307	69	2026-10-30 10:39:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29386	d68a3307	70	2026-10-30 10:40:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29387	d68a3307	71	2026-10-30 10:41:28.095821+00	"CompleteAgendaPhase"
29388	d68a3307	72	2026-10-30 10:42:28.095821+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Bob"}}
29389	d68a3307	73	2026-10-30 10:43:28.095821+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Carol"}}
29390	d68a3307	74	2026-10-30 10:44:28.095821+00	{"TakeStrategyCard": {"card": "Technology", "player": "Alice"}}
29391	d68a3307	75	2026-10-30 10:45:28.095821+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Bob"}}
29392	d68a3307	76	2026-10-30 10:46:28.095821+00	{"TakeStrategyCard": {"card": "Construction", "player": "Carol"}}
29393	d68a3307	77	2026-10-30 10:47:28.095821+00	{"TakeStrategyCard": {"card": "Trade", "player": "Alice"}}
29394	d68a3307	78	2026-10-30 10:48:28.095821+00	"CompleteStrategyPhase"
29395	d68a3307	79	2026-10-30 10:49:28.095821+00	{"TacticalActionBegin": {"player": "Bob"}}
29396	d68a3307	80	2026-10-30 10:50:28.095821+00	{"TacticalActionTakePlanet": {"planet": "Centauri", "player": "Bob"}}
29397	d68a3307	81	2026-10-30 10:51:28.095821+00	{"TacticalActionTakePlanet": {"planet": "Gral", "player": "Bob"}}
29398	d68a3307	82	2026-10-30 10:52:28.095821+00	{"TacticalActionCommit": {"player": "Bob"}}
29399	d68a3307	83	2026-10-30 10:53:28.095821+00	{"EndTurn": {"player": "Bob"}}
29400	d68a3307	84	2026-10-30 10:54:28.095821+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Carol"}}
29401	d68a3307	85	2026-10-30 10:55:28.095821+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "ErectAMonument"}}, "player": "Carol"}}
29402	d68a3307	86	2026-10-30 10:56:28.095821+00	"StrategicActionCommit"
29403	d68a3307	87	2026-10-30 10:57:28.095821+00	{"EndTurn": {"player": "Carol"}}
29404	d68a3307	88	2026-10-30 10:58:28.095821+00	{"StrategicActionBegin": {"card": "Technology", "player": "Alice"}}
29405	d68a3307	89	2026-10-30 10:59:28.095821+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": "DacxiveAnimators", "second_tech": "HyperMetabolism"}}, "player": "Alice"}}
29406	d68a3307	90	2026-10-30 11:00:28.095821+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "GravityDrive"}}, "player": "Bob"}}
29407	d68a3307	91	2026-10-30 11:01:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
29408	d68a3307	92	2026-10-30 11:02:28.095821+00	"StrategicActionCommit"
29409	d68a3307	93	2026-10-30 11:03:28.095821+00	{"EndTurn": {"player": "Alice"}}
29410	d68a3307	94	2026-10-30 11:04:28.095821+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Bob"}}
29411	d68a3307	95	2026-10-30 11:05:28.095821+00	"StrategicActionCommit"
29412	d68a3307	96	2026-10-30 11:06:28.095821+00	{"EndTurn": {"player": "Bob"}}
29413	d68a3307	97	2026-10-30 11:07:28.095821+00	{"StrategicActionBegin": {"card": "Construction", "player": "Carol"}}
29414	d68a3307	98	2026-10-30 11:08:28.095821+00	"StrategicActionCommit"
29415	d68a3307	99	2026-10-30 11:09:28.095821+00	{"EndTurn": {"player": "Carol"}}
29416	d68a3307	100	2026-10-30 11:10:28.095821+00	{"StrategicActionBegin": {"card": "Trade", "player": "Alice"}}
29417	d68a3307	101	2026-10-30 11:11:28.095821+00	"StrategicActionCommit"
29418	d68a3307	102	2026-10-30 11:12:28.095821+00	{"EndTurn": {"player": "Alice"}}
29419	d68a3307	103	2026-10-30 11:13:28.095821+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Bob"}}
29420	d68a3307	104	2026-10-30 11:14:28.095821+00	"StrategicActionCommit"
29421	d68a3307	105	2026-10-30 11:15:28.095821+00	{"EndTurn": {"player": "Bob"}}
29422	d68a3307	106	2026-10-30 11:16:28.095821+00	{"PassAction": {"player": "Carol"}}
29423	d68a3307	107	2026-10-30 11:17:28.095821+00	{"PassAction": {"player": "Alice"}}
29424	d68a3307	108	2026-10-30 11:18:28.095821+00	{"PassAction": {"player": "Bob"}}
29425	d68a3307	109	2026-10-30 11:19:28.095821+00	{"ScorePublicObjective": {"player": "Alice", "objective": "ErectAMonument"}}
29426	d68a3307	110	2026-10-30 11:20:28.095821+00	{"ScorePublicObjective": {"player": "Bob", "objective": "CornerTheMarket"}}
29427	d68a3307	111	2026-10-30 11:21:28.095821+00	{"ScoreSecretObjective": {"player": "Carol", "objective": "MonopolizeProduction"}}
29428	d68a3307	112	2026-10-30 11:22:28.095821+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29429	d68a3307	113	2026-10-30 11:23:28.095821+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29430	d68a3307	114	2026-10-30 11:24:28.095821+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29431	d68a3307	115	2026-10-30 11:25:28.095821+00	{"RevealPublicObjective": {"objective": "SwayTheCouncil"}}
29432	d68a3307	116	2026-10-30 11:26:28.095821+00	"CompleteStatusPhase"
29433	d68a3307	117	2026-10-30 11:27:28.095821+00	{"RevealAgenda": {"agenda": "SeedOfAnEmpire"}}
29434	d68a3307	118	2026-10-30 11:28:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29435	d68a3307	119	2026-10-30 11:29:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29436	d68a3307	120	2026-10-30 11:30:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29437	d68a3307	121	2026-10-30 11:31:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29438	d68a3307	122	2026-10-30 11:32:28.095821+00	{"RevealAgenda": {"agenda": "MinisterOfWar"}}
29439	d68a3307	123	2026-10-30 11:33:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Carol", "outcome": {"value": "Alice", "electKind": "Player"}}}
29440	d68a3307	124	2026-10-30 11:34:28.095821+00	{"CastAgendaVote": {"votes": 2, "player": "Alice", "outcome": {"value": "Alice", "electKind": "Player"}}}
29441	d68a3307	125	2026-10-30 11:35:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Bob", "outcome": {"value": "Bob", "electKind": "Player"}}}
29442	d68a3307	126	2026-10-30 11:36:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "Alice", "electKind": "Player"}}}
29443	d68a3307	127	2026-10-30 11:37:28.095821+00	"CompleteAgendaPhase"
29444	d68a3307	128	2026-10-30 11:38:28.095821+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Bob"}}
29445	d68a3307	129	2026-10-30 11:39:28.095821+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Carol"}}
29446	d68a3307	130	2026-10-30 11:40:28.095821+00	{"TakeStrategyCard": {"card": "Politics", "player": "Alice"}}
29447	d68a3307	131	2026-10-30 11:41:28.095821+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Bob"}}
29448	d68a3307	132	2026-10-30 11:42:28.095821+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Carol"}}
29449	d68a3307	133	2026-10-30 11:43:28.095821+00	{"TakeStrategyCard": {"card": "Construction", "player": "Alice"}}
29450	d68a3307	134	2026-10-30 11:44:28.095821+00	"CompleteStrategyPhase"
29451	d68a3307	135	2026-10-30 11:45:28.095821+00	{"TacticalActionBegin": {"player": "Carol"}}
29452	d68a3307	136	2026-10-30 11:46:28.095821+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Carol"}}
29453	d68a3307	137	2026-10-30 11:47:28.095821+00	{"TacticalActionCommit": {"player": "Carol"}}
29454	d68a3307	138	2026-10-30 11:48:28.095821+00	{"EndTurn": {"player": "Carol"}}
29455	d68a3307	139	2026-10-30 11:49:28.095821+00	{"StrategicActionBegin": {"card": "Politics", "player": "Alice"}}
29456	d68a3307	140	2026-10-30 11:50:28.095821+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Alice"}}, "player": "Alice"}}
29457	d68a3307	141	2026-10-30 11:51:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
29458	d68a3307	142	2026-10-30 11:52:28.095821+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Bob"}}
29459	d68a3307	143	2026-10-30 11:53:28.095821+00	"StrategicActionCommit"
29460	d68a3307	144	2026-10-30 11:54:28.095821+00	{"EndTurn": {"player": "Alice"}}
29461	d68a3307	145	2026-10-30 11:55:28.095821+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Bob"}}
29462	d68a3307	146	2026-10-30 11:56:28.095821+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "SwayTheCouncil"}}, "player": "Bob"}}
29463	d68a3307	147	2026-10-30 11:57:28.095821+00	"StrategicActionCommit"
29464	d68a3307	148	2026-10-30 11:58:28.095821+00	{"EndTurn": {"player": "Bob"}}
29465	d68a3307	149	2026-10-30 11:59:28.095821+00	{"GiveSupportForTheThrone": {"giver": "Carol", "receiver": "Alice"}}
29466	d68a3307	150	2026-10-30 12:00:28.095821+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Carol"}}
29467	d68a3307	151	2026-10-30 12:01:28.095821+00	"StrategicActionCommit"
29468	d68a3307	152	2026-10-30 12:02:28.095821+00	{"EndTurn": {"player": "Carol"}}
29469	d68a3307	153	2026-10-30 12:03:28.095821+00	{"StrategicActionBegin": {"card": "Construction", "player": "Alice"}}
29470	d68a3307	154	2026-10-30 12:04:28.095821+00	"StrategicActionCommit"
29471	d68a3307	155	2026-10-30 12:05:28.095821+00	{"EndTurn": {"player": "Alice"}}
29472	d68a3307	156	2026-10-30 12:06:28.095821+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Bob"}}
29473	d68a3307	157	2026-10-30 12:07:28.095821+00	"StrategicActionCommit"
29474	d68a3307	158	2026-10-30 12:08:28.095821+00	{"EndTurn": {"player": "Bob"}}
29475	d68a3307	159	2026-10-30 12:09:28.095821+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Carol"}}
29476	d68a3307	160	2026-10-30 12:10:28.095821+00	"StrategicActionCommit"
29477	d68a3307	161	2026-10-30 12:11:28.095821+00	{"EndTurn": {"player": "Carol"}}
29478	d68a3307	162	2026-10-30 12:12:28.095821+00	{"PassAction": {"player": "Alice"}}
29479	d68a3307	163	2026-10-30 12:13:28.095821+00	{"PassAction": {"player": "Bob"}}
29480	d68a3307	164	2026-10-30 12:14:28.095821+00	{"PassAction": {"player": "Carol"}}
29481	d68a3307	165	2026-10-30 12:15:28.095821+00	{"ScorePublicObjective": {"player": "Carol", "objective": "CornerTheMarket"}}
29482	d68a3307	166	2026-10-30 12:16:28.095821+00	{"ScorePublicObjective": {"player": "Alice", "objective": "SwayTheCouncil"}}
29483	d68a3307	167	2026-10-30 12:17:28.095821+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29484	d68a3307	168	2026-10-30 12:18:28.095821+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29485	d68a3307	169	2026-10-30 12:19:28.095821+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29486	d68a3307	170	2026-10-30 12:20:28.095821+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29487	d68a3307	171	2026-10-30 12:21:28.095821+00	{"RevealPublicObjective": {"objective": "ExpandBorders"}}
29488	d68a3307	172	2026-10-30 12:22:28.095821+00	"CompleteStatusPhase"
29489	d68a3307	173	2026-10-30 12:23:28.095821+00	{"RevealAgenda": {"agenda": "FleetRegulations"}}
29490	d68a3307	174	2026-10-30 12:24:28.095821+00	"VetoAgenda"
29491	d68a3307	175	2026-10-30 12:25:28.095821+00	{"RevealAgenda": {"agenda": "SharedResearch"}}
29492	d68a3307	176	2026-10-30 12:26:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29493	d68a3307	177	2026-10-30 12:27:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29494	d68a3307	178	2026-10-30 12:28:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29495	d68a3307	179	2026-10-30 12:29:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29496	d68a3307	180	2026-10-30 12:30:28.095821+00	{"RevealAgenda": {"agenda": "IncentiveProgram"}}
29497	d68a3307	181	2026-10-30 12:31:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29498	d68a3307	182	2026-10-30 12:32:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29499	d68a3307	183	2026-10-30 12:33:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29500	d68a3307	184	2026-10-30 12:34:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29501	d68a3307	185	2026-10-30 12:35:28.095821+00	"CompleteAgendaPhase"
29502	d68a3307	186	2026-10-30 12:36:28.095821+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Alice"}}
29505	d68a3307	189	2026-10-30 12:39:28.095821+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Alice"}}
29506	d68a3307	190	2026-10-30 12:40:28.095821+00	{"TakeStrategyCard": {"card": "Construction", "player": "Bob"}}
29507	d68a3307	191	2026-10-30 12:41:28.095821+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29508	d68a3307	192	2026-10-30 12:42:28.095821+00	"CompleteStrategyPhase"
29509	d68a3307	193	2026-10-30 12:43:28.095821+00	{"TacticalActionBegin": {"player": "Bob"}}
29510	d68a3307	194	2026-10-30 12:44:28.095821+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Bob"}}
29511	d68a3307	195	2026-10-30 12:45:28.095821+00	{"TacticalActionCommit": {"player": "Bob"}}
29512	d68a3307	196	2026-10-30 12:46:28.095821+00	{"EndTurn": {"player": "Bob"}}
29513	d68a3307	197	2026-10-30 12:47:28.095821+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Alice"}}
29514	d68a3307	198	2026-10-30 12:48:28.095821+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "ExpandBorders"}}, "player": "Alice"}}
29515	d68a3307	199	2026-10-30 12:49:28.095821+00	"StrategicActionCommit"
29516	d68a3307	200	2026-10-30 12:50:28.095821+00	{"EndTurn": {"player": "Alice"}}
29517	d68a3307	201	2026-10-30 12:51:28.095821+00	{"RepealLaw": {"law": "MinisterOfWar"}}
29518	d68a3307	202	2026-10-30 12:52:28.095821+00	{"StrategicActionBegin": {"card": "Trade", "player": "Carol"}}
29519	d68a3307	203	2026-10-30 12:53:28.095821+00	"StrategicActionCommit"
29520	d68a3307	204	2026-10-30 12:54:28.095821+00	{"EndTurn": {"player": "Carol"}}
29521	d68a3307	205	2026-10-30 12:55:28.095821+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Bob"}}
29522	d68a3307	206	2026-10-30 12:56:28.095821+00	"StrategicActionCommit"
29523	d68a3307	207	2026-10-30 12:57:28.095821+00	{"EndTurn": {"player": "Bob"}}
29524	d68a3307	208	2026-10-30 12:58:28.095821+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Alice"}}
29525	d68a3307	209	2026-10-30 12:59:28.095821+00	"StrategicActionCommit"
29526	d68a3307	210	2026-10-30 13:00:28.095821+00	{"EndTurn": {"player": "Alice"}}
29527	d68a3307	211	2026-10-30 13:01:28.095821+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29528	d68a3307	212	2026-10-30 13:02:28.095821+00	"StrategicActionCommit"
29529	d68a3307	213	2026-10-30 13:03:28.095821+00	{"EndTurn": {"player": "Carol"}}
29530	d68a3307	214	2026-10-30 13:04:28.095821+00	{"StrategicActionBegin": {"card": "Construction", "player": "Bob"}}
29531	d68a3307	215	2026-10-30 13:05:28.095821+00	"StrategicActionCommit"
29532	d68a3307	216	2026-10-30 13:06:28.095821+00	{"EndTurn": {"player": "Bob"}}
29533	d68a3307	217	2026-10-30 13:07:28.095821+00	{"PassAction": {"player": "Alice"}}
29534	d68a3307	218	2026-10-30 13:08:28.095821+00	{"PassAction": {"player": "Carol"}}
29535	d68a3307	219	2026-10-30 13:09:28.095821+00	{"PassAction": {"player": "Bob"}}
29536	d68a3307	220	2026-10-30 13:10:28.095821+00	{"ScorePublicObjective": {"player": "Bob", "objective": "ExpandBorders"}}
29537	d68a3307	221	2026-10-30 13:11:28.095821+00	{"ScorePublicObjective": {"player": "Alice", "objective": "DevelopWeaponry"}}
29538	d68a3307	222	2026-10-30 13:12:28.095821+00	{"ScoreSecretObjective": {"player": "Alice", "objective": "ThreatenEnemies"}}
29539	d68a3307	223	2026-10-30 13:13:28.095821+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29540	d68a3307	224	2026-10-30 13:14:28.095821+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29541	d68a3307	225	2026-10-30 13:15:28.095821+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29542	d68a3307	226	2026-10-30 13:16:28.095821+00	{"RevealPublicObjective": {"objective": "CentralizeGalacticTrade"}}
29543	d68a3307	227	2026-10-30 13:17:28.095821+00	"CompleteStatusPhase"
29544	d68a3307	228	2026-10-30 13:18:28.095821+00	{"RevealAgenda": {"agenda": "HomelandDefenseAct"}}
29545	d68a3307	229	2026-10-30 13:19:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Bob", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29546	d68a3307	230	2026-10-30 13:20:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Carol", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29547	d68a3307	231	2026-10-30 13:21:28.095821+00	{"CastAgendaVote": {"votes": 3, "player": "Alice", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29548	d68a3307	232	2026-10-30 13:22:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29549	d68a3307	233	2026-10-30 13:23:28.095821+00	{"RevealAgenda": {"agenda": "PublicExecution"}}
29550	d68a3307	234	2026-10-30 13:24:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Carol", "electKind": "Player"}}}
29551	d68a3307	235	2026-10-30 13:25:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Carol", "outcome": {"value": "Carol", "electKind": "Player"}}}
29552	d68a3307	236	2026-10-30 13:26:28.095821+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Carol", "electKind": "Player"}}}
29553	d68a3307	237	2026-10-30 13:27:28.095821+00	{"ResolveAgenda": {"outcome": {"value": "Carol", "electKind": "Player"}}}
29554	d68a3307	238	2026-10-30 13:28:28.095821+00	"CompleteAgendaPhase"
29555	1e7850a0	0	2026-10-30 09:37:29.174023+00	{"SetSettings": {"settings": {"maxPoints": 10, "expansions": {"codex1": true, "codex2": true, "codex3": true, "codex4": true, "thundersEdge": false, "prophecyOfKings": true}}}}
29556	1e7850a0	1	2026-10-30 09:38:29.174023+00	{"AddPlayer": {"player": {"name": "Alice", "color": "Pink", "faction": "NaaluCollective"}}}
29557	1e7850a0	2	2026-10-30 09:39:29.174023+00	{"AddPlayer": {"player": {"name": "Bob", "color": "Orange", "faction": "UniversitiesOfJolNar"}}}
29558	1e7850a0	3	2026-10-30 09:40:29.174023+00	{"AddPlayer": {"player": {"name": "Carol", "color": "Green", "faction": "NekroVirus"}}}
29559	1e7850a0	4	2026-10-30 09:41:29.174023+00	{"AddPlayer": {"player": {"name": "Dave", "color": "Red", "faction": "Winnu"}}}
29560	1e7850a0	5	2026-10-30 09:42:29.174023+00	{"AddPlayer": {"player": {"name": "Erin", "color": "Yellow", "faction": "ArgentFlight"}}}
29561	1e7850a0	6	2026-10-30 09:43:29.174023+00	{"AddPlayer": {"player": {"name": "Finn", "color": "Black", "faction": "CouncilKeleres"}}}
29562	1e7850a0	7	2026-10-30 09:44:29.174023+00	"CreationDone"
29563	1e7850a0	8	2026-10-30 09:45:29.174023+00	{"SetupPlayerTechs": {"player": "Dave", "technologies": ["NeuralMotivator"]}}
29564	1e7850a0	9	2026-10-30 09:46:29.174023+00	{"SetupPlayerTechs": {"player": "Erin", "technologies": ["NeuralMotivator", "PlasmaScoring"]}}
29565	1e7850a0	10	2026-10-30 09:47:29.174023+00	{"SetupTheTribunii": {"player": "Finn", "faction": "MentakCoalition"}}
29566	1e7850a0	11	2026-10-30 09:48:29.174023+00	{"SetupPlayerTechs": {"player": "Finn", "technologies": ["AntimassDeflectors", "DacxiveAnimators"]}}
29567	1e7850a0	12	2026-10-30 09:49:29.174023+00	{"SetupSpeaker": {"player": "Alice"}}
29568	1e7850a0	13	2026-10-30 09:50:29.174023+00	{"RevealInitialObjectives": {"firstObjective": "CornerTheMarket", "secondObjective": "DevelopWeaponry"}}
29569	1e7850a0	14	2026-10-30 09:51:29.174023+00	"StartGame"
29570	1e7850a0	15	2026-10-30 09:52:29.174023+00	{"PlayGiftOfPrescience": {"player": "Dave"}}
29571	1e7850a0	16	2026-10-30 09:53:29.174023+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Alice"}}
29572	1e7850a0	17	2026-10-30 09:54:29.174023+00	{"TakeStrategyCard": {"card": "Technology", "player": "Bob"}}
29573	1e7850a0	18	2026-10-30 09:55:29.174023+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29574	1e7850a0	19	2026-10-30 09:56:29.174023+00	{"TakeStrategyCard": {"card": "Trade", "player": "Dave"}}
29575	1e7850a0	20	2026-10-30 09:57:29.174023+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Erin"}}
29576	1e7850a0	21	2026-10-30 09:58:29.174023+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Finn"}}
29577	1e7850a0	22	2026-10-30 09:59:29.174023+00	"CompleteStrategyPhase"
29578	1e7850a0	23	2026-10-30 10:00:29.174023+00	{"FrontierCardActionBegin": {"card": "EnigmaticDevice", "player": "Dave"}}
29579	1e7850a0	24	2026-10-30 10:01:29.174023+00	{"FrontierCardActionCommit": {"data": {"EnigmaticDevice": {"tech": "GravityDrive"}}, "player": "Dave"}}
29580	1e7850a0	25	2026-10-30 10:02:29.174023+00	{"EndTurn": {"player": "Dave"}}
29581	1e7850a0	26	2026-10-30 10:03:29.174023+00	{"GainRelicAction": {"relic": "ShardOfTheThrone", "player": "Erin"}}
29582	1e7850a0	27	2026-10-30 10:04:29.174023+00	{"EndTurn": {"player": "Erin"}}
29583	1e7850a0	28	2026-10-30 10:05:29.174023+00	{"TacticalActionBegin": {"player": "Finn"}}
29584	1e7850a0	29	2026-10-30 10:06:29.174023+00	{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Finn"}}
29585	1e7850a0	30	2026-10-30 10:07:29.174023+00	{"TacticalActionTakePlanet": {"planet": "Primor", "player": "Finn"}}
29586	1e7850a0	31	2026-10-30 10:08:29.174023+00	{"TacticalActionCommit": {"player": "Finn"}}
29587	1e7850a0	32	2026-10-30 10:09:29.174023+00	{"EndTurn": {"player": "Finn"}}
29588	1e7850a0	33	2026-10-30 10:10:29.174023+00	{"ActionCardActionBegin": {"card": "Plagiarize", "player": "Carol"}}
29589	1e7850a0	34	2026-10-30 10:11:29.174023+00	{"ActionCardActionCommit": {"data": {"Plagiarize": {"tech": "GravityDrive"}}, "player": "Carol"}}
29590	1e7850a0	35	2026-10-30 10:12:29.174023+00	{"EndTurn": {"player": "Carol"}}
29591	1e7850a0	36	2026-10-30 10:13:29.174023+00	{"StrategicActionBegin": {"card": "Technology", "player": "Bob"}}
29592	1e7850a0	37	2026-10-30 10:14:29.174023+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": "GravityDrive", "second_tech": "HyperMetabolism"}}, "player": "Bob"}}
29593	1e7850a0	38	2026-10-30 10:15:29.174023+00	{"StrategicActionSecondary": {"action": {"Technology": {"tech": "SarweenTools"}}, "player": "Dave"}}
29594	1e7850a0	39	2026-10-30 10:16:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Erin"}}
29595	1e7850a0	40	2026-10-30 10:17:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Finn"}}
29596	1e7850a0	41	2026-10-30 10:18:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
29597	1e7850a0	42	2026-10-30 10:19:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
29598	1e7850a0	43	2026-10-30 10:20:29.174023+00	"StrategicActionCommit"
29599	1e7850a0	44	2026-10-30 10:21:29.174023+00	{"EndTurn": {"player": "Bob"}}
29600	1e7850a0	45	2026-10-30 10:22:29.174023+00	{"TrackTime": {"paused": true}}
29601	1e7850a0	46	2026-10-30 10:38:29.174023+00	{"TrackTime": {"paused": false}}
29602	1e7850a0	47	2026-10-30 10:39:29.174023+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Alice"}}
29603	1e7850a0	48	2026-10-30 10:40:29.174023+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "DevelopWeaponry"}}, "player": "Alice"}}
29604	1e7850a0	49	2026-10-30 10:41:29.174023+00	"StrategicActionCommit"
29605	1e7850a0	50	2026-10-30 10:42:29.174023+00	{"EndTurn": {"player": "Alice"}}
29606	1e7850a0	51	2026-10-30 10:43:29.174023+00	{"GiveSupportForTheThrone": {"giver": "Bob", "receiver": "Finn"}}
29607	1e7850a0	52	2026-10-30 10:44:29.174023+00	{"StrategicActionBegin": {"card": "Trade", "player": "Dave"}}
29608	1e7850a0	53	2026-10-30 10:45:29.174023+00	"StrategicActionCommit"
29609	1e7850a0	54	2026-10-30 10:46:29.174023+00	{"EndTurn": {"player": "Dave"}}
29610	1e7850a0	55	2026-10-30 10:47:29.174023+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Erin"}}
29611	1e7850a0	56	2026-10-30 10:48:29.174023+00	"StrategicActionCommit"
29612	1e7850a0	57	2026-10-30 10:49:29.174023+00	{"EndTurn": {"player": "Erin"}}
29613	1e7850a0	58	2026-10-30 10:50:29.174023+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Finn"}}
29614	1e7850a0	59	2026-10-30 10:51:29.174023+00	"StrategicActionCommit"
29615	1e7850a0	60	2026-10-30 10:52:29.174023+00	{"EndTurn": {"player": "Finn"}}
29616	1e7850a0	61	2026-10-30 10:53:29.174023+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29617	1e7850a0	62	2026-10-30 10:54:29.174023+00	"StrategicActionCommit"
29618	1e7850a0	63	2026-10-30 10:55:29.174023+00	{"EndTurn": {"player": "Carol"}}
29619	1e7850a0	64	2026-10-30 10:56:29.174023+00	{"PassAction": {"player": "Bob"}}
29620	1e7850a0	65	2026-10-30 10:57:29.174023+00	{"PassAction": {"player": "Alice"}}
29621	1e7850a0	66	2026-10-30 10:58:29.174023+00	{"PassAction": {"player": "Dave"}}
29622	1e7850a0	67	2026-10-30 10:59:29.174023+00	{"PassAction": {"player": "Erin"}}
29623	1e7850a0	68	2026-10-30 11:00:29.174023+00	{"PassAction": {"player": "Finn"}}
29624	1e7850a0	69	2026-10-30 11:01:29.174023+00	{"PassAction": {"player": "Carol"}}
29738	1e7850a0	183	2026-10-30 12:55:29.174023+00	{"TakeStrategyCard": {"card": "Technology", "player": "Bob"}}
29625	1e7850a0	70	2026-10-30 11:02:29.174023+00	{"ScorePublicObjective": {"player": "Finn", "objective": "CornerTheMarket"}}
29626	1e7850a0	71	2026-10-30 11:03:29.174023+00	{"ScoreSecretObjective": {"player": "Erin", "objective": "BecomeTheGatekeeper"}}
29627	1e7850a0	72	2026-10-30 11:04:29.174023+00	{"ScorePublicObjective": {"player": "Dave", "objective": null}}
29628	1e7850a0	73	2026-10-30 11:05:29.174023+00	{"ScorePublicObjective": {"player": "Erin", "objective": null}}
29629	1e7850a0	74	2026-10-30 11:06:29.174023+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29630	1e7850a0	75	2026-10-30 11:07:29.174023+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29631	1e7850a0	76	2026-10-30 11:08:29.174023+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
29632	1e7850a0	77	2026-10-30 11:09:29.174023+00	{"ScoreSecretObjective": {"player": "Dave", "objective": null}}
29633	1e7850a0	78	2026-10-30 11:10:29.174023+00	{"ScoreSecretObjective": {"player": "Finn", "objective": null}}
29634	1e7850a0	79	2026-10-30 11:11:29.174023+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29635	1e7850a0	80	2026-10-30 11:12:29.174023+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29636	1e7850a0	81	2026-10-30 11:13:29.174023+00	{"ScoreSecretObjective": {"player": "Alice", "objective": null}}
29637	1e7850a0	82	2026-10-30 11:14:29.174023+00	{"RevealPublicObjective": {"objective": "ErectAMonument"}}
29638	1e7850a0	83	2026-10-30 11:15:29.174023+00	"CompleteStatusPhase"
29639	1e7850a0	84	2026-10-30 11:16:29.174023+00	{"RevealAgenda": {"agenda": "Mutiny"}}
29640	1e7850a0	85	2026-10-30 11:17:29.174023+00	{"CastAgendaVote": {"votes": 3, "player": "Erin", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29641	1e7850a0	86	2026-10-30 11:18:29.174023+00	{"CastAgendaVote": {"votes": 2, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29642	1e7850a0	87	2026-10-30 11:19:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Dave", "outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29643	1e7850a0	88	2026-10-30 11:20:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Finn", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29644	1e7850a0	89	2026-10-30 11:21:29.174023+00	{"CastAgendaVote": {"votes": 0, "player": "Alice", "outcome": null}}
29645	1e7850a0	90	2026-10-30 11:22:29.174023+00	{"ResolveAgenda": {"outcome": {"value": "For", "electKind": "ForOrAgainst"}}}
29646	1e7850a0	91	2026-10-30 11:23:29.174023+00	{"RevealAgenda": {"agenda": "PoliticalCensure"}}
29647	1e7850a0	92	2026-10-30 11:24:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Erin", "outcome": {"value": "Bob", "electKind": "Player"}}}
29648	1e7850a0	93	2026-10-30 11:25:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Bob", "electKind": "Player"}}}
29649	1e7850a0	94	2026-10-30 11:26:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Dave", "outcome": {"value": "Bob", "electKind": "Player"}}}
29650	1e7850a0	95	2026-10-30 11:27:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Finn", "outcome": {"value": "Bob", "electKind": "Player"}}}
29651	1e7850a0	96	2026-10-30 11:28:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Bob", "electKind": "Player"}}}
29652	1e7850a0	97	2026-10-30 11:29:29.174023+00	{"ResolveAgenda": {"outcome": {"value": "Bob", "electKind": "Player"}}}
29653	1e7850a0	98	2026-10-30 11:30:29.174023+00	"CompleteAgendaPhase"
29654	1e7850a0	99	2026-10-30 11:31:29.174023+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Alice"}}
29655	1e7850a0	100	2026-10-30 11:32:29.174023+00	{"TakeStrategyCard": {"card": "Politics", "player": "Bob"}}
29656	1e7850a0	101	2026-10-30 11:33:29.174023+00	{"TakeStrategyCard": {"card": "Technology", "player": "Carol"}}
29657	1e7850a0	102	2026-10-30 11:34:29.174023+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Dave"}}
29658	1e7850a0	103	2026-10-30 11:35:29.174023+00	{"TakeStrategyCard": {"card": "Construction", "player": "Erin"}}
29659	1e7850a0	104	2026-10-30 11:36:29.174023+00	{"TakeStrategyCard": {"card": "Trade", "player": "Finn"}}
29660	1e7850a0	105	2026-10-30 11:37:29.174023+00	"CompleteStrategyPhase"
29661	1e7850a0	106	2026-10-30 11:38:29.174023+00	{"LeaderActionBegin": {"leader": "ZeuCxIII", "player": "Alice"}}
29662	1e7850a0	107	2026-10-30 11:39:29.174023+00	{"EndTurn": {"player": "Alice"}}
29663	1e7850a0	108	2026-10-30 11:40:29.174023+00	{"StrategicActionBegin": {"card": "Politics", "player": "Bob"}}
29664	1e7850a0	109	2026-10-30 11:41:29.174023+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Erin"}}, "player": "Bob"}}
29665	1e7850a0	110	2026-10-30 11:42:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
29666	1e7850a0	111	2026-10-30 11:43:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Erin"}}
29667	1e7850a0	112	2026-10-30 11:44:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Finn"}}
29668	1e7850a0	113	2026-10-30 11:45:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Carol"}}
29669	1e7850a0	114	2026-10-30 11:46:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dave"}}
29670	1e7850a0	115	2026-10-30 11:47:29.174023+00	"StrategicActionCommit"
29671	1e7850a0	116	2026-10-30 11:48:29.174023+00	{"EndTurn": {"player": "Bob"}}
29672	1e7850a0	117	2026-10-30 11:49:29.174023+00	{"FrontierCardActionBegin": {"card": "Mirage", "player": "Erin"}}
29673	1e7850a0	118	2026-10-30 11:50:29.174023+00	{"FrontierCardActionCommit": {"data": {"Mirage": {"system": null, "attachment": "ParadiseWorld"}}, "player": "Erin"}}
29674	1e7850a0	119	2026-10-30 11:51:29.174023+00	{"EndTurn": {"player": "Erin"}}
29675	1e7850a0	120	2026-10-30 11:52:29.174023+00	{"TacticalActionBegin": {"player": "Finn"}}
29676	1e7850a0	121	2026-10-30 11:53:29.174023+00	{"TacticalActionTakePlanet": {"planet": "Mirage", "player": "Finn"}}
29677	1e7850a0	122	2026-10-30 11:54:29.174023+00	{"TacticalActionCommit": {"player": "Finn"}}
29678	1e7850a0	123	2026-10-30 11:55:29.174023+00	{"EndTurn": {"player": "Finn"}}
29679	1e7850a0	124	2026-10-30 11:56:29.174023+00	{"StrategicActionBegin": {"card": "Technology", "player": "Carol"}}
29737	1e7850a0	182	2026-10-30 12:54:29.174023+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Alice"}}
29680	1e7850a0	125	2026-10-30 11:57:29.174023+00	{"StrategicActionSecondary": {"action": {"TechnologyJolNar": {"firstTech": "LightWaveDeflector", "secondTech": null}}, "player": "Bob"}}
29681	1e7850a0	126	2026-10-30 11:58:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Alice"}}
29682	1e7850a0	127	2026-10-30 11:59:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Erin"}}
29683	1e7850a0	128	2026-10-30 12:00:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Finn"}}
29684	1e7850a0	129	2026-10-30 12:01:29.174023+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Dave"}}
29685	1e7850a0	130	2026-10-30 12:02:29.174023+00	"StrategicActionCommit"
29686	1e7850a0	131	2026-10-30 12:03:29.174023+00	{"EndTurn": {"player": "Carol"}}
29687	1e7850a0	132	2026-10-30 12:04:29.174023+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Dave"}}
29688	1e7850a0	133	2026-10-30 12:05:29.174023+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "ErectAMonument"}}, "player": "Dave"}}
29689	1e7850a0	134	2026-10-30 12:06:29.174023+00	"StrategicActionCommit"
29690	1e7850a0	135	2026-10-30 12:07:29.174023+00	{"EndTurn": {"player": "Dave"}}
29691	1e7850a0	136	2026-10-30 12:08:29.174023+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Alice"}}
29692	1e7850a0	137	2026-10-30 12:09:29.174023+00	"StrategicActionCommit"
29693	1e7850a0	138	2026-10-30 12:10:29.174023+00	{"EndTurn": {"player": "Alice"}}
29694	1e7850a0	139	2026-10-30 12:11:29.174023+00	{"PassAction": {"player": "Bob"}}
29695	1e7850a0	140	2026-10-30 12:12:29.174023+00	{"StrategicActionBegin": {"card": "Construction", "player": "Erin"}}
29696	1e7850a0	141	2026-10-30 12:13:29.174023+00	"StrategicActionCommit"
29697	1e7850a0	142	2026-10-30 12:14:29.174023+00	{"EndTurn": {"player": "Erin"}}
29698	1e7850a0	143	2026-10-30 12:15:29.174023+00	{"StrategicActionBegin": {"card": "Trade", "player": "Finn"}}
29699	1e7850a0	144	2026-10-30 12:16:29.174023+00	"StrategicActionCommit"
29700	1e7850a0	145	2026-10-30 12:17:29.174023+00	{"EndTurn": {"player": "Finn"}}
29701	1e7850a0	146	2026-10-30 12:18:29.174023+00	{"PassAction": {"player": "Carol"}}
29702	1e7850a0	147	2026-10-30 12:19:29.174023+00	{"PassAction": {"player": "Dave"}}
29703	1e7850a0	148	2026-10-30 12:20:29.174023+00	{"PassAction": {"player": "Alice"}}
29704	1e7850a0	149	2026-10-30 12:21:29.174023+00	{"PassAction": {"player": "Erin"}}
29705	1e7850a0	150	2026-10-30 12:22:29.174023+00	{"PassAction": {"player": "Finn"}}
29706	1e7850a0	151	2026-10-30 12:23:29.174023+00	{"ScorePublicObjective": {"player": "Dave", "objective": "CornerTheMarket"}}
29707	1e7850a0	152	2026-10-30 12:24:29.174023+00	{"ScoreSecretObjective": {"player": "Alice", "objective": "ThreatenEnemies"}}
29708	1e7850a0	153	2026-10-30 12:25:29.174023+00	{"ScorePublicObjective": {"player": "Alice", "objective": null}}
29709	1e7850a0	154	2026-10-30 12:26:29.174023+00	{"ScorePublicObjective": {"player": "Bob", "objective": null}}
29710	1e7850a0	155	2026-10-30 12:27:29.174023+00	{"ScorePublicObjective": {"player": "Erin", "objective": null}}
29711	1e7850a0	156	2026-10-30 12:28:29.174023+00	{"ScorePublicObjective": {"player": "Finn", "objective": null}}
29712	1e7850a0	157	2026-10-30 12:29:29.174023+00	{"ScorePublicObjective": {"player": "Carol", "objective": null}}
29713	1e7850a0	158	2026-10-30 12:30:29.174023+00	{"ScoreSecretObjective": {"player": "Bob", "objective": null}}
29714	1e7850a0	159	2026-10-30 12:31:29.174023+00	{"ScoreSecretObjective": {"player": "Erin", "objective": null}}
29715	1e7850a0	160	2026-10-30 12:32:29.174023+00	{"ScoreSecretObjective": {"player": "Finn", "objective": null}}
29716	1e7850a0	161	2026-10-30 12:33:29.174023+00	{"ScoreSecretObjective": {"player": "Carol", "objective": null}}
29717	1e7850a0	162	2026-10-30 12:34:29.174023+00	{"ScoreSecretObjective": {"player": "Dave", "objective": null}}
29718	1e7850a0	163	2026-10-30 12:35:29.174023+00	{"RevealPublicObjective": {"objective": "SwayTheCouncil"}}
29719	1e7850a0	164	2026-10-30 12:36:29.174023+00	"CompleteStatusPhase"
29720	1e7850a0	165	2026-10-30 12:37:29.174023+00	{"RevealAgenda": {"agenda": "SeedOfAnEmpire"}}
29721	1e7850a0	166	2026-10-30 12:38:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Erin", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29722	1e7850a0	167	2026-10-30 12:39:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Finn", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29723	1e7850a0	168	2026-10-30 12:40:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29724	1e7850a0	169	2026-10-30 12:41:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29725	1e7850a0	170	2026-10-30 12:42:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Dave", "outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29726	1e7850a0	171	2026-10-30 12:43:29.174023+00	{"ResolveAgenda": {"outcome": {"value": "Against", "electKind": "ForOrAgainst"}}}
29727	1e7850a0	172	2026-10-30 12:44:29.174023+00	{"RevealAgenda": {"agenda": "JudicialAbolishment"}}
29728	1e7850a0	173	2026-10-30 12:45:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Erin", "outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29729	1e7850a0	174	2026-10-30 12:46:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Finn", "outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29730	1e7850a0	175	2026-10-30 12:47:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Alice", "outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29731	1e7850a0	176	2026-10-30 12:48:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Bob", "outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29732	1e7850a0	177	2026-10-30 12:49:29.174023+00	{"CastAgendaVote": {"votes": 1, "player": "Dave", "outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29733	1e7850a0	178	2026-10-30 12:50:29.174023+00	{"ResolveAgenda": {"outcome": {"value": "PoliticalCensure", "electKind": "Law"}}}
29734	1e7850a0	179	2026-10-30 12:51:29.174023+00	"CompleteAgendaPhase"
29735	1e7850a0	180	2026-10-30 12:52:29.174023+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Erin"}}
29736	1e7850a0	181	2026-10-30 12:53:29.174023+00	{"TakeStrategyCard": {"card": "Imperial", "player": "Finn"}}
29739	1e7850a0	184	2026-10-30 12:56:29.174023+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Carol"}}
29740	1e7850a0	185	2026-10-30 12:57:29.174023+00	{"TakeStrategyCard": {"card": "Construction", "player": "Dave"}}
29741	1e7850a0	186	2026-10-30 12:58:29.174023+00	"CompleteStrategyPhase"
29742	1e7850a0	187	2026-10-30 12:59:29.174023+00	{"TacticalActionBegin": {"player": "Alice"}}
29743	1e7850a0	188	2026-10-30 13:00:29.174023+00	{"TacticalActionTakePlanet": {"planet": "Lodor", "player": "Alice"}}
29744	1e7850a0	189	2026-10-30 13:01:29.174023+00	{"TacticalActionCommit": {"player": "Alice"}}
29745	1e7850a0	190	2026-10-30 13:02:29.174023+00	{"EndTurn": {"player": "Alice"}}
29746	1e7850a0	191	2026-10-30 13:03:29.174023+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Erin"}}
29747	1e7850a0	192	2026-10-30 13:04:29.174023+00	"StrategicActionCommit"
29748	1e7850a0	193	2026-10-30 13:05:29.174023+00	{"EndTurn": {"player": "Erin"}}
29749	1e7850a0	194	2026-10-30 13:06:29.174023+00	{"StrategicActionBegin": {"card": "Construction", "player": "Dave"}}
29750	1e7850a0	195	2026-10-30 13:07:29.174023+00	"StrategicActionCommit"
29751	1e7850a0	196	2026-10-30 13:08:29.174023+00	{"EndTurn": {"player": "Dave"}}
29752	1e7850a0	197	2026-10-30 13:09:29.174023+00	{"StrategicActionBegin": {"card": "Warfare", "player": "Carol"}}
29753	1e7850a0	198	2026-10-30 13:10:29.174023+00	"StrategicActionCommit"
29754	1e7850a0	199	2026-10-30 13:11:29.174023+00	{"EndTurn": {"player": "Carol"}}
29755	1e7850a0	200	2026-10-30 13:12:29.174023+00	{"StrategicActionBegin": {"card": "Technology", "player": "Bob"}}
29756	1e7850a0	201	2026-10-30 13:13:29.174023+00	{"StrategicActionPrimary": {"action": {"Technology": {"first_tech": null, "second_tech": null}}, "player": "Bob"}}
29757	1e7850a0	202	2026-10-30 13:14:29.174023+00	"StrategicActionCommit"
29758	1e7850a0	203	2026-10-30 13:15:29.174023+00	{"EndTurn": {"player": "Bob"}}
29759	1e7850a0	204	2026-10-30 13:16:29.174023+00	{"StrategicActionBegin": {"card": "Imperial", "player": "Finn"}}
29760	1e7850a0	205	2026-10-30 13:17:29.174023+00	{"StrategicActionPrimary": {"action": {"Imperial": {"scoreObjective": "ErectAMonument"}}, "player": "Finn"}}
29761	1e7850a0	206	2026-10-30 13:18:29.174023+00	"StrategicActionCommit"
29762	1e7850a0	207	2026-10-30 13:19:29.174023+00	{"EndTurn": {"player": "Finn"}}
29763	1e7850a0	208	2026-10-30 13:20:29.174023+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Alice"}}
29764	1e7850a0	209	2026-10-30 13:21:29.174023+00	"StrategicActionCommit"
29765	1e7850a0	210	2026-10-30 13:22:29.174023+00	{"EndTurn": {"player": "Alice"}}
29766	1e7850a0	211	2026-10-30 13:23:29.174023+00	{"PassAction": {"player": "Erin"}}
29767	1e7850a0	212	2026-10-30 13:24:29.174023+00	{"PassAction": {"player": "Dave"}}
29768	1e7850a0	213	2026-10-30 13:25:29.174023+00	{"PassAction": {"player": "Carol"}}
29769	1e7850a0	214	2026-10-30 13:26:29.174023+00	{"PassAction": {"player": "Bob"}}
29770	1e7850a0	215	2026-10-30 13:27:29.174023+00	{"PassAction": {"player": "Finn"}}
29771	1e7850a0	216	2026-10-30 13:28:29.174023+00	{"PassAction": {"player": "Alice"}}
31125	942cbdd8	3	2026-11-05 23:11:02.112041+00	{"SetupTheTribunii": {"player": "Tux", "faction": "MentakCoalition"}}
31126	942cbdd8	4	2026-11-05 23:11:02.11845+00	{"SetupPlayerTechs": {"player": "Vidde", "technologies": ["Psychoarchaeology"]}}
31127	942cbdd8	5	2026-11-05 23:11:02.125948+00	{"SetupPlayerTechs": {"player": "Yoda", "technologies": ["AntimassDeflectors"]}}
31128	942cbdd8	6	2026-11-05 23:11:02.133233+00	{"SetupPlayerTechs": {"player": "Tux", "technologies": ["AntimassDeflectors", "Psychoarchaeology"]}}
31129	942cbdd8	7	2026-11-05 23:11:02.14034+00	{"RevealInitialObjectives": {"firstObjective": "BuildDefenses", "secondObjective": "CornerTheMarket"}}
31130	942cbdd8	8	2026-11-05 23:11:02.146888+00	"StartGame"
31131	942cbdd8	9	2026-11-05 23:11:02.153848+00	{"RollForTheFracture": {"player": "Yoda", "entered_play": false}}
31132	942cbdd8	10	2026-11-05 23:11:02.161137+00	{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}
31133	942cbdd8	11	2026-11-05 23:11:02.168184+00	{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}
31134	942cbdd8	12	2026-11-05 23:11:02.174976+00	{"TakeStrategyCard": {"card": "Politics", "player": "Yoda"}}
31135	942cbdd8	13	2026-11-05 23:11:02.18143+00	{"TakeStrategyCard": {"card": "Construction", "player": "Tux"}}
31136	942cbdd8	14	2026-11-05 23:11:02.188378+00	{"TakeStrategyCard": {"card": "Trade", "player": "Vidde"}}
31137	942cbdd8	15	2026-11-05 23:11:02.195622+00	{"TakeStrategyCard": {"card": "Warfare", "player": "Yoda"}}
31138	942cbdd8	16	2026-11-05 23:11:02.203424+00	"CompleteStrategyPhase"
31139	942cbdd8	17	2026-11-05 23:11:02.210937+00	{"SetCustodians": {"player": "Tux"}}
31140	942cbdd8	18	2026-11-05 23:11:02.218066+00	{"PlayGalacticEventAction": {"action": "StellarAtomics", "player": "Tux"}}
31141	942cbdd8	19	2026-11-05 23:11:02.224308+00	{"EndTurn": {"player": "Tux"}}
31142	942cbdd8	20	2026-11-05 23:11:02.231836+00	{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}
31143	942cbdd8	21	2026-11-05 23:11:02.238067+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31144	942cbdd8	22	2026-11-05 23:11:02.244465+00	{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Yoda"}}
31145	942cbdd8	23	2026-11-05 23:11:02.251019+00	"StrategicActionCommit"
31146	942cbdd8	24	2026-11-05 23:11:02.258588+00	{"EndTurn": {"player": "Vidde"}}
31147	942cbdd8	25	2026-11-05 23:11:02.265222+00	{"StrategicActionBegin": {"card": "Politics", "player": "Yoda"}}
31148	942cbdd8	26	2026-11-05 23:11:02.271967+00	{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Yoda"}}, "player": "Yoda"}}
31149	942cbdd8	27	2026-11-05 23:11:02.278227+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}
31150	942cbdd8	28	2026-11-05 23:11:02.285678+00	{"StrategicActionSecondary": {"action": "Politics", "player": "Vidde"}}
31151	942cbdd8	29	2026-11-05 23:11:02.292017+00	"StrategicActionCommit"
31152	942cbdd8	30	2026-11-05 23:11:02.297766+00	{"EndTurn": {"player": "Yoda"}}
31153	942cbdd8	31	2026-11-05 23:11:02.303198+00	{"StrategicActionBegin": {"card": "Leadership", "player": "Tux"}}
31154	942cbdd8	32	2026-11-05 23:11:02.308721+00	{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}
31155	942cbdd8	33	2026-11-05 23:11:02.315408+00	{"StrategicActionSecondary": {"action": "Leadership", "player": "Yoda"}}
\.


--
-- Name: game_event_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.game_event_id_seq', 31512, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: game_event game_event_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_event
    ADD CONSTRAINT game_event_pkey PRIMARY KEY (id);


--
-- Name: game_event game_event_unique_seq; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_event
    ADD CONSTRAINT game_event_unique_seq UNIQUE (seq, game_id);


--
-- Name: game game_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game
    ADD CONSTRAINT game_pkey PRIMARY KEY (id);


--
-- Name: game_event_seq_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX game_event_seq_index ON public.game_event USING btree (game_id, seq);


--
-- Name: game_event set_seq; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_seq BEFORE INSERT ON public.game_event FOR EACH ROW EXECUTE FUNCTION public.next_game_event_seq();


--
-- Name: game_event game_event_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_event
    ADD CONSTRAINT game_event_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict qMi5VtZWDZ3cwM8mCPVjeijuoQLbiccgfcBY8hwRUO1eCVokVUHfV0UiFmmfdZS

