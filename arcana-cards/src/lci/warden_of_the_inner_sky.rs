//! Warden of the Inner Sky — `{W}` 1/2 Human Soldier.
//! "As long as this creature has three or more counters on it, it has
//! flying and vigilance." (GAP — counter-gated static keyword grant.)
//! "Tap three untapped artifacts and/or creatures you control: Put a
//! +1/+1 counter on this creature. Scry 1. Activate only as a sorcery."
//!
//! The Scryfall "Scry" keyword entry is reminder-only (not a modeled
//! KeywordAbility), so the keyword line is empty. The counter-threshold
//! static (flying + vigilance once 3+ counters) has no expressible
//! primitive and is GAP'd. The activated ability is wired: the cost taps
//! three other untapped artifact-or-creature permanents you control; the
//! effect adds a +1/+1 counter to this creature and scries 1.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warden of the Inner Sky");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "As long as this creature has three or more counters on
    // it, it has flying and vigilance" — counter-gated keyword grant, no
    // expressible primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Tap three untapped artifacts and/or creatures you control: Put a +1/+1 counter on this creature. Scry 1. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                tap_other: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                        .controlled_by(ControllerConstraint::You),
                ),
                tap_other_count: 3,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grow_and_scry,
        }),
    )
}

fn grow_and_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Scry {
            player: ctx.controller,
            count: 1,
        },
    ])]
}
