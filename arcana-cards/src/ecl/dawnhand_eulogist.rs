//! Dawnhand Eulogist — `{3}{B}` 3/3 Creature — Elf Warlock.
//! Menace.
//! When this creature enters, mill three cards. Then if there is an Elf
//! card in your graveyard, each opponent loses 2 life and you gain 2
//! life.
//!
//! Menace is a base keyword. The ETB mills three, then — checked at
//! resolution (after the mill) — if an Elf card is in your graveyard,
//! each opponent loses 2 and you gain 2.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dawnhand Eulogist");
    let elf = reg.interner_mut().intern("Elf");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_mill_then_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_mill_then_drain(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let you = trig.controller;
    let mut effects = vec![Effect::Mill {
        player: you,
        count: 3,
    }];

    // "Then if there is an Elf card in your graveyard …". FIDELITY GAP:
    // the check reads `state` as the trigger resolves, i.e. BEFORE the
    // milled cards land (documented pre-mill off-by-one) — an Elf milled
    // by this same ETB won't be seen, but pre-existing Elves are.
    let elf_filter = script::subtype_filter(reg, "Elf");
    if script::graveyard_matching(state, &elf_filter, you, you) > 0 {
        for opp in script::opponents(state, you) {
            effects.push(Effect::LoseLife {
                player: opp,
                amount: 2,
            });
        }
        effects.push(Effect::GainLife {
            player: you,
            amount: 2,
        });
    }

    effects
}
