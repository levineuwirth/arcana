//! Lulu, Curious Hollyphant — `{2}{W}{U}` 2/4 Legendary Creature — Elephant Angel.
//! Flying.
//! Whenever you attack with one or more other creatures with flying, draw that many
//! cards, then discard a card. (No "whenever you attack with N creatures" TriggerCondition,
//! and "that many" = count of other attacking flyers has no script helper for attacking
//! creatures — GAP. Closest available trigger is SelfAttacks; effect body GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lulu, Curious Hollyphant");
    let elephant = reg.interner_mut().intern("Elephant");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — no "whenever you attack with one or more other flyers" variant;
            // SelfAttacks is the closest available.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_draw_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_draw_discard(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "draw that many cards" where "that many" = number of OTHER attacking creatures
    // with flying — no script helper counts attacking creatures, so the dynamic draw
    // count (and dependent discard) is unexpressible.
    Vec::new()
}
