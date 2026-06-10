//! Sol Talisman — artifact with no mana cost.
//! "Suspend 3—{1}" and "{T}: Add {C}{C}."
//! Suspend is not in the usable keyword surface — GAP'd. The mana ability is
//! wired faithfully ({T}: Add {C}{C} — one ability, two colorless units).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sol Talisman");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        // GAP: "Suspend 3—{1}" — Suspend is not an expressible keyword
        // (not in the usable KeywordAbility surface); the suspend cast path
        // is unmodeled, so this card is castable only via effects.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add {C}{C}.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_two_colorless,
        }),
    )
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
        ],
    }]
}
