//! Brass Infiniscope — `{4}` artifact (Final Fantasy, 2025).
//! "{T}: Add {C}{C}. When you next cast a spell with {X} in its mana
//! cost this turn, you draw a card and gain half X life, rounded down."
//! The two-colorless mana ability is wired; the delayed
//! next-X-spell-cast rider (draw + gain half X life) has no available
//! primitive in this API surface and is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brass Infiniscope");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}{C}. When you next cast a spell with {X} \
                       in its mana cost this turn, you draw a card and gain \
                       half X life, rounded down."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_colorless,
            },
        ),
    )
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you next cast a spell with {X} in its mana cost this turn,
    // you draw a card and gain half X life, rounded down" — a delayed
    // next-cast rider keyed on an X mana cost has no Effect variant here.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
        ],
    }]
}
