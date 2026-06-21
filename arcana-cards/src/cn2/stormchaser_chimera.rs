//! Stormchaser Chimera — `{2}{U}{R}` 2/3 Chimera with Flying.
//! {2}{U}{R}: Scry 1, then reveal the top card of your library. This creature
//! gets +X/+0 until end of turn, where X is that card's mana value.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormchaser Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}{R}: Scry 1, then reveal the top card of your library. This creature gets +X/+0 until end of turn, where X is that card's mana value.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: scry_and_pump,
        }),
    )
}

fn scry_and_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card; this creature gets +X/+0 where X is that card's
    // mana value" — no reveal-and-read-mv primitive feeds a dynamic pump. Only the
    // Scry 1 portion is expressible.
    vec![Effect::Scry {
        player: ctx.controller,
        count: 1,
    }]
}
