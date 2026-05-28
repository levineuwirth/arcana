//! Slagdrill Scrapper — `{R}` 1/2 red Artifact Creature — Robot Scout.
//! "{2}, {T}, Sacrifice another artifact or land: Draw a card."
//!
//! GAP: "Sacrifice another artifact or land" — the ActivationCost sacrifice
//! field sacrifices THIS creature, not another permanent of a chosen type.
//! The sorcery-speed draw is modeled without the correct sacrifice target.

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
    let name = reg.interner_mut().intern("Slagdrill Scrapper");
    let robot = reg.interner_mut().intern("Robot");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice another artifact or land: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_other_draw,
            }),
    )
}

fn sac_other_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationCost cannot sacrifice a specific OTHER permanent;
    // the sacrifice cost is modeled as mana+tap only above.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
