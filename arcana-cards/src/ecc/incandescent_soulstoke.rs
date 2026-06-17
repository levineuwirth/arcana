//! Incandescent Soulstoke — `{2}{R}` 2/2 Creature — Elemental Shaman.
//!
//! * Other Elemental creatures you control get +1/+1.
//! * {1}{R}, {T}: You may put an Elemental creature card from your hand onto the
//!   battlefield. That creature gains haste until end of turn. Sacrifice it at
//!   the beginning of the next end step.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incandescent Soulstoke");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: static anthem "Other Elemental creatures you control get +1/+1" —
        // no continuous static-buff primitive in this card shape.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{R}, {T}: You may put an Elemental creature card from your hand \
                   onto the battlefield. That creature gains haste until end of turn. \
                   Sacrifice it at the beginning of the next end step."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_elemental,
        }),
    )
}

fn put_elemental(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // Put an Elemental creature card from hand onto the battlefield.
    // GAP: the "gains haste until end of turn" and "sacrifice at the next end
    // step" riders attach to the put creature, whose id is unknown after the
    // engine-driven hand pick — no continuation primitive exposes it here.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Elemental"),
        tapped: false,
    }]
}
