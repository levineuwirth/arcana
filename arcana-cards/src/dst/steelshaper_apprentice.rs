//! Steelshaper Apprentice — `{2}{W}{W}` 1/3 white Human Soldier.
//! "{W}, {T}, Return this creature to its owner's hand: Search your library
//! for an Equipment card, reveal that card, put it into your hand, then
//! shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Steelshaper Apprentice");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}, Return this creature to its owner's hand: Search your library for an Equipment card, reveal that card, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_equipment,
            }),
    )
}

fn tutor_equipment(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let equip_filter = arcana_core::script::subtype_filter(reg, "Equipment");
    // GAP: "return this creature to hand" as part of cost is not in
    // ActivationCost; the bounce is omitted.
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: equip_filter,
        reveal: true,
    }]
}
