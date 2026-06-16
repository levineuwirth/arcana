//! Sandstalker Moloch — `{1}{G}{G}` 4/2 Lizard with Flash.
//! "When this creature enters, if an opponent cast a blue and/or black
//! spell this turn, look at the top four cards of your library. You may
//! reveal a permanent card from among them and put it into your hand.
//! Put the rest on the bottom in a random order."

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandstalker Moloch");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if an opponent cast a blue and/or black
            // spell this turn" — no caster-color spell-history condition helper.
            intervening_if: None,
            effect: etb_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_dig(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // A "permanent card" = artifact / creature / enchantment / land / planeswalker.
    let permanent_filter = ObjectFilter::new().with_types_any(TypeLine(
        TypeLine::ARTIFACT
            | TypeLine::CREATURE
            | TypeLine::ENCHANTMENT
            | TypeLine::LAND
            | TypeLine::PLANESWALKER,
    ));
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(permanent_filter),
        rest: DigRest::BottomRandom,
    }]
}
