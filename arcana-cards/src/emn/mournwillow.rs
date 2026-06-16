//! Mournwillow — `{1}{B}{G}` 3/2 Creature — Plant Skeleton with Haste.
//!
//! * Haste (keyword). (Delirium is the intervening-if condition word, not a
//!   KeywordAbility — no keyword emitted for it.)
//! * Delirium — When this creature enters, if there are four or more card types
//!   among cards in your graveyard, creatures with power 2 or less can't block
//!   this turn.
//!   GAP: the Delirium intervening-if (>= 4 card types in graveyard) has no
//!   predicate in the available conditions set, so the gate is omitted (the
//!   trigger fires unconditionally — a known over-fire). The effect (board-wide
//!   ForbidBlocking on creatures with power <= 2) is faithful.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mournwillow");
    let plant = reg.interner_mut().intern("Plant");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Delirium intervening-if (>= 4 card types in your graveyard)
            // has no available predicate; trigger fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: small_creatures_cant_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn small_creatures_cant_block(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_max_power(2),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ForbidBlocking {
            target: NULL_OBJECT_ID,
            duration: Duration::EndOfTurn,
        }),
    }]
}
