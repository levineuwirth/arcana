//! Djinn of Wishes — `{3}{U}{U}` 4/4 Djinn with Flying.
//! "This creature enters with three wish counters on it."
//! "{2}{U}{U}, Remove a wish counter from this creature: Reveal the top card
//!  of your library. You may play that card without paying its mana cost. If
//!  you don't, exile it."
//!
//! Flying is expressible. The enters-with-counters clause is a static and is
//! GAP'd. The activation COST (mana + remove a wish counter) is expressible;
//! the effect (reveal-top, free-cast or exile) has no faithful Effect form
//! here — ImpulseExile plays at normal cost, not "without paying mana cost",
//! so the effect body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Djinn of Wishes");
    let djinn = reg.interner_mut().intern("Djinn");
    let wish = reg.interner_mut().intern("wish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "enters with three wish counters on it" (no enters-with
    // primitive in this API surface).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{U}, Remove a wish counter from this creature: Reveal the top card of your library. You may play that card without paying its mana cost. If you don't, exile it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{U}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::Named(wish), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_and_free_cast,
            }),
    )
}

fn reveal_and_free_cast(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card, you may play it without paying its mana cost,
    // else exile it" has no faithful Effect (ImpulseExile plays at full cost).
    Vec::new()
}
