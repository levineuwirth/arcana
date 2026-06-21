//! Shadowheart, Dark Justiciar — `{3}{B}` 3/4 legendary black Human Elf
//! Cleric.
//!
//! Oracle:
//! * {1}{B}, {T}, Sacrifice another creature: Draw X cards, where X is
//!   that creature's power.
//! * Choose a Background. (GAP'd — Commander-variant keyword.)
//!
//! The activated ability's COST is faithful — `{1}{B}` + tap + sacrifice
//! another creature (`sacrifice_other`). Its EFFECT is GAP'd: X equals
//! the sacrificed creature's power, but by resolution the creature has
//! left the battlefield and ActivationContext exposes no handle to it, so
//! the dynamic draw count cannot be computed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadowheart, Dark Justiciar");
    let human = reg.interner_mut().intern("Human");
    let elf = reg.interner_mut().intern("Elf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(elf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Choose a Background" — Commander-variant keyword, no primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}, {T}, Sacrifice another creature: Draw X cards, where X is that creature's power.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                tap: true,
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_for_sacrificed_power,
        }),
    )
}

fn draw_for_sacrificed_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Draw X cards, where X is that creature's power." The sacrificed
    // creature is gone by resolution and ActivationContext exposes no
    // handle to it, so X cannot be computed.
    Vec::new()
}
