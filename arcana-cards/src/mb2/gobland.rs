//! Gobland — Land Creature — Mountain Goblin, 2/1 (red).
//! (Gobland isn't a spell, it's affected by summoning sickness, and it has
//!  "{T}: Add {R}.")
//! Gobland can't block.
//!
//! Decomposition:
//! - No mana cost (it's a land — cast as a land drop, not a spell; the engine
//!   handles land-creature summoning sickness via the type line).
//! - "{T}: Add {R}." (printed in the reminder line) → an explicit tap mana
//!   ability, mirroring basic Mountain.
//! - "Gobland can't block." — GAP: a pure continuous self-static with no
//!   trigger or cost; no static-can't-block primitive in this card class.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gobland");
    let mountain = reg.interner_mut().intern("Mountain");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mountain);
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::LAND | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            }),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}
