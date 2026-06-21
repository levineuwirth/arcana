//! Elvish Dreadlord — `{3}{B}{B}` 3/3 Zombie Elf with Deathtouch.
//!
//! "When this creature dies, non-Elf creatures get -3/-3 until end of
//! turn.
//! Encore {5}{B}{B}."
//!
//! Deathtouch is a base keyword. The dies trigger gives every non-Elf
//! creature -3/-3 until end of turn (ForEach over the matching ids).
//! Encore is not a supported keyword and its graveyard-activated
//! token-copy-per-opponent body has no primitive — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Dreadlord");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: Encore {5}{B}{B} — Encore is not a supported keyword and its
    // graveyard-activated "token copy per opponent that attacks" body has
    // no primitive in this API surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_minus_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_minus_three(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf");
    let mut filter = arcana_core::targets::ObjectFilter::creature();
    if let Some(elf) = elf {
        filter = filter.without_subtype_sym(elf);
    }
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -3,
            toughness: -3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
