//! Tainted Monkey — `{1}{B}` 1/1 black Monkey. "{T}: Choose a word. Target
//! player mills a card. If a card with the chosen word in its text box was
//! milled this way, they lose 3 life."
//!
//! GAP: "Choose a word" and "if a card with the chosen word in its text box
//! was milled" — word choice and text inspection of milled cards not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tainted Monkey");
    let monkey = reg.interner_mut().intern("Monkey");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monkey);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose a word. Target player mills a card. If a card with the chosen word in its text box was milled this way, they lose 3 life.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_and_maybe_drain,
            }),
    )
}

fn mill_and_maybe_drain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "choose a word" + "if milled card has that word" not expressible.
    vec![Effect::Mill { player: *p, count: 1 }]
}
