//! Abzan Battle Priest — `{3}{W}` 3/2 white Human Cleric.
//! Outlast {W} ({W}, {T}: Put a +1/+1 counter on this creature. Sorcery speed.)
//! Each creature you control with a +1/+1 counter on it has lifelink. (static — GAP)
//!
//! Outlast is not in the usable keyword surface, but its synthesized activated
//! ability ("{W}, {T}: put a +1/+1 counter on ~; sorcery speed") IS expressible
//! as a plain ActivatedAbilityDef, so it is wired directly.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abzan Battle Priest");
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
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Each creature you control with a +1/+1 counter on it has lifelink."
    // — static continuous keyword-granting ability; not expressible here.
    reg.register(
        CardDefinition::new(name, chars)
            // Outlast {W}: {W}, {T}: Put a +1/+1 counter on this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}: Put a +1/+1 counter on this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: outlast_counter,
            }),
    )
}

fn outlast_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
