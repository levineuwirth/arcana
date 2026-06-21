//! Survivor of the Unseen — `{2}{U}` 2/1 Human Wizard.
//!
//! Oracle:
//! * Cumulative upkeep `{2}`.
//! * `{T}: Draw two cards, then put a card from your hand on top of your
//!   library.`
//!
//! Cumulative upkeep is not in the usable `KeywordAbility` surface, so it is
//! GAP'd. The `{T}` ability draws two cards; the "then put a card from your
//! hand on top of your library" tail has no primitive (no hand→top-of-library
//! mover), so it is GAP'd and only the draw is emitted.

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
    let name = reg.interner_mut().intern("Survivor of the Unseen");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: keyword — Cumulative upkeep {2} is not in the usable KeywordAbility set.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw two cards, then put a card from your hand on top of your library.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two,
            }),
    )
}

fn draw_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put a card from your hand on top of your library" — no
    // hand→top-of-library mover primitive; only the draw is emitted.
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}
