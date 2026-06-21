//! Dune Chanter — `{2}{G}` 2/3 green Plant Druid.
//!
//! Reach.
//! GAP (static): "Lands you control and land cards you own that aren't on the
//!   battlefield are Deserts in addition to their other types." — not a
//!   triggered/activated ability.
//! GAP (static): "Lands you control have '{T}: Add one mana of any color.'" —
//!   grant-ability-to-other-permanents static, not expressible here.
//! {T}: Mill two cards. (GAP: "You gain 1 life for each land card milled this
//!   way." — the count of land cards milled is not observable post-mill.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dune Chanter");
    let plant = reg.interner_mut().intern("Plant");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Mill two cards. You gain 1 life for each land card milled this way.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: mill_two,
        }),
    )
}

fn mill_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "gain 1 life for each land card milled this way" — milled-card types
    // not observable; emit the mill only.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 2,
    }]
}
