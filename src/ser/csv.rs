use crate::data::encounter_type::EncounterType;
use crate::data::quest::QuestState;
use crate::data::vis::Vis;
use crate::game::{GameLanguage, LocationId, QuestId};
use crate::global::data::Data;
use csv::StringRecord;
use std::collections::HashMap;

const HEADER: [&str; 7] = [
    "type",
    "location",
    "quest",
    "status",
    "prerequisite",
    "visibility",
    "note",
];

impl Data {
    pub(crate) fn save_csv(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(8 * 1024);
        let mut writer = csv::Writer::from_writer(&mut result);

        let _ = writer.write_record(HEADER);

        let _ = writer.write_record([
            "language",
            "",
            "",
            self.quest_locale.language().as_str(),
            "",
            "",
            "This file is in UTF-8 😀",
        ]);
        for (_, quest, quest_name) in self.quest_iter() {
            if quest.state != QuestState::NotFound
                || !quest.note.is_empty()
                || quest.vis != Vis::Visible
            {
                let _ = writer.write_record([
                    "quest",
                    "",
                    quest_name,
                    quest.state.to_csv(),
                    "",
                    quest.vis.to_csv(),
                    &quest.note,
                ]);
            }
        }
        for location_id in LocationId::all() {
            if location_id != LocationId::prologue() {
                if let Some(note) = self.location.get(&location_id) {
                    if !note.is_empty() {
                        let _ = writer.write_record([
                            "location",
                            location_id.name(self.quest_locale.language()), // language does not anyway matter as this is never prologue
                            "",
                            "",
                            "",
                            "",
                            note,
                        ]);
                    }
                }
                for (_, quest, quest_name) in self.quest_iter() {
                    if let Some(quest_location) = quest.encounter.get(&location_id) {
                        for (encounter_type, encounter) in quest_location {
                            let _ = writer.write_record([
                                "encounter",
                                location_id.name(self.quest_locale.language()), // language does not matter anyway as this is never prologue
                                quest_name,
                                encounter_type.to_csv(),
                                encounter
                                    .prerequisite
                                    .map_or("", |prerequisite| self.quest_locale.get(prerequisite)),
                                encounter.vis.to_csv(),
                                "",
                            ]);
                        }
                    }
                }
            }
        }

        drop(writer);
        result
    }

    pub(crate) fn load_csv(&mut self, mut file: &[u8]) -> Result<String, MyError> {
        // remove UTF-8 BOM (this *should* never be used, but Microsoft products often do)
        if let Some(f) = file.strip_prefix(&[0xef, 0xbb, 0xbf]) {
            file = f;
        }
        let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

        // check header
        let headers = reader.headers()?;
        for (pos, field) in HEADER.iter().enumerate() {
            if headers.get(pos).unwrap_or_default() != *field {
                return Err(MyError::Header);
            }
        }

        let mut line = StringRecord::new();

        // check language
        reader.read_record(&mut line)?;
        if line.get(0).unwrap_or_default() != "language" {
            return Err(MyError::Language);
        }
        let requested_language = line.get(3).unwrap_or_default();
        let game_language = GameLanguage::iter()
            .zip(GameLanguage::names())
            .find(|(_, name)| *name == requested_language)
            .ok_or(MyError::Language)?
            .0;

        // reset everything and use the correct language
        self.reset();
        self.quest_locale.set_language(game_language);

        let mut errors = String::new();

        let mut locations = HashMap::with_capacity(LocationId::all().count());
        for location_id in LocationId::all() {
            if location_id != LocationId::prologue() {
                locations.insert(
                    location_id.name(self.quest_locale.language()), // language does not matter anyway as this is never prologue
                    location_id,
                );
            }
        }

        let quests = self.quest_locale.all_str().map(|(a, b)| (b, a)).collect();

        while reader.read_record(&mut line)? {
            if self.load_csv3(&line, &locations, &quests).is_none() {
                if let Some(position) = line.position() {
                    if !errors.is_empty() {
                        errors.push(',');
                        errors.push(' ');
                    }
                    errors.push_str(&(position.record() + 1).to_string());
                }
            }
        }

        self.cleanup();

        Ok(errors)
    }

    fn load_csv3(
        &mut self,
        line: &StringRecord,
        locations: &HashMap<&str, LocationId>,
        quests: &HashMap<&str, QuestId>,
    ) -> Option<()> {
        let type_ = [
            (Type::Quest, "quest"),
            (Type::Location, "location"),
            (Type::Encounter, "encounter"),
        ]
        .iter()
        .find(|(_, n)| *n == line.get(0).unwrap_or_default())?
        .0;

        let location_id = read(line, 1, locations)?;
        let quest_id = read(line, 2, quests)?;
        let prerequisite = read(line, 4, quests)?;
        let vis = Vis::try_from_csv(line.get(5).unwrap_or_default())?;
        let note = line.get(6).unwrap_or_default();
        match type_ {
            Type::Quest => {
                let quest_id = quest_id?;
                let state = QuestState::try_from_csv(line.get(3).unwrap_or_default())?;
                let quest = self.quest.entry(quest_id).or_default();
                quest.state = state;
                quest.vis = vis;
                quest.note = note.to_string().into();
            }
            Type::Location => {
                let location_id = location_id?;
                let location = self.location.entry(location_id).or_default();
                *location = note.to_string().into();
            }
            Type::Encounter => {
                let location_id = location_id?;
                let quest_id = quest_id?;
                let encounter_type = EncounterType::try_from_csv(line.get(3).unwrap_or_default())?;
                let quest = self.quest.entry(quest_id).or_default();
                let quest_location = quest.encounter.entry(location_id).or_default();
                quest_location.insert(encounter_type, prerequisite, vis);
            }
        }

        Some(())
    }
}

pub(crate) enum MyError {
    CsvError,
    Header,
    Language,
}

impl From<csv::Error> for MyError {
    fn from(_: csv::Error) -> Self {
        MyError::CsvError
    }
}

#[derive(Copy, Clone)]
enum Type {
    Quest,
    Location,
    Encounter,
}

#[allow(clippy::option_option)]
fn read<T>(line: &StringRecord, column: usize, map: &HashMap<&str, T>) -> Option<Option<T>>
where
    T: Copy,
{
    let input = line.get(column).unwrap_or_default();
    if input.is_empty() {
        return Some(None);
    }
    map.get(input).copied().map(Some)
}
