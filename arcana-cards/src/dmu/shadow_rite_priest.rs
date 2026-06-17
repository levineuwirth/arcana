//! Shadow-Rite Priest — `{1}{B}` 2/2 Creature — Human Cleric.
//! Other Clerics you control get +1/+1. (GAP: static anthem.)
//! {3}{B}{B}, {T}, Sacrifice another Cleric: Search your library for a black
//!   creature card, put it onto the battlefield, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadow-Rite Priest");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // sacrifice-cost filter: another Cleric (the source is auto-excluded).
    let sac_cleric = script_subtype(reg, "Cleric");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Other Clerics you control get +1/+1" is a static anthem (no
    // trigger/cost), not expressible here.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{B}{B}, {T}, Sacrifice another Cleric: Search your library for a black creature card, put it onto the battlefield, then shuffle.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                tap: true,
                sacrifice_other: Some(sac_cleric),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_black_creature,
        }),
    )
}

fn script_subtype(reg: &mut CardRegistry, name: &str) -> ObjectFilter {
    arcana_core::script::subtype_filter(reg, name)
}

fn tutor_black_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_colors(ColorSet::black()),
        tapped: false,
    }]
}
