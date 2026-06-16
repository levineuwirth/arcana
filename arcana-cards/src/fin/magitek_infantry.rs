//! Magitek Infantry — `{W}` 1/1 white Artifact Creature — Robot Soldier.
//! This creature gets +1/+0 as long as you control another artifact. (static — GAP)
//! {2}{W}: Search your library for a card named Magitek Infantry, put it onto
//! the battlefield tapped, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magitek Infantry");
    let robot = reg.interner_mut().intern("Robot");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "This creature gets +1/+0 as long as you control another artifact."
    // — a conditional static continuous P/T buff, no triggered/activated form.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Search your library for a card named Magitek Infantry, put it onto the battlefield tapped, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_self,
            }),
    )
}

/// Search the library for a card named "Magitek Infantry" and put it onto the
/// battlefield tapped.
fn tutor_self(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Magitek Infantry");
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        tapped: true,
    }]
}
