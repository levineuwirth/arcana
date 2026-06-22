//! Farmstead Gleaner — `{3}` 2/2 Artifact Creature — Scarecrow.
//! This creature doesn't untap during your untap step. (static — GAP)
//! {2}, {Q}: Put a +1/+1 counter on this creature. ({Q} is the untap symbol.)
//!
//! The "doesn't untap during your untap step" static is GAP'd (no untap-step
//! restriction primitive). The activated ability puts a +1/+1 counter on the
//! source; its {2} mana cost is modeled, but the {Q} (untap-this-permanent)
//! cost is GAP'd — ActivationCost has no untap-self cost field — so the
//! activation is a documented partial (mana + effect, missing the {Q} gate).

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
    let name = reg.interner_mut().intern("Farmstead Gleaner");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "doesn't untap during your untap step" — no untap-restriction
    // primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {Q}: Put a +1/+1 counter on this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                // GAP: {Q} (untap this permanent) cost — no untap-self cost field.
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter,
        }),
    )
}

fn add_counter(
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
