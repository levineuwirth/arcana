//! Knacksaw Clique — `{3}{U}` 1/4 Faerie Rogue with Flying.
//! "{1}{U}, {Q}: Target opponent exiles the top card of their library. Until
//!  end of turn, you may play that card."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knacksaw Clique");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}{U}, {Q}: Target opponent exiles the top card of their
            // library. Until end of turn, you may play that card."
            // GAP cost: the {Q} (untap-symbol) cost component is not an
            // ActivationCost field — only the {1}{U} mana portion is modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, {Q}: Target opponent exiles the top card of their library. Until end of turn, you may play that card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: opponent_impulse,
            }),
    )
}

fn opponent_impulse(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Target opponent exiles the top card of their library. Until end of
    // turn, you may play that card." — ImpulseExile only exiles from YOUR
    // library with play-permission for YOU; there is no primitive for exiling
    // an opponent's library top and granting YOU permission to play it.
    Vec::new()
}
