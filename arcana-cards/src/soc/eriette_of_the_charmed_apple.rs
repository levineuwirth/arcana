//! Eriette of the Charmed Apple — `{1}{W}{B}` 2/4 Legendary Creature —
//! Human Warlock.
//!
//! Oracle:
//! * "Each creature that's enchanted by an Aura you control can't attack you
//!   or planeswalkers you control." — GAP: a static combat-restriction
//!   continuous effect keyed off Aura attachment; no trigger word, no cost.
//! * "At the beginning of your end step, each opponent loses X life and you
//!   gain X life, where X is the number of Auras you control." — end-step
//!   trigger; X computed at resolution from the count of Aura enchantments you
//!   control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eriette of the Charmed Apple");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Each creature enchanted by an Aura you control can't attack
    // you or planeswalkers you control." — continuous combat restriction; not
    // expressible in this card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: end_step_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_drain(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of Auras you control.
    let aura_filter = script::subtype_filter(reg, "Aura").controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &aura_filter, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: x,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: x,
    });
    vec![Effect::Sequence(effects)]
}
