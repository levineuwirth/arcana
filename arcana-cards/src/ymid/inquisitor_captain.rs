//! Inquisitor Captain — `{3}{W}` 3/3 Human Cleric.
//!
//! * Vigilance.
//! * When this creature enters, if you cast it and there are twenty or more
//!   creature cards with mana value 3 or less among cards in your graveyard,
//!   hand, and library, seek two creature cards with mana value 3 or less.
//!   Put one onto the battlefield and shuffle the other into your library.
//!
//! Vigilance is wired as a base keyword. The ETB ability is GAP'd: there is no
//! Seek effect, and the intervening-if ("you cast it" + a 20+ count spanning
//! graveyard/hand/library filtered by mana value) is not expressible with the
//! available conditions/script helpers.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inquisitor Captain");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_seek,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_seek(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no Seek effect; cast-it gate + 20+ creature-cards-mv<=3 count across
    // graveyard/hand/library is not expressible with available helpers.
    Vec::new()
}
