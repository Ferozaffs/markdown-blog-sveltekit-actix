-- Adminer 4.8.1 PostgreSQL 15.3 (Debian 15.3-1.pgdg120+1) dump

DROP TABLE IF EXISTS "keys";
CREATE TABLE "public"."keys" (
    "key" text NOT NULL
);

DROP TABLE IF EXISTS "posts";
CREATE TABLE "public"."posts" (
    "id" uuid NOT NULL,
    "name" text NOT NULL,
    "image" text NOT NULL,
    "project_id" uuid,
    "tags" text,
    "content" text NOT NULL,
    "date" date,
    "description" text
) WITH (oids = false);

INSERT INTO "posts" ("id", "name", "image", "project_id", "tags", "content", "date", "description") VALUES
('4b3e0de2-04ad-4bfb-b3d0-fd9881a9f008',	'New Galactic Neighbor Discovered',	'testImage3.jpg',	'4c38bf38-59f1-4603-9438-e2766663cf2b',	'Space,Science',	'# New Galactic Neighbor Discovered: Introducing Quasarium Galaxy

*By Astrid Stellaris | August 9, 2023*

![Quasarium Galaxy](https://fakeimageurl.com/quasarium-galaxy.jpg)

In an astonishing astronomical revelation, astronomers have unveiled the existence of a previously unknown galaxy located just beyond our own Milky Way. Named the "Quasarium Galaxy," this newfound cosmic neighbor is shrouded in intrigue and has ignited a fervor of excitement within the scientific community.

## An Unexpected Discovery

The Quasarium Galaxy was serendipitously detected by the Kepler Advanced Space Telescope during its routine observations of distant celestial bodies. The galaxy''s unique characteristics and dazzling quasar emissions set it apart from neighboring galaxies and piqued the curiosity of researchers.

## Quasar Symphony: A Celestial Concert

One of the most remarkable aspects of the Quasarium Galaxy is its quasar symphony – an awe-inspiring collection of harmonious emissions resembling musical notes. Scientists believe these emissions are caused by interactions between the galaxy''s supermassive black hole and surrounding cosmic phenomena.

![Quasar Symphony](https://fakeimageurl.com/quasar-symphony.jpg)

Dr. Celestia Starlight, lead astronomer of the discovery team, described the quasar symphony as "an ethereal and cosmic orchestra, playing a melody that resonates throughout the cosmos."

## Unraveling the Quasarium Mysteries

The Quasarium Galaxy''s intricate dance of quasars and celestial bodies has raised numerous questions and sparked intensive research efforts:

- **Origin of the Celestial Melody:** Astronomers are diligently studying the underlying mechanisms that give rise to the quasar symphony, aiming to unlock the secrets of these mesmerizing emissions.

- **Cosmic Impact on the Milky Way:** The proximity of the Quasarium Galaxy raises concerns about potential gravitational influences on our own Milky Way and its delicate spiral structure.

## The Quasarium Observatory Initiative

To further investigate the Quasarium Galaxy''s enigmas, international space agencies are collaborating on the construction of the Quasarium Observatory. This state-of-the-art observatory will house a fleet of telescopes equipped with advanced sensors capable of capturing the intricate details of the quasar symphony.

## A New Era of Cosmic Exploration

The discovery of the Quasarium Galaxy marks a turning point in our understanding of the universe''s vastness and complexity. As astronomers set their sights on this cosmic marvel, the possibilities for new discoveries and revelations seem boundless.

*Disclaimer: This article is a work of fiction and meant for entertainment purposes only.*

---

Please note that the above article is entirely fictional and created for entertainment purposes. The Quasarium Galaxy, its characteristics, and the events described in the article are purely imaginative.',	'2023-01-01',	'This is a test description'),
('af1d4580-29bf-4810-9eff-9f4911c40e75',	'Quest for the Elixir of Eldoria',	'testImage4.jpg',	'1e33f43d-0193-460a-9128-bffb1d12e57c',	'Fantasy,Science',	'# Quest for the Elixir of Eldoria: Unveiling a Mythical Journey

*By Sir Reginald Loreweaver | August 9, 2023*

![Elixir of Eldoria](https://fakeimageurl.com/elixir-eldoria.jpg)

In a tale that seems plucked from the annals of classic fantasy, a daring group of adventurers has embarked on a quest to uncover the fabled Elixir of Eldoria – a legendary potion said to bestow eternal youth and wisdom upon its imbiber. This epic journey through enchanted forests, treacherous mountains, and ancient ruins captivates the imagination and harks back to times of heroic deeds and magical wonder.

## A Prophetic Scroll Unveiled

The adventurers'' quest was set into motion when a cryptic scroll, rumored to be penned by an ancient seer, was discovered hidden within the library of the Whispering Citadel. The scroll foretells the Elixir''s existence and hints at the challenging path required to obtain it.

## The Fellowship of Heroes

The fellowship comprises a diverse cast of characters, each possessing unique talents:

- **Lorelei the Enchantress:** A master of arcane arts, she uses her magic to decipher hidden enchantments and navigate magical traps.

- **Garrick the Valiant:** A skilled warrior with a heart of gold, he wields his sword and shield to protect his companions from peril.

- **Thalia the Sage:** A sage well-versed in ancient lore and riddles, she deciphers clues and guides the fellowship with her wisdom.

## Trials and Tribulations

The adventurers'' journey unfolds through a series of trials that test not only their physical prowess but also their resolve and camaraderie:

- **The Whispering Woods:** An enchanted forest teeming with mystical creatures and perplexing illusions challenges the fellowship''s wit and unity.

- **The Guardian''s Gauntlet:** To prove their worth, the adventurers must traverse a treacherous mountain pass guarded by an ancient stone giant.

- **Ruins of Arcanum:** Amidst the ancient ruins, they face puzzles designed to baffle even the most astute minds.

## Unveiling the Elixir''s Power

The climax of the quest culminates in a hidden chamber beneath the Lost Citadel, where the Elixir of Eldoria is said to be guarded by an ageless guardian. When the fellowship finally unveils the Elixir, they are faced with a moral dilemma: to keep the power for themselves or share it with the world.

## A Tale for the Ages

As the adventurers'' journey unfolds, their trials and triumphs resonate with the timeless themes of courage, friendship, and the allure of the unknown. The Quest for the Elixir of Eldoria invites us all to embrace the spirit of adventure and reminds us that even in the realm of fantasy, the human spirit''s quest for knowledge and greatness knows no bounds.

*Disclaimer: This article is a work of fiction and is inspired by classic fantasy themes.*

---

Please note that the above article is entirely fictional and created for creative purposes. The Elixir of Eldoria and the events described in the article are products of imagination and do not reflect real occurrences.',	'2023-03-03',	'This is another test description');

DROP TABLE IF EXISTS "project_categories";
CREATE TABLE "public"."project_categories" (
    "id" uuid NOT NULL,
    "category" text NOT NULL,
    "description" text
) WITH (oids = false);

INSERT INTO "project_categories" ("id", "category", "description") VALUES
('71f99ce7-64c6-4344-927a-8fa15acd357e',	'Large',	'Project spanning a long time'),
('c9284c8d-02a4-4b25-9818-20c8a95178c6',	'Small',	'Projects spanning a short time'),
('6ce5b42a-0c54-4aec-b2a5-0b8b4e538d4b',	'Misc',	'Random projects');

DROP TABLE IF EXISTS "projects";
CREATE TABLE "public"."projects" (
    "id" uuid NOT NULL,
    "name" text NOT NULL,
    "image" text NOT NULL,
    "status" integer NOT NULL,
    "category_id" uuid,
    "content" text
) WITH (oids = false);

INSERT INTO "projects" ("id", "name", "image", "status", "category_id", "content") VALUES
('1e33f43d-0193-460a-9128-bffb1d12e57c',	'Groundbreaking Discovery',	'testImage2.jpg',	1,	'c9284c8d-02a4-4b25-9818-20c8a95178c6',	'# Groundbreaking Discovery: Unicorns Spotted in the Amazon Rainforest!

*By Jane Doe | August 9, 2023*

![Unicorn in Amazon Rainforest](http://127.0.0.1:8080/images/testImage2.jpg)

In a stunning turn of events, researchers claim to have discovered a mystical population of unicorns residing deep within the heart of the Amazon rainforest. The revelation has sent shockwaves throughout the scientific community, sparking both excitement and skepticism.

## The Quest for the Enchanted Creatures

Led by renowned cryptozoologist Dr. A. Fantasia, a team of intrepid explorers embarked on an extraordinary journey into the uncharted regions of the Amazon. Armed with cutting-edge technology and fueled by their unwavering belief in the fantastical, the team set out to validate centuries-old rumors of the unicorn''s existence.

## Unveiling the Magical Encounter

After weeks of trekking through dense foliage and navigating treacherous terrains, the team stumbled upon a hidden glade bathed in a surreal iridescent light. In the heart of this ethereal oasis stood a herd of radiant unicorns, their resplendent horns glistening in the sunlight.

![Unicorn Herd](http://127.0.0.1:8080/images/testImage.jpg)

Dr. Fantasia described the moment as "transcendent" and "a confirmation of the mystical realms that have long been relegated to the realm of fairy tales and legends."

## Captivating the World''s Imagination

News of the discovery has ignited the imagination of people worldwide, with social media buzzing over the incredible revelation. Celebrities and influencers alike have flocked to Twitter and Instagram to share their thoughts, speculations, and unicorn-themed memes.

## Scientific Skepticism Persists

However, not everyone is convinced by the unicorn sightings. Prominent skeptics, including Dr. Rationalis from the Scientific Skeptics Society, have raised concerns about the lack of concrete evidence and the potential for elaborate hoaxes. "Extraordinary claims require extraordinary evidence," Dr. Rationalis stated in a recent press release.

## Conservation Efforts and Ethical Dilemmas

As discussions about the unicorns'' existence continue, conservationists and ethicists are grappling with the potential impact of this discovery on the delicate ecosystem of the Amazon rainforest. Some experts worry that increased attention could lead to habitat destruction, endangering not only the newly-found unicorns but also countless other species that call the rainforest home.

## The Future of Cryptozoology

Regardless of the ongoing debates, the discovery has reignited interest in the field of cryptozoology, which studies hidden and undiscovered animals. Researchers around the world are now inspired to explore other legendary creatures in hopes of unraveling more mysteries of the natural world.

In the end, whether this revelation is a true breakthrough or an elaborate hoax, the unicorn discovery will undoubtedly go down in history as a testament to the enduring power of imagination and the allure of the unknown.

*Disclaimer: This article is a work of fiction and meant for entertainment purposes only.*

---

Please note that the above article is entirely fictional and created for entertainment purposes. Unicorns do not exist in reality, and the events described in the article are purely imaginative.

'),
('4c38bf38-59f1-4603-9438-e2766663cf2b',	'Unveiling the Quantum Realm',	'testImage.jpg',	0,	'71f99ce7-64c6-4344-927a-8fa15acd357e',	'# Unveiling the Quantum Realm: A Journey into the Microscopic Cosmos

*By John Smith | August 9, 2023*

In a groundbreaking leap for science, a team of physicists claims to have successfully penetrated the enigmatic realm of quantum dimensions, offering a glimpse into the mysterious world that governs the behavior of particles. This journey into the quantum cosmos has unveiled astonishing insights that challenge our understanding of reality itself.

## Key Discoveries

- **Wave-Particle Duality Confirmed:** The experiment reinforced the long-standing theory of wave-particle duality, demonstrating that particles such as electrons exhibit both wave-like and particle-like behavior simultaneously.

- **Entanglement at Macroscopic Scales:** Researchers observed entanglement, a phenomenon where two particles become correlated to the extent that the state of one particle instantly influences the state of the other, even across significant distances.

- **Multiverse Hypothesis Explored:** Preliminary data hints at potential connections to the controversial multiverse hypothesis, suggesting that particles exist in multiple states across parallel universes.

## The Quantum Journey

The experimental setup involved an intricate arrangement of advanced particle accelerators and quantum sensors. Here''s a simplified overview of the process:

| Stage | Description |
|-------|-------------|
| 1     | Particle acceleration to near-light speeds. |
| 2     | Particles directed through a series of precisely calibrated electromagnetic fields. |
| 3     | Quantum sensors measure particle properties in real-time. |
| 4     | Data transmitted to a supercomputer for analysis. |

## Implications and Applications

The implications of this groundbreaking achievement are vast and could revolutionize various fields:

- **Quantum Computing Advancements:** Insights gained from this experiment could lead to the development of more powerful quantum computers, potentially solving complex problems that are currently unsolvable.

- **Teleportation Technology:** The study''s findings could pave the way for advanced teleportation technologies, although this prospect remains in the realm of science fiction for now.

- **Fundamental Physics Understanding:** This journey into the quantum realm deepens our understanding of fundamental physics laws, challenging conventional interpretations of reality.

## Ethical and Philosophical Considerations

The foray into the quantum realm also raises ethical and philosophical questions:

- **Privacy and Security:** The potential for quantum-level surveillance and encryption prompts discussions on privacy and data security.

- **Reality and Perception:** The blurring of lines between macroscopic and quantum realms fuels debates about the nature of reality and our limited human perceptions.

## A New Frontier Beckons

As the team continues to analyze the vast amount of data collected, the scientific community braces for the paradigm shifts that may result from this momentous discovery. With the quantum realm now within our reach, the boundaries of human knowledge are expanding, inviting us to venture further into the uncharted territories of existence.

*Disclaimer: This article is a creative work and is not based on real scientific findings.*

---

Please note that the above article is entirely fictional and created for creative purposes. The concepts and discoveries mentioned in the article are products of imagination and do not reflect real scientific advancements.');

-- 2023-08-14 22:11:37.083273+00