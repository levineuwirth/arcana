//! Ill-Tempered Cyclops — `{3}{R}` 3/3 Creature — Cyclops. Red.
//! Trample.
//! "{5}{R}: Monstrosity 3. (If this creature isn't monstrous, put three
//!  +1/+1 counters on it and it becomes monstrous.)"
//!
//! ("Monstrosity" is not in the demonstrated keyword surface, so it isn't
//! listed in `keywords`.) The activated ability places three +1/+1
//! counters on Ill-Tempered Cyclops. GAP: the "becomes monstrous" status
//! latch and the "if it isn't monstrous" precondition are not
//! expressible (no Effect sets the monstrous status; no ActivationCost
//! field gates on it), so this can be activated more than once — a
//! documented fidelity deviation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ill-Tempered Cyclops");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{R}: Monstrosity 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: monstrosity_three,
            }),
    )
}

fn monstrosity_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes monstrous" latch + "if it isn't monstrous" gate not
    // expressible — only the +1/+1 counter placement is emitted.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}
